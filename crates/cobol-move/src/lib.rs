//! cobol-move: COBOL MOVE engine.
//!
//! Implements the MOVE verb with category-dispatched type coercion,
//! MOVE CORRESPONDING, INITIALIZE, and diagnostic reporting.

pub mod corresponding;
pub mod diagnostics;
pub mod engine;
pub mod initialize;

// Re-export public API
pub use corresponding::{move_corresponding, move_corresponding_by_name};
pub use diagnostics::{MoveDiagnostic, MoveWarning, MoveWarningKind};
pub use engine::{
    cobol_move, cobol_move_numeric, is_legal_move, move_alphanumeric_literal,
    move_numeric_literal,
};
pub use initialize::{cobol_initialize, cobol_initialize_group, cobol_initialize_numeric};
