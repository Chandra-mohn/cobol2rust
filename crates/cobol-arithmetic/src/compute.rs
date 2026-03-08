use rust_decimal::Decimal;

use cobol_core::config::{RuntimeConfig, RoundingMode};
use cobol_core::traits::CobolNumeric;

use crate::result::ArithResult;
use crate::store::store_arithmetic_result;

/// Safe division for COMPUTE expressions.
///
/// Returns `Decimal::MAX` on division by zero so that `cobol_compute` will
/// detect a size error instead of panicking.
#[inline]
pub fn cobol_checked_div(lhs: Decimal, rhs: Decimal) -> Decimal {
    lhs.checked_div(rhs).unwrap_or(Decimal::MAX)
}

/// COMPUTE dest = expression [ROUNDED]
///
/// The expression is evaluated by the transpiler into a single Decimal value.
/// This function stores the result into the destination with proper rounding
/// and truncation.
pub fn cobol_compute(
    expression_value: Decimal,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    store_arithmetic_result(expression_value, dest, rounded, config)
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
    fn compute_basic() {
        let mut dest = PackedDecimal::new(5, 0, true);
        let result = cobol_compute(dec!(42), &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(42));
    }

    #[test]
    fn compute_with_rounding() {
        let mut dest = PackedDecimal::new(5, 2, true);
        let result = cobol_compute(
            dec!(3.14159),
            &mut dest,
            Some(RoundingMode::NearestAwayFromZero),
            &default_config(),
        );
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(3.14));
    }

    #[test]
    fn compute_size_error() {
        let mut dest = PackedDecimal::new(3, 0, true);
        let result = cobol_compute(dec!(12345), &mut dest, None, &default_config());
        assert!(result.size_error);
        // Left-truncation: 12345 -> 345
        assert_eq!(dest.to_decimal(), dec!(345));
    }

    #[test]
    fn compute_expression() {
        // COMPUTE RESULT = A + B * C
        // Transpiler evaluates: 10 + 5 * 3 = 25
        let mut dest = PackedDecimal::new(5, 0, true);
        let expr = dec!(10) + dec!(5) * dec!(3);
        let result = cobol_compute(expr, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(25));
    }
}
