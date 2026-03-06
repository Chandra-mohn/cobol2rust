//! cobol-types: COBOL data type implementations.
//!
//! Provides `PackedDecimal` (COMP-3), `PicX` (alphanumeric), `CompBinary` (COMP/COMP-5),
//! `CobolArray` (OCCURS), Level88 predicates, figurative constants, and `PicA` (alphabetic).

pub mod alpha_edited;
pub mod cobol_array;
pub mod comp_binary;
pub mod comp1_float;
pub mod comp2_float;
pub mod figurative;
pub mod level88;
pub mod packed_decimal;
pub mod pic_a;
pub mod numeric_edited;
pub mod pic_x;
pub mod redefines;
pub mod var_array;
pub mod zoned_decimal;

// Re-export key types at crate root
pub use alpha_edited::{AlphaEditSymbol, AlphanumericEdited};
pub use cobol_array::CobolArray;
pub use comp_binary::CompBinary;
pub use comp1_float::Comp1Float;
pub use comp2_float::Comp2Float;
pub use figurative::FigurativeConstant;
pub use level88::{ConditionValue, Level88Predicate, Level88Value};
pub use packed_decimal::PackedDecimal;
pub use pic_a::PicA;
pub use numeric_edited::NumericEdited;
pub use pic_x::PicX;
pub use redefines::{RedefinesGroup, sync_field_to_redefines, sync_redefines_to_field};
pub use var_array::CobolVarArray;
pub use zoned_decimal::ZonedDecimal;
