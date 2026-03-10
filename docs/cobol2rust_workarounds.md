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

## W-003: (Template for future workarounds)

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
