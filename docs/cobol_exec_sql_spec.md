# EXEC SQL Support Specification

## Overview

Add COBOL EXEC SQL support to cobol2rust, enabling transpilation of embedded SQL
statements into Rust code that calls a generic database abstraction trait. Users
provide a database-specific implementation (Postgres, DB2, SQLite, etc.) at runtime.

## Design Decisions

- **SQL first** -- CICS/IMS deferred to future work
- **Trait-based abstraction** -- generic enough for any database backend
- **Auto-inject SQLCA** -- no user burden; detected when EXEC SQL is present
- **Bidirectional host variables** -- SELECT INTO writes, INSERT/UPDATE reads,
  mirroring COBOL behavior exactly
- **Always included** -- no feature flag; cobol-sql is part of the full workspace

## Architecture

```
cobol-core -> cobol-types -> cobol-sql (NEW)
                                |
cobol-move -> cobol-arithmetic -+-> cobol-runtime -> cobol-transpiler
```

### New Crate: cobol-sql

Location: `crates/cobol-sql/`

Dependencies: cobol-types (for CobolField trait), thiserror, rust_decimal

Re-exported via cobol-runtime::prelude (like all other crates).

## COBOL EXEC SQL Constructs Supported

### Phase 1: Core DML + SQLCA

| Construct | COBOL Syntax | Rust Codegen |
|---|---|---|
| SELECT INTO | `EXEC SQL SELECT ... INTO :H1, :H2 FROM ... END-EXEC` | `sql_rt.exec_query(sql, &params, &mut [&mut h1, &mut h2], &mut sqlca)` |
| INSERT | `EXEC SQL INSERT INTO ... VALUES(:H1, :H2) END-EXEC` | `sql_rt.exec_update(sql, &[&h1, &h2], &mut sqlca)` |
| UPDATE | `EXEC SQL UPDATE ... SET ... WHERE ... END-EXEC` | `sql_rt.exec_update(sql, &params, &mut sqlca)` |
| DELETE | `EXEC SQL DELETE FROM ... WHERE ... END-EXEC` | `sql_rt.exec_update(sql, &params, &mut sqlca)` |
| COMMIT | `EXEC SQL COMMIT END-EXEC` | `sql_rt.commit(&mut sqlca)` |
| ROLLBACK | `EXEC SQL ROLLBACK END-EXEC` | `sql_rt.rollback(&mut sqlca)` |
| INCLUDE SQLCA | `EXEC SQL INCLUDE SQLCA END-EXEC` | Auto-injected, no codegen needed |

### Phase 2: Cursors

| Construct | COBOL Syntax | Rust Codegen |
|---|---|---|
| DECLARE CURSOR | `EXEC SQL DECLARE C1 CURSOR FOR SELECT ... END-EXEC` | `sql_rt.declare_cursor("C1", sql, &params, &mut sqlca)` |
| OPEN | `EXEC SQL OPEN C1 END-EXEC` | `sql_rt.open_cursor("C1", &mut sqlca)` |
| FETCH | `EXEC SQL FETCH C1 INTO :H1, :H2 END-EXEC` | `sql_rt.fetch_cursor("C1", &mut [&mut h1, &mut h2], &mut sqlca)` |
| CLOSE | `EXEC SQL CLOSE C1 END-EXEC` | `sql_rt.close_cursor("C1", &mut sqlca)` |

### Phase 3: Dynamic SQL + Advanced

| Construct | COBOL Syntax | Rust Codegen |
|---|---|---|
| PREPARE | `EXEC SQL PREPARE S1 FROM :H-SQL END-EXEC` | `sql_rt.prepare("S1", &h_sql, &mut sqlca)` |
| EXECUTE | `EXEC SQL EXECUTE S1 USING :H1 END-EXEC` | `sql_rt.execute_prepared("S1", &[&h1], &mut sqlca)` |
| EXECUTE IMMEDIATE | `EXEC SQL EXECUTE IMMEDIATE :H-SQL END-EXEC` | `sql_rt.execute_immediate(&h_sql, &mut sqlca)` |
| SAVEPOINT | `EXEC SQL SAVEPOINT S1 END-EXEC` | `sql_rt.savepoint("S1", &mut sqlca)` |

## Trait Design

```rust
/// Core SQL runtime trait -- implement per database backend.
pub trait CobolSqlRuntime {
    /// Execute a query that returns rows (SELECT INTO).
    /// `into_fields` receives the result columns in order.
    fn exec_query(
        &mut self,
        sql: &str,
        params: &[&dyn CobolField],
        into_fields: &mut [&mut dyn CobolField],
        sqlca: &mut Sqlca,
    );

    /// Execute a statement that modifies data (INSERT/UPDATE/DELETE).
    /// Returns affected row count via SQLCA.
    fn exec_update(
        &mut self,
        sql: &str,
        params: &[&dyn CobolField],
        sqlca: &mut Sqlca,
    );

    /// Declare a named cursor for a SELECT statement.
    fn declare_cursor(
        &mut self,
        name: &str,
        sql: &str,
        params: &[&dyn CobolField],
        sqlca: &mut Sqlca,
    );

    /// Open a previously declared cursor.
    fn open_cursor(&mut self, name: &str, sqlca: &mut Sqlca);

    /// Fetch the next row from an open cursor into host variables.
    fn fetch_cursor(
        &mut self,
        name: &str,
        into_fields: &mut [&mut dyn CobolField],
        sqlca: &mut Sqlca,
    );

    /// Close an open cursor.
    fn close_cursor(&mut self, name: &str, sqlca: &mut Sqlca);

    /// Commit the current transaction.
    fn commit(&mut self, sqlca: &mut Sqlca);

    /// Rollback the current transaction.
    fn rollback(&mut self, sqlca: &mut Sqlca);

    /// Prepare a dynamic SQL statement.
    fn prepare(
        &mut self,
        name: &str,
        sql_source: &dyn CobolField,
        sqlca: &mut Sqlca,
    );

    /// Execute a previously prepared statement.
    fn execute_prepared(
        &mut self,
        name: &str,
        params: &[&dyn CobolField],
        sqlca: &mut Sqlca,
    );

    /// Execute a dynamic SQL string immediately.
    fn execute_immediate(
        &mut self,
        sql_source: &dyn CobolField,
        sqlca: &mut Sqlca,
    );

    /// Create a savepoint.
    fn savepoint(&mut self, name: &str, sqlca: &mut Sqlca);

    /// Rollback to a savepoint.
    fn rollback_to_savepoint(&mut self, name: &str, sqlca: &mut Sqlca);
}
```

## SQLCA Struct

Auto-injected into WorkingStorage when any EXEC SQL is detected in the program.

```rust
/// SQL Communication Area -- matches IBM COBOL SQLCA layout.
#[derive(Debug, Clone)]
pub struct Sqlca {
    /// SQL return code: 0=success, 100=not found, <0=error
    pub sqlcode: i32,
    /// ANSI SQL state (5 chars, e.g. "00000", "02000")
    pub sqlstate: [u8; 5],
    /// Error message text
    pub sqlerrm: SqlErrm,
    /// Number of rows affected by last statement
    pub sqlerrd: [i32; 6],  // sqlerrd[2] = row count
    /// Warning flags
    pub sqlwarn: [u8; 11],   // sqlwarn0..sqlwarnA
}

#[derive(Debug, Clone)]
pub struct SqlErrm {
    /// Length of error message
    pub sqlerrml: i16,
    /// Error message content (up to 70 bytes, IBM standard)
    pub sqlerrmc: [u8; 70],
}
```

COBOL programs reference SQLCA fields directly:
- `SQLCODE` -> `ws.sqlca.sqlcode`
- `SQLSTATE` -> `ws.sqlca.sqlstate`
- `SQLERRD(3)` -> `ws.sqlca.sqlerrd[2]` (1-based to 0-based)
- `SQLWARN0` through `SQLWARNA` -> `ws.sqlca.sqlwarn[0..11]`

## Host Variable Binding

COBOL host variables use `:` prefix inside EXEC SQL blocks:

```cobol
EXEC SQL
    SELECT ENAME, SAL INTO :WS-ENAME, :WS-SAL
    FROM EMP WHERE EMPNO = :WS-EMPNO
END-EXEC.
```

### Resolution rules:
1. Strip the `:` prefix
2. Map to WORKING-STORAGE field name (same cobol_to_rust_name conversion)
3. For SELECT INTO targets: pass as `&mut dyn CobolField` (write direction)
4. For WHERE/VALUES params: pass as `&dyn CobolField` (read direction)
5. For SQL text passed to runtime: replace `:HOST-VAR` with positional `?`
   placeholders (or `$1, $2` depending on backend -- trait impl decides)

### Null indicators:
```cobol
EXEC SQL SELECT ENAME INTO :WS-ENAME :WS-ENAME-IND FROM ... END-EXEC
```
When an indicator variable follows a host variable (no comma), it signals
NULL handling. Indicator values: 0=not null, -1=null, >0=truncated.

This will need an extended binding struct:

```rust
pub struct HostVar<'a> {
    pub field: &'a mut dyn CobolField,
    pub indicator: Option<&'a mut dyn CobolField>,
}
```

## Transpiler Pipeline Changes

### 1. Preprocessor (preserve EXEC SQL)

Currently: EXEC SQL blocks are converted to `*>EXECSQL` tagged comments and
stripped from the source before parsing.

Change: Preserve EXEC SQL blocks through preprocessing. Two options:
- (a) Pass them through as-is for the main parser to handle
- (b) Extract them in preprocessing and store separately, injecting placeholder
  tokens that the proc_listener picks up

Option (b) is safer -- the main COBOL grammar doesn't understand SQL syntax
inside EXEC blocks, so we extract the raw SQL text and replace with a marker.

### 2. AST Nodes

New variants in `Statement` enum:

```rust
/// EXEC SQL ... END-EXEC
ExecSql(ExecSqlStatement),
```

```rust
#[derive(Debug, Clone, Serialize)]
pub struct ExecSqlStatement {
    pub sql_type: SqlStatementType,
    /// Raw SQL text (with :host-var references intact)
    pub raw_sql: String,
    /// Host variables referenced as input params (read direction)
    pub input_vars: Vec<HostVarRef>,
    /// Host variables referenced as output targets (write direction, SELECT INTO)
    pub output_vars: Vec<HostVarRef>,
    /// Cursor name (for DECLARE/OPEN/FETCH/CLOSE)
    pub cursor_name: Option<String>,
    /// Prepared statement name (for PREPARE/EXECUTE)
    pub prepared_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HostVarRef {
    /// COBOL field name (without : prefix)
    pub field_name: String,
    /// Optional null indicator field name
    pub indicator: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum SqlStatementType {
    SelectInto,
    Insert,
    Update,
    Delete,
    DeclareCursor,
    OpenCursor,
    FetchCursor,
    CloseCursor,
    Commit,
    Rollback,
    Prepare,
    Execute,
    ExecuteImmediate,
    Savepoint,
    IncludeSqlca,
    Other(String),  // catch-all for unrecognized SQL
}
```

### 3. SQL Text Extraction (no SQL parser)

The text between EXEC SQL and END-EXEC is treated as an opaque string.
No SQL grammar or parser is used. Instead, simple text processing:

1. Identify statement type via keyword matching on first word(s)
   (SELECT, INSERT, UPDATE, DELETE, DECLARE CURSOR, OPEN, FETCH, CLOSE,
   COMMIT, ROLLBACK, PREPARE, EXECUTE, EXECUTE IMMEDIATE, SAVEPOINT)
2. Extract host variable references via `:NAME` regex pattern matching
3. Distinguish INTO targets (output) from other params (input) by
   position relative to the INTO keyword
4. Extract cursor and prepared statement names by position after keywords

The actual SQL is passed through as-is to the runtime trait.

### 4. Codegen (proc_gen.rs)

New match arm in `generate_statement()`:

```rust
Statement::ExecSql(sql) => generate_exec_sql(w, sql, ctx),
```

The `generate_exec_sql` function:
1. Builds the parameterized SQL string (`:HOST` -> `?`)
2. Resolves host var names to WorkingStorage field references
3. Emits the appropriate trait method call based on sql_type

### 5. Data Division (auto-inject SQLCA)

When any `ExecSql` statement is found in the procedure division (or
`INCLUDE SQLCA` is explicit), the data_gen phase auto-adds:

```rust
pub sqlca: Sqlca,
```

to the WorkingStorage struct, initialized with `Sqlca::default()`.

### 6. WorkingStorage struct gains sql_runtime

Transpiled programs that use EXEC SQL get an additional field:

```rust
pub sql_runtime: Box<dyn CobolSqlRuntime>,
```

Injected into the `new()` constructor as a parameter. Programs without
EXEC SQL are unaffected.

## Generated Code Example

COBOL input:
```cobol
WORKING-STORAGE SECTION.
01 WS-EMPNO     PIC 9(6).
01 WS-ENAME     PIC X(20).
01 WS-SAL       PIC 9(7)V99.

PROCEDURE DIVISION.
    MOVE 100 TO WS-EMPNO.
    EXEC SQL
        SELECT ENAME, SAL
        INTO :WS-ENAME, :WS-SAL
        FROM EMP
        WHERE EMPNO = :WS-EMPNO
    END-EXEC.
    IF SQLCODE = 0
        DISPLAY "Found: " WS-ENAME
    END-IF.
```

Rust output:
```rust
use cobol_runtime::prelude::*;

pub struct WorkingStorage {
    pub ws_empno: PackedDecimal,
    pub ws_ename: PicX,
    pub ws_sal: PackedDecimal,
    pub sqlca: Sqlca,
}

impl WorkingStorage {
    pub fn new() -> Self {
        Self {
            ws_empno: PackedDecimal::new(6, 0, false).init_zero(),
            ws_ename: PicX::new(20, b""),
            ws_sal: PackedDecimal::new(9, 2, false).init_zero(),
            sqlca: Sqlca::default(),
        }
    }
}

pub fn run(ws: &mut WorkingStorage, sql_rt: &mut dyn CobolSqlRuntime) {
    // MOVE 100 TO WS-EMPNO
    move_numeric_literal(
        "100".parse::<Decimal>().unwrap(),
        &mut ws.ws_empno, &RuntimeConfig::default(),
    );

    // EXEC SQL SELECT ... END-EXEC
    sql_rt.exec_query(
        "SELECT ENAME, SAL FROM EMP WHERE EMPNO = ?",
        &[&ws.ws_empno],
        &mut [&mut ws.ws_ename, &mut ws.ws_sal],
        &mut ws.sqlca,
    );

    // IF SQLCODE = 0
    if ws.sqlca.sqlcode == 0 {
        print!("Found: ");
        println!("{}", ws.ws_ename.display_bytes());
    }
}
```

## Implementation Phases

### Phase 1: Parse + AST (no codegen yet)
- Modify preprocessor to preserve EXEC SQL blocks
- Extract SQL text, detect statement type via keyword matching
- Extract host variable references via `:NAME` pattern matching
- Add ExecSql variant to Statement enum
- Wire into proc_listener
- `cobol2rust check` now reports SQL statement counts

### Phase 2: cobol-sql crate + SQLCA
- Create crate with trait, Sqlca, HostVar types
- Add to workspace, wire into cobol-runtime re-exports
- Auto-inject SQLCA in data_gen when SQL detected

### Phase 3: Codegen for core DML
- SELECT INTO, INSERT, UPDATE, DELETE
- Host variable resolution and parameterization
- SQLCODE/SQLSTATE field access in IF conditions

### Phase 4: Cursor support
- DECLARE CURSOR, OPEN, FETCH, CLOSE codegen
- Cursor name tracking across statements

### Phase 5: Dynamic SQL
- PREPARE, EXECUTE, EXECUTE IMMEDIATE
- SAVEPOINT/ROLLBACK TO SAVEPOINT

## Testing Strategy

- Unit tests: SQL text extraction (statement type, host vars, cursor names)
- Unit tests: codegen output for each SQL statement type
- Integration tests: full transpile of SQL-containing COBOL programs
- Runtime tests: mock CobolSqlRuntime impl that validates calls
- Stress tests: real COBOL programs from cobol/ directory that use SQL

## Decisions

1. **WHENEVER** -- Deferred. `EXEC SQL WHENEVER SQLERROR GOTO ...` is a
   compile-time directive that inserts IF SQLCODE < 0 checks after every SQL
   statement. Will be added in a future phase if needed.

2. **INCLUDE copybooks** -- Deferred. `EXEC SQL INCLUDE member END-EXEC` is
   like COPY but for SQL. Will integrate with existing COPY/REPLACING support
   in a future phase.

3. **No SQL parser** -- The SQL text between EXEC SQL and END-EXEC is treated
   as an opaque string. We do NOT parse SQL syntax. Host variable extraction
   uses simple `:NAME` pattern matching on the raw text. Statement type
   detection uses keyword matching (SELECT/INSERT/UPDATE/DELETE/CURSOR/etc.).
   The actual SQL is passed through as-is to the runtime trait -- the backend
   is responsible for understanding the dialect.
