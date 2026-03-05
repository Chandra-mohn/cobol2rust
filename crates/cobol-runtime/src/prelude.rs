//! Prelude module: re-exports everything transpiled COBOL programs need.
//!
//! Usage: `use cobol_runtime::prelude::*;`

// Core traits and types
pub use cobol_core::{
    ArithError, ArithMode, CobolDialect, CobolError, CobolField, CobolGroup, CobolNumeric,
    CobolNumericEdited, CollatingSequence, DataCategory, DiagnosticLevel, EditSymbol, FileError,
    NumProc, RoundingMode, RuntimeConfig, SortError,
};

// Core decimal utilities
pub use cobol_core::decimal_ext::{
    left_truncate_to_precision, max_for_precision, round_decimal, truncate_decimal, would_overflow,
};

// Numeric parsing utilities
pub use cobol_core::numeric_parse::{
    parse_numeric_display, parse_with_implied_decimal, parse_zoned_decimal,
};

// Data types
pub use cobol_types::{
    CobolArray, CompBinary, ConditionValue, FigurativeConstant, Level88Predicate, Level88Value,
    NumericEdited, PackedDecimal, PicA, PicX, ZonedDecimal,
};

// MOVE engine
pub use cobol_move::{
    MoveDiagnostic, MoveWarning, MoveWarningKind, cobol_initialize, cobol_initialize_group,
    cobol_initialize_numeric, cobol_move, cobol_move_numeric, is_legal_move,
    move_corresponding, move_corresponding_by_name,
};

// Arithmetic verbs
pub use cobol_arithmetic::{
    ArithResult, cobol_add, cobol_add_giving, cobol_compute, cobol_divide,
    cobol_divide_by_giving, cobol_divide_giving, cobol_multiply, cobol_multiply_giving,
    cobol_subtract, cobol_subtract_giving, store_arithmetic_result,
};

// File I/O
#[cfg(feature = "io")]
pub use cobol_io::{
    CobolFile, FileAccessMode, FileOpenMode, FileOrganization, FileResolver, FileStatusCode,
    RelativeFile, SequentialFile,
};
#[cfg(feature = "io")]
pub use cobol_io::IndexedFile;

// Decimal type and macro
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;

// Runtime program lifecycle
pub use crate::display::{accept_from_sysin, display_upon_syserr, display_upon_sysout};
pub use crate::perform_stack::PerformStack;
pub use crate::program::CobolProgram;
pub use crate::special_regs;
