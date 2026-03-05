//! cobol-runtime: Top-level COBOL runtime crate.
//!
//! Re-exports all public types through a prelude module, provides
//! program lifecycle management, special registers, and DISPLAY/ACCEPT.

pub mod display;
pub mod perform_stack;
pub mod prelude;
pub mod program;
pub mod special_regs;
