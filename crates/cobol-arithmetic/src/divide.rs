use cobol_core::config::{RuntimeConfig, RoundingMode};
use cobol_core::traits::CobolNumeric;

use crate::result::ArithResult;
use crate::store::store_arithmetic_result;

/// DIVIDE src INTO dest [ROUNDED] [REMAINDER rem]
///
/// Divides `dest` by `src`, stores quotient in `dest`, optionally stores remainder.
pub fn cobol_divide(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    remainder: Option<&mut dyn CobolNumeric>,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let divisor = src.to_decimal();
    let dividend = dest.to_decimal();

    if divisor.is_zero() {
        return ArithResult::size_error();
    }

    let quotient = dividend / divisor;

    // If REMAINDER is specified, compute it
    if let Some(rem) = remainder {
        // REMAINDER = dividend - (truncated_quotient * divisor)
        let trunc_quotient = cobol_core::decimal_ext::truncate_decimal(quotient, dest.scale());
        let rem_value = dividend - (trunc_quotient * divisor);
        store_arithmetic_result(rem_value, rem, None, config);
    }

    store_arithmetic_result(quotient, dest, rounded, config)
}

/// DIVIDE src INTO src2 GIVING dest [ROUNDED] [REMAINDER rem]
pub fn cobol_divide_giving(
    src: &dyn CobolNumeric,
    src2: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    remainder: Option<&mut dyn CobolNumeric>,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let divisor = src.to_decimal();
    let dividend = src2.to_decimal();

    if divisor.is_zero() {
        return ArithResult::size_error();
    }

    let quotient = dividend / divisor;

    if let Some(rem) = remainder {
        let trunc_quotient = cobol_core::decimal_ext::truncate_decimal(quotient, dest.scale());
        let rem_value = dividend - (trunc_quotient * divisor);
        store_arithmetic_result(rem_value, rem, None, config);
    }

    store_arithmetic_result(quotient, dest, rounded, config)
}

/// DIVIDE src BY src2 GIVING dest [ROUNDED]
///
/// Convenience for DIVIDE a BY b GIVING c (src/src2 stored in dest).
pub fn cobol_divide_by_giving(
    dividend: &dyn CobolNumeric,
    divisor: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let d = divisor.to_decimal();
    if d.is_zero() {
        return ArithResult::size_error();
    }
    let quotient = dividend.to_decimal() / d;
    store_arithmetic_result(quotient, dest, rounded, config)
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
    fn divide_basic() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(5));
        let mut dest = PackedDecimal::new(5, 0, true);
        dest.set_decimal(dec!(100));

        let result = cobol_divide(&src, &mut dest, None, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(20));
    }

    #[test]
    fn divide_by_zero() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(0));
        let mut dest = PackedDecimal::new(5, 0, true);
        dest.set_decimal(dec!(100));

        let result = cobol_divide(&src, &mut dest, None, None, &default_config());
        assert!(result.size_error);
    }

    #[test]
    fn divide_with_remainder() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(3));
        let mut dest = PackedDecimal::new(5, 0, true);
        dest.set_decimal(dec!(10));
        let mut rem = PackedDecimal::new(5, 0, true);

        let result = cobol_divide(&src, &mut dest, Some(&mut rem), None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(3)); // 10/3 = 3 (truncated)
        assert_eq!(rem.to_decimal(), dec!(1)); // 10 - 3*3 = 1
    }

    #[test]
    fn divide_giving() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(4));
        let mut src2 = PackedDecimal::new(5, 0, true);
        src2.set_decimal(dec!(100));
        let mut dest = PackedDecimal::new(5, 0, true);

        let result =
            cobol_divide_giving(&src, &src2, &mut dest, None, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(25));
    }

    #[test]
    fn divide_with_rounding() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(3));
        let mut dest = PackedDecimal::new(5, 2, true);
        dest.set_decimal(dec!(10));

        let result = cobol_divide(
            &src,
            &mut dest,
            None,
            Some(RoundingMode::NearestAwayFromZero),
            &default_config(),
        );
        assert!(!result.size_error);
        // 10/3 = 3.333... -> rounded to 2 places = 3.33
        assert_eq!(dest.to_decimal(), dec!(3.33));
    }
}
