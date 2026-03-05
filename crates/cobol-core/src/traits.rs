use std::cmp::Ordering;

use rust_decimal::Decimal;

use crate::category::DataCategory;
use crate::editing::EditSymbol;

/// Root trait: every COBOL data item implements this.
///
/// This is the universal interface for the MOVE engine, I/O, and display.
pub trait CobolField: std::fmt::Debug {
    /// The item's data category (determines MOVE/comparison rules).
    fn category(&self) -> DataCategory;

    /// Storage size in bytes (for record I/O, GROUP layout).
    fn byte_length(&self) -> usize;

    /// Raw storage bytes (for GROUP moves, I/O).
    fn as_bytes(&self) -> &[u8];

    /// Mutable raw storage bytes.
    fn as_bytes_mut(&mut self) -> &mut [u8];

    /// Display representation as bytes.
    ///
    /// For numeric items: the DISPLAY-format string (e.g., "00123").
    /// For alphanumeric items: same as `as_bytes()`.
    fn display_bytes(&self) -> Vec<u8>;

    /// Fill all storage bytes with a single value.
    fn fill_bytes(&mut self, byte: u8);

    /// JUSTIFIED RIGHT flag (only meaningful for alphabetic/alphanumeric).
    fn is_justified_right(&self) -> bool {
        false
    }

    /// BLANK WHEN ZERO flag (only meaningful for numeric/numeric-edited).
    fn has_blank_when_zero(&self) -> bool {
        false
    }

    /// COBOL INITIALIZE default: called by INITIALIZE verb.
    fn initialize_default(&mut self);
}

/// Numeric items: any field with a numeric value.
///
/// Covers PIC 9 DISPLAY, COMP, COMP-3, COMP-5, COMP-1, COMP-2.
pub trait CobolNumeric: CobolField {
    /// Get the value as a canonical Decimal.
    fn to_decimal(&self) -> Decimal;

    /// Set the value from a canonical Decimal (with truncation per COBOL rules).
    fn set_decimal(&mut self, value: Decimal);

    /// Number of decimal places (V position).
    fn scale(&self) -> u32;

    /// Total digit count (integer + decimal).
    fn precision(&self) -> u32;

    /// Whether this is a signed type (PIC S...).
    fn is_signed(&self) -> bool;

    /// Numeric comparison (decimal-point-aligned).
    fn compare_numeric(&self, other: &dyn CobolNumeric) -> Ordering {
        self.to_decimal().cmp(&other.to_decimal())
    }
}

/// Numeric-Edited items: numeric with editing mask (PIC Z, *, $, +, -, CR, DB).
pub trait CobolNumericEdited: CobolField {
    /// The editing mask (parsed PIC string).
    fn edit_mask(&self) -> &[EditSymbol];

    /// Number of integer digit positions.
    fn integer_positions(&self) -> u32;

    /// Number of decimal digit positions.
    fn decimal_positions(&self) -> u32;

    /// Set the display from a numeric value (apply editing).
    fn set_from_numeric(&mut self, value: Decimal, is_negative: bool);

    /// Extract the numeric value (de-edit) -- IBM extension.
    fn de_edit(&self) -> Option<Decimal>;
}

/// Group items: structured records with subordinate fields.
pub trait CobolGroup: CobolField {
    /// Access elementary fields by traversal order.
    fn elementary_fields(&self) -> Vec<&dyn CobolField>;

    /// Access elementary fields (mutable).
    fn elementary_fields_mut(&mut self) -> Vec<&mut dyn CobolField>;

    /// Lookup an elementary field by COBOL data-name.
    fn field_by_name(&self, name: &str) -> Option<&dyn CobolField>;

    /// Lookup an elementary field by name (mutable).
    fn field_by_name_mut(&mut self, name: &str) -> Option<&mut dyn CobolField>;

    /// List all elementary field names (COBOL data-names, uppercase).
    ///
    /// Used by MOVE CORRESPONDING at runtime to find matching fields.
    fn field_names(&self) -> Vec<String>;
}
