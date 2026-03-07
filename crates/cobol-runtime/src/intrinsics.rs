//! COBOL intrinsic functions runtime implementation.
//!
//! Each function is exposed as `cobol_function_<name>()` matching the
//! codegen pattern in proc_gen.rs. Functions are split into numeric
//! (returning Decimal) and string (returning Vec<u8>) categories.

use rust_decimal::Decimal;
use rust_decimal::prelude::*;

// ---------------------------------------------------------------------------
// Numeric functions
// ---------------------------------------------------------------------------

/// FUNCTION LENGTH(x) -- returns byte length of a field.
pub fn cobol_function_length(data: &[u8]) -> Decimal {
    Decimal::from(data.len())
}

/// FUNCTION ABS(x) -- absolute value.
pub fn cobol_function_abs(x: Decimal) -> Decimal {
    x.abs()
}

/// FUNCTION MOD(x, y) -- modulo (COBOL: result has sign of dividend x).
///
/// MOD(x, y) = x - (INTEGER(x / y) * y)
pub fn cobol_function_mod(x: Decimal, y: Decimal) -> Decimal {
    if y.is_zero() {
        return Decimal::ZERO;
    }
    let quotient = (x / y).trunc();
    x - quotient * y
}

/// FUNCTION REM(x, y) -- remainder.
///
/// REM(x, y) = x - (INTEGER-PART(x / y) * y)
/// Same as MOD for COBOL IBM semantics.
pub fn cobol_function_rem(x: Decimal, y: Decimal) -> Decimal {
    cobol_function_mod(x, y)
}

/// FUNCTION INTEGER(x) -- greatest integer <= x.
pub fn cobol_function_integer(x: Decimal) -> Decimal {
    x.floor()
}

/// FUNCTION INTEGER-PART(x) -- integer part (truncate toward zero).
pub fn cobol_function_integer_part(x: Decimal) -> Decimal {
    x.trunc()
}

/// FUNCTION MAX(args) -- maximum of N values.
pub fn cobol_function_max(args: &[Decimal]) -> Decimal {
    args.iter().copied().fold(Decimal::MIN, Decimal::max)
}

/// FUNCTION MIN(args) -- minimum of N values.
pub fn cobol_function_min(args: &[Decimal]) -> Decimal {
    args.iter().copied().fold(Decimal::MAX, Decimal::min)
}

/// FUNCTION ORD(x) -- ordinal position of character (1-based).
pub fn cobol_function_ord(data: &[u8]) -> Decimal {
    if data.is_empty() {
        return Decimal::ZERO;
    }
    Decimal::from(data[0] as u32 + 1) // COBOL ORD is 1-based
}

/// FUNCTION ORD-MAX(args) -- position (1-based) of max value.
pub fn cobol_function_ord_max(args: &[Decimal]) -> Decimal {
    if args.is_empty() {
        return Decimal::ZERO;
    }
    let mut max_idx = 0;
    let mut max_val = args[0];
    for (i, &val) in args.iter().enumerate().skip(1) {
        if val > max_val {
            max_val = val;
            max_idx = i;
        }
    }
    Decimal::from(max_idx + 1)
}

/// FUNCTION ORD-MIN(args) -- position (1-based) of min value.
pub fn cobol_function_ord_min(args: &[Decimal]) -> Decimal {
    if args.is_empty() {
        return Decimal::ZERO;
    }
    let mut min_idx = 0;
    let mut min_val = args[0];
    for (i, &val) in args.iter().enumerate().skip(1) {
        if val < min_val {
            min_val = val;
            min_idx = i;
        }
    }
    Decimal::from(min_idx + 1)
}

/// FUNCTION NUMVAL(x) -- de-edit a numeric string to Decimal.
///
/// Handles: leading/trailing spaces, leading/trailing sign (+/-),
/// commas, decimal points.
pub fn cobol_function_numval(data: &[u8]) -> Decimal {
    let s = String::from_utf8_lossy(data);
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Decimal::ZERO;
    }

    // Remove commas (used as thousands separators)
    let cleaned: String = trimmed
        .chars()
        .filter(|&c| c != ',')
        .collect();

    // Handle trailing sign: "123-" or "123+"
    let (numeric_part, is_negative) = if cleaned.ends_with('-') {
        (&cleaned[..cleaned.len() - 1], true)
    } else if cleaned.ends_with('+') {
        (&cleaned[..cleaned.len() - 1], false)
    } else {
        (cleaned.as_str(), false)
    };

    match Decimal::from_str(numeric_part) {
        Ok(d) => if is_negative { -d } else { d },
        Err(_) => Decimal::ZERO,
    }
}

/// FUNCTION NUMVAL-C(x, currency) -- de-edit with currency symbol.
///
/// Removes the currency symbol before parsing.
pub fn cobol_function_numval_c(data: &[u8], currency: &[u8]) -> Decimal {
    let s = String::from_utf8_lossy(data);
    let curr = String::from_utf8_lossy(currency);
    let trimmed = s.trim();

    // Remove currency symbol
    let without_currency = trimmed.replace(curr.trim(), "");
    cobol_function_numval(without_currency.as_bytes())
}

/// FUNCTION RANDOM(seed) -- pseudo-random number 0..1.
///
/// Uses a simple LCG (linear congruential generator).
/// When seed is 0, uses the previous state. First call with 0 uses default seed.
pub fn cobol_function_random(seed: Decimal) -> Decimal {
    use std::sync::atomic::{AtomicU64, Ordering};
    static STATE: AtomicU64 = AtomicU64::new(12345);

    let seed_val = seed.to_u64().unwrap_or(0);
    if seed_val != 0 {
        STATE.store(seed_val, Ordering::Relaxed);
    }

    let prev = STATE.load(Ordering::Relaxed);
    // LCG parameters (Numerical Recipes)
    let next = prev.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    STATE.store(next, Ordering::Relaxed);

    // Map to 0..1 range with 9 decimal places
    let frac = (next >> 32) as u32; // Upper 32 bits
    let divisor = Decimal::from(u32::MAX);
    Decimal::from(frac) / divisor
}

// ---------------------------------------------------------------------------
// Math functions (f64-based, converted back to Decimal)
// ---------------------------------------------------------------------------

/// FUNCTION SQRT(x) -- square root.
pub fn cobol_function_sqrt(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.sqrt()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION LOG(x) -- natural logarithm.
pub fn cobol_function_log(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.ln()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION LOG10(x) -- base-10 logarithm.
pub fn cobol_function_log10(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.log10()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION SIN(x) -- sine (x in radians).
pub fn cobol_function_sin(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.sin()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION COS(x) -- cosine (x in radians).
pub fn cobol_function_cos(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.cos()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION TAN(x) -- tangent (x in radians).
pub fn cobol_function_tan(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.tan()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION ASIN(x) -- arc sine.
pub fn cobol_function_asin(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.asin()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION ACOS(x) -- arc cosine.
pub fn cobol_function_acos(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.acos()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION ATAN(x) -- arc tangent.
pub fn cobol_function_atan(x: Decimal) -> Decimal {
    let f = x.to_f64().unwrap_or(0.0);
    Decimal::from_f64(f.atan()).unwrap_or(Decimal::ZERO)
}

/// FUNCTION FACTORIAL(x) -- factorial.
pub fn cobol_function_factorial(x: Decimal) -> Decimal {
    let n = x.to_u64().unwrap_or(0);
    let mut result = Decimal::ONE;
    for i in 2..=n {
        result *= Decimal::from(i);
    }
    result
}

// ---------------------------------------------------------------------------
// String functions
// ---------------------------------------------------------------------------

/// FUNCTION CURRENT-DATE -- returns 21-character date/time string.
///
/// Format: YYYYMMDDHHMMSSss+HHMM (21 chars)
/// ss = hundredths of second, +HHMM = UTC offset.
pub fn cobol_function_current_date() -> Vec<u8> {
    // Use std::time for a portable implementation
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = now.as_secs();
    let millis = now.subsec_millis();

    // Simple UTC breakdown (no timezone library dependency)
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;
    let hundredths = millis / 10;

    // Approximate date from days since epoch (1970-01-01)
    // Using a simple algorithm for Gregorian calendar
    let (year, month, day) = days_to_date(days);

    format!(
        "{year:04}{month:02}{day:02}{hours:02}{minutes:02}{seconds:02}{hundredths:02}+0000"
    )
    .into_bytes()
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_date(days: u64) -> (u64, u64, u64) {
    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year, m, d)
}

/// FUNCTION WHEN-COMPILED -- returns compile timestamp (fixed string).
pub fn cobol_function_when_compiled() -> Vec<u8> {
    // In a real transpiler, this would be set at compile time.
    // For runtime, return a placeholder matching the format.
    b"20240101120000000000000".to_vec()
}

/// FUNCTION TRIM(x, type) -- remove leading/trailing spaces.
///
/// Type: 1 = LEADING, 2 = TRAILING, 3 = BOTH (default).
pub fn cobol_function_trim(data: &[u8], trim_type: u8) -> Vec<u8> {
    let start = if trim_type == 1 || trim_type == 3 {
        data.iter().position(|&b| b != b' ').unwrap_or(data.len())
    } else {
        0
    };

    let end = if trim_type == 2 || trim_type == 3 {
        data.iter()
            .rposition(|&b| b != b' ')
            .map_or(0, |p| p + 1)
    } else {
        data.len()
    };

    if start >= end {
        Vec::new()
    } else {
        data[start..end].to_vec()
    }
}

/// FUNCTION UPPER-CASE(x) -- convert to uppercase.
pub fn cobol_function_upper_case(data: &[u8]) -> Vec<u8> {
    data.to_ascii_uppercase()
}

/// FUNCTION LOWER-CASE(x) -- convert to lowercase.
pub fn cobol_function_lower_case(data: &[u8]) -> Vec<u8> {
    data.to_ascii_lowercase()
}

/// FUNCTION REVERSE(x) -- reverse byte order.
pub fn cobol_function_reverse(data: &[u8]) -> Vec<u8> {
    data.iter().rev().copied().collect()
}

/// FUNCTION CONCATENATE(args) -- concatenate multiple byte slices.
pub fn cobol_function_concatenate(args: &[&[u8]]) -> Vec<u8> {
    let total_len: usize = args.iter().map(|a| a.len()).sum();
    let mut result = Vec::with_capacity(total_len);
    for arg in args {
        result.extend_from_slice(arg);
    }
    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    // -- Numeric functions --

    #[test]
    fn test_length() {
        assert_eq!(cobol_function_length(b"HELLO"), dec!(5));
        assert_eq!(cobol_function_length(b""), dec!(0));
        assert_eq!(cobol_function_length(b"A"), dec!(1));
    }

    #[test]
    fn test_abs() {
        assert_eq!(cobol_function_abs(dec!(-42)), dec!(42));
        assert_eq!(cobol_function_abs(dec!(42)), dec!(42));
        assert_eq!(cobol_function_abs(dec!(0)), dec!(0));
    }

    #[test]
    fn test_mod() {
        assert_eq!(cobol_function_mod(dec!(11), dec!(3)), dec!(2));
        assert_eq!(cobol_function_mod(dec!(-11), dec!(3)), dec!(-2));
        assert_eq!(cobol_function_mod(dec!(11), dec!(-3)), dec!(2));
        assert_eq!(cobol_function_mod(dec!(10), dec!(5)), dec!(0));
        assert_eq!(cobol_function_mod(dec!(7), dec!(0)), dec!(0)); // div by zero guard
    }

    #[test]
    fn test_rem() {
        assert_eq!(cobol_function_rem(dec!(11), dec!(3)), dec!(2));
    }

    #[test]
    fn test_integer() {
        assert_eq!(cobol_function_integer(dec!(3.7)), dec!(3));
        assert_eq!(cobol_function_integer(dec!(-3.7)), dec!(-4));
        assert_eq!(cobol_function_integer(dec!(5.0)), dec!(5));
    }

    #[test]
    fn test_integer_part() {
        assert_eq!(cobol_function_integer_part(dec!(3.7)), dec!(3));
        assert_eq!(cobol_function_integer_part(dec!(-3.7)), dec!(-3));
    }

    #[test]
    fn test_max() {
        assert_eq!(cobol_function_max(&[dec!(1), dec!(5), dec!(3)]), dec!(5));
        assert_eq!(cobol_function_max(&[dec!(-1), dec!(-5)]), dec!(-1));
    }

    #[test]
    fn test_min() {
        assert_eq!(cobol_function_min(&[dec!(1), dec!(5), dec!(3)]), dec!(1));
        assert_eq!(cobol_function_min(&[dec!(-1), dec!(-5)]), dec!(-5));
    }

    #[test]
    fn test_ord() {
        assert_eq!(cobol_function_ord(b"A"), dec!(66)); // 'A' = 65, +1 = 66
        assert_eq!(cobol_function_ord(b""), dec!(0));
    }

    #[test]
    fn test_ord_max() {
        assert_eq!(
            cobol_function_ord_max(&[dec!(10), dec!(30), dec!(20)]),
            dec!(2)
        );
    }

    #[test]
    fn test_ord_min() {
        assert_eq!(
            cobol_function_ord_min(&[dec!(10), dec!(30), dec!(20)]),
            dec!(1)
        );
    }

    #[test]
    fn test_numval() {
        assert_eq!(cobol_function_numval(b"  123.45 "), dec!(123.45));
        assert_eq!(cobol_function_numval(b"  -42  "), dec!(-42));
        assert_eq!(cobol_function_numval(b"1,234.56"), dec!(1234.56));
        assert_eq!(cobol_function_numval(b"123-"), dec!(-123));
        assert_eq!(cobol_function_numval(b"123+"), dec!(123));
        assert_eq!(cobol_function_numval(b"   "), dec!(0));
    }

    #[test]
    fn test_numval_c() {
        assert_eq!(
            cobol_function_numval_c(b"$1,234.56", b"$"),
            dec!(1234.56)
        );
        assert_eq!(
            cobol_function_numval_c(b"EUR 100.00", b"EUR"),
            dec!(100.00)
        );
    }

    #[test]
    fn test_random_returns_0_to_1() {
        let r = cobol_function_random(dec!(42));
        assert!(r >= Decimal::ZERO);
        assert!(r < Decimal::ONE);
    }

    #[test]
    fn test_random_different_seeds_differ() {
        let r1 = cobol_function_random(dec!(100));
        let r2 = cobol_function_random(dec!(200));
        assert_ne!(r1, r2);
    }

    // -- Math functions --

    #[test]
    fn test_sqrt() {
        let result = cobol_function_sqrt(dec!(4));
        assert!((result - dec!(2)).abs() < dec!(0.0001));
    }

    #[test]
    fn test_sqrt_zero() {
        assert_eq!(cobol_function_sqrt(dec!(0)), dec!(0));
    }

    #[test]
    fn test_log() {
        let result = cobol_function_log(dec!(1));
        assert!(result.abs() < dec!(0.0001)); // ln(1) = 0
    }

    #[test]
    fn test_log10() {
        let result = cobol_function_log10(dec!(100));
        assert!((result - dec!(2)).abs() < dec!(0.0001));
    }

    #[test]
    fn test_sin() {
        let result = cobol_function_sin(dec!(0));
        assert!(result.abs() < dec!(0.0001));
    }

    #[test]
    fn test_cos() {
        let result = cobol_function_cos(dec!(0));
        assert!((result - dec!(1)).abs() < dec!(0.0001));
    }

    #[test]
    fn test_tan() {
        let result = cobol_function_tan(dec!(0));
        assert!(result.abs() < dec!(0.0001));
    }

    #[test]
    fn test_asin() {
        let result = cobol_function_asin(dec!(0));
        assert!(result.abs() < dec!(0.0001));
    }

    #[test]
    fn test_acos() {
        let result = cobol_function_acos(dec!(1));
        assert!(result.abs() < dec!(0.0001)); // acos(1) = 0
    }

    #[test]
    fn test_atan() {
        let result = cobol_function_atan(dec!(0));
        assert!(result.abs() < dec!(0.0001));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(cobol_function_factorial(dec!(0)), dec!(1));
        assert_eq!(cobol_function_factorial(dec!(1)), dec!(1));
        assert_eq!(cobol_function_factorial(dec!(5)), dec!(120));
        assert_eq!(cobol_function_factorial(dec!(10)), dec!(3628800));
    }

    // -- String functions --

    #[test]
    fn test_current_date_format() {
        let date = cobol_function_current_date();
        assert_eq!(date.len(), 21);
        // Should end with +0000 (UTC)
        assert_eq!(&date[17..21], b"0000");
    }

    #[test]
    fn test_trim_both() {
        assert_eq!(cobol_function_trim(b"  HELLO  ", 3), b"HELLO");
    }

    #[test]
    fn test_trim_leading() {
        assert_eq!(cobol_function_trim(b"  HELLO  ", 1), b"HELLO  ");
    }

    #[test]
    fn test_trim_trailing() {
        assert_eq!(cobol_function_trim(b"  HELLO  ", 2), b"  HELLO");
    }

    #[test]
    fn test_trim_all_spaces() {
        assert_eq!(cobol_function_trim(b"     ", 3), b"");
    }

    #[test]
    fn test_upper_case() {
        assert_eq!(cobol_function_upper_case(b"hello World"), b"HELLO WORLD");
    }

    #[test]
    fn test_lower_case() {
        assert_eq!(cobol_function_lower_case(b"HELLO World"), b"hello world");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(cobol_function_reverse(b"HELLO"), b"OLLEH");
        assert_eq!(cobol_function_reverse(b""), b"");
        assert_eq!(cobol_function_reverse(b"A"), b"A");
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(
            cobol_function_concatenate(&[b"HELLO", b" ", b"WORLD"]),
            b"HELLO WORLD"
        );
        assert_eq!(cobol_function_concatenate(&[]), b"");
    }
}
