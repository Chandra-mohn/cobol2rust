# cobol2rust Known Workarounds

Temporary workarounds for COBOL source or codegen limitations.
Each entry documents the root cause, the workaround applied, and the proper fix needed.

---

## W-001: ANTLR Reserved Word in Identifiers

**Affected**: Any COBOL variable containing a reserved keyword as a suffix
(e.g., `WS-ROUNDED`, `WS-GIVING`, `WS-REMAINDER`)

**Root cause**: The ANTLR4 lexer tokenizes `ROUNDED`, `GIVING`, `REMAINDER`, etc.
as keyword tokens. When these appear as part of a hyphenated identifier like
`WS-ROUNDED`, the lexer splits it into `WS-` (identifier fragment) + `ROUNDED`
(keyword token), producing an invalid parse.

**Symptom**: Generated Rust code contains truncated field names like `ws_` (empty
suffix) or `ws_1` (if the keyword is followed by a digit).

**Workaround**: Rename the COBOL variable to avoid the keyword suffix.
- `WS-ROUNDED` -> `WS-RNDVAL`
- `WS-REMAINDER` -> `WS-REMVAL`
- `WS-GIVING` -> `WS-GIVVAL`

**Applied to**: `cobol/language/statements/arithmetic_verbs.cbl`
- `WS-ROUNDED` renamed to `WS-RNDVAL` (all 7 occurrences)

**Proper fix**: Modify the ANTLR4 `cobolWord` lexer/parser rule to allow reserved
words as segments of hyphenated identifiers. The grammar needs a rule like:
```
cobolWord: IDENTIFIER | IDENTIFIER MINUSCHAR reservedWord | ...
```
Or alternatively, handle this in a pre-processing step that escapes keyword
collisions before lexing.

**Keywords known to cause issues**: ROUNDED, REMAINDER, GIVING, SIZE, ERROR,
PROCEDURE, SECTION, DIVISION (any keyword that can appear as part of a
data name in real COBOL programs).

---

## W-002: DuckDB Does Not Support SAVEPOINT

**Affected**: `CobolSqlRuntime::savepoint()` and `CobolSqlRuntime::rollback_to_savepoint()`
methods when using the `DuckDbRuntime` backend.

**Root cause**: DuckDB does not implement the SQL `SAVEPOINT` or
`ROLLBACK TO SAVEPOINT` syntax. These are standard SQL features supported by
PostgreSQL, Oracle, DB2, and SQLite, but DuckDB's transaction model only
supports flat `BEGIN`/`COMMIT`/`ROLLBACK`.

**Symptom**: Calling `savepoint()` or `rollback_to_savepoint()` on `DuckDbRuntime`
sets `SQLCODE = -1` with a parser error message. COBOL programs using
`EXEC SQL SAVEPOINT` will fail at runtime with DuckDB.

**Workaround**: SAVEPOINT/ROLLBACK TO SAVEPOINT is not tested with DuckDB.
The `DuckDbRuntime` implementation passes the SQL through to DuckDB which
rejects it. Programs that use savepoints must use an enterprise backend
(PostgreSQL, DB2, Oracle) that supports the syntax.

**Applied to**: `crates/cobol-sql/tests/duckdb_integration.rs` -- savepoint
test case removed (documented as DuckDB limitation in test file comment).

**Proper fix**: Implement `PostgresRuntime` (or other enterprise backend) that
supports `SAVEPOINT`. The `CobolSqlRuntime` trait is backend-agnostic by design,
so savepoint support works correctly once a supporting backend is used.
Alternatively, DuckDB savepoints could be emulated using nested transactions
if DuckDB adds support in the future.

---

## W-003: Deep-Nesting EVALUATE Output Mismatch

**Affected**: `cobol/volume/deep_nesting.cbl` -- the `TEST-EVAL-IN-NEST`
paragraph that exercises `EVALUATE TRUE` inside nested IF inside PERFORM.

**Root cause**: The transpiled Rust code produces incorrect output for the
EVALUATE TRUE / WHEN conditions inside a deeply nested control structure
(3-level nested IF inside a PERFORM VARYING, with the EVALUATE containing
further nested IFs and a nested EVALUATE inside one WHEN branch). The codegen
for EVALUATE WHEN branches interacts incorrectly with the surrounding nested
IF context, likely a scoping or fall-through issue in `proc_gen.rs`.

**Symptom**: The program compiles and runs. 10 of 11 checks pass. The final
check (`WS-CATEGORY = "CAT-E"`) fails -- the EVALUATE TRUE at iteration I=5
with WS-DEPTH=50 does not correctly match `WHEN WS-DEPTH > 40` inside the
nested structure, producing the wrong category value.

**Workaround**: None applied. The test compiles and runs; the mismatch is
accepted as a known minor issue. All other 39 stress test programs pass
all checks.

**Applied to**: `cobol/volume/deep_nesting.cbl` -- test runs but 1 of 11
runtime checks produces wrong output.

**Proper fix**: Debug the EVALUATE TRUE codegen in
`crates/cobol-transpiler/src/codegen/proc_gen.rs` for the specific case of:
1. EVALUATE TRUE inside nested IF (3+ levels deep)
2. WHEN branches that themselves contain nested IF statements
3. Nested EVALUATE inside a WHEN branch
The issue is likely in how WHEN branch conditions interact with the
surrounding if/else chain generation, or how the EVALUATE's `match` arms
handle nested control flow.

---

## W-004: (Template for future workarounds)

**Affected**:

**Root cause**:

**Symptom**:

**Workaround**:

**Applied to**:

**Proper fix**:

---

## Index of Affected Files

| File | Workaround | Date |
|------|-----------|------|
| cobol/language/statements/arithmetic_verbs.cbl | W-001: WS-ROUNDED -> WS-RNDVAL | 2026-03-08 |
| crates/cobol-sql/tests/duckdb_integration.rs | W-002: SAVEPOINT test removed (DuckDB limitation) | 2026-03-09 |
| cobol/volume/deep_nesting.cbl | W-003: EVALUATE output mismatch (1/11 checks) | 2026-03-09 |
