use cobol_core::config::{RuntimeConfig, RoundingMode};
use cobol_core::traits::CobolNumeric;

use crate::result::ArithResult;
use crate::store::store_arithmetic_result;

/// MULTIPLY src BY dest [ROUNDED]
///
/// Multiplies `dest` by `src` and stores back in `dest`.
pub fn cobol_multiply(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let result = src.to_decimal() * dest.to_decimal();
    store_arithmetic_result(result, dest, rounded, config)
}

/// MULTIPLY src BY src2 GIVING dest [ROUNDED]
pub fn cobol_multiply_giving(
    src: &dyn CobolNumeric,
    src2: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let result = src.to_decimal() * src2.to_decimal();
    store_arithmetic_result(result, dest, rounded, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_types::PackedDecimal;
    use rust_decimal_macros::dec;

    fn default_config() -> RuntimeConfig {
        RuntimeConfig::default()
    }

    #[test]
    fn multiply_basic() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(12));
        let mut dest = PackedDecimal::new(5, 0, true);
        dest.set_decimal(dec!(10));

        let result = cobol_multiply(&src, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(120));
    }

    #[test]
    fn multiply_giving() {
        let mut src1 = PackedDecimal::new(5, 0, true);
        src1.set_decimal(dec!(12));
        let mut src2 = PackedDecimal::new(5, 0, true);
        src2.set_decimal(dec!(10));
        let mut dest = PackedDecimal::new(5, 0, true);

        let result = cobol_multiply_giving(&src1, &src2, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(120));
    }

    #[test]
    fn multiply_size_error() {
        let mut src = PackedDecimal::new(3, 0, true);
        src.set_decimal(dec!(100));
        let mut dest = PackedDecimal::new(3, 0, true);
        dest.set_decimal(dec!(100));

        let result = cobol_multiply(&src, &mut dest, None, &default_config());
        // 100 * 100 = 10000, doesn't fit in PIC 9(3)
        assert!(result.size_error);
    }

    #[test]
    fn multiply_with_rounding() {
        let mut src = PackedDecimal::new(5, 2, true);
        src.set_decimal(dec!(1.23));
        let mut dest = PackedDecimal::new(5, 2, true);
        dest.set_decimal(dec!(4.56));

        let result = cobol_multiply(
            &src,
            &mut dest,
            Some(RoundingMode::NearestAwayFromZero),
            &default_config(),
        );
        assert!(!result.size_error);
        // 1.23 * 4.56 = 5.6088 -> rounded to 2 places = 5.61
        assert_eq!(dest.to_decimal(), dec!(5.61));
    }
}
