//! Numeric-Edited data type (PIC Z, *, $, +, -, CR, DB).
//!
//! Stores an edited display string produced by applying a PIC editing mask
//! to a numeric value. The edited result is the "as_bytes" representation.
//!
//! Editing rules follow IBM Enterprise COBOL semantics:
//! - Zero suppression: Z replaces leading zeros with spaces, * with asterisks
//! - Floating insertion: $, +, - float leftward to first significant digit
//! - Fixed insertion: commas, periods, B (space), 0, / inserted at position
//! - Sign suffixes: CR/DB appear only when value is negative
//! - BLANK WHEN ZERO: entire field becomes spaces when value is zero

use rust_decimal::Decimal;

use cobol_core::category::DataCategory;
use cobol_core::editing::EditSymbol;
use cobol_core::traits::{CobolField, CobolNumericEdited};

/// Numeric-Edited field (PIC with editing symbols).
///
/// The display buffer holds the fully-edited string representation.
/// The editing mask and digit positions are set at creation time from
/// the parsed PIC clause.
#[derive(Clone)]
pub struct NumericEdited {
    /// Edited display buffer (the "value" of a numeric-edited field).
    data: Vec<u8>,
    /// Parsed editing mask from PIC clause.
    mask: Vec<EditSymbol>,
    /// Number of integer digit positions (9, Z, *, or float positions before V).
    int_positions: u32,
    /// Number of decimal digit positions (after V/period).
    dec_positions: u32,
    /// BLANK WHEN ZERO flag.
    blank_when_zero: bool,
}

impl std::fmt::Debug for NumericEdited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = String::from_utf8_lossy(&self.data);
        f.debug_struct("NumericEdited")
            .field("display", &display)
            .field("int_positions", &self.int_positions)
            .field("dec_positions", &self.dec_positions)
            .field("blank_when_zero", &self.blank_when_zero)
            .finish()
    }
}

impl NumericEdited {
    /// Create a new NumericEdited field.
    ///
    /// - `mask`: The parsed editing mask from the PIC clause.
    /// - `int_positions`: Number of integer digit positions.
    /// - `dec_positions`: Number of decimal digit positions.
    /// - `blank_when_zero`: BLANK WHEN ZERO clause present.
    pub fn new(
        mask: Vec<EditSymbol>,
        int_positions: u32,
        dec_positions: u32,
        blank_when_zero: bool,
    ) -> Self {
        let byte_len = compute_byte_length(&mask);
        Self {
            data: vec![b' '; byte_len],
            mask,
            int_positions,
            dec_positions,
            blank_when_zero,
        }
    }
}

/// Compute the byte length of an editing mask.
fn compute_byte_length(mask: &[EditSymbol]) -> usize {
    let mut len = 0usize;
    for sym in mask {
        match sym {
            EditSymbol::CreditRight | EditSymbol::DebitRight => len += 2,
            _ => len += 1,
        }
    }
    len
}

/// Count how many digit positions exist in a mask (9, Z, *, float currency/sign).
fn count_digit_positions(mask: &[EditSymbol]) -> usize {
    mask.iter()
        .filter(|s| matches!(
            s,
            EditSymbol::Digit
                | EditSymbol::ZeroSuppress(_)
                | EditSymbol::FloatCurrency(_)
                | EditSymbol::FloatPlus
                | EditSymbol::FloatMinus
        ))
        .count()
}

/// Apply the editing mask to format a numeric value.
///
/// This is the core editing algorithm. It:
/// 1. Converts the value to a digit string with proper precision
/// 2. Walks the mask left-to-right, placing digits and edit characters
/// 3. Applies zero suppression (replacing leading zeros with fill char)
/// 4. Handles floating insertion (currency, sign)
/// 5. Applies CR/DB suffix for negative values
fn apply_editing(
    mask: &[EditSymbol],
    value: Decimal,
    is_negative: bool,
    _int_positions: u32,
    dec_positions: u32,
    blank_when_zero: bool,
) -> Vec<u8> {
    let total_digits = count_digit_positions(mask);
    let byte_len = compute_byte_length(mask);

    // Handle BLANK WHEN ZERO
    if blank_when_zero && value.is_zero() {
        return vec![b' '; byte_len];
    }

    // Scale to integer and get digit string
    let abs_val = value.abs();
    let scale_factor = Decimal::from(10i64.pow(dec_positions));
    let scaled = (abs_val * scale_factor).trunc();
    let digit_str = scaled.to_string();

    // Right-justify digits with leading zeros
    let mut digits = vec![b'0'; total_digits];
    let digit_bytes = digit_str.as_bytes();
    if digit_bytes.len() >= total_digits {
        // Left-truncation: take rightmost digits
        let start = digit_bytes.len() - total_digits;
        digits.copy_from_slice(&digit_bytes[start..]);
    } else {
        let start = total_digits - digit_bytes.len();
        digits[start..].copy_from_slice(digit_bytes);
    }

    // Find the position of the first significant (non-zero) digit
    let first_significant = digits.iter().position(|&d| d != b'0').unwrap_or(total_digits);

    // Determine if we have floating insertion symbols
    let has_float_currency = mask.iter().any(|s| matches!(s, EditSymbol::FloatCurrency(_)));
    let has_float_plus = mask.iter().any(|s| matches!(s, EditSymbol::FloatPlus));
    let has_float_minus = mask.iter().any(|s| matches!(s, EditSymbol::FloatMinus));
    let has_float = has_float_currency || has_float_plus || has_float_minus;

    // Count float positions to determine where float char goes
    let float_count: usize = mask.iter()
        .filter(|s| matches!(
            s,
            EditSymbol::FloatCurrency(_) | EditSymbol::FloatPlus | EditSymbol::FloatMinus
        ))
        .count();

    // Build output
    let mut result = Vec::with_capacity(byte_len);
    let mut digit_idx = 0usize;
    let mut suppress_active = true; // Start with suppression on
    let mut past_period = false;

    // For floating insertion: the float character appears at the position
    // just before the first significant digit (or at the last float position
    // if all digits are zero).
    // We need to figure out which mask position corresponds to the float char placement.
    let float_placement_digit_idx = if has_float {
        // The float char goes at the digit position just before the first significant digit,
        // but no earlier than digit position 0 and no later than (float_count - 1).
        if first_significant == 0 {
            // All digits significant -- float char goes before position 0
            // In COBOL this means the float char occupies position 0 and digits shift,
            // but since float positions ARE digit positions, we place float at position 0
            0
        } else if first_significant >= float_count {
            // First significant digit is past all float positions
            float_count.saturating_sub(1)
        } else {
            // Float char goes at position just before first significant
            first_significant.saturating_sub(1).max(0)
        }
    } else {
        usize::MAX // No floating
    };

    for sym in mask {
        match sym {
            EditSymbol::Digit => {
                // Always display the digit
                suppress_active = false;
                result.push(digits[digit_idx]);
                digit_idx += 1;
            }
            EditSymbol::ZeroSuppress(fill) => {
                let d = digits[digit_idx];
                if suppress_active && d == b'0' && !past_period {
                    result.push(*fill);
                } else {
                    suppress_active = false;
                    result.push(d);
                }
                digit_idx += 1;
            }
            EditSymbol::FloatCurrency(ch) => {
                let d = digits[digit_idx];
                if digit_idx == float_placement_digit_idx {
                    // Place the float character here
                    result.push(*ch);
                    suppress_active = false;
                } else if suppress_active && d == b'0' && !past_period {
                    result.push(b' ');
                } else {
                    suppress_active = false;
                    result.push(d);
                }
                digit_idx += 1;
            }
            EditSymbol::FloatPlus => {
                let d = digits[digit_idx];
                if digit_idx == float_placement_digit_idx {
                    result.push(if is_negative { b'-' } else { b'+' });
                    suppress_active = false;
                } else if suppress_active && d == b'0' && !past_period {
                    result.push(b' ');
                } else {
                    suppress_active = false;
                    result.push(d);
                }
                digit_idx += 1;
            }
            EditSymbol::FloatMinus => {
                let d = digits[digit_idx];
                if digit_idx == float_placement_digit_idx {
                    result.push(if is_negative { b'-' } else { b' ' });
                    suppress_active = false;
                } else if suppress_active && d == b'0' && !past_period {
                    result.push(b' ');
                } else {
                    suppress_active = false;
                    result.push(d);
                }
                digit_idx += 1;
            }
            EditSymbol::Period => {
                past_period = true;
                suppress_active = false; // Digits after period always show
                result.push(b'.');
            }
            EditSymbol::Comma => {
                if suppress_active {
                    // During suppression, comma becomes the fill character
                    // For Z -> space, for * -> asterisk
                    let fill = find_suppress_fill(mask);
                    result.push(fill);
                } else {
                    result.push(b',');
                }
            }
            EditSymbol::InsertSpace => {
                result.push(b' ');
            }
            EditSymbol::InsertZero => {
                result.push(b'0');
            }
            EditSymbol::InsertSlash => {
                result.push(b'/');
            }
            EditSymbol::FixedCurrency(ch) => {
                result.push(*ch);
            }
            EditSymbol::FixedPlus => {
                result.push(if is_negative { b'-' } else { b'+' });
            }
            EditSymbol::FixedMinus => {
                result.push(if is_negative { b'-' } else { b' ' });
            }
            EditSymbol::CreditRight => {
                if is_negative {
                    result.push(b'C');
                    result.push(b'R');
                } else {
                    result.push(b' ');
                    result.push(b' ');
                }
            }
            EditSymbol::DebitRight => {
                if is_negative {
                    result.push(b'D');
                    result.push(b'B');
                } else {
                    result.push(b' ');
                    result.push(b' ');
                }
            }
            EditSymbol::SignPosition | EditSymbol::AlphaChar => {
                // Not applicable in numeric-edited; treat as space
                result.push(b' ');
            }
        }
    }

    result
}

/// Find the fill character used by zero-suppression symbols in the mask.
fn find_suppress_fill(mask: &[EditSymbol]) -> u8 {
    for sym in mask {
        if let EditSymbol::ZeroSuppress(fill) = sym {
            return *fill;
        }
    }
    // For float patterns, suppressed positions become spaces
    b' '
}

/// De-edit: extract a numeric Decimal from an edited display string.
///
/// This is the IBM extension for MOVE from numeric-edited to numeric.
/// Strips all non-digit characters except sign indicators and decimal point.
fn de_edit_value(data: &[u8], mask: &[EditSymbol]) -> Option<Decimal> {
    let s = std::str::from_utf8(data).ok()?;

    // Detect negative from CR/DB suffix or - sign
    let is_negative = s.ends_with("CR")
        || s.ends_with("DB")
        || s.contains('-');

    // Extract digits and decimal point
    let mut numeric = String::new();
    let mut has_decimal = false;

    for &b in data {
        match b {
            b'0'..=b'9' => numeric.push(b as char),
            b'.' => {
                if !has_decimal {
                    numeric.push('.');
                    has_decimal = true;
                }
            }
            _ => {} // Skip edit chars (spaces, commas, currency, etc.)
        }
    }

    if numeric.is_empty() {
        return Some(Decimal::ZERO);
    }

    // If no decimal point found but mask has decimal positions, insert one
    if !has_decimal {
        // Check if mask has a Period
        let has_period = mask.iter().any(|s| matches!(s, EditSymbol::Period));
        if !has_period {
            // All integer, just parse as-is
        }
        // Otherwise the value is what we extracted
    }

    match Decimal::from_str_exact(&numeric) {
        Ok(d) => Some(if is_negative && !d.is_zero() { -d } else { d }),
        Err(_) => Some(Decimal::ZERO),
    }
}

impl CobolField for NumericEdited {
    fn category(&self) -> DataCategory {
        DataCategory::NumericEdited
    }

    fn byte_length(&self) -> usize {
        self.data.len()
    }

    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn display_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn fill_bytes(&mut self, byte: u8) {
        self.data.fill(byte);
    }

    fn has_blank_when_zero(&self) -> bool {
        self.blank_when_zero
    }

    fn initialize_default(&mut self) {
        // INITIALIZE sets numeric-edited to zero value (apply editing to 0)
        self.data = apply_editing(
            &self.mask,
            Decimal::ZERO,
            false,
            self.int_positions,
            self.dec_positions,
            self.blank_when_zero,
        );
    }
}

impl CobolNumericEdited for NumericEdited {
    fn edit_mask(&self) -> &[EditSymbol] {
        &self.mask
    }

    fn integer_positions(&self) -> u32 {
        self.int_positions
    }

    fn decimal_positions(&self) -> u32 {
        self.dec_positions
    }

    fn set_from_numeric(&mut self, value: Decimal, is_negative: bool) {
        self.data = apply_editing(
            &self.mask,
            value,
            is_negative,
            self.int_positions,
            self.dec_positions,
            self.blank_when_zero,
        );
    }

    fn de_edit(&self) -> Option<Decimal> {
        de_edit_value(&self.data, &self.mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    /// Helper: build mask from a slice of EditSymbols.
    fn make(mask: &[EditSymbol], int_pos: u32, dec_pos: u32) -> NumericEdited {
        NumericEdited::new(mask.to_vec(), int_pos, dec_pos, false)
    }

    // --- Zero Suppression (Z) ---

    #[test]
    fn zero_suppress_basic() {
        // PIC Z(3)9: 4 integer positions, zero-suppress first 3
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), false);
        assert_eq!(ne.as_bytes(), b"  42");
    }

    #[test]
    fn zero_suppress_all_zeros() {
        // PIC Z(4): all zero-suppress
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(0), false);
        // All zeros suppressed to spaces
        assert_eq!(ne.as_bytes(), b"    ");
    }

    #[test]
    fn zero_suppress_full_value() {
        // PIC Z(3)9: value fills all positions
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(1234), false);
        assert_eq!(ne.as_bytes(), b"1234");
    }

    // --- Asterisk Check Protect ---

    #[test]
    fn asterisk_fill() {
        // PIC **9: asterisk for leading zeros
        let mask = vec![
            EditSymbol::ZeroSuppress(b'*'),
            EditSymbol::ZeroSuppress(b'*'),
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 3, 0);
        ne.set_from_numeric(dec!(5), false);
        assert_eq!(ne.as_bytes(), b"**5");
    }

    // --- Decimal Point ---

    #[test]
    fn with_decimal_point() {
        // PIC Z(3)9.99: 4 int, 2 dec
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
            EditSymbol::Period,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 2);
        ne.set_from_numeric(dec!(12.34), false);
        assert_eq!(ne.as_bytes(), b"  12.34");
    }

    #[test]
    fn decimal_with_zero_integer() {
        // PIC Z(3)9.99: value < 1
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
            EditSymbol::Period,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 2);
        ne.set_from_numeric(dec!(0.50), false);
        assert_eq!(ne.as_bytes(), b"   0.50");
    }

    // --- Comma Insertion ---

    #[test]
    fn comma_with_value() {
        // PIC Z,ZZ9: 4 digits with comma
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Comma,
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(1234), false);
        assert_eq!(ne.as_bytes(), b"1,234");
    }

    #[test]
    fn comma_suppressed_with_small_value() {
        // PIC Z,ZZ9: small value, comma suppressed
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Comma,
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), false);
        assert_eq!(ne.as_bytes(), b"   42");
    }

    // --- Fixed Currency ---

    #[test]
    fn fixed_currency() {
        // PIC $9,999: fixed $ then digits
        let mask = vec![
            EditSymbol::FixedCurrency(b'$'),
            EditSymbol::Digit,
            EditSymbol::Comma,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(1234), false);
        assert_eq!(ne.as_bytes(), b"$1,234");
    }

    // --- Floating Currency ---

    #[test]
    fn floating_currency_basic() {
        // PIC $$$9.99: 4 digit positions, float $ in first 3
        let mask = vec![
            EditSymbol::FloatCurrency(b'$'),
            EditSymbol::FloatCurrency(b'$'),
            EditSymbol::FloatCurrency(b'$'),
            EditSymbol::Digit,
            EditSymbol::Period,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 2);
        ne.set_from_numeric(dec!(12.34), false);
        assert_eq!(ne.as_bytes(), b" $12.34");
    }

    #[test]
    fn floating_currency_large() {
        // PIC $$$9.99: large value fills all positions
        let mask = vec![
            EditSymbol::FloatCurrency(b'$'),
            EditSymbol::FloatCurrency(b'$'),
            EditSymbol::FloatCurrency(b'$'),
            EditSymbol::Digit,
            EditSymbol::Period,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 2);
        ne.set_from_numeric(dec!(1234.56), false);
        // All float positions consumed by digits, $ goes at position 0
        assert_eq!(ne.as_bytes(), b"$234.56");
    }

    // --- Floating Sign ---

    #[test]
    fn floating_plus_positive() {
        // PIC +++9: float + for sign
        let mask = vec![
            EditSymbol::FloatPlus,
            EditSymbol::FloatPlus,
            EditSymbol::FloatPlus,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), false);
        assert_eq!(ne.as_bytes(), b" +42");
    }

    #[test]
    fn floating_plus_negative() {
        let mask = vec![
            EditSymbol::FloatPlus,
            EditSymbol::FloatPlus,
            EditSymbol::FloatPlus,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), true);
        assert_eq!(ne.as_bytes(), b" -42");
    }

    #[test]
    fn floating_minus_positive() {
        // PIC ---9: float - (space for positive)
        let mask = vec![
            EditSymbol::FloatMinus,
            EditSymbol::FloatMinus,
            EditSymbol::FloatMinus,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), false);
        assert_eq!(ne.as_bytes(), b"  42");
    }

    #[test]
    fn floating_minus_negative() {
        let mask = vec![
            EditSymbol::FloatMinus,
            EditSymbol::FloatMinus,
            EditSymbol::FloatMinus,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), true);
        assert_eq!(ne.as_bytes(), b" -42");
    }

    // --- Fixed Sign ---

    #[test]
    fn fixed_plus_positive() {
        // PIC +9999
        let mask = vec![
            EditSymbol::FixedPlus,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), false);
        assert_eq!(ne.as_bytes(), b"+0042");
    }

    #[test]
    fn fixed_minus_negative() {
        // PIC 9999-
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::FixedMinus,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), true);
        assert_eq!(ne.as_bytes(), b"0042-");
    }

    #[test]
    fn fixed_minus_positive() {
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::FixedMinus,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), false);
        assert_eq!(ne.as_bytes(), b"0042 ");
    }

    // --- CR/DB Suffix ---

    #[test]
    fn cr_suffix_negative() {
        // PIC 9999CR
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::CreditRight,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), true);
        assert_eq!(ne.as_bytes(), b"0042CR");
    }

    #[test]
    fn cr_suffix_positive() {
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::CreditRight,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), false);
        assert_eq!(ne.as_bytes(), b"0042  ");
    }

    #[test]
    fn db_suffix_negative() {
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::DebitRight,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), true);
        assert_eq!(ne.as_bytes(), b"0042DB");
    }

    // --- Insertion Characters ---

    #[test]
    fn insert_space() {
        // PIC 99B99
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::InsertSpace,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(1234), false);
        assert_eq!(ne.as_bytes(), b"12 34");
    }

    #[test]
    fn insert_zero() {
        // PIC 990099
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::InsertZero,
            EditSymbol::InsertZero,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(1234), false);
        assert_eq!(ne.as_bytes(), b"120034");
    }

    #[test]
    fn insert_slash() {
        // PIC 99/99/99: date-like format
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::InsertSlash,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::InsertSlash,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 6, 0);
        ne.set_from_numeric(dec!(123199), false);
        assert_eq!(ne.as_bytes(), b"12/31/99");
    }

    // --- BLANK WHEN ZERO ---

    #[test]
    fn blank_when_zero() {
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
        ];
        let mut ne = NumericEdited::new(mask, 4, 0, true);
        ne.set_from_numeric(dec!(0), false);
        assert_eq!(ne.as_bytes(), b"    ");
    }

    #[test]
    fn blank_when_zero_nonzero() {
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
        ];
        let mut ne = NumericEdited::new(mask, 4, 0, true);
        ne.set_from_numeric(dec!(5), false);
        assert_eq!(ne.as_bytes(), b"   5");
    }

    // --- De-editing ---

    #[test]
    fn de_edit_basic() {
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Period,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 2);
        ne.set_from_numeric(dec!(12.34), false);
        assert_eq!(ne.de_edit(), Some(dec!(12.34)));
    }

    #[test]
    fn de_edit_negative_cr() {
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::CreditRight,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(42), true);
        assert_eq!(ne.de_edit(), Some(dec!(-42)));
    }

    #[test]
    fn de_edit_with_commas() {
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Comma,
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 4, 0);
        ne.set_from_numeric(dec!(1234), false);
        assert_eq!(ne.de_edit(), Some(dec!(1234)));
    }

    // --- Category and Trait ---

    #[test]
    fn category_is_numeric_edited() {
        let ne = make(&[EditSymbol::Digit], 1, 0);
        assert_eq!(ne.category(), DataCategory::NumericEdited);
    }

    #[test]
    fn byte_length_with_cr() {
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::CreditRight,
        ];
        let ne = make(&mask, 2, 0);
        // 2 digits + 2 for CR = 4 bytes
        assert_eq!(ne.byte_length(), 4);
    }

    #[test]
    fn initialize_default_formats_zero() {
        let mask = vec![
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::ZeroSuppress(b' '),
            EditSymbol::Digit,
            EditSymbol::Period,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 3, 2);
        ne.set_from_numeric(dec!(12.34), false);
        ne.initialize_default();
        assert_eq!(ne.as_bytes(), b"  0.00");
    }

    #[test]
    fn trait_object_numeric_edited() {
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Period,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 2, 2);
        let edited: &mut dyn CobolNumericEdited = &mut ne;
        edited.set_from_numeric(dec!(12.34), false);
        assert_eq!(edited.integer_positions(), 2);
        assert_eq!(edited.decimal_positions(), 2);
        assert_eq!(edited.de_edit(), Some(dec!(12.34)));
    }

    // --- Left Truncation ---

    #[test]
    fn left_truncation() {
        // PIC 999: 3 digits, value 12345 -> 345
        let mask = vec![
            EditSymbol::Digit,
            EditSymbol::Digit,
            EditSymbol::Digit,
        ];
        let mut ne = make(&mask, 3, 0);
        ne.set_from_numeric(dec!(12345), false);
        assert_eq!(ne.as_bytes(), b"345");
    }
}
