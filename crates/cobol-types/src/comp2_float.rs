// COMP-2 (double-precision floating-point) wrapper type.
//
// COBOL COMP-2 maps to IEEE 754 double-precision (f64, 8 bytes).
// This wrapper implements CobolField + CobolNumeric so the type
// participates in MOVE, arithmetic, and comparisons through the
// standard trait dispatch.
//
// Known limitation: uses IEEE 754, not IBM HFP (hexadecimal
// floating-point). IBM HFP support is deferred.

use cobol_core::{CobolField, CobolNumeric, DataCategory};
use rust_decimal::prelude::*;
use std::cmp::Ordering;

use crate::comp1_float::format_float_display;

/// COMP-2: double-precision floating-point (8 bytes, IEEE 754).
///
/// In IBM Enterprise COBOL, COMP-2 fields have no PIC clause and
/// store values as 8-byte double-precision floats. They participate
/// in all numeric operations via Decimal conversion.
#[derive(Clone)]
pub struct Comp2Float {
    value: f64,
    /// Cached byte representation for `as_bytes()` support.
    byte_cache: [u8; 8],
}

impl std::fmt::Debug for Comp2Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Comp2Float({})", self.value)
    }
}

impl Comp2Float {
    /// Create a new `Comp2Float` with value 0.0.
    pub fn new() -> Self {
        Self {
            value: 0.0,
            byte_cache: [0u8; 8],
        }
    }

    /// Create from an f64 value.
    pub fn from_f64(v: f64) -> Self {
        let mut s = Self {
            value: v,
            byte_cache: [0u8; 8],
        };
        s.sync_byte_cache();
        s
    }

    /// Get the raw f64 value.
    pub fn as_f64(&self) -> f64 {
        self.value
    }

    /// Update the cached byte representation (native-endian IEEE 754).
    fn sync_byte_cache(&mut self) {
        self.byte_cache = self.value.to_ne_bytes();
    }
}

impl Default for Comp2Float {
    fn default() -> Self {
        Self::new()
    }
}

impl CobolField for Comp2Float {
    fn category(&self) -> DataCategory {
        DataCategory::Numeric
    }

    fn byte_length(&self) -> usize {
        8
    }

    fn as_bytes(&self) -> &[u8] {
        &self.byte_cache
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.byte_cache
    }

    fn display_bytes(&self) -> Vec<u8> {
        let s = format_float_display(self.value);
        s.into_bytes()
    }

    fn fill_bytes(&mut self, byte: u8) {
        self.byte_cache = [byte; 8];
        self.value = f64::from_ne_bytes(self.byte_cache);
    }

    fn initialize_default(&mut self) {
        self.value = 0.0;
        self.sync_byte_cache();
    }
}

impl CobolNumeric for Comp2Float {
    fn to_decimal(&self) -> Decimal {
        Decimal::from_f64_retain(self.value).unwrap_or(Decimal::ZERO)
    }

    fn set_decimal(&mut self, value: Decimal) {
        self.value = value.to_f64().unwrap_or(0.0);
        self.sync_byte_cache();
    }

    fn scale(&self) -> u32 {
        // Floats have no fixed scale. Return high value so MOVE engine
        // preserves fractional digits (truncate_decimal uses this as
        // the number of decimal places to keep).
        15
    }

    fn precision(&self) -> u32 {
        // Set high enough that left_truncate_to_precision does not
        // discard integer digits. max_integer_digits = precision - scale = 13.
        28
    }

    fn is_signed(&self) -> bool {
        true
    }

    fn compare_numeric(&self, other: &dyn CobolNumeric) -> Ordering {
        self.to_decimal().cmp(&other.to_decimal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn comp2_new_default_zero() {
        let c = Comp2Float::new();
        assert_eq!(c.as_f64(), 0.0);
        assert_eq!(c.to_decimal(), dec!(0));
    }

    #[test]
    fn comp2_from_f64() {
        let c = Comp2Float::from_f64(std::f64::consts::PI);
        assert!((c.as_f64() - std::f64::consts::PI).abs() < 1e-15);
    }

    #[test]
    fn comp2_as_bytes_length() {
        let c = Comp2Float::from_f64(1.0);
        assert_eq!(c.as_bytes().len(), 8);
        assert_eq!(c.byte_length(), 8);
    }

    #[test]
    fn comp2_as_bytes_roundtrip() {
        let c1 = Comp2Float::from_f64(42.5);
        let bytes = c1.as_bytes().to_vec();
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes);
        let recovered = f64::from_ne_bytes(arr);
        assert_eq!(recovered, 42.5);
    }

    #[test]
    fn comp2_display_bytes() {
        let c = Comp2Float::from_f64(12345.0);
        let display = String::from_utf8(c.display_bytes()).unwrap();
        assert_eq!(display, "12345");
    }

    #[test]
    fn comp2_to_decimal() {
        let c = Comp2Float::from_f64(100.0);
        assert_eq!(c.to_decimal(), dec!(100));
    }

    #[test]
    fn comp2_set_decimal() {
        let mut c = Comp2Float::new();
        c.set_decimal(dec!(999));
        assert_eq!(c.as_f64(), 999.0);
    }

    #[test]
    fn comp2_category_is_numeric() {
        let c = Comp2Float::new();
        assert_eq!(c.category(), DataCategory::Numeric);
    }

    #[test]
    fn comp2_is_signed() {
        let c = Comp2Float::new();
        assert!(c.is_signed());
    }

    #[test]
    fn comp2_precision_and_scale() {
        let c = Comp2Float::new();
        assert_eq!(c.precision(), 28);
        assert_eq!(c.scale(), 15);
    }

    #[test]
    fn comp2_initialize_default() {
        let mut c = Comp2Float::from_f64(999.0);
        c.initialize_default();
        assert_eq!(c.as_f64(), 0.0);
    }

    #[test]
    fn comp2_negative_value() {
        let c = Comp2Float::from_f64(-42.5);
        assert_eq!(c.as_f64(), -42.5);
        let d = c.to_decimal();
        assert!(d < dec!(0));
    }

    #[test]
    fn comp2_compare_numeric() {
        let a = Comp2Float::from_f64(10.0);
        let b = Comp2Float::from_f64(20.0);
        assert_eq!(a.compare_numeric(&b), Ordering::Less);
        assert_eq!(b.compare_numeric(&a), Ordering::Greater);
    }

    #[test]
    fn comp2_fill_bytes() {
        let mut c = Comp2Float::new();
        c.fill_bytes(0xFF);
        assert_eq!(c.as_bytes(), &[0xFF; 8]);
    }

    #[test]
    fn comp2_display_fractional() {
        let c = Comp2Float::from_f64(3.5);
        let display = String::from_utf8(c.display_bytes()).unwrap();
        assert_eq!(display, "3.5");
    }

    #[test]
    fn comp2_high_precision() {
        let c = Comp2Float::from_f64(1.234_567_890_123_45);
        let d = c.to_decimal();
        // Should preserve ~15 significant digits
        let diff = (d - dec!(1.23456789012345)).abs();
        assert!(diff < dec!(0.000000000001));
    }
}
