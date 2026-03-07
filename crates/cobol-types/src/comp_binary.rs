use rust_decimal::Decimal;
use rust_decimal::prelude::*;

use cobol_core::category::DataCategory;
use cobol_core::traits::{CobolField, CobolNumeric};

/// Binary integer storage (COMP, COMP-4, COMP-5).
///
/// COBOL binary fields store values in 2, 4, or 8 bytes depending on PIC digit count:
/// - PIC 9(1-4): 2 bytes (i16/u16)
/// - PIC 9(5-9): 4 bytes (i32/u32)
/// - PIC 9(10-18): 8 bytes (i64/u64)
///
/// Two modes:
/// - COMP (PIC-limited): values are limited to the range of the PIC clause
/// - COMP-5 (full binary): values use the full range of the storage type
#[derive(Clone)]
pub struct CompBinary {
    /// Stored as i64 internally; actual byte size determined by precision
    value: i64,
    precision: u32,
    scale: u32,
    signed: bool,
    /// COMP (true) vs COMP-5 (false): PIC-limited vs full binary range
    pic_limited: bool,
    /// Cached big-endian byte representation for `as_bytes()` support.
    /// Only the first `storage_bytes()` bytes are valid.
    byte_cache: [u8; 8],
}

impl std::fmt::Debug for CompBinary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompBinary")
            .field("value", &self.to_decimal())
            .field("precision", &self.precision)
            .field("scale", &self.scale)
            .field("signed", &self.signed)
            .field("pic_limited", &self.pic_limited)
            .field("storage_bytes", &self.storage_bytes())
            .finish()
    }
}

impl CompBinary {
    /// Create a new `CompBinary` initialized to zero.
    ///
    /// - `precision`: total digit count from PIC clause
    /// - `scale`: decimal places (digits after V)
    /// - `signed`: PIC S... or not
    /// - `pic_limited`: true for COMP/COMP-4 (PIC-range limited), false for COMP-5
    pub fn new(precision: u32, scale: u32, signed: bool, pic_limited: bool) -> Self {
        Self {
            value: 0,
            precision,
            scale,
            signed,
            pic_limited,
            byte_cache: [0u8; 8],
        }
    }

    /// Storage size in bytes based on PIC digit count.
    pub fn storage_bytes(&self) -> usize {
        match self.precision {
            1..=4 => 2,
            5..=9 => 4,
            _ => 8, // 10..=18 and anything larger
        }
    }

    /// Store a raw integer value (unscaled).
    pub fn set_raw(&mut self, raw: i64) {
        if self.pic_limited {
            // COBOL left-truncation for PIC-limited fields
            let max = 10i64.pow(self.precision);
            self.value = raw.abs() % max;
            if raw < 0 && self.signed {
                self.value = -self.value;
            }
        } else {
            self.value = raw;
        }
        self.sync_byte_cache();
    }

    /// Update the cached big-endian byte representation.
    fn sync_byte_cache(&mut self) {
        self.byte_cache = self.value.to_be_bytes();
    }

    /// Get the raw integer value (unscaled).
    pub fn raw(&self) -> i64 {
        self.value
    }

    /// Get big-endian byte representation at the correct storage size.
    pub fn to_be_bytes(&self) -> Vec<u8> {
        match self.storage_bytes() {
            2 => (self.value as i16).to_be_bytes().to_vec(),
            4 => (self.value as i32).to_be_bytes().to_vec(),
            _ => self.value.to_be_bytes().to_vec(),
        }
    }

    /// Set value from big-endian bytes.
    pub fn from_be_bytes(&mut self, bytes: &[u8]) {
        self.value = match bytes.len() {
            2 => {
                let mut buf = [0u8; 2];
                buf.copy_from_slice(bytes);
                i64::from(i16::from_be_bytes(buf))
            }
            4 => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(bytes);
                i64::from(i32::from_be_bytes(buf))
            }
            8 => {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(bytes);
                i64::from_be_bytes(buf)
            }
            _ => 0,
        };
        self.sync_byte_cache();
    }
}

impl CobolField for CompBinary {
    fn category(&self) -> DataCategory {
        DataCategory::Numeric
    }

    fn byte_length(&self) -> usize {
        self.storage_bytes()
    }

    fn as_bytes(&self) -> &[u8] {
        // Return the relevant slice from the byte cache (big-endian, right-aligned in [u8; 8]).
        let sb = self.storage_bytes();
        &self.byte_cache[8 - sb..]
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        let sb = self.storage_bytes();
        &mut self.byte_cache[8 - sb..]
    }

    fn display_bytes(&self) -> Vec<u8> {
        let decimal = self.to_decimal();
        crate::packed_decimal::format_display(decimal, self.precision, self.scale, self.signed)
    }

    fn fill_bytes(&mut self, _byte: u8) {
        self.value = 0;
        self.sync_byte_cache();
    }

    fn initialize_default(&mut self) {
        self.value = 0;
        self.sync_byte_cache();
    }
}

impl CobolNumeric for CompBinary {
    fn to_decimal(&self) -> Decimal {
        let mut d = Decimal::from(self.value);
        if self.scale > 0 {
            let divisor = Decimal::from(10i64.pow(self.scale));
            d /= divisor;
        }
        d
    }

    fn set_decimal(&mut self, value: Decimal) {
        // Scale to integer
        let scale_factor = Decimal::from(10i64.pow(self.scale));
        let scaled = (value * scale_factor).trunc();
        let raw = scaled.to_i64().unwrap_or(0);
        self.set_raw(raw); // set_raw calls sync_byte_cache
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
        let cb = CompBinary::new(5, 0, true, true);
        assert_eq!(cb.to_decimal(), dec!(0));
    }

    #[test]
    fn storage_bytes_by_precision() {
        assert_eq!(CompBinary::new(1, 0, true, true).storage_bytes(), 2);
        assert_eq!(CompBinary::new(4, 0, true, true).storage_bytes(), 2);
        assert_eq!(CompBinary::new(5, 0, true, true).storage_bytes(), 4);
        assert_eq!(CompBinary::new(9, 0, true, true).storage_bytes(), 4);
        assert_eq!(CompBinary::new(10, 0, true, true).storage_bytes(), 8);
        assert_eq!(CompBinary::new(18, 0, true, true).storage_bytes(), 8);
    }

    #[test]
    fn set_and_get_decimal() {
        let mut cb = CompBinary::new(5, 0, true, true);
        cb.set_decimal(dec!(12345));
        assert_eq!(cb.to_decimal(), dec!(12345));
    }

    #[test]
    fn set_and_get_with_scale() {
        let mut cb = CompBinary::new(5, 2, true, true);
        cb.set_decimal(dec!(123.45));
        assert_eq!(cb.to_decimal(), dec!(123.45));
    }

    #[test]
    fn negative_value() {
        let mut cb = CompBinary::new(5, 0, true, true);
        cb.set_decimal(dec!(-12345));
        assert_eq!(cb.to_decimal(), dec!(-12345));
    }

    #[test]
    fn pic_limited_left_truncation() {
        // 12345 into PIC 9(3) COMP -> 345 (left-truncation)
        let mut cb = CompBinary::new(3, 0, false, true);
        cb.set_decimal(dec!(12345));
        assert_eq!(cb.to_decimal(), dec!(345));
    }

    #[test]
    fn comp5_no_truncation() {
        // COMP-5 allows full binary range
        let mut cb = CompBinary::new(3, 0, false, false);
        cb.set_decimal(dec!(12345));
        assert_eq!(cb.to_decimal(), dec!(12345));
    }

    #[test]
    fn big_endian_bytes() {
        let mut cb = CompBinary::new(4, 0, true, true);
        cb.set_decimal(dec!(256));
        let bytes = cb.to_be_bytes();
        assert_eq!(bytes, vec![0x01, 0x00]); // 256 as i16 big-endian
    }

    #[test]
    fn from_be_bytes() {
        let mut cb = CompBinary::new(4, 0, true, true);
        cb.from_be_bytes(&[0x01, 0x00]); // 256 as i16 big-endian
        assert_eq!(cb.to_decimal(), dec!(256));
    }

    #[test]
    fn category_is_numeric() {
        let cb = CompBinary::new(5, 0, true, true);
        assert_eq!(cb.category(), DataCategory::Numeric);
    }

    #[test]
    fn initialize_default_sets_zero() {
        let mut cb = CompBinary::new(5, 0, true, true);
        cb.set_decimal(dec!(12345));
        cb.initialize_default();
        assert_eq!(cb.to_decimal(), dec!(0));
    }

    #[test]
    fn trait_object_numeric() {
        let mut cb = CompBinary::new(5, 2, true, true);
        let num: &mut dyn CobolNumeric = &mut cb;
        num.set_decimal(dec!(123.45));
        assert_eq!(num.to_decimal(), dec!(123.45));
        assert_eq!(num.scale(), 2);
        assert_eq!(num.precision(), 5);
        assert!(num.is_signed());
    }

    #[test]
    fn display_bytes_format() {
        let mut cb = CompBinary::new(5, 2, true, true);
        cb.set_decimal(dec!(1.23));
        let display = cb.display_bytes();
        assert_eq!(display, b"+00123");
    }

    #[test]
    fn as_bytes_returns_correct_slice() {
        let mut cb = CompBinary::new(4, 0, true, true);
        cb.set_decimal(dec!(256));
        let bytes = cb.as_bytes();
        assert_eq!(bytes, &[0x01, 0x00]); // 256 as i16 big-endian
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn as_bytes_4byte_field() {
        let mut cb = CompBinary::new(9, 0, true, true);
        cb.set_decimal(dec!(1000));
        let bytes = cb.as_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, &[0x00, 0x00, 0x03, 0xE8]); // 1000 as i32 big-endian
    }

    #[test]
    fn as_bytes_roundtrip() {
        let mut cb1 = CompBinary::new(5, 0, true, true);
        cb1.set_decimal(dec!(12345));
        let bytes = cb1.as_bytes().to_vec();

        let mut cb2 = CompBinary::new(5, 0, true, true);
        cb2.from_be_bytes(&bytes);
        assert_eq!(cb2.to_decimal(), dec!(12345));
    }

    #[test]
    fn as_bytes_mut_modifiable() {
        let mut cb = CompBinary::new(4, 0, true, true);
        cb.set_decimal(dec!(0));
        let bytes = cb.as_bytes_mut();
        // Write 256 as big-endian i16
        bytes[0] = 0x01;
        bytes[1] = 0x00;
        // Note: value field is NOT auto-synced from byte_cache mutation.
        // This is a known limitation -- from_be_bytes should be used for external writes.
        assert_eq!(cb.as_bytes(), &[0x01, 0x00]);
    }

    #[test]
    fn as_bytes_negative_value() {
        let mut cb = CompBinary::new(4, 0, true, true);
        cb.set_decimal(dec!(-1));
        let bytes = cb.as_bytes();
        // -1 as i16 big-endian: 0xFF, 0xFF
        assert_eq!(bytes, &[0xFF, 0xFF]);
    }
}
