//! The `CobolSqlRuntime` trait -- core database abstraction.
//!
//! Transpiled COBOL programs call methods on this trait for all SQL
//! operations. Users provide an implementation for their target database
//! (Postgres, DB2, SQLite, etc.).

use crate::host_var::{HostVar, HostVarMut};
use crate::sqlca::Sqlca;

/// Core SQL runtime trait -- implement per database backend.
///
/// All methods receive a mutable `Sqlca` reference and must set
/// `sqlca.sqlcode` (and optionally other SQLCA fields) before returning.
///
/// The `sql` parameter contains the parameterized SQL string with
/// host variable references replaced by positional placeholders.
/// The placeholder style (`?`, `$1`, `:1`, etc.) is determined by
/// the implementation, which receives the original SQL from the
/// transpiler with `?` placeholders.
pub trait CobolSqlRuntime {
    // -- Core DML (Phase 1) --

    /// Execute a query that returns a single row (SELECT INTO).
    ///
    /// The runtime reads input params from `params`, executes the query,
    /// and writes result columns into `into_fields` in order.
    ///
    /// Expected SQLCA behavior:
    /// - SQLCODE 0: row found, fields populated
    /// - SQLCODE 100: no row found
    /// - SQLCODE < 0: error
    fn exec_query(
        &mut self,
        sql: &str,
        params: &[HostVar<'_>],
        into_fields: &mut [HostVarMut<'_>],
        sqlca: &mut Sqlca,
    );

    /// Execute a data modification statement (INSERT, UPDATE, DELETE).
    ///
    /// Sets `sqlca.sqlerrd[2]` to the number of rows affected.
    fn exec_update(
        &mut self,
        sql: &str,
        params: &[HostVar<'_>],
        sqlca: &mut Sqlca,
    );

    /// Commit the current transaction.
    fn commit(&mut self, sqlca: &mut Sqlca);

    /// Rollback the current transaction.
    fn rollback(&mut self, sqlca: &mut Sqlca);

    // -- Cursors (Phase 2) --

    /// Declare a named cursor for a SELECT statement.
    ///
    /// The cursor is not opened yet -- call `open_cursor` to activate it.
    /// Parameters are bound at declare time (IBM COBOL behavior).
    fn declare_cursor(
        &mut self,
        name: &str,
        sql: &str,
        params: &[HostVar<'_>],
        sqlca: &mut Sqlca,
    );

    /// Open a previously declared cursor.
    fn open_cursor(&mut self, name: &str, sqlca: &mut Sqlca);

    /// Fetch the next row from an open cursor.
    ///
    /// Expected SQLCA behavior:
    /// - SQLCODE 0: row fetched, fields populated
    /// - SQLCODE 100: end of result set
    /// - SQLCODE < 0: error
    fn fetch_cursor(
        &mut self,
        name: &str,
        into_fields: &mut [HostVarMut<'_>],
        sqlca: &mut Sqlca,
    );

    /// Close an open cursor.
    fn close_cursor(&mut self, name: &str, sqlca: &mut Sqlca);

    // -- Dynamic SQL (Phase 3) --

    /// Prepare a dynamic SQL statement from a host variable value.
    fn prepare(
        &mut self,
        name: &str,
        sql_source: &HostVar<'_>,
        sqlca: &mut Sqlca,
    );

    /// Execute a previously prepared statement with parameters.
    fn execute_prepared(
        &mut self,
        name: &str,
        params: &[HostVar<'_>],
        sqlca: &mut Sqlca,
    );

    /// Execute a dynamic SQL string immediately (no prepare step).
    fn execute_immediate(
        &mut self,
        sql_source: &HostVar<'_>,
        sqlca: &mut Sqlca,
    );

    /// Create a savepoint.
    fn savepoint(&mut self, name: &str, sqlca: &mut Sqlca);

    /// Rollback to a previously created savepoint.
    fn rollback_to_savepoint(&mut self, name: &str, sqlca: &mut Sqlca);
}
