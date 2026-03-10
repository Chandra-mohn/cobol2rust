//! COBOL EXEC SQL runtime support.
//!
//! Provides a trait-based database abstraction (`CobolSqlRuntime`) that
//! transpiled COBOL programs call for all SQL operations. Users implement
//! the trait for their target database (Postgres, DB2, SQLite, etc.).
//!
//! Also provides the `Sqlca` (SQL Communication Area) struct that mirrors
//! the IBM COBOL SQLCA layout, and `HostVar` for bidirectional host
//! variable binding.

mod host_var;
mod runtime;
mod sqlca;

#[cfg(feature = "duckdb")]
mod duckdb_runtime;

pub use host_var::{HostVar, HostVarMut};
pub use runtime::CobolSqlRuntime;
pub use sqlca::{Sqlca, SqlErrm, SqlWarnings};

#[cfg(feature = "duckdb")]
pub use duckdb_runtime::DuckDbRuntime;
