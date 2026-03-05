use rust_decimal::Decimal;
use rust_decimal::prelude::*;

use crate::config::RoundingMode;

/// Round a decimal value according to a COBOL rounding mode.
///
/// The `target_scale` is the number of decimal places in the destination field.
/// Rounding is applied to digits beyond `target_scale`.
pub fn round_decimal(value: Decimal, target_scale: u32, mode: RoundingMode) -> Decimal {
    match mode {
        RoundingMode::Truncation => truncate_decimal(value, target_scale),
        RoundingMode::NearestAwayFromZero => {
            value.round_dp_with_strategy(target_scale, RoundingStrategy::MidpointAwayFromZero)
        }
        RoundingMode::NearestEven => {
            value.round_dp_with_strategy(target_scale, RoundingStrategy::MidpointNearestEven)
        }
        RoundingMode::AwayFromZero => {
            value.round_dp_with_strategy(target_scale, RoundingStrategy::AwayFromZero)
        }
        RoundingMode::NearestTowardZero => {
            value.round_dp_with_strategy(target_scale, RoundingStrategy::MidpointTowardZero)
        }
        RoundingMode::TowardGreater => {
            value.round_dp_with_strategy(target_scale, RoundingStrategy::ToPositiveInfinity)
        }
        RoundingMode::TowardLesser => {
            value.round_dp_with_strategy(target_scale, RoundingStrategy::ToNegativeInfinity)
        }
        RoundingMode::Prohibited => {
            // If rounding is needed, this should trigger a SIZE ERROR
            // at the caller level. Here we just truncate.
            truncate_decimal(value, target_scale)
        }
    }
}

/// Truncate a decimal value to the given number of decimal places.
///
/// This is the default COBOL behavior when ROUNDED is not specified.
/// Digits beyond `target_scale` are simply dropped (toward zero).
pub fn truncate_decimal(value: Decimal, target_scale: u32) -> Decimal {
    value.round_dp_with_strategy(target_scale, RoundingStrategy::ToZero)
}

/// COBOL left-truncation: truncate the integer part to fit within `max_integer_digits`.
///
/// In COBOL, when a result exceeds the integer capacity of the destination,
/// the HIGH-ORDER (leftmost) digits are truncated. This is opposite to most
/// languages which would overflow or raise an error.
///
/// Example: 12345 stored in PIC 9(3) = 345 (not 123).
pub fn left_truncate_to_precision(value: Decimal, precision: u32, scale: u32) -> Decimal {
    let max_integer_digits = precision.saturating_sub(scale);
    if max_integer_digits == 0 {
        // Only fractional digits -- truncate the entire integer part
        return value.fract();
    }

    // Compute the modulus for the integer part
    let modulus = Decimal::from(10u64.pow(max_integer_digits));
    let sign = value.signum();
    let abs_value = value.abs();

    // Split into integer and fractional parts
    let integer_part = abs_value.trunc();
    let frac_part = abs_value.fract();

    // Left-truncate the integer part
    let truncated_integer = integer_part % modulus;

    // Reconstruct
    (truncated_integer + frac_part) * sign
}

/// Return the maximum value representable with the given precision and scale.
///
/// For PIC 9(5)V99, precision=7, scale=2, max = 99999.99
pub fn max_for_precision(precision: u32, scale: u32) -> Decimal {
    let total_nines = "9".repeat(precision as usize);
    let mut d = Decimal::from_str(&total_nines).unwrap_or(Decimal::MAX);
    d.set_scale(scale).ok();
    d
}

/// Check whether a value exceeds the capacity of a field with the given precision/scale.
///
/// Returns true if the value (after rounding to target scale) would need
/// more integer digits than `precision - scale`.
pub fn would_overflow(value: Decimal, precision: u32, scale: u32) -> bool {
    let max_integer_digits = precision.saturating_sub(scale);
    let abs_int = value.abs().trunc();
    let max_int = Decimal::from(10u64.pow(max_integer_digits));
    abs_int >= max_int
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn left_truncation_basic() {
        // 12345 -> PIC 9(3) = 345
        let result = left_truncate_to_precision(dec!(12345), 3, 0);
        assert_eq!(result, dec!(345));
    }

    #[test]
    fn left_truncation_negative() {
        // -12345 -> PIC S9(3) = -345
        let result = left_truncate_to_precision(dec!(-12345), 3, 0);
        assert_eq!(result, dec!(-345));
    }

    #[test]
    fn left_truncation_with_scale() {
        // 12345.67 -> PIC 9(3)V99 (precision=5, scale=2) = 345.67
        let result = left_truncate_to_precision(dec!(12345.67), 5, 2);
        assert_eq!(result, dec!(345.67));
    }

    #[test]
    fn left_truncation_no_truncation_needed() {
        // 123 -> PIC 9(5) = 123 (no change)
        let result = left_truncate_to_precision(dec!(123), 5, 0);
        assert_eq!(result, dec!(123));
    }

    #[test]
    fn left_truncation_zero() {
        let result = left_truncate_to_precision(dec!(0), 3, 0);
        assert_eq!(result, dec!(0));
    }

    #[test]
    fn round_nearest_away_from_zero() {
        // 1.235 rounded to 2 places = 1.24 (away from zero)
        let result = round_decimal(dec!(1.235), 2, RoundingMode::NearestAwayFromZero);
        assert_eq!(result, dec!(1.24));
    }

    #[test]
    fn round_nearest_even() {
        // 1.245 rounded to 2 places = 1.24 (banker's: tie to even)
        let result = round_decimal(dec!(1.245), 2, RoundingMode::NearestEven);
        assert_eq!(result, dec!(1.24));

        // 1.255 rounded to 2 places = 1.26 (banker's: tie to even)
        let result = round_decimal(dec!(1.255), 2, RoundingMode::NearestEven);
        assert_eq!(result, dec!(1.26));
    }

    #[test]
    fn round_truncation() {
        // 1.239 truncated to 2 places = 1.23
        let result = round_decimal(dec!(1.239), 2, RoundingMode::Truncation);
        assert_eq!(result, dec!(1.23));
    }

    #[test]
    fn round_away_from_zero() {
        // 1.231 rounded away from zero to 2 places = 1.24
        let result = round_decimal(dec!(1.231), 2, RoundingMode::AwayFromZero);
        assert_eq!(result, dec!(1.24));

        // -1.231 rounded away from zero to 2 places = -1.24
        let result = round_decimal(dec!(-1.231), 2, RoundingMode::AwayFromZero);
        assert_eq!(result, dec!(-1.24));
    }

    #[test]
    fn round_toward_greater() {
        // 1.231 toward greater to 2 places = 1.24 (ceiling)
        let result = round_decimal(dec!(1.231), 2, RoundingMode::TowardGreater);
        assert_eq!(result, dec!(1.24));

        // -1.239 toward greater to 2 places = -1.23 (ceiling)
        let result = round_decimal(dec!(-1.239), 2, RoundingMode::TowardGreater);
        assert_eq!(result, dec!(-1.23));
    }

    #[test]
    fn round_toward_lesser() {
        // 1.239 toward lesser to 2 places = 1.23 (floor)
        let result = round_decimal(dec!(1.239), 2, RoundingMode::TowardLesser);
        assert_eq!(result, dec!(1.23));

        // -1.231 toward lesser to 2 places = -1.24 (floor)
        let result = round_decimal(dec!(-1.231), 2, RoundingMode::TowardLesser);
        assert_eq!(result, dec!(-1.24));
    }

    #[test]
    fn would_overflow_detects_overflow() {
        // 12345 in PIC 9(3) -> overflow
        assert!(would_overflow(dec!(12345), 3, 0));
        // 123 in PIC 9(3) -> no overflow
        assert!(!would_overflow(dec!(123), 3, 0));
        // 999 in PIC 9(3) -> no overflow
        assert!(!would_overflow(dec!(999), 3, 0));
        // 1000 in PIC 9(3) -> overflow
        assert!(would_overflow(dec!(1000), 3, 0));
    }

    #[test]
    fn max_for_precision_values() {
        let max = max_for_precision(3, 0);
        assert_eq!(max, dec!(999));

        let max = max_for_precision(5, 2);
        assert_eq!(max, dec!(999.99));
    }
}
