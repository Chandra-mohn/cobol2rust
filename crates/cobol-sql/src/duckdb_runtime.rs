//! DuckDB implementation of `CobolSqlRuntime`.
//!
//! Provides an embeddable, zero-infrastructure SQL backend for transpiled
//! COBOL programs. Uses in-memory DuckDB by default, suitable for testing
//! and lightweight workloads. The same `CobolSqlRuntime` trait can be
//! implemented for enterprise databases (Postgres, DB2, Oracle) later.

use std::collections::HashMap;

use duckdb::types::Value;
use duckdb::{params_from_iter, Connection};

use crate::host_var::{HostVar, HostVarMut};
use crate::runtime::CobolSqlRuntime;
use crate::sqlca::Sqlca;

/// Cursor state: stores the SQL, bound parameter values, and fetched rows.
#[derive(Debug)]
struct CursorState {
    sql: String,
    param_values: Vec<Value>,
    rows: Vec<Vec<Value>>,
    position: usize,
}

/// DuckDB-backed SQL runtime.
///
/// Wraps a DuckDB `Connection` and implements all 13 `CobolSqlRuntime`
/// methods. Cursors are emulated by executing the query at OPEN time
/// and buffering all rows (suitable for typical COBOL result set sizes).
#[derive(Debug)]
pub struct DuckDbRuntime {
    conn: Connection,
    cursors: HashMap<String, CursorState>,
    prepared: HashMap<String, String>,
}

impl DuckDbRuntime {
    /// Create a runtime backed by an in-memory DuckDB database.
    pub fn in_memory() -> Result<Self, duckdb::Error> {
        let conn = Connection::open_in_memory()?;
        Ok(Self {
            conn,
            cursors: HashMap::new(),
            prepared: HashMap::new(),
        })
    }

    /// Create a runtime backed by a DuckDB database file.
    pub fn from_path(path: &str) -> Result<Self, duckdb::Error> {
        let conn = Connection::open(path)?;
        Ok(Self {
            conn,
            cursors: HashMap::new(),
            prepared: HashMap::new(),
        })
    }

    /// Get a reference to the underlying connection (for setup/teardown).
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Execute raw SQL directly on the connection (for test setup).
    pub fn execute_batch(&self, sql: &str) -> Result<(), duckdb::Error> {
        self.conn.execute_batch(sql)
    }
}

/// Convert a COBOL host variable to a DuckDB `Value`.
///
/// Uses `display_bytes()` instead of `as_bytes()` to get the human-readable
/// representation. This is critical for packed decimal (COMP-3) and binary
/// (COMP/COMP-5) fields whose raw storage is not text.
fn host_var_to_value(hv: &HostVar<'_>) -> Value {
    if hv.is_null() {
        return Value::Null;
    }
    let display = hv.field.display_bytes();
    let s = std::str::from_utf8(&display).unwrap_or("").trim().to_string();
    // Strip leading sign character for parsing (+00123 -> 00123)
    let s = s.strip_prefix('+').unwrap_or(&s);

    // Try parsing as integer first, then float, then keep as string.
    if let Ok(i) = s.parse::<i64>() {
        Value::BigInt(i)
    } else if let Ok(f) = s.parse::<f64>() {
        Value::Double(f)
    } else {
        Value::Text(s.to_string())
    }
}

/// Write a DuckDB `Value` into a COBOL host variable (output direction).
fn value_to_host_var(val: &Value, hv: &mut HostVarMut<'_>) {
    if val == &Value::Null {
        // Set indicator to -1 if present, fill field with spaces.
        if let Some(ref mut ind) = hv.indicator {
            ind.set_raw_bytes(b"-1");
        }
        hv.field.fill_bytes(b' ');
    } else {
        // Set indicator to 0 (non-null) if present.
        if let Some(ref mut ind) = hv.indicator {
            ind.set_raw_bytes(b"0");
        }
        let text = value_to_string(val);
        hv.field.set_raw_bytes(text.as_bytes());
    }
}

/// Convert a DuckDB Value to its string representation for COBOL field storage.
fn value_to_string(val: &Value) -> String {
    match val {
        Value::Null => String::new(),
        Value::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
        Value::TinyInt(i) => i.to_string(),
        Value::SmallInt(i) => i.to_string(),
        Value::Int(i) => i.to_string(),
        Value::BigInt(i) => i.to_string(),
        Value::HugeInt(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Double(f) => f.to_string(),
        Value::Text(s) => s.clone(),
        Value::Blob(b) => String::from_utf8_lossy(b).to_string(),
        _ => format!("{val:?}"),
    }
}

/// Replace `?` placeholders with DuckDB positional `$1`, `$2`, etc.
fn rewrite_placeholders(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let mut idx = 0usize;
    for ch in sql.chars() {
        if ch == '?' {
            idx += 1;
            result.push('$');
            result.push_str(&idx.to_string());
        } else {
            result.push(ch);
        }
    }
    result
}

impl CobolSqlRuntime for DuckDbRuntime {
    fn exec_query(
        &mut self,
        sql: &str,
        params: &[HostVar<'_>],
        into_fields: &mut [HostVarMut<'_>],
        sqlca: &mut Sqlca,
    ) {
        sqlca.reset();
        let rewritten = rewrite_placeholders(sql);
        let values: Vec<Value> = params.iter().map(host_var_to_value).collect();

        let result = self.conn.prepare(&rewritten).and_then(|mut stmt| {
            let mut rows = stmt.query(params_from_iter(values.iter()))?;
            if let Some(row) = rows.next()? {
                let col_count = into_fields.len();
                for (i, field) in into_fields.iter_mut().enumerate() {
                    if i < col_count {
                        let val: Value = row.get(i)?;
                        value_to_host_var(&val, field);
                    }
                }
                Ok(true)
            } else {
                Ok(false)
            }
        });

        match result {
            Ok(true) => sqlca.set_sqlcode(0),
            Ok(false) => sqlca.set_sqlcode(100),
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn exec_update(
        &mut self,
        sql: &str,
        params: &[HostVar<'_>],
        sqlca: &mut Sqlca,
    ) {
        sqlca.reset();
        let rewritten = rewrite_placeholders(sql);
        let values: Vec<Value> = params.iter().map(host_var_to_value).collect();

        match self
            .conn
            .prepare(&rewritten)
            .and_then(|mut stmt| stmt.execute(params_from_iter(values.iter())))
        {
            Ok(count) => {
                sqlca.set_sqlcode(0);
                #[allow(clippy::cast_possible_wrap)]
                {
                    sqlca.sqlerrd[2] = count as i32;
                }
            }
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn commit(&mut self, sqlca: &mut Sqlca) {
        sqlca.reset();
        match self.conn.execute_batch("COMMIT") {
            Ok(()) => sqlca.set_sqlcode(0),
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn rollback(&mut self, sqlca: &mut Sqlca) {
        sqlca.reset();
        match self.conn.execute_batch("ROLLBACK") {
            Ok(()) => sqlca.set_sqlcode(0),
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn declare_cursor(
        &mut self,
        name: &str,
        sql: &str,
        params: &[HostVar<'_>],
        sqlca: &mut Sqlca,
    ) {
        sqlca.reset();
        let values: Vec<Value> = params.iter().map(host_var_to_value).collect();
        self.cursors.insert(
            name.to_uppercase(),
            CursorState {
                sql: sql.to_string(),
                param_values: values,
                rows: Vec::new(),
                position: 0,
            },
        );
        sqlca.set_sqlcode(0);
    }

    fn open_cursor(&mut self, name: &str, sqlca: &mut Sqlca) {
        sqlca.reset();
        let key = name.to_uppercase();

        let Some(cursor) = self.cursors.get(&key) else {
            sqlca.set_error(-501, "CURSOR NOT DECLARED");
            return;
        };

        let rewritten = rewrite_placeholders(&cursor.sql);
        let param_values = cursor.param_values.clone();

        // Count output columns from the SQL text (count commas in SELECT list).
        // DuckDB's column_count() panics before execution, so we infer from results.
        let result = self.conn.prepare(&rewritten).and_then(|mut stmt| {
            let mut rows = stmt.query(params_from_iter(param_values.iter()))?;
            let mut buffered = Vec::new();
            while let Some(row) = rows.next()? {
                // Probe columns until we get an error to find the count.
                let mut row_values = Vec::new();
                for i in 0.. {
                    match row.get::<_, Value>(i) {
                        Ok(val) => row_values.push(val),
                        Err(_) => break,
                    }
                }
                buffered.push(row_values);
            }
            Ok(buffered)
        });

        match result {
            Ok(buffered) => {
                if let Some(cursor) = self.cursors.get_mut(&key) {
                    cursor.rows = buffered;
                    cursor.position = 0;
                }
                sqlca.set_sqlcode(0);
            }
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn fetch_cursor(
        &mut self,
        name: &str,
        into_fields: &mut [HostVarMut<'_>],
        sqlca: &mut Sqlca,
    ) {
        sqlca.reset();
        let key = name.to_uppercase();

        let Some(cursor) = self.cursors.get_mut(&key) else {
            sqlca.set_error(-501, "CURSOR NOT OPEN");
            return;
        };

        if cursor.position >= cursor.rows.len() {
            sqlca.set_sqlcode(100);
            return;
        }

        let row = &cursor.rows[cursor.position];
        for (i, field) in into_fields.iter_mut().enumerate() {
            if i < row.len() {
                value_to_host_var(&row[i], field);
            }
        }
        cursor.position += 1;
        sqlca.set_sqlcode(0);
    }

    fn close_cursor(&mut self, name: &str, sqlca: &mut Sqlca) {
        sqlca.reset();
        let key = name.to_uppercase();
        if self.cursors.remove(&key).is_some() {
            sqlca.set_sqlcode(0);
        } else {
            sqlca.set_error(-501, "CURSOR NOT OPEN");
        }
    }

    fn prepare(
        &mut self,
        name: &str,
        sql_source: &HostVar<'_>,
        sqlca: &mut Sqlca,
    ) {
        sqlca.reset();
        let sql_text = std::str::from_utf8(sql_source.as_bytes())
            .unwrap_or("")
            .trim()
            .to_string();

        if sql_text.is_empty() {
            sqlca.set_error(-518, "PREPARE: EMPTY SQL TEXT");
            return;
        }

        // Validate by preparing against DuckDB (but don't keep the stmt).
        let rewritten = rewrite_placeholders(&sql_text);
        match self.conn.prepare(&rewritten) {
            Ok(_) => {
                self.prepared.insert(name.to_uppercase(), sql_text);
                sqlca.set_sqlcode(0);
            }
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn execute_prepared(
        &mut self,
        name: &str,
        params: &[HostVar<'_>],
        sqlca: &mut Sqlca,
    ) {
        sqlca.reset();
        let key = name.to_uppercase();

        let Some(sql_text) = self.prepared.get(&key).cloned() else {
            sqlca.set_error(-518, "STATEMENT NOT PREPARED");
            return;
        };

        let rewritten = rewrite_placeholders(&sql_text);
        let values: Vec<Value> = params.iter().map(host_var_to_value).collect();

        match self
            .conn
            .prepare(&rewritten)
            .and_then(|mut stmt| stmt.execute(params_from_iter(values.iter())))
        {
            Ok(count) => {
                sqlca.set_sqlcode(0);
                #[allow(clippy::cast_possible_wrap)]
                {
                    sqlca.sqlerrd[2] = count as i32;
                }
            }
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn execute_immediate(
        &mut self,
        sql_source: &HostVar<'_>,
        sqlca: &mut Sqlca,
    ) {
        sqlca.reset();
        let sql_text = std::str::from_utf8(sql_source.as_bytes())
            .unwrap_or("")
            .trim()
            .to_string();

        if sql_text.is_empty() {
            sqlca.set_error(-518, "EXECUTE IMMEDIATE: EMPTY SQL TEXT");
            return;
        }

        match self.conn.execute_batch(&sql_text) {
            Ok(()) => sqlca.set_sqlcode(0),
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn savepoint(&mut self, name: &str, sqlca: &mut Sqlca) {
        sqlca.reset();
        match self
            .conn
            .execute_batch(&format!("SAVEPOINT {name}"))
        {
            Ok(()) => sqlca.set_sqlcode(0),
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }

    fn rollback_to_savepoint(&mut self, name: &str, sqlca: &mut Sqlca) {
        sqlca.reset();
        match self
            .conn
            .execute_batch(&format!("ROLLBACK TO SAVEPOINT {name}"))
        {
            Ok(()) => sqlca.set_sqlcode(0),
            Err(e) => sqlca.set_error(-1, &e.to_string()),
        }
    }
}
