# cobol2rust Rustification Roadmap

Phase 2 code quality improvement -- transforming COBOL-shaped Rust into idiomatic Rust.

## Context

The Phase 1 transpiler produces correct, compilable Rust that preserves COBOL semantics
exactly. The output is structurally "COBOL inside Rust" -- it uses COBOL-native types
(`PackedDecimal`, `PicX`, `CompBinary`), a flat `WorkingStorage` struct, and a paragraph
dispatcher loop. This is the standard approach used by all major COBOL migration tools
(Micro Focus, AWS Blu Age, etc.).

Phase 2 transforms this output into progressively more idiomatic Rust while preserving
behavioral equivalence.

## Tiers

### Tier 1: Cosmetic Cleanup (Low Risk, Fully Automatable)

**Effort**: 2-3 sessions | **Code noise reduction**: ~30-40%

These transformations are mechanical pattern replacements with no semantic risk.

#### 1.1 Literal Constant Extraction

Before:
```rust
if ws.ws_a.to_decimal() > "25".parse::<Decimal>().unwrap() {
```

After:
```rust
const MAX_THRESHOLD: Decimal = dec!(25);
if ws.ws_a.to_decimal() > MAX_THRESHOLD {
```

- Pattern: every `"N".parse::<Decimal>().unwrap()` becomes a `const` or `dec!()` macro
- Implementation: codegen pass that collects literals, deduplicates, emits constants
- Risk: none -- identical runtime behavior

#### 1.2 Display Trait Simplification

Before:
```rust
print!("{}", String::from_utf8_lossy(&ws.ws_result.display_bytes()));
println!();
```

After:
```rust
println!("{}", ws.ws_result);
```

- Implementation: add `impl Display` to `PicX`, `PackedDecimal`, etc.
- Replace the verbose call pattern in codegen
- Risk: none if Display impl matches `display_bytes()` output

#### 1.3 Dead Field Elimination

- Static analysis on generated Rust: find `WorkingStorage` fields that are never read
- Remove write-only fields (common in COBOL: temp vars, FILLER)
- Risk: low -- if a field is never read, removing it can't change behavior

#### 1.4 Unused Import/Allow Cleanup

- Remove blanket `#![allow(unused_imports, unused_variables, non_snake_case)]`
- Replace with targeted per-item allows only where needed
- Risk: none (compile-time only)

### Tier 2: Type Promotion (Medium Risk, Partially Automatable)

**Effort**: significant engineering | **Code readability improvement**: major

These require per-field safety analysis to prove the transformation is semantics-preserving.

#### 2.1 PicX -> String

Safe when ALL of the following hold:
- No reference modification (`field(start:length)` subscripting)
- No REDEFINES overlay on or by this field
- No group-level MOVE that treats the field as raw bytes
- No byte-level comparison (`as_bytes() == ...`) that depends on space-padding

Estimated coverage: 60-70% of PicX fields qualify.

Before:
```rust
pub ws_status: PicX /* 2 */,
// ...
if ws.ws_status.as_bytes() == "OK".as_bytes() {
```

After:
```rust
pub ws_status: String,
// ...
if ws.ws_status == "OK" {
```

**Caution**: COBOL strings are fixed-width, space-padded, left-justified. A `String`
conversion must handle:
- Trailing space trimming on comparison
- Truncation on assignment (COBOL silently truncates; Rust `String` grows)
- SPACES figurative constant -> `String::new()` or `" ".repeat(n)`

#### 2.2 PackedDecimal -> Native Numeric

Two promotion paths:

**PackedDecimal -> i64** (safe when):
- Scale is 0 (no fractional digits)
- No overflow-dependent behavior (COBOL silently left-truncates: 12345 -> PIC 9(3) = 345)
- No REDEFINES over the field
- Precision fits in i64 (up to 18 digits)

**PackedDecimal -> Decimal** (safe when):
- No REDEFINES over the field
- Program doesn't depend on PIC-specific truncation behavior

Estimated coverage: 60-70% of numeric fields.

Before:
```rust
pub ws_count: PackedDecimal /* Display P7 S0 unsigned */,
// ...
ws.ws_count = { let mut _p = PackedDecimal::new(7, 0, false);
    _p.pack("0".parse::<Decimal>().unwrap()); _p };
```

After:
```rust
pub ws_count: i64,
// ...
ws.ws_count = 0;
```

#### 2.3 Local Variable Extraction

Move temp fields out of WorkingStorage into function-local variables.

Safe when:
- Field is only used within a single paragraph
- Not passed across PERFORM calls
- Not part of a REDEFINES group
- Not referenced by CALL BY REFERENCE

Before:
```rust
pub struct WorkingStorage {
    pub ws_temp_result: PackedDecimal,  // only used in CALC-PARA
    // ... 50 other fields
}
fn calc_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    cobol_compute(expr, &mut ws.ws_temp_result, None, &ctx.config);
}
```

After:
```rust
fn calc_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    let mut temp_result: i64 = 0;
    temp_result = expr;
}
```

### Tier 3: Structural Transformation (High Risk, Requires Semantic Understanding)

**Effort**: per-program, AI-assisted | **Result**: truly idiomatic Rust

These transformations require understanding the business logic, not just the syntax.

#### 3.1 Paragraph Dispatcher Elimination

The `run()` function:
```rust
pub fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    let mut _pc: usize = 0;
    loop {
        match _pc {
            0 => main_para(ws, ctx),
            1 => process_para(ws, ctx),
            2 => cleanup_para(ws, ctx),
            _ => break,
        }
        if ctx.stopped || ctx.exit_program { break; }
        if let Some(target) = ctx.goto_target.take() { ... }
        _pc += 1;
    }
}
```

Can be eliminated when:
- **No GO TO** in the program -> direct sequential calls
- **No PERFORM THRU** ranges -> each paragraph is independent
- **Simple PERFORM** only -> paragraph becomes a function call

Feasibility by program complexity:
- Linear programs (no GO TO, no THRU): ~40% of programs -> straightforward
- GO TO with simple patterns (GO TO end-of-program): ~20% -> case-by-case
- Complex GO TO / ALTER / PERFORM THRU: ~40% -> keep dispatcher

#### 3.2 WorkingStorage Decomposition

Break the flat struct into domain-specific structs:

```rust
// Before: one flat struct with 80 fields
pub struct WorkingStorage { ... }

// After: domain structs
pub struct Employee { id: i64, name: String, department: String }
pub struct PayCalculation { gross: Decimal, tax: Decimal, net: Decimal }
pub struct ReportCounters { records_read: i64, records_written: i64 }
```

Challenges:
- COBOL group items give structural hints but are often organizational, not semantic
- REDEFINES creates overlapping memory -- fields that share storage can't be separated
- GROUP-level MOVEs treat the struct as a flat byte buffer

This is fundamentally a domain modeling task, not a code transformation.

#### 3.3 Status Flags -> Result/Enum

```rust
// Before
pub ws_eof_flag: PackedDecimal,  // 88 WS-EOF VALUE 1
if ws.ws_eof_flag.to_decimal() == dec!(1) { ... }

// After
enum FileStatus { Reading, Eof }
match file_status {
    FileStatus::Eof => { ... }
}
```

Level-88 conditions provide the value names, but the control flow pattern determines
whether `Result`, `enum`, or `bool` is the right Rust type. This requires reading and
understanding the business logic.

#### 3.4 File I/O Modernization

Replace `SequentialFile` with idiomatic Rust I/O:
- `BufReader<File>` for input
- `BufWriter<File>` for output
- Serde for record serialization

Challenges:
- COBOL FILE STATUS checking is implicit and pervasive
- Fixed-width record I/O has no clean Rust equivalent
- SORT/MERGE with INPUT/OUTPUT PROCEDURE is deeply COBOL-specific
- Variable-length records, LINAGE, and other COBOL file features

## Implementation Strategy

### Recommended Approach

```
Phase 1 (current):  COBOL -> COBOL-in-Rust (semantic preservation)
                    |
                    v
Tier 1:             --rustify flag on codegen (cosmetic cleanup)
                    |
                    v
Tier 2:             cobol2rust refactor command (per-field safety analysis)
                    |
                    v
Tier 3:             AI-assisted per-program refactoring (human oversight)
```

### Tier 1 as Codegen Flags

Add `--rustify` flag to `cobol2rust transpile`:
- Emits `const` declarations for numeric literals
- Uses `Display` trait instead of `display_bytes()` pattern
- Strips dead fields from WorkingStorage
- No semantic risk, can be enabled by default

### Tier 2 as Analysis + Transform

Add `cobol2rust refactor` subcommand:
- Analyzes generated Rust for safe type promotions
- Produces a refactoring plan (which fields can be promoted, which can't and why)
- Applies transformations with `--apply` flag
- Generates a safety report showing what changed and what was preserved

### Tier 3 as AI-Assisted Workflow

Not a codegen feature -- this is a workflow:
1. Human/AI reads the COBOL program and understands the business logic
2. Designs idiomatic Rust data model
3. Rewrites using the Phase 1 output as a behavioral reference
4. Validates equivalence via test cases or property-based testing

## Risk Matrix

| Transformation          | Coverage | Automatable  | Semantic Risk | Effort   |
|------------------------|----------|-------------|---------------|----------|
| Literal constants       | 15-20%   | Yes          | None          | Low      |
| Display simplification  | 5-10%    | Yes          | None          | Low      |
| Dead field removal       | 5-10%    | Yes          | None          | Low      |
| PicX -> String          | 20-30%   | Partially    | Medium        | Medium   |
| Decimal -> native       | 20-30%   | Partially    | Medium        | Medium   |
| Local var extraction     | 10-15%   | Partially    | Low           | Medium   |
| Dispatcher elimination   | Structural | Case-by-case | High          | High     |
| Struct decomposition     | Structural | No           | High          | High     |
| Status -> Result/enum   | 5-10%    | No           | High          | High     |
| File I/O modernization  | 5-10%    | No           | High          | High     |

## Key Principle

**Behavioral equivalence is the invariant.** Every transformation must produce output
identical to the Phase 1 transpilation for all valid inputs. The stress test suite
(47 tests) serves as the regression baseline. Any Tier 2/3 transformation that
changes observable behavior is a bug, not an improvement.

## Dependencies

- Phase 1 transpiler must be stable (current: 46/47 stress tests passing)
- Corpus scan results inform which transformations have the highest ROI
- `cobol2rust report --type transpile` provides coverage/verb data for prioritization
