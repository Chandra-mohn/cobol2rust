use cobol_core::config::{RuntimeConfig, RoundingMode};
use cobol_core::traits::CobolNumeric;

use crate::result::ArithResult;
use crate::store::store_arithmetic_result;

/// SUBTRACT src FROM dest [ROUNDED]
///
/// Subtracts `src` from `dest` and stores back in `dest`.
pub fn cobol_subtract(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let result = dest.to_decimal() - src.to_decimal();
    store_arithmetic_result(result, dest, rounded, config)
}

/// SUBTRACT src FROM src2 GIVING dest [ROUNDED]
pub fn cobol_subtract_giving(
    src: &dyn CobolNumeric,
    src2: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let result = src2.to_decimal() - src.to_decimal();
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
    fn subtract_basic() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(100));
        let mut dest = PackedDecimal::new(5, 0, true);
        dest.set_decimal(dec!(300));

        let result = cobol_subtract(&src, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(200));
    }

    #[test]
    fn subtract_giving() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(100));
        let mut src2 = PackedDecimal::new(5, 0, true);
        src2.set_decimal(dec!(300));
        let mut dest = PackedDecimal::new(5, 0, true);

        let result = cobol_subtract_giving(&src, &src2, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(200));
    }

    #[test]
    fn subtract_negative_result() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(300));
        let mut dest = PackedDecimal::new(5, 0, true);
        dest.set_decimal(dec!(100));

        let result = cobol_subtract(&src, &mut dest, None, &default_config());
        assert!(!result.size_error);
        assert_eq!(dest.to_decimal(), dec!(-200));
    }
}
