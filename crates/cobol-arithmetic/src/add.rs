use cobol_core::config::{RuntimeConfig, RoundingMode};
use cobol_core::traits::CobolNumeric;

use crate::result::ArithResult;
use crate::store::store_arithmetic_result;

/// ADD src TO dest [ROUNDED]
///
/// Adds the value of `src` to the current value of `dest` and stores back in `dest`.
pub fn cobol_add(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let result = src.to_decimal() + dest.to_decimal();
    store_arithmetic_result(result, dest, rounded, config)
}

/// ADD src TO src2 GIVING dest [ROUNDED]
///
/// Adds `src` and `src2`, stores result in `dest`.
pub fn cobol_add_giving(
    src: &dyn CobolNumeric,
    src2: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let result = src.to_decimal() + src2.to_decimal();
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
    fn add_basic() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(100));
        let mut dest = PackedDecimal::new(5, 0, true);
        dest.set_decimal(dec!(200));

        let result = cobol_add(&src, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(300));
    }

    #[test]
    fn add_with_rounding() {
        // Use scale=3 source to preserve 1.005
        let mut src = PackedDecimal::new(5, 3, true);
        src.set_decimal(dec!(1.005));
        let mut dest = PackedDecimal::new(5, 2, true);
        dest.set_decimal(dec!(0));

        // ADD 1.005 TO 0, ROUNDED -> 1.01 (nearest away from zero)
        let result = cobol_add(
            &src,
            &mut dest,
            Some(RoundingMode::NearestAwayFromZero),
            &default_config(),
        );
        assert!(!result.size_error);
        // 0 + 1.005 rounded to scale 2 = 1.01
        assert_eq!(dest.to_decimal(), dec!(1.01));
    }

    #[test]
    fn add_size_error() {
        let mut src = PackedDecimal::new(3, 0, true);
        src.set_decimal(dec!(999));
        let mut dest = PackedDecimal::new(3, 0, true);
        dest.set_decimal(dec!(1));

        let result = cobol_add(&src, &mut dest, None, &default_config());
        // 999 + 1 = 1000, doesn't fit in PIC 9(3) -> size error
        assert!(result.size_error);
        // But COBOL left-truncates: 1000 -> 000
        assert_eq!(dest.to_decimal(), dec!(0));
    }

    #[test]
    fn add_giving() {
        let mut src1 = PackedDecimal::new(5, 0, true);
        src1.set_decimal(dec!(100));
        let mut src2 = PackedDecimal::new(5, 0, true);
        src2.set_decimal(dec!(200));
        let mut dest = PackedDecimal::new(5, 0, true);

        let result = cobol_add_giving(&src1, &src2, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(300));
    }

    #[test]
    fn add_left_truncation() {
        // 12345 + 1 stored in PIC 9(3) = 346 (left-truncation)
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(12345));
        let mut dest = PackedDecimal::new(3, 0, true);
        dest.set_decimal(dec!(1));

        let result = cobol_add(&src, &mut dest, None, &default_config());
        assert!(result.size_error);
        assert_eq!(dest.to_decimal(), dec!(346));
    }
}
