# COBOL2Rust Runtime Feature Inventory

Date: 2026-03-06
Project Status: Phase 3 Complete (Sessions 1-30), Phase 4 TODO

## Crate Structure (7 crates)
1. **cobol-core** - traits, enums, errors, config
2. **cobol-types** - concrete data types
3. **cobol-move** - MOVE engine
4. **cobol-arithmetic** - arithmetic operations
5. **cobol-io** - file I/O (feature-gated: sqlite)
6. **cobol-sort** - SORT/MERGE engine
7. **cobol-runtime** - program lifecycle, prelude

## Implemented Features (Phase 1-3)

### cobol-core (Foundation)
- [OK] DataCategory enum (all 7 categories)
- [OK] CobolField trait (root trait)
- [OK] CobolNumeric trait
- [OK] CobolNumericEdited trait
- [OK] CobolGroup trait
- [OK] RuntimeConfig + DiagnosticLevel
- [OK] RoundingMode enum (5 modes)
- [OK] NumProc enum (NOPFD/PFD/MIG)
- [OK] CobolDialect enum (IBM/GNU/MicroFocus)
- [OK] ArithMode enum (SIZE ERROR handling)
- [OK] CallError enum (CALL/CANCEL errors)
- [OK] FileError enum (I/O errors)
- [OK] SortError enum
- [OK] EBCDIC tables (CP037, CP1140, CP500 - 256-byte const tables)
- [OK] Numeric parsing module (display, implied, zoned)
- [OK] Decimal extension utilities (truncate, round, overflow checks)
- [OK] Editing module (EditSymbol enum for PIC editing)
- [OK] Category module (data classification)

### cobol-types (Data Types)
- [OK] PackedDecimal (COMP-3, BCD nibble-based)
- [OK] ZonedDecimal (DISPLAY numeric, zone nibble encoding)
- [OK] CompBinary (COMP, COMP-4, COMP-5; 2/4/8-byte)
- [OK] PicX (alphanumeric, fixed-length with const generic)
- [OK] PicA (alphabetic only)
- [OK] NumericEdited (Z/*/$/+/-/CR/DB editing)
- [OK] CobolArray<T> (fixed OCCURS with 1-based indexing)
- [OK] CobolVarArray<T> (variable OCCURS DEPENDING ON)
- [OK] Level88Predicate (condition names)
- [OK] FigurativeConstant (SPACES, ZEROS, etc.)
- [OK] RedefinesGroup (overlaid fields with sync helpers)

Missing from Phase 4:
- [MISSING] NationalString / PicN (PIC N, DBCS)
- [MISSING] AlphaEdited (alphabetic with editing - not implemented)
- [MISSING] CobolFloat32/CobolFloat64 (COMP-1, COMP-2 floats)
- [MISSING] P-scaling support (implied decimal positions)
- [MISSING] SYNC alignment (record layout alignment)
- [MISSING] IBM HFP format (3/8-byte floating point)

### cobol-move (MOVE Engine)
- [OK] Central dispatch (cobol_move function)
- [OK] Legality matrix (all 49 legal move combinations)
- [OK] Move to alphabetic
- [OK] Move to alphanumeric
- [OK] Move to numeric
- [OK] Move to numeric-edited
- [OK] Move to group
- [OK] MOVE CORRESPONDING (by-name and by-value variants)
- [OK] INITIALIZE verb (all 7 categories)
- [OK] cobol_move_numeric (numeric-to-numeric)
- [OK] De-editing support (IBM extension, gated by config)
- [OK] Sign loss diagnostics
- [OK] National category placeholder (routed to alphanumeric)

### cobol-arithmetic (Arithmetic)
- [OK] ADD verb
- [OK] SUBTRACT verb
- [OK] MULTIPLY verb
- [OK] DIVIDE verb (with REMAINDER)
- [OK] COMPUTE expression evaluator
- [OK] ROUNDED phrase (NearestAwayFromZero, Truncate, etc.)
- [OK] ON SIZE ERROR handling
- [OK] Rounding logic (5 modes: NEAREST/TRUNCATE/UP/DOWN/AWAY)
- [OK] store_arithmetic_result helper

### cobol-runtime (Program Lifecycle)
- [OK] CobolProgram struct
- [OK] PerformStack (paragraph/perform stack tracking)
- [OK] DISPLAY verb (to stdout, stderr variants)
- [OK] ACCEPT verb (from stdin)
- [OK] STOP RUN
- [OK] CallDispatcher (registry for CALL/CANCEL)
- [OK] CALL verb (BY REF/CONTENT/VALUE/OMITTED)
- [OK] CANCEL verb
- [OK] INSPECT verb (TALLYING, REPLACING, CONVERTING)
- [OK] STRING verb (with DELIMITED BY, WITH POINTER)
- [OK] UNSTRING verb (single/multiple delimiters, COUNT IN)
- [OK] Intrinsic functions (27 functions: numeric, math, string, date)
- [OK] Reference modification (ref_mod_read, ref_mod_write)
- [OK] Special registers module
- [OK] Prelude module (re-exports all public APIs)

### cobol-io (File I/O)
- [OK] CobolFile trait (abstract file ops)
- [OK] SequentialFile (fixed/variable-length records)
- [OK] IndexedFile (SQLite backend for VSAM KSDS)
- [OK] RelativeFile (slot-based, random/sequential/dynamic)
- [OK] FileStatusCode enum (2-byte COBOL file codes)
- [OK] FileOpenMode (INPUT, OUTPUT, EXTEND, I-O)
- [OK] FileOrganization (SEQUENTIAL, INDEXED, RELATIVE)
- [OK] FileAccessMode (SEQUENTIAL, RANDOM, DYNAMIC)
- [OK] FileResolver (config-based, env var, explicit, default paths)
- [OK] OPEN verb
- [OK] CLOSE verb
- [OK] READ verb (with AT END, INVALID KEY)
- [OK] WRITE verb (with INVALID KEY)
- [OK] REWRITE verb (with INVALID KEY)
- [OK] DELETE verb
- [OK] ADVANCING semantics
- [PARTIAL] Line sequential (mentioned in arch doc, not confirmed in code)
- [PARTIAL] Print file with LINAGE (mentioned in arch doc)

### cobol-sort (SORT/MERGE)
- [OK] CobolSortEngine (adaptive in-memory + external)
- [OK] CobolMergeEngine (k-way merge)
- [OK] InMemorySort (Vec::sort_by)
- [OK] ExternalMergeSort (tempfile-based k-way merge)
- [OK] SortKeySpec (key definition)
- [OK] SortKeyType (ALPHANUMERIC, ZONED, PACKED, BINARY)
- [OK] CollatingTable (ASCII, EBCDIC, Custom)
- [OK] Releaser (INPUT PROCEDURE)
- [OK] Returner (OUTPUT PROCEDURE)
- [OK] SortReturn enum (0=Success, 16=Failed)
- [OK] sort_with_procedures
- [OK] sort_with_input_procedure
- [OK] sort_with_output_procedure
- [OK] SORT verb (USING/GIVING)
- [OK] MERGE verb
- [OK] RELEASE/RETURN statements

## Phase 4 TODO (Polish & Production)

### Missing Type Implementations
1. **NationalString (PIC N)** - DBCS/double-byte national character set
   - Would need DataCategory::National handling in MOVE
   - Platform encoding considerations (UTF-16, etc.)
   - Priority: P3

2. **AlphaEdited** - Alphabetic with editing (PIC A with B, 0, / insertion)
   - Currently NOT a separate type; AlphanumericEdited handles this
   - May need dedicated type for semantic clarity
   - Priority: P2

3. **CobolFloat32/CobolFloat64** - COMP-1, COMP-2 floats
   - COMP-1: 4-byte float (NOT IEEE-754, IBM HFP format)
   - COMP-2: 8-byte float
   - Currently missing entirely
   - Priority: P2

4. **IBM HFP Format** - Hexadecimal Floating Point
   - Non-IEEE format used by IBM mainframes
   - 3-byte and 8-byte variants
   - Required for COMP-1/COMP-2 fidelity
   - Priority: P3

5. **P-scaling Support** - Implied decimal positions
   - PIC 9(5)P(2) = 5 digits with 2 implied decimal positions
   - Currently no special handling
   - Priority: P3

6. **SYNC Alignment** - Record layout alignment
   - SYNC LEFT / SYNC RIGHT
   - Affects binary/float field byte alignment in records
   - Currently flat byte layout
   - Priority: P3

7. **RENAMES (Level 66)** - Field aliasing/renaming
   - 66-level items that rename ranges of lower-level fields
   - Would require special tracking in data hierarchy
   - Priority: P3

### Missing Transpiler Features
- [TODO] Full Statement coverage (some statement types have "// TODO" in codegen)
- [TODO] RENAMES codegen (level 66 extraction/generation)
- [TODO] P-scaling codegen (precision calculation with implied positions)
- [TODO] SYNC alignment codegen (record layout with alignment bytes)
- [TODO] National type codegen (NationalString type generation)
- [TODO] Float type codegen (CobolFloat32/64 generation)

### Test Coverage Status
- [OK] 91 test functions across crates
- [OK] 668+ runtime unit tests (from memory: Session 30)
- [OK] 3 E2E compile tests
- [OK] 14 E2E integration tests
- [OK] Comprehensive MOVE matrix tests
- [MISSING] Fuzzing (proptest mentioned in roadmap)
- [MISSING] Performance benchmarks

### Documentation & Polish
- [TODO] Rustdoc for all public APIs (P1 in Phase 4)
- [TODO] CI/CD pipeline (GitHub Actions, P1)
- [TODO] Crates.io publication (P2)
- [TODO] Fuzzing with proptest (P2)
- [TODO] Memory-mapped I/O for LDS emulation (P3)

## Key Architectural Decisions

### Numeric Strategy
- **Decimal as canonical IR** - rust_decimal (28-digit limit)
- **No i128 fallback yet** - IBM COBOL max 18 digits fits in Decimal
- **Trait-based polymorphism** - CobolNumeric, CobolField hierarchy
- **Scale tracking per type** - precision + scale stored in each type

### Type Safety
- **Const generics for PicX** - PicX<const N: usize> eliminates bounds checks
- **Newtype wrappers** - Distinct types for PackedDecimal, ZonedDecimal, etc.
- **Enum for categories** - Compile-time routing in MOVE engine

### I/O & Storage
- **Trait abstraction for files** - Swappable backends (SQLite for indexed)
- **EBCDIC at boundaries** - ASCII internally, convert only at I/O
- **Variable-length support** - RDW handling in sequential files
- **Feature-gated dependencies** - io/sort optional, sqlite behind feature

### Performance Considerations
- **Adaptive sort** - Threshold for in-memory vs external merge
- **Byte caching** - CompBinary maintains byte cache for as_bytes()
- **Zero-copy where possible** - Reference modification without allocation
- **REDEFINES without copying** - RedefinesGroup uses shared buffer

## Code Quality Notes
- All 668+ tests passing as of Session 30
- Compile tests gated by feature flag
- No unimplemented!() calls in runtime crates
- National category routed to alphanumeric (temporary, Phase 4 task)
- Only 1 TODO in transpiler proc_gen.rs (unsupported statement catch-all)
