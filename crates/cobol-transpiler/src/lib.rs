//! cobol-transpiler: COBOL-to-Rust transpiler.
//!
//! Parses COBOL source using ANTLR4-generated lexer/parser, builds a typed AST,
//! resolves symbols, and generates idiomatic Rust code targeting cobol-runtime.

pub mod ast;
pub mod codegen;
pub mod diagnostics;
pub mod error;
mod generated;
pub mod parser;
pub mod symbol_table;
pub mod transpile;
