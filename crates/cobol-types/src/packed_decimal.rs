use rust_decimal::Decimal;

use cobol_core::category::DataCategory;
use cobol_core::traits::{CobolField, CobolNumeric};

/// COMP-3 / PACKED-DECIMAL storage.
///
/// Stores decimal value as BCD nibbles with trailing sign nibble.
/// Byte layout: `[d1 d2][d3 d4]...[dN sign]`
/// Sign nibble: `0xC` = positive, `0xD` = negative, `0xF` = unsigned.
///
/// Storage size is always `(precision / 2) + 1` bytes.
#[derive(Clone)]
pub struct PackedDecimal {
    data: Vec<u8>,
    precision: u32,
    scale: u32,
    signed: bool,
}

impl std::fmt::Debug for PackedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedDecimal")
            .field("value", &self.unpack())
            .field("precision", &self.precision)
            .field("scale", &self.scale)
            .field("signed", &self.signed)
            .field("bytes", &self.data)
            .finish()
    }
}

impl PackedDecimal {
    /// Create a new `PackedDecimal` initialized to zero.
    ///
    /// - `precision`: total number of digits (integer + decimal)
    /// - `scale`: number of decimal places (digits after V)
    /// - `signed`: whether the field is signed (PIC S...)
    pub fn new(precision: u32, scale: u32, signed: bool) -> Self {
        let byte_len = (precision / 2) + 1;
        let mut data = vec![0u8; byte_len as usize];
        // Initialize sign nibble: C=positive, F=unsigned
        let last = data.len() - 1;
        data[last] = if signed { 0x0C } else { 0x0F };
        Self {
            data,
            precision,
            scale,
            signed,
        }
    }

    /// Pack a `Decimal` value into BCD bytes.
    ///
    /// The value is scaled to remove the decimal point, then each digit
    /// is stored as a 4-bit BCD nibble, with the sign nibble at the end.
    pub fn pack(&mut self, value: Decimal) {
        // Scale the value to integer form: 123.45 with scale=2 -> 12345
        let scale_factor = Decimal::from(10u64.pow(self.scale));
        let scaled = (value * scale_factor).trunc().abs();

        // Convert to digit string
        let digit_str = scaled.to_string();
        let digits: Vec<u8> = digit_str.bytes().map(|b| b - b'0').collect();

        // Pad or truncate to precision digits (left-pad with zeros, left-truncate if overflow)
        let prec = self.precision as usize;
        let final_digits: Vec<u8> = if digits.len() >= prec {
            // Left-truncation: take the rightmost `precision` digits
            digits[digits.len() - prec..].to_vec()
        } else {
            // Left-pad with zeros
            let mut padded = vec![0u8; prec - digits.len()];
            padded.extend_from_slice(&digits);
            padded
        };

        // Determine sign nibble
        let sign_nibble: u8 = if !self.signed {
            0x0F
        } else if value.is_sign_negative() && !value.is_zero() {
            0x0D
        } else {
            0x0C
        };

        // Pack digits + sign into BCD bytes via nibble array
        self.data.fill(0);
        let total_nibbles = self.precision + 1; // digits + sign
        let total_bytes = self.data.len();
        let mut nibbles: Vec<u8> = Vec::with_capacity(total_nibbles as usize);
        nibbles.extend_from_slice(&final_digits);
        nibbles.push(sign_nibble);

        // If total nibbles is even, we need a leading zero nibble
        // Actually, total_bytes = (precision / 2) + 1, and total_nibbles = precision + 1
        // For precision=5: total_nibbles=6, total_bytes=3 -> 6 nibbles in 3 bytes, perfect
        // For precision=4: total_nibbles=5, total_bytes=3 -> 5 nibbles in 3 bytes (6 slots)
        //   Need to left-pad nibbles with one 0
        let slots = total_bytes * 2;
        while nibbles.len() < slots {
            nibbles.insert(0, 0);
        }

        for byte_idx in 0..total_bytes {
            let high = nibbles[byte_idx * 2];
            let low = nibbles[byte_idx * 2 + 1];
            self.data[byte_idx] = (high << 4) | low;
        }
    }

    /// Unpack BCD bytes into a `Decimal` value.
    pub fn unpack(&self) -> Decimal {
        if self.data.is_empty() {
            return Decimal::ZERO;
        }

        // Extract all nibbles
        let mut nibbles: Vec<u8> = Vec::with_capacity(self.data.len() * 2);
        for &byte in &self.data {
            nibbles.push((byte >> 4) & 0x0F);
            nibbles.push(byte & 0x0F);
        }

        // Last nibble is the sign
        let sign_nibble = nibbles.pop().unwrap_or(0x0F);
        let is_negative = sign_nibble == 0x0D;

        // Remove any leading zero nibbles from padding
        // We want exactly `precision` digit nibbles (take the last `precision`)
        let prec = self.precision as usize;
        let digit_nibbles = if nibbles.len() > prec {
            &nibbles[nibbles.len() - prec..]
        } else {
            &nibbles
        };

        // Build the integer value from digits
        let mut result = Decimal::ZERO;
        for &d in digit_nibbles {
            result = result * Decimal::from(10) + Decimal::from(u32::from(d));
        }

        // Apply scale
        if self.scale > 0 {
            let divisor = Decimal::from(10u64.pow(self.scale));
            result /= divisor;
        }

        // Apply sign
        if is_negative {
            result = -result;
        }

        result
    }

    /// Get the raw BCD bytes.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl CobolField for PackedDecimal {
    fn category(&self) -> DataCategory {
        DataCategory::Numeric
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
        format_display(self.unpack(), self.precision, self.scale, self.signed)
    }

    fn fill_bytes(&mut self, byte: u8) {
        for b in &mut self.data {
            *b = byte;
        }
    }

    fn initialize_default(&mut self) {
        self.set_decimal(Decimal::ZERO);
    }

    fn set_value_from_decimal(&mut self, value: Decimal) {
        self.set_decimal(value);
    }
}

impl CobolNumeric for PackedDecimal {
    fn to_decimal(&self) -> Decimal {
        self.unpack()
    }

    fn set_decimal(&mut self, value: Decimal) {
        self.pack(value);
    }

    fn scale(&self) -> u32 {
        self.scale
    }

    fn precision(&self) -> u32 {
        self.precision
    }

    fn is_signed(&self) -> bool {
        self.signed
    }
}

/// Format a decimal value as a COBOL DISPLAY representation.
///
/// Produces a fixed-length byte string with leading zeros and optional sign.
/// Example: `123.45` with precision=7, scale=2 -> `"0012345"` (or `"+0012345"` if signed).
pub fn format_display(value: Decimal, precision: u32, scale: u32, signed: bool) -> Vec<u8> {
    let scale_factor = Decimal::from(10u64.pow(scale));
    let scaled = (value.abs() * scale_factor).trunc();
    let digit_str = scaled.to_string();

    let prec = precision as usize;
    let mut result = Vec::with_capacity(prec + usize::from(signed));

    // Build digits, left-padded with zeros
    let digits: Vec<u8> = if digit_str.len() >= prec {
        digit_str.as_bytes()[digit_str.len() - prec..].to_vec()
    } else {
        let mut padded = vec![b'0'; prec - digit_str.len()];
        padded.extend_from_slice(digit_str.as_bytes());
        padded
    };

    if signed {
        if value.is_sign_negative() && !value.is_zero() {
            result.push(b'-');
        } else {
            result.push(b'+');
        }
    }
    result.extend_from_slice(&digits);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn new_initializes_to_zero() {
        let pd = PackedDecimal::new(5, 2, true);
        assert_eq!(pd.unpack(), dec!(0));
        assert_eq!(pd.byte_length(), 3); // (5/2)+1 = 3
    }

    #[test]
    fn pack_unpack_positive() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.pack(dec!(123.45));
        assert_eq!(pd.unpack(), dec!(123.45));
    }

    #[test]
    fn pack_unpack_negative() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.pack(dec!(-123.45));
        assert_eq!(pd.unpack(), dec!(-123.45));
    }

    #[test]
    fn pack_unpack_zero() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.pack(dec!(0));
        assert_eq!(pd.unpack(), dec!(0));
    }

    #[test]
    fn pack_unpack_no_scale() {
        let mut pd = PackedDecimal::new(5, 0, true);
        pd.pack(dec!(12345));
        assert_eq!(pd.unpack(), dec!(12345));
    }

    #[test]
    fn unsigned_field() {
        let mut pd = PackedDecimal::new(3, 0, false);
        pd.pack(dec!(123));
        assert_eq!(pd.unpack(), dec!(123));
        // Sign nibble should be F
        let last_byte = pd.raw_bytes()[pd.byte_length() - 1];
        assert_eq!(last_byte & 0x0F, 0x0F);
    }

    #[test]
    fn left_truncation() {
        // 12345 into PIC 9(3) -> 345 (COBOL left-truncation)
        let mut pd = PackedDecimal::new(3, 0, false);
        pd.pack(dec!(12345));
        assert_eq!(pd.unpack(), dec!(345));
    }

    #[test]
    fn byte_length_correctness() {
        // PIC S9(5) COMP-3 -> (5/2)+1 = 3 bytes
        assert_eq!(PackedDecimal::new(5, 0, true).byte_length(), 3);
        // PIC S9(7) COMP-3 -> (7/2)+1 = 4 bytes
        assert_eq!(PackedDecimal::new(7, 0, true).byte_length(), 4);
        // PIC 9(1) COMP-3 -> (1/2)+1 = 1 byte
        assert_eq!(PackedDecimal::new(1, 0, false).byte_length(), 1);
        // PIC S9(9) COMP-3 -> (9/2)+1 = 5 bytes
        assert_eq!(PackedDecimal::new(9, 0, true).byte_length(), 5);
        // PIC S9(18) COMP-3 -> (18/2)+1 = 10 bytes
        assert_eq!(PackedDecimal::new(18, 0, true).byte_length(), 10);
    }

    #[test]
    fn round_trip_various_values() {
        let test_cases: Vec<(u32, u32, bool, Decimal)> = vec![
            (5, 2, true, dec!(0.01)),
            (5, 2, true, dec!(999.99)),
            (5, 2, true, dec!(-999.99)),
            (7, 0, false, dec!(1234567)),
            (9, 4, true, dec!(12345.6789)),
            (1, 0, false, dec!(7)),
        ];

        for (precision, scale, signed, value) in test_cases {
            let mut pd = PackedDecimal::new(precision, scale, signed);
            pd.pack(value);
            assert_eq!(
                pd.unpack(),
                value,
                "Round-trip failed for precision={precision}, scale={scale}, signed={signed}, value={value}"
            );
        }
    }

    #[test]
    fn display_bytes_format() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.pack(dec!(1.23));
        let display = pd.display_bytes();
        assert_eq!(display, b"+00123");
    }

    #[test]
    fn initialize_default_sets_zero() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.pack(dec!(123.45));
        pd.initialize_default();
        assert_eq!(pd.unpack(), dec!(0));
    }

    #[test]
    fn category_is_numeric() {
        let pd = PackedDecimal::new(5, 2, true);
        assert_eq!(pd.category(), DataCategory::Numeric);
    }

    #[test]
    fn trait_object_numeric() {
        let mut pd = PackedDecimal::new(5, 2, true);
        let num: &mut dyn CobolNumeric = &mut pd;
        num.set_decimal(dec!(123.45));
        assert_eq!(num.to_decimal(), dec!(123.45));
        assert_eq!(num.scale(), 2);
        assert_eq!(num.precision(), 5);
        assert!(num.is_signed());
    }
}
