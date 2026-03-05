use rust_decimal::Decimal;

use cobol_core::config::{RuntimeConfig, RoundingMode};
use cobol_core::decimal_ext;
use cobol_core::traits::CobolNumeric;

use crate::result::ArithResult;

/// Store an arithmetic result into a destination field.
///
/// This is the core shared logic used by all arithmetic verbs.
/// It handles:
/// 1. Rounding (if ROUNDED phrase is specified)
/// 2. Overflow/size error detection
/// 3. Left-truncation (COBOL silent truncation)
/// 4. Sign handling for unsigned destinations
///
/// If size error occurs and is detected:
/// - Destination is NOT modified (COBOL standard behavior)
/// - ArithResult::size_error is returned
pub fn store_arithmetic_result(
    value: Decimal,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    _config: &RuntimeConfig,
) -> ArithResult {
    let dest_scale = dest.scale();
    let dest_precision = dest.precision();

    // Step 1: Round or truncate to destination scale
    let rounded_value = match rounded {
        Some(mode) => decimal_ext::round_decimal(value, dest_scale, mode),
        None => decimal_ext::truncate_decimal(value, dest_scale),
    };

    // Step 2: Check for size error (overflow)
    let has_overflow = decimal_ext::would_overflow(rounded_value, dest_precision, dest_scale);

    if has_overflow {
        // If ROUNDED MODE IS PROHIBITED and rounding was needed, it's a size error
        if rounded == Some(RoundingMode::Prohibited) {
            return ArithResult::size_error();
        }

        // COBOL left-truncation: silently drop high-order digits
        let truncated =
            decimal_ext::left_truncate_to_precision(rounded_value, dest_precision, dest_scale);

        // Handle sign for unsigned destinations
        let final_value = if !dest.is_signed() {
            truncated.abs()
        } else {
            truncated
        };

        dest.set_decimal(final_value);
        return ArithResult::size_error();
    }

    // Step 3: No overflow -- store directly
    let final_value = if !dest.is_signed() {
        rounded_value.abs()
    } else {
        rounded_value
    };

    dest.set_decimal(final_value);
    ArithResult::ok()
}
