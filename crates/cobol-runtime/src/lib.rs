//! cobol-runtime: Top-level COBOL runtime crate.
//!
//! Re-exports all public types through a prelude module, provides
//! program lifecycle management, special registers, and DISPLAY/ACCEPT.

pub mod call;
pub mod display;
pub mod inspect;
pub mod intrinsics;
pub mod perform_stack;
pub mod prelude;
pub mod program;
pub mod special_regs;
pub mod ref_mod;
pub mod string_verb;
pub mod unstring_verb;

// Re-export rust_decimal crate so generated code can resolve the `dec!()` macro.
// The macro expands to `rust_decimal::Decimal::from_parts(...)`, which requires
// `rust_decimal` as a visible path in the calling crate.
pub use rust_decimal;
