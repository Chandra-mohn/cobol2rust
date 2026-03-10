//! Prelude module: re-exports everything transpiled COBOL programs need.
//!
//! Usage: `use cobol_runtime::prelude::*;`

// Core traits and types
pub use cobol_core::{
    ArithError, ArithMode, CallError, CobolDialect, CobolError, CobolField, CobolGroup,
    CobolNumeric, CobolNumericEdited, CodePage, CollatingSequence, DataCategory, DiagnosticLevel,
    EditSymbol, FileError, NumProc, RoundingMode, RuntimeConfig, SortError,
};

// EBCDIC conversion utilities
pub use cobol_core::ebcdic::{
    ascii_to_ebcdic, ascii_to_ebcdic_byte, ascii_to_ebcdic_copy, ebcdic_collating_weights,
    ebcdic_to_ascii, ebcdic_to_ascii_byte, ebcdic_to_ascii_copy,
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
    AlphaEditSymbol, AlphanumericEdited, CobolArray, CobolVarArray, Comp1Float, Comp2Float,
    CompBinary, ConditionValue, FigurativeConstant, Level88Predicate, Level88Value,
    NumericEdited, PackedDecimal, PicA, PicX, RedefinesGroup, ZonedDecimal,
    sync_field_to_redefines, sync_redefines_to_field,
};

// MOVE engine
pub use cobol_move::{
    MoveDiagnostic, MoveWarning, MoveWarningKind, cobol_initialize, cobol_initialize_group,
    cobol_initialize_numeric, cobol_move, cobol_move_numeric, is_legal_move,
    move_alphanumeric_literal, move_corresponding, move_corresponding_by_name,
    move_numeric_literal,
};

// Arithmetic verbs
pub use cobol_arithmetic::{
    ArithResult, cobol_add, cobol_add_giving, cobol_checked_div, cobol_compute, cobol_divide,
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

// Sort/merge
#[cfg(feature = "sort")]
pub use cobol_sort::{
    CobolMergeEngine, CobolSortEngine, CollatingTable, Releaser, Returner, SortConfig,
    SortKeySpec, SortKeyType, SortReturn, sort_with_input_procedure, sort_with_output_procedure,
    sort_with_procedures,
};

// SQL runtime
#[cfg(feature = "sql")]
pub use cobol_sql::{CobolSqlRuntime, HostVar, HostVarMut, Sqlca, SqlErrm, SqlWarnings};

// Decimal type, macro, and conversion traits
pub use rust_decimal::Decimal;
pub use rust_decimal::prelude::ToPrimitive;
pub use rust_decimal_macros::dec;

// INSPECT verb
pub use crate::inspect::{
    BeforeAfter, InspectWhat, ReplacingSpec, TallyingSpec, cobol_inspect_converting,
    cobol_inspect_replacing, cobol_inspect_tallying,
};

// STRING verb
pub use crate::string_verb::{StringDelimiter, StringResult, StringSourceSpec, cobol_string};

// UNSTRING verb
pub use crate::unstring_verb::{
    UnstringDelimSpec, UnstringResult, cobol_unstring, cobol_unstring_simple,
};

// Intrinsic functions
pub use crate::intrinsics::{
    // Numeric
    cobol_function_abs, cobol_function_acos, cobol_function_asin, cobol_function_atan,
    cobol_function_cos, cobol_function_factorial, cobol_function_integer,
    cobol_function_integer_part, cobol_function_length, cobol_function_log,
    cobol_function_log10, cobol_function_max, cobol_function_min, cobol_function_mod,
    cobol_function_char, cobol_function_numval, cobol_function_numval_c, cobol_function_ord,
    cobol_function_ord_max, cobol_function_ord_min, cobol_function_random,
    cobol_function_rem, cobol_function_sin, cobol_function_sqrt, cobol_function_tan,
    // String
    cobol_function_concatenate, cobol_function_current_date, cobol_function_lower_case,
    cobol_function_reverse, cobol_function_trim, cobol_function_upper_case,
    cobol_function_when_compiled,
};

// Reference modification
pub use crate::ref_mod::{ref_mod_read, ref_mod_read_to_end, ref_mod_write, ref_mod_write_to_end};

// CALL/CANCEL
pub use crate::call::{
    CallDispatcher, CallParam, CallParamMode, CallableProgram, call_param_by_content,
    call_param_by_ref, call_param_by_value, call_param_omitted, cobol_call, cobol_cancel,
};

// Decimal-to-usize helper for array subscript conversion
#[inline]
pub fn decimal_to_usize(d: Decimal) -> usize {
    // Truncate to integer, then convert; default to 0 on failure
    let truncated = d.trunc();
    let s = truncated.to_string();
    s.parse::<usize>().unwrap_or(0)
}

// Runtime program lifecycle
pub use crate::display::{
    accept_date, accept_date_yyyymmdd, accept_day, accept_day_of_week, accept_day_yyyyddd,
    accept_from_sysin, accept_time, display_upon_syserr, display_upon_sysout,
};
pub use crate::perform_stack::PerformStack;
pub use crate::program::CobolProgram;
pub use crate::special_regs;
