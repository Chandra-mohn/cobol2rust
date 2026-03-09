# Sessions 33-38: Completing the cobol2rust Transpiler

## Overview

After 32 sessions (688 tests), the core runtime and transpiler are production-quality
for common COBOL batch programs. These 6 sessions close the remaining gaps identified
in the post-Session-32 audit. Items A-G from the gap analysis are organized by
dependency order and grouped to maximize per-session coherence.

| Session | Items | Scope | Est. Tests |
|---------|-------|-------|------------|
| 33 | A + D | Parser gaps (SET, START, EXIT) + GO TO DEPENDING ON | ~25 |
| 34 | C | RENAMES (level 66) | ~15 |
| 35 | G | AlphanumericEdited type | ~20 |
| 36 | B | COMP-1/COMP-2 float wrapper types | ~25 |
| 37 | E | Real-world COBOL test suite | ~15 |
| 38 | F | CI/CD pipeline + rustdoc documentation | ~0 (infra) |

**Running total after Session 38**: ~788 tests (688 + ~100 new)

---

## Session 33: Parser Gaps + GO TO DEPENDING ON

**Items**: A (SET, START, EXIT PARAGRAPH/SECTION extraction) + D (GO TO DEPENDING ON)

**Rationale**: All four are transpiler-only changes (parser extraction + codegen).
No new runtime types needed. Grouped for maximum session coherence.

### 33.1 SET Statement Parser Extraction

**Problem**: `generate_set()` exists in proc_gen.rs (lines 1126-1205) and handles all
4 SetAction variants (To, UpBy, DownBy, ToBool), but `extract_statement()` in
proc_listener.rs never dispatches to a SET extractor. The codegen is unreachable.

**Files**:
- `crates/cobol-transpiler/src/parser/proc_listener.rs` -- add extraction

**Implementation**:

```
extract_statement():
    if let Some(c) = ctx.setStatement() {
        return Some(extract_set(&*c));
    }

fn extract_set(ctx: &SetStatementContext) -> Statement:
    if let Some(to_ctx) = ctx.setToStatement():
        // Parse targets from setTo() children
        // Parse value from setToValue()
        // Detect ON/OFF -> SetAction::ToBool(true/false)
        // Detect identifier/literal -> SetAction::To(operand)
    else if let Some(updown_ctx) = ctx.setUpDownByStatement():
        // Parse targets
        // Detect UP BY vs DOWN BY keyword
        // Parse setByValue -> operand
        // Return SetAction::UpBy or SetAction::DownBy
```

**ANTLR grammar rules** (Cobol85.g4):
- `setStatement -> SET (setToStatement+ | setUpDownByStatement)`
- `setToStatement -> setTo+ TO setToValue+`
- `setUpDownByStatement -> setTo+ (UP BY | DOWN BY) setByValue`

**Tests** (4):
1. `extract_set_to_literal` -- SET WS-X TO 5
2. `extract_set_to_true` -- SET COND-NAME TO TRUE
3. `extract_set_up_by` -- SET WS-IDX UP BY 1
4. `extract_set_down_by` -- SET WS-IDX DOWN BY 2

### 33.2 START Statement (Parser + Codegen)

**Problem**: No `extract_start()` and no `generate_start()` exist. StartStatement AST
is fully defined with file_name, key_condition, invalid_key, not_invalid_key.

**Files**:
- `crates/cobol-transpiler/src/parser/proc_listener.rs` -- add extraction
- `crates/cobol-transpiler/src/codegen/proc_gen.rs` -- add generation
- `crates/cobol-transpiler/src/codegen/proc_gen.rs` -- wire into generate_statement

**Parser extraction**:

```
fn extract_start(ctx: &StartStatementContext) -> Statement:
    file_name = ctx.fileName().get_text().to_uppercase()
    key_condition = ctx.startKey().map(|sk| {
        // Map grammar operators to ComparisonOp:
        //   EQUAL TO | EQUALCHAR            -> Equal
        //   GREATER THAN | MORETHANCHAR     -> GreaterThan
        //   NOT LESS THAN | NOT LESSTHANCHAR -> GreaterOrEqual
        //   GREATER OR EQUAL | MORETHANOREQUAL -> GreaterOrEqual
        key = extract_data_ref_from_qualified(sk.qualifiedDataName())
        StartKeyCondition { key, op }
    })
    invalid_key = extract_invalid_key_stmts(ctx.invalidKeyPhrase())
    not_invalid_key = extract_not_invalid_key_stmts(ctx.notInvalidKeyPhrase())
```

**Codegen**:

```
fn generate_start(w, start, cmap, ptable):
    let fname = cobol_to_rust_name(&start.file_name, "")
    let start_call = if let Some(ref cond) = start.key_condition:
        let key_expr = data_ref_expr(&cond.key)
        let op_str = match cond.op:
            Equal -> "std::cmp::Ordering::Equal"
            GreaterThan -> "std::cmp::Ordering::Greater"
            GreaterOrEqual -> "std::cmp::Ordering::Greater" // or Equal
        format!("ws.{fname}.start({key_expr}.as_bytes(), {op_str})")
    else:
        format!("ws.{fname}.start_first()")

    // Wrap in match if INVALID KEY / NOT INVALID KEY handlers present
    // (same pattern as generate_read/generate_delete)
```

**Note**: The `CobolFile` trait in cobol-io needs a `start()` method. Check if
IndexedFile and RelativeFile already have it; if not, add stubs that return
FileStatusCode.

**Tests** (4):
1. `extract_start_simple` -- START FILE-A
2. `extract_start_with_key` -- START FILE-A KEY >= WS-KEY
3. `generate_start_simple` -- codegen for basic START
4. `generate_start_with_invalid_key` -- START with INVALID KEY handler

### 33.3 EXIT PARAGRAPH / EXIT SECTION Extraction

**Problem**: `extract_statement()` always returns `Statement::ExitProgram` for any
EXIT statement. The ANTLR grammar has `EXIT PROGRAM?` (PROGRAM is optional).
EXIT PARAGRAPH and EXIT SECTION are not in the current grammar.

**Approach**: Since the grammar only has `EXIT PROGRAM?`, use text-based detection
in the preprocessor or listener:
- If the EXIT statement text contains "PROGRAM" -> ExitProgram
- If text contains "PARAGRAPH" -> ExitParagraph
- If text contains "SECTION" -> ExitSection
- Plain "EXIT" (no qualifier) -> ExitParagraph (standard behavior)

**Files**:
- `crates/cobol-transpiler/src/parser/proc_listener.rs` -- fix exit extraction

**Implementation**:

```
if ctx.exitStatement().is_some():
    let text = ctx.exitStatement().unwrap().get_text().to_uppercase()
    if text.contains("PROGRAM"):
        return Some(Statement::ExitProgram)
    else if text.contains("SECTION"):
        return Some(Statement::ExitSection)
    else if text.contains("PARAGRAPH"):
        return Some(Statement::ExitParagraph)
    else:
        // Plain EXIT -> same as EXIT PARAGRAPH
        return Some(Statement::ExitParagraph)
```

**Codegen**: Already handled in generate_statement (ExitParagraph and ExitSection
both generate `return;`).

**Tests** (2):
1. `extract_exit_paragraph` -- EXIT PARAGRAPH
2. `extract_exit_section` -- EXIT SECTION

### 33.4 GO TO DEPENDING ON

**Problem**: GoToStatement.depending field exists in AST but parser always sets it
to `None`. Codegen ignores the `depending` field entirely.

**Parser fix** (proc_listener.rs, extract_goto):

```
// In the goToDependingOnStatement branch:
let depending = dep.identifier().map(|id| extract_data_ref_from_identifier(&*id));
// Use: depending: depending, (instead of depending: None)
```

**Also fix target extraction**: Current code does text.split_whitespace() which is
fragile. Use the ANTLR procedureName() children instead:

```
let targets: Vec<String> = dep.procedureName_all()
    .iter()
    .map(|pn| pn.get_text().trim().to_uppercase())
    .collect();
```

**Codegen** (proc_gen.rs, generate_goto):

```
fn generate_goto(w, g):
    if let Some(ref dep_ref) = g.depending:
        // GO TO DEPENDING ON: index-based target selection
        let dep_expr = data_ref_expr(dep_ref)
        w.open_block("{")
        w.line(&format!("let _goto_idx = {dep_expr}.to_decimal().to_i64().unwrap_or(0) as usize;"))
        // Build match on index (1-based COBOL indexing)
        w.open_block("match _goto_idx {")
        for (i, target) in g.targets.iter().enumerate():
            let upper = target.to_uppercase()
            w.line(&format!("{} => ctx.goto_target = Some(\"{upper}\".to_string()),", i + 1))
        w.line("_ => {}")  // out of range: no transfer (COBOL standard)
        w.close_block("}")
        w.line("if ctx.goto_target.is_some() { return; }")
        w.close_block("}")
    else:
        // Simple GO TO (existing code)
        if let Some(target) = g.targets.first():
            w.line(&format!("ctx.goto_target = Some(\"{}\".to_string());", target.to_uppercase()))
            w.line("return;")
```

**Tests** (4):
1. `extract_goto_depending` -- parser extracts depending field
2. `generate_goto_depending` -- codegen generates match on index
3. `e2e_goto_depending_on` -- full transpile test
4. `e2e_goto_depending_out_of_range` -- index out of range does nothing

### 33.5 Wire Everything Together

**generate_statement dispatch**: Add `Statement::Start(s) => generate_start(w, s, cmap, ptable),`

**extract_statement dispatch**: Add SET, START dispatches

**Total new tests**: ~14 extraction + codegen + 4 E2E = ~18
**Running total**: 688 + 18 = ~706

---

## Session 34: RENAMES (Level 66)

**Item**: C

**Rationale**: RENAMES is a self-contained data division feature. It needs parser
extraction, AST enhancement, symbol table integration, and codegen -- a full
vertical slice but small in scope.

### 34.1 AST Enhancement

**File**: `crates/cobol-transpiler/src/ast.rs`

Add fields to DataEntry:

```rust
/// For level 66 RENAMES: the target field name.
pub renames_target: Option<String>,
/// For level 66 RENAMES THRU: the range end field name.
pub renames_thru: Option<String>,
```

### 34.2 Data Listener Extraction

**File**: `crates/cobol-transpiler/src/parser/data_listener.rs`

Add handler for Format2 entries:

```
fn enter_dataDescriptionEntryFormat2(&mut self, ctx: ...):
    let name = ctx.dataName().get_text().to_uppercase()
    let renames_clause = ctx.dataRenamesClause()
    let target = first qualifiedDataName text
    let thru = if THROUGH/THRU keyword present, second qualifiedDataName text

    Create DataEntry {
        level: 66,
        name,
        renames_target: Some(target),
        renames_thru: thru,
        pic: None,
        usage: None,
        ...defaults...
    }
    self.entries.push(entry)
```

### 34.3 Symbol Table Integration

**File**: `crates/cobol-transpiler/src/symbol_table.rs`

When adding a level-66 entry to the symbol table:
- Resolve the target name to its SymbolEntry
- If single target: create an alias (same RustType, same offset)
- If range (THRU): create a PicX spanning the byte range from target to thru-end

```
For RENAMES X THRU Y:
    start_offset = symbol_table.lookup(X).offset
    end_offset = symbol_table.lookup(Y).offset + symbol_table.lookup(Y).size
    total_size = end_offset - start_offset
    RustType = PicX { length: total_size }
```

### 34.4 Codegen

**File**: `crates/cobol-transpiler/src/codegen/data_gen.rs`

For single RENAMES: generate a method or type alias that references the target field.

```rust
// RENAMES WS-FIELD -> type alias approach
// fn ws_renamed(&self) -> &PicX<10> { &self.ws_field }
// fn ws_renamed_mut(&mut self) -> &mut PicX<10> { &self.ws_field }
```

For range RENAMES (THRU): generate a computed byte-slice accessor.

```rust
// RENAMES WS-A THRU WS-C -> byte slice over the range
// fn ws_range(&self) -> PicX<30> { ... copy bytes from offset to end ... }
```

**Simpler alternative**: Generate the RENAMES field as an additional field in
WorkingStorage that is initialized from the target at construction time, with
a comment noting the RENAMES relationship. This avoids lifetime complexity but
means RENAMES fields are copies, not aliases. Acceptable for Phase 4.

### 34.5 Tests (~15)

**Extraction tests** (3):
1. `extract_renames_single` -- 66 WS-ALIAS RENAMES WS-FIELD
2. `extract_renames_thru` -- 66 WS-RANGE RENAMES WS-A THRU WS-C
3. `extract_renames_hierarchy` -- level 66 attaches to 01 record

**Symbol table tests** (3):
4. `renames_single_alias_type` -- resolves to same type as target
5. `renames_thru_computes_range` -- PicX with correct total length
6. `renames_unknown_target_error` -- graceful handling of missing target

**Codegen tests** (3):
7. `generate_renames_single_field` -- accessor method generated
8. `generate_renames_thru_field` -- byte range field generated
9. `generate_renames_skips_storage` -- no duplicate storage allocation

**E2E tests** (3):
10. `e2e_renames_simple` -- full transpile with RENAMES
11. `e2e_renames_thru` -- full transpile with RENAMES THRU
12. `e2e_renames_in_move` -- MOVE to/from RENAMES field

**Running total**: ~706 + 12 = ~718

---

## Session 35: AlphanumericEdited Type

**Item**: G

**Rationale**: AlphanumericEdited is a self-contained new type following the
NumericEdited pattern. Needs a runtime type with editing engine, trait impls,
codegen updates, and prelude export.

### 35.1 Runtime Type

**New file**: `crates/cobol-types/src/alpha_edited.rs`

```rust
pub struct AlphanumericEdited {
    /// Raw display bytes (edited form).
    data: Vec<u8>,
    /// Edit symbol pattern (derived from PIC clause).
    edit_pattern: Vec<AlphaEditSymbol>,
    /// Total display length.
    display_length: usize,
}

enum AlphaEditSymbol {
    /// X or A -- data position (pass through from source)
    Data,
    /// B -- insert space (0x20)
    Space,
    /// 0 -- insert zero (0x30)
    Zero,
    /// / -- insert slash (0x2F)
    Slash,
}
```

**Alphanumeric editing** is much simpler than numeric editing:
- Only 3 insertion symbols: B (space), 0 (zero), / (slash)
- No suppression, no floating symbols, no sign handling
- Source data characters fill the Data positions left-to-right
- Insertion symbols are placed at their fixed positions

**Trait implementations**:
- `CobolField`: category = AlphanumericEdited, as_bytes returns edited form
- `Display`: show edited form
- `set_from_bytes()`: de-edit (strip insertion chars) then re-edit
- MOVE behavior: source is de-edited alphanumeric; dest receives edited form

### 35.2 PIC Parser Enhancement

**File**: `crates/cobol-transpiler/src/parser/pic_parser.rs`

The PIC parser already detects `AlphanumericEdited` category. Need to also
extract the edit symbol pattern so it can be passed to the runtime type.

Add to PicClause:
```rust
pub alpha_edit_symbols: Vec<AlphaEditSymbolEntry>,
```

Where each entry records symbol type + position.

### 35.3 Symbol Table + Codegen

**File**: `crates/cobol-transpiler/src/symbol_table.rs`

Add new RustType variant:
```rust
AlphanumericEdited { length: u32 },
```

Map `PicCategory::AlphanumericEdited` to this new variant instead of PicX.

**File**: `crates/cobol-transpiler/src/codegen/data_gen.rs`

Generate `AlphanumericEdited::new(length, &edit_pattern)` instead of PicX.

### 35.4 Prelude Export

**File**: `crates/cobol-runtime/src/prelude.rs`

Add `AlphanumericEdited` to re-exports.

### 35.5 Tests (~20)

**Runtime type tests** (10):
1. `alpha_edited_new` -- construct from pattern
2. `alpha_edited_edit_space` -- PIC X(3)BX(3) inserts space
3. `alpha_edited_edit_zero` -- PIC X(3)0X(3) inserts zero
4. `alpha_edited_edit_slash` -- PIC X(2)/X(2)/X(4) date format
5. `alpha_edited_display` -- display_bytes returns edited form
6. `alpha_edited_as_bytes` -- raw bytes include insertions
7. `alpha_edited_fill` -- fill_bytes sets all positions
8. `alpha_edited_de_edit` -- strip insertions, return raw data
9. `alpha_edited_set_from_short` -- source shorter than data positions: space-pad
10. `alpha_edited_set_from_long` -- source longer than data positions: truncate

**Codegen tests** (4):
11. `generate_alpha_edited_field` -- correct type generated
12. `generate_alpha_edited_init` -- correct initializer
13. `pic_parser_alpha_edited_pattern` -- PIC analysis extracts edit symbols
14. `pic_parser_alpha_edited_mixed` -- PIC X(5)BB detects correctly

**E2E tests** (3):
15. `e2e_alpha_edited_display` -- DISPLAY of edited field
16. `e2e_alpha_edited_move` -- MOVE alphanumeric TO edited
17. `e2e_alpha_edited_initialize` -- INITIALIZE edited field

**Running total**: ~718 + 17 = ~735

---

## Session 36: COMP-1/COMP-2 Float Wrapper Types

**Item**: B

**Rationale**: Float support requires new wrapper types that implement CobolField
and CobolNumeric traits, plus codegen updates. This is the highest-effort session
but follows established patterns (PackedDecimal, CompBinary as templates).

### 36.1 New Types

**New file**: `crates/cobol-types/src/comp1_float.rs`

```rust
pub struct Comp1Float {
    value: f32,
}

impl Comp1Float {
    pub fn new() -> Self { Self { value: 0.0 } }
    pub fn from_f32(v: f32) -> Self { Self { value: v } }
    pub fn as_f32(&self) -> f32 { self.value }
}
```

**New file**: `crates/cobol-types/src/comp2_float.rs`

```rust
pub struct Comp2Float {
    value: f64,
}
```

### 36.2 Trait Implementations

**CobolField**:
- `category()` -> DataCategory::Numeric
- `byte_length()` -> 4 (Comp1) / 8 (Comp2)
- `as_bytes()` -> IEEE 754 byte representation (native endian)
- `display_bytes()` -> format as decimal string (e.g., "3.14")
- `fill_bytes()` -> fill IEEE 754 bytes
- `initialize_default()` -> set to 0.0

**CobolNumeric**:
- `to_decimal()` -> `Decimal::from_f64_retain(self.value as f64).unwrap_or(Decimal::ZERO)`
- `set_decimal(value)` -> `self.value = value.to_f64().unwrap_or(0.0) as f32/f64`
- `scale()` -> 0 (floats have no fixed scale)
- `precision()` -> 7 (Comp1) / 15 (Comp2) -- significant digits
- `is_signed()` -> true (floats are always signed)
- `compare_numeric()` -> compare via f64 intermediary

**Design decision**: Use IEEE 754 (not IBM HFP). IBM HFP support deferred.
Document this as a known limitation.

### 36.3 Move Engine Compatibility

**File**: `crates/cobol-move/src/engine.rs`

No changes needed if CobolNumeric trait impl is correct. The move engine
already dispatches through `to_decimal()` / `set_decimal()` which will
work (with precision loss that matches COBOL behavior).

### 36.4 Arithmetic Compatibility

**File**: `crates/cobol-arithmetic/src/`

No changes needed. All arithmetic functions take `&dyn CobolNumeric` and use
`to_decimal()` / `set_decimal()`. Float wrappers implement these.

### 36.5 Codegen Updates

**File**: `crates/cobol-transpiler/src/codegen/data_gen.rs`

Change type string generation:
```
RustType::Float32 -> "Comp1Float"   (was "f32")
RustType::Float64 -> "Comp2Float"   (was "f64")
```

Change initialization:
```
RustType::Float32 -> "Comp1Float::new()"   (was "0.0f32")
RustType::Float64 -> "Comp2Float::new()"   (was "0.0f64")
```

Change VALUE literals:
```
(Literal::Numeric(n), RustType::Float32) -> "Comp1Float::from_f32({n}f32)"
(Literal::Numeric(n), RustType::Float64) -> "Comp2Float::from_f64({n}f64)"
```

### 36.6 Prelude Export

**File**: `crates/cobol-runtime/src/prelude.rs`

Add `Comp1Float` and `Comp2Float` to re-exports.

### 36.7 Tests (~25)

**Comp1Float tests** (8):
1. `comp1_new` -- default is 0.0
2. `comp1_from_f32` -- construct with value
3. `comp1_as_bytes` -- 4-byte IEEE 754
4. `comp1_display_bytes` -- string representation
5. `comp1_to_decimal` -- convert to Decimal
6. `comp1_set_decimal` -- set from Decimal
7. `comp1_compare` -- numeric comparison
8. `comp1_initialize` -- default initialization

**Comp2Float tests** (8):
9-16. Same 8 tests for Comp2Float with f64

**Move tests** (3):
17. `move_comp1_to_packed` -- float to PackedDecimal
18. `move_packed_to_comp2` -- PackedDecimal to float
19. `move_comp1_to_comp2` -- float to float (precision change)

**Codegen tests** (3):
20. `generate_comp1_field` -- correct Comp1Float type
21. `generate_comp2_field` -- correct Comp2Float type
22. `generate_comp1_value` -- VALUE 3.14 -> Comp1Float::from_f32(3.14f32)

**E2E tests** (3):
23. `e2e_comp1_display` -- DISPLAY of COMP-1 field
24. `e2e_comp2_arithmetic` -- ADD/MULTIPLY with COMP-2
25. `e2e_comp1_move` -- MOVE between float and packed

**Running total**: ~735 + 25 = ~760

---

## Session 37: Real-World COBOL Test Suite

**Item**: E

**Rationale**: With all major features implemented, validate end-to-end correctness
against realistic COBOL programs. This session creates a test corpus of progressively
complex programs and verifies transpilation + expected output patterns.

### 37.1 Test Programs

Create `crates/cobol-transpiler/tests/programs/` directory with .cbl files:

**Program 1: PAYROLL.cbl** -- Classic batch payroll
- Multiple 01-level records, REDEFINES, level 88
- PERFORM VARYING, COMPUTE, numeric editing
- File I/O (sequential READ/WRITE)
- Tests: data layout, arithmetic, file operations

**Program 2: REPORT.cbl** -- Report generation
- NumericEdited fields (PIC $ZZZ,ZZ9.99)
- STRING/UNSTRING for formatting
- INSPECT for character manipulation
- Tests: editing engine, string operations

**Program 3: SEARCH.cbl** -- Table search with SORT
- OCCURS with DEPENDING ON
- SORT USING/GIVING
- Binary search pattern (PERFORM UNTIL)
- Tests: arrays, sort, loops

**Program 4: SUBPROG.cbl** -- CALL/CANCEL program
- CALL BY REFERENCE/CONTENT/VALUE
- RETURNING
- ON EXCEPTION / NOT ON EXCEPTION
- Tests: inter-program communication

**Program 5: CONTROL.cbl** -- Control flow stress test
- GO TO (forward, backward, DEPENDING ON)
- PERFORM THRU with multiple ranges
- STOP RUN, EXIT PROGRAM
- Paragraph fall-through
- Tests: dispatch loop, all control flow

**Program 6: COPYBOOK.cbl** + common.cpy -- COPY integration
- COPY REPLACING
- Nested copybooks
- Tests: preprocessor integration

### 37.2 Test Approach

Each test:
1. Reads .cbl file from test fixture directory
2. Transpiles to Rust source
3. Asserts structural correctness (function names, types, API calls)
4. For select programs: compile test (feature-gated, requires Rust toolchain)

### 37.3 Tests (~15)

1-6. One structural test per program (verify transpile succeeds, key patterns present)
7-12. One focused test per program (verify specific feature correctness)
13-15. Regression tests for previously-found edge cases

**Running total**: ~760 + 15 = ~775

---

## Session 38: CI/CD Pipeline + Documentation

**Item**: F

**Rationale**: Infrastructure session. No new runtime code. Sets up continuous
integration, generates rustdoc, and publishes workspace documentation.

### 38.1 GitHub Actions CI

**New file**: `.github/workflows/ci.yml`

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --workspace
      - run: cargo test --workspace
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo fmt --all -- --check

  doc:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo doc --workspace --no-deps
      - uses: actions/upload-artifact@v4
        with:
          name: docs
          path: target/doc
```

### 38.2 Rustdoc for Public APIs

Add `#![doc = ...]` headers and `///` docs to all public items across:
- cobol-core (traits, config, errors)
- cobol-types (all 10+ types)
- cobol-move (move engine, initialize)
- cobol-arithmetic (all 5 verbs)
- cobol-io (file types, status codes)
- cobol-sort (sort/merge engines)
- cobol-runtime (prelude, intrinsic functions)

### 38.3 Workspace README

Update top-level README.md with:
- Project overview and architecture diagram
- Quick start guide
- Feature matrix (what's supported)
- Known limitations

### 38.4 Cargo.toml Metadata

Fill in missing `package.repository`, `package.readme`, `package.keywords`,
`package.categories` across all 8 crate Cargo.toml files. This eliminates
the clippy metadata warnings.

---

## Dependency Graph

```
Session 33 (Parser + GO TO DEPENDING)
    |
    v
Session 34 (RENAMES)     Session 35 (AlphaEdited)     Session 36 (Floats)
    |                          |                            |
    +------+-------------------+----------------------------+
           |
           v
    Session 37 (Real-world Test Suite)
           |
           v
    Session 38 (CI/CD + Docs)
```

Sessions 34, 35, 36 are independent of each other and can be done in any order.
Session 37 benefits from having all three complete. Session 38 is last.

---

## Risk Assessment

| Session | Risk | Mitigation |
|---------|------|------------|
| 33 | ANTLR grammar missing EXIT PARAGRAPH/SECTION | Text-based detection fallback |
| 33 | SET extraction complexity (multiple targets) | Start with single target, iterate |
| 34 | RENAMES THRU byte range computation | Use existing layout offset data |
| 35 | AlphaEdited editing rules | Only 3 symbols (B/0//) -- simple |
| 36 | Float precision loss in to_decimal() | Document as known; matches COBOL |
| 36 | IBM HFP format | Defer entirely; document IEEE 754 only |
| 37 | Test programs may reveal codegen bugs | Fix forward; add regression tests |
| 38 | CI matrix (OS, Rust versions) | Start with ubuntu + stable only |

---

## Success Criteria

After Session 38:
- **~775+ tests** passing across all crates
- **Zero clippy warnings** (metadata + code)
- **CI pipeline** running on every push
- **Full rustdoc** for all public APIs
- **Real-world COBOL programs** transpiling correctly
- **All common COBOL verbs** supported (SET, START, all EXIT variants)
- **GO TO DEPENDING ON** working with dispatch loop
- **RENAMES** (level 66) resolving to correct byte ranges
- **AlphanumericEdited** editing and de-editing
- **COMP-1/COMP-2 floats** participating in MOVE and arithmetic
