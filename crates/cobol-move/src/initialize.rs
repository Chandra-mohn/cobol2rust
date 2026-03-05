use cobol_core::category::DataCategory;
use cobol_core::traits::{CobolField, CobolGroup, CobolNumeric};

/// COBOL INITIALIZE verb: set fields to default values.
///
/// Rules:
/// - Alphabetic/Alphanumeric/AlphanumericEdited -> SPACES
/// - Numeric/NumericEdited -> ZEROS
/// - Group -> recursively initialize elementary fields
///
/// The `REPLACING` phrase is handled by the caller providing overrides.
pub fn cobol_initialize(field: &mut dyn CobolField) {
    match field.category() {
        DataCategory::Alphabetic
        | DataCategory::Alphanumeric
        | DataCategory::AlphanumericEdited
        | DataCategory::National => {
            field.fill_bytes(b' ');
        }
        DataCategory::Numeric | DataCategory::NumericEdited => {
            field.initialize_default();
        }
        DataCategory::Group => {
            // For groups, we call initialize_default which should recursively
            // initialize subordinate fields. The CobolGroup trait handles this.
            field.initialize_default();
        }
    }
}

/// Initialize a numeric field to zero.
pub fn cobol_initialize_numeric(field: &mut dyn CobolNumeric) {
    field.set_decimal(rust_decimal::Decimal::ZERO);
}

/// Initialize a group's elementary fields according to their categories.
pub fn cobol_initialize_group(group: &mut dyn CobolGroup) {
    for field in group.elementary_fields_mut() {
        cobol_initialize(field);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_types::{PackedDecimal, PicX};
    use rust_decimal_macros::dec;

    #[test]
    fn initialize_alphanumeric_fills_spaces() {
        let mut f = PicX::new(5, b"HELLO");
        cobol_initialize(&mut f);
        assert_eq!(f.as_bytes(), b"     ");
    }

    #[test]
    fn initialize_numeric_fills_zero() {
        let mut f = PackedDecimal::new(5, 2, true);
        f.set_decimal(dec!(123.45));
        cobol_initialize(&mut f);
        assert_eq!(f.to_decimal(), dec!(0));
    }

    #[test]
    fn initialize_numeric_via_numeric_trait() {
        let mut f = PackedDecimal::new(5, 0, true);
        f.set_decimal(dec!(999));
        cobol_initialize_numeric(&mut f);
        assert_eq!(f.to_decimal(), dec!(0));
    }
}
