//! COBOL numeric display parsing utilities.
//!
//! Parses various COBOL display representations into `Decimal` values.
//! Handles: plain display, zoned decimal bytes, sign-separate,
//! implied decimal (PIC 9V99), and edited numeric stripping.

use rust_decimal::Decimal;

/// Parse a COBOL display representation to a Decimal value.
///
/// Handles formats:
/// - Plain: "12345", "00123"
/// - Signed: "+00123", "-00456"
/// - Decimal: "123.45"
/// - Implied decimal: raw bytes with known scale (e.g., "12345" with scale 2 = 123.45)
/// - Edited: strips commas, currency symbols, spaces, CR/DB before parsing
/// - Empty/spaces: returns zero
pub fn parse_numeric_display(bytes: &[u8]) -> Decimal {
    let s = match std::str::from_utf8(bytes) {
        Ok(v) => v.trim(),
        Err(_) => return Decimal::ZERO,
    };

    if s.is_empty() {
        return Decimal::ZERO;
    }

    // Handle CR/DB suffix BEFORE stripping edit chars (B is both edit char and part of DB)
    let (after_suffix, negate) = strip_sign_suffix(s);

    // Strip editing characters: commas, currency, spaces, asterisks
    let cleaned = strip_edit_chars(after_suffix);

    if cleaned.is_empty() {
        return Decimal::ZERO;
    }

    let numeric_str = cleaned.as_str();

    // Try parsing as decimal
    match Decimal::from_str_exact(numeric_str) {
        Ok(d) => if negate { -d } else { d },
        Err(_) => {
            // Try stripping leading sign and retrying
            if let Some(rest) = numeric_str.strip_prefix('+') {
                Decimal::from_str_exact(rest).unwrap_or(Decimal::ZERO)
            } else {
                Decimal::ZERO
            }
        }
    }
}

/// Parse COBOL display bytes with an implied decimal point.
///
/// COBOL PIC 9(3)V99 stores "12345" meaning 123.45.
/// This function interprets raw digit bytes with a known scale.
pub fn parse_with_implied_decimal(bytes: &[u8], scale: u32) -> Decimal {
    let s = match std::str::from_utf8(bytes) {
        Ok(v) => v.trim(),
        Err(_) => return Decimal::ZERO,
    };

    if s.is_empty() {
        return Decimal::ZERO;
    }

    // Strip sign character if present
    let (sign_negative, digits) = extract_sign(s);

    // Parse as integer, then apply scale
    let cleaned: String = digits.chars().filter(char::is_ascii_digit).collect();
    if cleaned.is_empty() {
        return Decimal::ZERO;
    }

    match Decimal::from_str_exact(&cleaned) {
        Ok(mut d) => {
            if scale > 0 {
                let divisor = Decimal::from(10i64.pow(scale));
                d /= divisor;
            }
            if sign_negative { -d } else { d }
        }
        Err(_) => Decimal::ZERO,
    }
}

/// Parse zoned decimal bytes (EBCDIC-style zone encoding).
///
/// Each byte = zone nibble (high) + digit nibble (low).
/// Sign is encoded in the zone nibble of the last byte:
/// - 0xC0..=0xCF = positive
/// - 0xD0..=0xDF = negative
/// - 0xF0..=0xFF = unsigned (positive)
///
/// For ASCII-native representation (zone = 0x3), the last byte's
/// zone nibble encodes the sign similarly.
pub fn parse_zoned_decimal(bytes: &[u8], scale: u32) -> Decimal {
    if bytes.is_empty() {
        return Decimal::ZERO;
    }

    let mut digits = String::with_capacity(bytes.len());
    let mut negative = false;

    for (i, &byte) in bytes.iter().enumerate() {
        let digit = byte & 0x0F;
        if digit > 9 {
            continue;
        }

        // Check sign on last byte
        if i == bytes.len() - 1 {
            let zone = byte >> 4;
            negative = zone == 0x0D; // D-zone = negative
        }

        digits.push(char::from(b'0' + digit));
    }

    if digits.is_empty() {
        return Decimal::ZERO;
    }

    match Decimal::from_str_exact(&digits) {
        Ok(mut d) => {
            if scale > 0 {
                let divisor = Decimal::from(10i64.pow(scale));
                d /= divisor;
            }
            if negative { -d } else { d }
        }
        Err(_) => Decimal::ZERO,
    }
}

/// Strip editing characters from a numeric display string.
///
/// Removes: commas, currency symbols ($), asterisks (*), leading/trailing spaces,
/// slash (/), B (blank insertion).
fn strip_edit_chars(s: &str) -> String {
    s.chars()
        .filter(|&c| c != ',' && c != '$' && c != '*' && c != '/' && c != 'B' && c != ' ')
        .collect()
}

/// Strip CR/DB suffix and detect negative sign.
///
/// Returns (`remaining_string`, `is_negative`).
fn strip_sign_suffix(s: &str) -> (&str, bool) {
    if let Some(rest) = s.strip_suffix("CR") {
        (rest, true)
    } else if let Some(rest) = s.strip_suffix("DB") {
        (rest, true)
    } else {
        (s, false)
    }
}

/// Extract sign from a numeric string.
///
/// Returns (`is_negative`, `remaining_digits`).
fn extract_sign(s: &str) -> (bool, &str) {
    if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = s.strip_prefix('+') {
        (false, rest)
    } else {
        (false, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    // --- parse_numeric_display ---

    #[test]
    fn parse_plain_integer() {
        assert_eq!(parse_numeric_display(b"12345"), dec!(12345));
    }

    #[test]
    fn parse_leading_zeros() {
        assert_eq!(parse_numeric_display(b"00123"), dec!(123));
    }

    #[test]
    fn parse_signed_positive() {
        assert_eq!(parse_numeric_display(b"+00123"), dec!(123));
    }

    #[test]
    fn parse_signed_negative() {
        assert_eq!(parse_numeric_display(b"-00456"), dec!(-456));
    }

    #[test]
    fn parse_decimal_point() {
        assert_eq!(parse_numeric_display(b"123.45"), dec!(123.45));
    }

    #[test]
    fn parse_all_zeros() {
        assert_eq!(parse_numeric_display(b"00000"), dec!(0));
    }

    #[test]
    fn parse_empty() {
        assert_eq!(parse_numeric_display(b""), dec!(0));
    }

    #[test]
    fn parse_all_spaces() {
        assert_eq!(parse_numeric_display(b"     "), dec!(0));
    }

    #[test]
    fn parse_edited_with_commas() {
        assert_eq!(parse_numeric_display(b"1,234.56"), dec!(1234.56));
    }

    #[test]
    fn parse_edited_with_currency() {
        assert_eq!(parse_numeric_display(b"$1,234.56"), dec!(1234.56));
    }

    #[test]
    fn parse_edited_with_asterisks() {
        assert_eq!(parse_numeric_display(b"***1234"), dec!(1234));
    }

    #[test]
    fn parse_cr_suffix() {
        assert_eq!(parse_numeric_display(b"123.45CR"), dec!(-123.45));
    }

    #[test]
    fn parse_db_suffix() {
        assert_eq!(parse_numeric_display(b"123.45DB"), dec!(-123.45));
    }

    #[test]
    fn parse_invalid_utf8_returns_zero() {
        assert_eq!(parse_numeric_display(&[0xFF, 0xFE]), dec!(0));
    }

    // --- parse_with_implied_decimal ---

    #[test]
    fn implied_decimal_basic() {
        // PIC 9(3)V99: "12345" means 123.45
        assert_eq!(parse_with_implied_decimal(b"12345", 2), dec!(123.45));
    }

    #[test]
    fn implied_decimal_no_scale() {
        assert_eq!(parse_with_implied_decimal(b"12345", 0), dec!(12345));
    }

    #[test]
    fn implied_decimal_with_sign() {
        assert_eq!(parse_with_implied_decimal(b"-12345", 2), dec!(-123.45));
    }

    #[test]
    fn implied_decimal_leading_zeros() {
        assert_eq!(parse_with_implied_decimal(b"00100", 2), dec!(1.00));
    }

    #[test]
    fn implied_decimal_empty() {
        assert_eq!(parse_with_implied_decimal(b"", 2), dec!(0));
    }

    // --- parse_zoned_decimal ---

    #[test]
    fn zoned_decimal_unsigned() {
        // Unsigned zone = 0xF: bytes [0xF1, 0xF2, 0xF3] = "123"
        assert_eq!(parse_zoned_decimal(&[0xF1, 0xF2, 0xF3], 0), dec!(123));
    }

    #[test]
    fn zoned_decimal_positive() {
        // Positive zone on last byte = 0xC: [0xF1, 0xF2, 0xC3] = +123
        assert_eq!(parse_zoned_decimal(&[0xF1, 0xF2, 0xC3], 0), dec!(123));
    }

    #[test]
    fn zoned_decimal_negative() {
        // Negative zone on last byte = 0xD: [0xF1, 0xF2, 0xD3] = -123
        assert_eq!(parse_zoned_decimal(&[0xF1, 0xF2, 0xD3], 0), dec!(-123));
    }

    #[test]
    fn zoned_decimal_with_scale() {
        // PIC S9(3)V99: [0xF1, 0xF2, 0xF3, 0xF4, 0xC5] = +123.45
        assert_eq!(
            parse_zoned_decimal(&[0xF1, 0xF2, 0xF3, 0xF4, 0xC5], 2),
            dec!(123.45)
        );
    }

    #[test]
    fn zoned_decimal_empty() {
        assert_eq!(parse_zoned_decimal(&[], 0), dec!(0));
    }

    #[test]
    fn zoned_decimal_all_zeros() {
        assert_eq!(parse_zoned_decimal(&[0xF0, 0xF0, 0xF0], 0), dec!(0));
    }
}
