# Stress Test Fix Workflow

## MANDATORY APPROACH: One-at-a-time Build-Test-Fix Loop

**NEVER bulk-build or bulk-test all 40 programs at once.**

### Loop for each program:
1. **BUILD** one program: `cargo build --manifest-path <program>/Cargo.toml`
2. **READ** the compile errors (if any)
3. **DIAGNOSE** root cause in the generated main.rs or the codegen
4. **FIX** the codegen (in proc_gen.rs/data_gen.rs/transpile.rs)
5. **RE-TRANSPILE** that one program
6. **RE-BUILD** to verify the fix
7. **RUN** the binary to check runtime behavior
8. **MOVE ON** to the next program only after current one passes

### Grouping allowed:
- If the same root cause affects multiple programs, fix the codegen once,
  then re-transpile and verify each affected program one at a time.

### Current status (Session 43+):
- 35/40 compile (34 clean compile + sort-verb compiles with runtime output issue)
- 5 remaining failures:
  - arithmetic-verbs: ROUNDED grammar tokenization (deferred)
  - deep-nesting: Deeply nested levels 11-16 with PIC on group items
  - initialize-set: INDEXED BY creates implicit index names not in WS
  - move-variants: Duplicate child field names + MOVE CORRESPONDING needs CobolGroup
  - realistic-batch: Multiple issues (INDEXED BY, 88-level conditions, FILE SECTION children)
- FIXED this session: sort-verb (INPUT PROCEDURE), arrays-occurs (VarArray Index + OCCURS max),
  merge-verb, compute-verb

### COBOL sources: /Users/chandramohn/workspace/cobol2rust/cobol/
### Build workspace: /tmp/cobol_stress_workspace/programs/
### CLI binary: /Volumes/RustBuild/cobol2rust-target/release/cobol2rust
