//! Error types for the COBOL transpiler.
//!
//! Covers all phases: preprocessing, parsing (ANTLR4), semantic analysis,
//! and code generation.

use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

/// Unified result type for the transpiler.
pub type Result<T> = std::result::Result<T, TranspileError>;

/// Top-level transpiler error.
#[derive(Debug, Error, Diagnostic)]
pub enum TranspileError {
    /// Error during COBOL source preprocessing (fixed-format handling).
    #[error("preprocess error at line {line}: {message}")]
    Preprocess {
        line: usize,
        message: String,
    },

    /// Error from the ANTLR4 lexer/parser.
    #[error("parse error at line {line}, column {column}: {message}")]
    Parse {
        line: usize,
        column: usize,
        message: String,
    },

    /// ANTLR4 internal error (unrecoverable).
    #[error("ANTLR4 error: {message}")]
    AntlrError {
        message: String,
    },

    /// Invalid PIC clause.
    #[error("invalid PIC clause '{clause}': {reason}")]
    InvalidPic {
        clause: String,
        reason: String,
    },

    /// Semantic analysis error (symbol resolution, type checking, etc.).
    #[error("semantic error: {message}")]
    Semantic {
        message: String,
    },

    /// Unresolved reference to a data item or paragraph.
    #[error("unresolved reference '{name}'")]
    UnresolvedReference {
        name: String,
    },

    /// Duplicate symbol definition.
    #[error("duplicate definition of '{name}'")]
    DuplicateDefinition {
        name: String,
    },

    /// Code generation error.
    #[error("codegen error: {message}")]
    CodeGen {
        message: String,
    },

    /// Unsupported COBOL feature encountered during transpilation.
    #[error("unsupported feature: {feature}")]
    Unsupported {
        feature: String,
    },

    /// Copybook not found in any search path.
    #[error("copybook '{name}' not found (searched: {paths_searched:?})")]
    CopyNotFound {
        name: String,
        paths_searched: Vec<PathBuf>,
    },

    /// Cyclic COPY dependency detected.
    #[error("cyclic COPY dependency: {}", chain.join(" -> "))]
    CopyCyclic {
        chain: Vec<String>,
    },

    /// COPY nesting depth exceeded.
    #[error("COPY nesting depth {depth} exceeds maximum {max}")]
    CopyDepthExceeded {
        depth: usize,
        max: usize,
    },

    /// Malformed REPLACING clause in COPY statement.
    #[error("malformed REPLACING clause in COPY '{copybook}': {message}")]
    CopyReplacingInvalid {
        copybook: String,
        message: String,
    },
}
