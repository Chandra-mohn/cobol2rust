//! Zoned Decimal (USAGE DISPLAY numeric) storage.
//!
//! Each digit occupies one byte. The zone nibble (high 4 bits) is normally 0xF
//! (for ASCII-compatible representation) or 0x3 (for EBCDIC).
//! The sign is embedded in the zone nibble of the last byte:
//! - 0xC = positive
//! - 0xD = negative
//! - 0xF = unsigned (positive)
//!
//! Storage size is always equal to the precision (one byte per digit).

use rust_decimal::Decimal;

use cobol_core::category::DataCategory;
use cobol_core::traits::{CobolField, CobolNumeric};

/// Zoned Decimal storage (PIC 9 USAGE DISPLAY).
///
/// Each byte stores one digit with zone nibble encoding.
/// This is the default COBOL numeric storage (USAGE DISPLAY).
#[derive(Clone)]
pub struct ZonedDecimal {
    /// Raw zoned bytes: one byte per digit, zone in high nibble, digit in low nibble.
    data: Vec<u8>,
    precision: u32,
    scale: u32,
    signed: bool,
}

impl std::fmt::Debug for ZonedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZonedDecimal")
            .field("value", &self.to_decimal())
            .field("precision", &self.precision)
            .field("scale", &self.scale)
            .field("signed", &self.signed)
            .field("bytes", &self.data)
            .finish()
    }
}

impl ZonedDecimal {
    /// Create a new `ZonedDecimal` initialized to zero.
    ///
    /// - `precision`: total digit count from PIC clause
    /// - `scale`: decimal places (digits after V)
    /// - `signed`: PIC S... or not
    pub fn new(precision: u32, scale: u32, signed: bool) -> Self {
        // Initialize all bytes to zoned zero: 0xF0
        let mut data = vec![0xF0u8; precision as usize];
        // Last byte sign: 0xF0 for unsigned, 0xC0 for positive signed
        if signed && !data.is_empty() {
            let last = data.len() - 1;
            data[last] = 0xC0; // Positive sign, digit 0
        }
        Self {
            data,
            precision,
            scale,
            signed,
        }
    }

    /// Create a `ZonedDecimal` from a Decimal value.
    pub fn from_decimal(precision: u32, scale: u32, signed: bool, value: Decimal) -> Self {
        let mut zd = Self::new(precision, scale, signed);
        zd.set_decimal(value);
        zd
    }

    /// Get the raw zoned bytes.
    pub fn zoned_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Set from raw zoned bytes.
    ///
    /// The byte slice must have exactly `precision` bytes.
    pub fn set_zoned_bytes(&mut self, bytes: &[u8]) {
        if bytes.len() == self.data.len() {
            self.data.copy_from_slice(bytes);
        }
    }

    /// Extract the sign from the zone nibble of the last byte.
    fn extract_sign(&self) -> bool {
        if !self.signed || self.data.is_empty() {
            return false;
        }
        let zone = self.data[self.data.len() - 1] >> 4;
        zone == 0x0D // D-zone = negative
    }

    /// Extract digits from zoned bytes as a string.
    fn extract_digits(&self) -> String {
        let mut digits = String::with_capacity(self.data.len());
        for &byte in &self.data {
            let digit = byte & 0x0F;
            if digit <= 9 {
                digits.push(char::from(b'0' + digit));
            } else {
                digits.push('0'); // Invalid digit -> 0
            }
        }
        digits
    }

    /// Encode digits and sign back into zoned bytes.
    fn encode_zoned(&mut self, digits: &[u8], negative: bool) {
        let prec = self.precision as usize;
        let len = self.data.len();

        for i in 0..len {
            let digit = if i < prec { digits.get(i).copied().unwrap_or(0) } else { 0 };
            if i == len - 1 && self.signed {
                // Last byte: encode sign in zone nibble
                let zone = if negative { 0xD0 } else { 0xC0 };
                self.data[i] = zone | (digit & 0x0F);
            } else {
                self.data[i] = 0xF0 | (digit & 0x0F);
            }
        }
    }
}

impl CobolField for ZonedDecimal {
    fn category(&self) -> DataCategory {
        DataCategory::Numeric
    }

    fn byte_length(&self) -> usize {
        self.precision as usize
    }

    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn display_bytes(&self) -> Vec<u8> {
        let value = self.to_decimal();
        crate::packed_decimal::format_display(value, self.precision, self.scale, self.signed)
    }

    fn fill_bytes(&mut self, byte: u8) {
        self.data.fill(byte);
    }

    fn initialize_default(&mut self) {
        // Reset to zoned zeros
        for i in 0..self.data.len() {
            self.data[i] = 0xF0;
        }
        if self.signed && !self.data.is_empty() {
            let last = self.data.len() - 1;
            self.data[last] = 0xC0;
        }
    }
}

impl CobolNumeric for ZonedDecimal {
    fn to_decimal(&self) -> Decimal {
        let digits = self.extract_digits();
        let negative = self.extract_sign();

        if digits.is_empty() {
            return Decimal::ZERO;
        }

        match Decimal::from_str_exact(&digits) {
            Ok(mut d) => {
                if self.scale > 0 {
                    let divisor = Decimal::from(10i64.pow(self.scale));
                    d /= divisor;
                }
                if negative { -d } else { d }
            }
            Err(_) => Decimal::ZERO,
        }
    }

    fn set_decimal(&mut self, value: Decimal) {
        let negative = value.is_sign_negative() && !value.is_zero();

        // Scale to integer
        let scale_factor = Decimal::from(10i64.pow(self.scale));
        let scaled = (value.abs() * scale_factor).trunc();

        // Convert to digit string
        let digit_str = scaled.to_string();
        let prec = self.precision as usize;

        // Build digit bytes (BCD-style, right-justified with leading zeros)
        let mut digit_bytes = vec![0u8; prec];
        let str_bytes = digit_str.as_bytes();

        if str_bytes.len() >= prec {
            // Left-truncation: take rightmost `prec` digits
            let start = str_bytes.len() - prec;
            for (i, &b) in str_bytes[start..].iter().enumerate() {
                digit_bytes[i] = b - b'0';
            }
        } else {
            // Right-justify with leading zeros
            let start = prec - str_bytes.len();
            for (i, &b) in str_bytes.iter().enumerate() {
                digit_bytes[start + i] = b - b'0';
            }
        }

        self.encode_zoned(&digit_bytes, negative);
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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn new_initializes_to_zero() {
        let zd = ZonedDecimal::new(5, 0, false);
        assert_eq!(zd.to_decimal(), dec!(0));
        assert_eq!(zd.data, vec![0xF0, 0xF0, 0xF0, 0xF0, 0xF0]);
    }

    #[test]
    fn new_signed_zero() {
        let zd = ZonedDecimal::new(5, 0, true);
        assert_eq!(zd.to_decimal(), dec!(0));
        // Last byte has C-zone (positive signed)
        assert_eq!(zd.data[4], 0xC0);
    }

    #[test]
    fn set_and_get_positive() {
        let mut zd = ZonedDecimal::new(5, 0, false);
        zd.set_decimal(dec!(12345));
        assert_eq!(zd.to_decimal(), dec!(12345));
        // Verify zoned encoding
        assert_eq!(zd.data[0], 0xF1);
        assert_eq!(zd.data[1], 0xF2);
        assert_eq!(zd.data[2], 0xF3);
        assert_eq!(zd.data[3], 0xF4);
        assert_eq!(zd.data[4], 0xF5);
    }

    #[test]
    fn set_and_get_negative() {
        let mut zd = ZonedDecimal::new(5, 0, true);
        zd.set_decimal(dec!(-123));
        assert_eq!(zd.to_decimal(), dec!(-123));
        // Last byte should have D-zone (negative)
        assert_eq!(zd.data[4] >> 4, 0x0D);
        assert_eq!(zd.data[4] & 0x0F, 3); // digit 3
    }

    #[test]
    fn set_and_get_with_scale() {
        let mut zd = ZonedDecimal::new(7, 2, true);
        zd.set_decimal(dec!(123.45));
        assert_eq!(zd.to_decimal(), dec!(123.45));
    }

    #[test]
    fn left_truncation() {
        // 12345 into precision 3 -> 345
        let mut zd = ZonedDecimal::new(3, 0, false);
        zd.set_decimal(dec!(12345));
        assert_eq!(zd.to_decimal(), dec!(345));
    }

    #[test]
    fn leading_zeros() {
        let mut zd = ZonedDecimal::new(5, 0, false);
        zd.set_decimal(dec!(42));
        assert_eq!(zd.to_decimal(), dec!(42));
        // First three bytes should be zoned zeros
        assert_eq!(zd.data[0], 0xF0);
        assert_eq!(zd.data[1], 0xF0);
        assert_eq!(zd.data[2], 0xF0);
    }

    #[test]
    fn from_decimal_constructor() {
        let zd = ZonedDecimal::from_decimal(5, 2, true, dec!(-12.34));
        assert_eq!(zd.to_decimal(), dec!(-12.34));
    }

    #[test]
    fn category_is_numeric() {
        let zd = ZonedDecimal::new(5, 0, false);
        assert_eq!(zd.category(), DataCategory::Numeric);
    }

    #[test]
    fn byte_length_equals_precision() {
        let zd = ZonedDecimal::new(9, 0, false);
        assert_eq!(zd.byte_length(), 9);
    }

    #[test]
    fn as_bytes_returns_zoned_data() {
        let mut zd = ZonedDecimal::new(3, 0, false);
        zd.set_decimal(dec!(123));
        assert_eq!(zd.as_bytes(), &[0xF1, 0xF2, 0xF3]);
    }

    #[test]
    fn initialize_default_resets() {
        let mut zd = ZonedDecimal::new(5, 0, true);
        zd.set_decimal(dec!(12345));
        zd.initialize_default();
        assert_eq!(zd.to_decimal(), dec!(0));
    }

    #[test]
    fn set_zoned_bytes_direct() {
        let mut zd = ZonedDecimal::new(3, 0, true);
        // Set bytes for -123: [0xF1, 0xF2, 0xD3]
        zd.set_zoned_bytes(&[0xF1, 0xF2, 0xD3]);
        assert_eq!(zd.to_decimal(), dec!(-123));
    }

    #[test]
    fn unsigned_negative_becomes_positive() {
        let mut zd = ZonedDecimal::new(5, 0, false);
        zd.set_decimal(dec!(-123));
        // Unsigned: stores absolute value, zone stays 0xF
        assert_eq!(zd.to_decimal(), dec!(123));
    }

    #[test]
    fn trait_object_numeric() {
        let mut zd = ZonedDecimal::new(7, 2, true);
        let num: &mut dyn CobolNumeric = &mut zd;
        num.set_decimal(dec!(123.45));
        assert_eq!(num.to_decimal(), dec!(123.45));
        assert_eq!(num.scale(), 2);
        assert_eq!(num.precision(), 7);
        assert!(num.is_signed());
    }

    #[test]
    fn display_bytes_format() {
        let mut zd = ZonedDecimal::new(5, 2, true);
        zd.set_decimal(dec!(1.23));
        let display = zd.display_bytes();
        assert_eq!(display, b"+00123");
    }

    #[test]
    fn zero_value_signed() {
        let zd = ZonedDecimal::new(3, 0, true);
        assert_eq!(zd.to_decimal(), dec!(0));
        // Zero should have positive sign (C-zone)
        assert_eq!(zd.data[2] >> 4, 0x0C);
    }

    #[test]
    fn large_precision() {
        let mut zd = ZonedDecimal::new(18, 0, true);
        zd.set_decimal(dec!(123456789012345678));
        assert_eq!(zd.to_decimal(), dec!(123456789012345678));
    }
}
