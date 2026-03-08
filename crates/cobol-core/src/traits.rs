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

    /// Write raw bytes into the field's storage, truncating or padding as needed.
    fn set_raw_bytes(&mut self, src: &[u8]) {
        let dest = self.as_bytes_mut();
        let copy_len = src.len().min(dest.len());
        dest[..copy_len].copy_from_slice(&src[..copy_len]);
        // Pad remainder with spaces
        for b in &mut dest[copy_len..] {
            *b = b' ';
        }
    }

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

    /// COBOL class condition: IS NUMERIC.
    ///
    /// For alphanumeric fields, checks if all bytes are digits (0x30-0x39)
    /// or spaces. For numeric fields, always true.
    fn is_numeric(&self) -> bool {
        self.as_bytes().iter().all(|&b| b.is_ascii_digit() || b == b' ' || b == b'+' || b == b'-' || b == b'.')
    }

    /// COBOL class condition: IS ALPHABETIC.
    ///
    /// Checks if all bytes are letters (A-Z, a-z) or spaces.
    fn is_alphabetic(&self) -> bool {
        self.as_bytes().iter().all(|&b| b.is_ascii_alphabetic() || b == b' ')
    }

    /// COBOL class condition: IS ALPHABETIC-LOWER.
    fn is_alphabetic_lower(&self) -> bool {
        self.as_bytes().iter().all(|&b| b.is_ascii_lowercase() || b == b' ')
    }

    /// COBOL class condition: IS ALPHABETIC-UPPER.
    fn is_alphabetic_upper(&self) -> bool {
        self.as_bytes().iter().all(|&b| b.is_ascii_uppercase() || b == b' ')
    }

    /// Set a numeric value into this field using the correct internal format.
    ///
    /// For numeric types (COMP-3, COMP, COMP-1, COMP-2, DISPLAY numeric),
    /// this calls `set_decimal` to write using the proper internal storage
    /// format (BCD, binary, float, etc.).
    ///
    /// Default implementation: formats as display string and writes raw bytes.
    /// Numeric types MUST override this.
    fn set_value_from_decimal(&mut self, value: Decimal) {
        let formatted = format!("{value}");
        let src = formatted.as_bytes();
        let dest = self.as_bytes_mut();
        let copy_len = src.len().min(dest.len());
        dest[..copy_len].copy_from_slice(&src[..copy_len]);
        dest[copy_len..].fill(b' ');
    }
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

    /// COBOL sign condition: IS POSITIVE (value > 0).
    fn is_positive(&self) -> bool {
        self.to_decimal() > Decimal::ZERO
    }

    /// COBOL sign condition: IS NEGATIVE (value < 0).
    fn is_negative(&self) -> bool {
        self.to_decimal() < Decimal::ZERO
    }

    /// COBOL sign condition: IS ZERO (value == 0).
    fn is_zero(&self) -> bool {
        self.to_decimal() == Decimal::ZERO
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
