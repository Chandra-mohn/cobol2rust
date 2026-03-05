//! cobol-arithmetic: COBOL arithmetic verbs.
//!
//! Implements ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE with
//! ROUNDED, ON SIZE ERROR, and left-truncation semantics.

pub mod add;
pub mod compute;
pub mod divide;
pub mod multiply;
pub mod result;
pub mod store;
pub mod subtract;

// Re-export public API
pub use add::{cobol_add, cobol_add_giving};
pub use compute::cobol_compute;
pub use divide::{cobol_divide, cobol_divide_by_giving, cobol_divide_giving};
pub use multiply::{cobol_multiply, cobol_multiply_giving};
pub use result::ArithResult;
pub use store::store_arithmetic_result;
pub use subtract::{cobol_subtract, cobol_subtract_giving};
