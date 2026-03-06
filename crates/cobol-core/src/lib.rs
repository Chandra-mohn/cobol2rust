//! cobol-core: Foundation traits, configuration, and error types.
//!
//! This crate defines the canonical trait hierarchy (`CobolField`, `CobolNumeric`, etc.),
//! runtime configuration, and shared error types used by all other runtime crates.

pub mod category;
pub mod config;
pub mod decimal_ext;
pub mod ebcdic;
pub mod editing;
pub mod error;
pub mod numeric_parse;
pub mod traits;

// Re-export key types at crate root for convenience
pub use category::DataCategory;
pub use config::{
    ArithMode, CobolDialect, CollatingSequence, DiagnosticLevel, NumProc, RoundingMode,
    RuntimeConfig,
};
pub use ebcdic::CodePage;
pub use editing::EditSymbol;
pub use error::{ArithError, CallError, CobolError, FileError, SortError};
pub use traits::{CobolField, CobolGroup, CobolNumeric, CobolNumericEdited};
