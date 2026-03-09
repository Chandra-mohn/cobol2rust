# Codegen Fix Plan: 28 Failing Stress Tests

**Current state**: 7/35 compile+run, 28 fail with codegen errors
**Target**: 30+/35 compile+run (file I/O and SORT/MERGE may need external resources)

---

## Phase 1: Data Generation (data_gen.rs) -- Foundation

These fixes address how `WorkingStorage` struct fields are generated.
Many Phase 2-4 fixes depend on Phase 1 being correct first.

### Fix 1A: Group Fields as Struct Members (10 programs)

**Problem**: Group-level items (01-level records with children) are NOT generated
as fields in the `WorkingStorage` struct. Only leaf-level elementary items appear.
The procedure division references `ws.ws_employee`, `ws.ws_group`, etc. but these
don't exist in the struct.

**Affected**: group_records, level88, figurative_constants, display_accept,
initialize_set, indexed_file, relative_file, sequential_file, sort_verb, merge_verb

**Fix approach**: Generate group fields as a separate struct type, or as a
byte-array wrapper that owns the children's storage. Options:

- **Option A - Flat byte buffer**: Group field = `PicX` spanning the total byte
  length, children are views into it. Simple but loses type safety.
- **Option B - Nested struct**: Generate a `WsEmployee` struct with typed children,
  impl `CobolField` + `CobolGroup`. Complex but correct.
- **Option C - Dual representation**: Keep leaf fields in WorkingStorage AND add
  group field as PicX overlay. Procedure refs use leaf names for elementary access,
  group name for GROUP-level MOVE/DISPLAY.

**Recommended**: Option C -- least disruptive, matches current codegen pattern.
Add the group field as `PicX::new(total_byte_len, initial_bytes)` to WorkingStorage.
When code does `MOVE ws_employee TO ...`, it reads the flat PicX bytes.
When code does `MOVE "X" TO ws_emp_name`, it writes to the leaf field (unchanged).
Limitation: group and children are decoupled (writing to leaf doesn't update group).
For full correctness, Option B is needed eventually.

**Files**: `crates/cobol-transpiler/src/codegen/data_gen.rs`

### Fix 1B: COMP Fields Use CompBinary, Not Primitives (6 programs)

**Problem**: COMP/COMP-5 fields generate as `i16`, `i32`, `i64`, `u16`, `u32`
primitive types. These don't implement `CobolField`, `CobolNumeric`, or have
`.display_bytes()`. Every arithmetic/move/display operation fails.

**Affected**: binary_types, arithmetic_verbs, call_cancel, comparison_ops,
sign_conditions, nested_control

**Fix**: Change `data_gen.rs` type mapping:
```
COMP PIC S9(1-4)  -> CompBinary::new(prec, scale, signed, comp_type)
COMP PIC S9(5-9)  -> CompBinary::new(...)
COMP PIC S9(10-18) -> CompBinary::new(...)
```
`CompBinary` already exists and implements all required traits. The primitive
types were likely a premature optimization. Remove all `i16`/`i32`/`i64`/`u16`/`u32`
mappings from generated code.

**Files**: `crates/cobol-transpiler/src/codegen/data_gen.rs`

### Fix 1C: Duplicate Field Names (2 programs)

**Problem**: When two different GROUP records contain children with the same
COBOL name (e.g., `WS-NAME` in both `WS-SRC-REC` and `WS-DST-REC`), the
generated struct has duplicate field names, which is a Rust compile error.

**Also**: FILLER items (unnamed placeholders) all generate as `filler`, colliding.

**Affected**: group_records, move_variants

**Fix**:
- Prefix child fields with parent group name when collision detected
- Or always prefix: `ws_src_rec_name`, `ws_dst_rec_name`
- FILLER: use `_filler_1`, `_filler_2`, etc. (numbered, prefixed with `_`)

**Files**: `crates/cobol-transpiler/src/codegen/data_gen.rs`

### Fix 1D: PicX Default Implementation (1 program)

**Problem**: Generated code uses `Default::default()` for PicX but PicX doesn't
implement `Default`.

**Affected**: alphanum_pic

**Fix**: Derive or implement `Default` for PicX (empty/zero-length field).
Or change codegen to use `PicX::new(len, b"")` instead of `Default::default()`.

**Files**: `crates/cobol-types/src/pic_x.rs` or `data_gen.rs`

### Fix 1E: Keyword Escaping (1 program)

**Problem**: COBOL field named `TRUE` generates Rust identifier `true`, which is
a reserved keyword.

**Affected**: compound_conditions

**Fix**: Check generated field names against Rust keywords list. Escape with
`r#` prefix: `r#true`, `r#false`, `r#type`, `r#match`, etc.

**Files**: `crates/cobol-transpiler/src/codegen/data_gen.rs` (struct fields),
`proc_gen.rs` (field references)

---

## Phase 2: Condition Codegen (proc_gen.rs) -- Comparisons and IF

These fixes address how IF/EVALUATE conditions are translated to Rust.

### Fix 2A: Figurative Constants in Conditions (2+ programs)

**Problem**: `IF WS-FIELD = SPACES` generates `ws.ws_field == SPACES` where
`SPACES` is not a valid Rust identifier. Only MOVE was fixed (Session 41),
not conditions.

**Affected**: comparison_ops, figurative_constants, (others indirectly)

**Fix**: In `condition_to_expr()`, when a comparison operand is a figurative
constant, generate the appropriate check:
```rust
// IF field = SPACES -> check all bytes are 0x20
ws.ws_field.as_bytes().iter().all(|&b| b == b' ')
// IF field = ZEROS -> check numeric zero
ws.ws_field.to_decimal() == Decimal::ZERO  // for numeric
ws.ws_field.as_bytes().iter().all(|&b| b == b'0')  // for alphanumeric
// IF field = HIGH-VALUES -> all bytes 0xFF
ws.ws_field.as_bytes().iter().all(|&b| b == 0xFF)
// IF field = LOW-VALUES -> all bytes 0x00
ws.ws_field.as_bytes().iter().all(|&b| b == 0x00)
```

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`

### Fix 2B: Alphanumeric Comparisons (2 programs)

**Problem**: `IF WS-ALPHA-A < WS-ALPHA-B` generates
`ws.ws_alpha_a.to_decimal() < ws.ws_alpha_b.to_decimal()` but PicX doesn't
have `to_decimal()`. Alphanumeric comparisons should use byte-level comparison.

**Affected**: comparison_ops, compound_conditions

**Fix**: In `condition_to_expr()`, detect when BOTH operands are alphanumeric
(PicX/PicA) and generate byte comparison:
```rust
ws.ws_alpha_a.as_bytes() < ws.ws_alpha_b.as_bytes()
```
For mixed (numeric vs alphanumeric), de-edit the alphanumeric first.

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`
**Requires**: Access to data division type info in condition codegen

### Fix 2C: Sign Conditions (3 programs)

**Problem**: `IF WS-X IS POSITIVE` generates `ws.ws_x.is_positive()` but this
method doesn't exist on `PackedDecimal`/`CompBinary`.

**Affected**: sign_conditions, (nested_control indirectly)

**Fix**: Generate decimal-based sign checks:
```rust
// IS POSITIVE -> to_decimal() > Decimal::ZERO
ws.ws_x.to_decimal() > Decimal::ZERO
// IS NEGATIVE -> to_decimal() < Decimal::ZERO
ws.ws_x.to_decimal() < Decimal::ZERO
// IS ZERO -> to_decimal() == Decimal::ZERO
ws.ws_x.to_decimal() == Decimal::ZERO
```

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`

### Fix 2D: CLASS Conditions (3 programs)

**Problem**: `IF WS-FIELD IS NUMERIC` generates `ws.ws_field.is_numeric()` but
PicX doesn't have this method.

**Affected**: comparison_ops, if_evaluate, (compound_conditions)

**Fix options**:
- **Option A**: Add `is_numeric()` / `is_alphabetic()` methods to `PicX`
  (checks if all bytes are digits / letters respectively)
- **Option B**: Generate inline check in codegen:
  `ws.ws_field.as_bytes().iter().all(|&b| b.is_ascii_digit())`

**Recommended**: Option A -- add methods to PicX in cobol-types, they're useful
runtime methods.

**Files**: `crates/cobol-types/src/pic_x.rs` (add methods),
`crates/cobol-transpiler/src/codegen/proc_gen.rs` (generate calls)

### Fix 2E: Comparison Chaining (2 programs)

**Problem**: `IF WS-A > 50 AND < 100` generates Rust like
`ws.ws_a > 50 && < 100` or `ws.ws_a > 50 < 100` which is invalid.

**Affected**: if_evaluate, nested_control

**Fix**: Detect abbreviated conditions in the parser/codegen and expand:
```rust
// IF WS-A > 50 AND < 100 ->
ws.ws_a.to_decimal() > "50".parse::<Decimal>().unwrap()
    && ws.ws_a.to_decimal() < "100".parse::<Decimal>().unwrap()
```

**Files**: Parser (condition normalization) or `proc_gen.rs`

---

## Phase 3: Statement Codegen (proc_gen.rs) -- Specific Verbs

### Fix 3A: Array Indexing (7 programs)

**Problem**: `WS-TABLE(WS-IDX)` generates `ws.ws_table[ws.ws_idx]` but
`CobolArray` doesn't implement `Index`. Also, COBOL uses 1-based indexing.

**Affected**: arrays_occurs, display_accept, initialize_set, move_variants,
nested_control, perform_variants, reference_mod

**Fix options**:
- **Option A**: Implement `Index<usize>` and `IndexMut<usize>` for CobolArray
  with 1-based indexing (subtract 1 internally)
- **Option B**: Generate `.get(idx)` / `.get_mut(idx)` calls in codegen
- **Option C**: Generate `[idx as usize - 1]` with Index trait on raw Vec

**Recommended**: Option A -- implement Index trait with automatic 1-to-0
conversion. Cleanest generated code.

For multi-dimensional arrays: flatten index calculation in codegen.
`WS-TABLE(I, J)` with row OCCURS 5, col OCCURS 10 ->
`ws.ws_table[((i - 1) * 10 + (j - 1) + 1)]` (convert to 1-based for Index)

**Files**: `crates/cobol-types/src/cobol_array.rs` (Index trait),
`crates/cobol-transpiler/src/codegen/proc_gen.rs` (subscript codegen)

### Fix 3B: EVALUATE WHEN Codegen (2 programs)

**Problem**: `EVALUATE TRUE WHEN WS-A > 75` generates malformed
`true == ws.ws_a>75` instead of proper condition. THRU keyword embedded in
strings. ALSO keyword not handled.

**Affected**: if_evaluate, (nested_control)

**Fix**: Rewrite EVALUATE codegen to properly handle:
- `EVALUATE TRUE` -> chain of `if/else if` with condition expressions
- `WHEN val1 THRU val2` -> range check `>= val1 && <= val2`
- `ALSO` -> compound conditions with `&&`

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`

### Fix 3C: Intrinsic Function Codegen (2 programs)

**Problem**: `FUNCTION LENGTH(WS-STR)` generates `ws.functionlength(ws_str)` --
the function name is concatenated with the field access, creating invalid syntax.

**Affected**: intrinsic_functions, display_accept

**Fix**: Intrinsic functions should generate as standalone function calls:
```rust
function_length(&ws.ws_str)        // not ws.functionlength(ws_str)
function_upper_case(&ws.ws_str)
function_current_date()
```
These functions exist in `cobol-runtime` (27 intrinsic functions implemented
in Session 23-24).

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs` (operand_expr or
dedicated intrinsic handler)

### Fix 3D: Section PERFORM (1 program)

**Problem**: `PERFORM PROCESSING-SECTION` generates
`processing_section(ws, ctx)` but no such function is defined. Sections contain
multiple paragraphs that need to be called in sequence.

**Affected**: section_perform

**Fix**: Generate a wrapper function for each section that calls its paragraphs
in order:
```rust
fn processing_section(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    proc_init(ws, ctx);
    proc_work(ws, ctx);
    proc_finish(ws, ctx);
}
```

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`

### Fix 3E: MOVE CORRESPONDING (1 program)

**Problem**: `MOVE CORRESPONDING WS-SRC TO WS-DST` generates
`cobol_move_corresponding(...)` but this function isn't found in scope.

**Affected**: move_variants

**Fix**: Ensure `move_corresponding` or `move_corresponding_by_name` is
re-exported in the prelude. It exists in `cobol-move` but may need the
correct function signature in generated code.

**Files**: Check `cobol-runtime/src/prelude.rs` exports,
`proc_gen.rs` for correct function call generation

### Fix 3F: INSPECT/STRING Argument Patterns (2 programs)

**Problem**: INSPECT TALLYING/REPLACING and STRING/UNSTRING generate function
calls with wrong argument counts or types.

**Affected**: inspect_tallying, string_unstring

**Fix**: Audit each INSPECT/STRING codegen path against the runtime API:
- `cobol_inspect_tallying(source_bytes, &[TallySpec])`
- `cobol_string(sources, delimiters, &mut dest, &mut pointer)`
- `cobol_unstring(source, delimiters, &mut dests, &mut counts, &mut pointer)`

Match generated code to these signatures.

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`

### Fix 3G: Reference Modification Casts (2 programs)

**Problem**: `WS-FIELD(pos:len)` generates code that tries to cast `Decimal`
as `usize` directly, which Rust doesn't allow. Also `to_u32()` not found.

**Affected**: reference_mod, string_unstring

**Fix**: Use explicit conversion:
```rust
ws.ws_field.to_decimal().to_u32().unwrap_or(0) as usize
// or
ws.ws_field.to_decimal().try_into().unwrap_or(0usize)
```
Ensure `use num_traits::ToPrimitive;` is in the prelude.

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`,
possibly `cobol-runtime/src/prelude.rs` (re-export ToPrimitive)

---

## Phase 4: File I/O and SORT/MERGE Codegen

### Fix 4A: File Descriptor Fields in WorkingStorage (5 programs)

**Problem**: File descriptors (FD records), file status fields, and record
areas from ENVIRONMENT/DATA division aren't generated as WorkingStorage fields.
References like `ws.seq_file`, `ws.idx_record` fail.

**Affected**: indexed_file, relative_file, sequential_file, sort_verb, merge_verb

**Fix**: Generate file-related fields in WorkingStorage:
- File handle: `pub seq_file: CobolFile`
- Record area: `pub seq_record: PicX` (or structured group)
- File status: `pub seq_status: PicX` (2 bytes)

**Files**: `crates/cobol-transpiler/src/codegen/data_gen.rs`

### Fix 4B: SORT/MERGE RELEASE/RETURN Codegen (2 programs)

**Problem**: SORT/MERGE `RELEASE`/`RETURN` verbs reference variables (`releaser`)
that aren't defined. The sort/merge runtime API isn't being called correctly.

**Affected**: sort_verb, merge_verb

**Fix**: Generate proper sort engine calls matching the `cobol-sort` crate API.

**Files**: `crates/cobol-transpiler/src/codegen/proc_gen.rs`

---

## Implementation Order and Session Estimates

| Session | Fixes | Expected Result |
|---------|-------|-----------------|
| S42 | 1A (group fields) + 1B (COMP types) | ~12-15/35 compile |
| S43 | 1C-1E (dupes, Default, keywords) + 2A-2C (figuratives, alphanum, sign) | ~18-22/35 compile |
| S44 | 3A (array indexing) + 3C (intrinsics) + 3D (sections) | ~25-28/35 compile |
| S45 | 2D-2E (class, chaining) + 3B (EVALUATE) + 3E-3G (CORR, INSPECT, refmod) | ~30-32/35 compile |
| S46 | 4A-4B (file I/O, SORT/MERGE) | ~33-35/35 compile |

### Dependencies
```
Phase 1 (foundation) --> Phase 2 (conditions) --> Phase 3 (statements)
                     \-> Phase 3A (arrays)
                     \-> Phase 4 (file I/O)

Within Phase 1: 1B before 2C (sign conditions need CobolNumeric)
Within Phase 2: 2A before others (figuratives are most common)
Within Phase 3: 3A before 3G (ref mod uses array indexing sometimes)
```

### Validation Strategy
After each phase, re-run the workspace build:
```bash
cobol2rust transpile --workspace --continue-on-error \
  --runtime-path crates/cobol-runtime \
  -v cobol/language/ -o /tmp/cobol-workspace

cd /tmp/cobol-workspace
for d in programs/*/; do
  name=$(basename "$d")
  cargo build -p "$name" 2>/dev/null && echo "[OK] $name" || echo "[FAIL] $name"
done
```

Also run the 797-test suite after every change to prevent regressions.
