// COMP-1 (single-precision floating-point) wrapper type.
//
// COBOL COMP-1 maps to IEEE 754 single-precision (f32, 4 bytes).
// This wrapper implements CobolField + CobolNumeric so the type
// participates in MOVE, arithmetic, and comparisons through the
// standard trait dispatch.
//
// Known limitation: uses IEEE 754, not IBM HFP (hexadecimal
// floating-point). IBM HFP support is deferred.

use cobol_core::{CobolField, CobolNumeric, DataCategory};
use rust_decimal::prelude::*;
use std::cmp::Ordering;

/// COMP-1: single-precision floating-point (4 bytes, IEEE 754).
///
/// In IBM Enterprise COBOL, COMP-1 fields have no PIC clause and
/// store values as 4-byte single-precision floats. They participate
/// in all numeric operations via Decimal conversion.
#[derive(Clone)]
pub struct Comp1Float {
    value: f32,
    /// Cached byte representation for `as_bytes()` support.
    byte_cache: [u8; 4],
}

impl std::fmt::Debug for Comp1Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Comp1Float({})", self.value)
    }
}

impl Comp1Float {
    /// Create a new `Comp1Float` with value 0.0.
    pub fn new() -> Self {
        Self {
            value: 0.0,
            byte_cache: [0u8; 4],
        }
    }

    /// Create from an f32 value.
    pub fn from_f32(v: f32) -> Self {
        let mut s = Self {
            value: v,
            byte_cache: [0u8; 4],
        };
        s.sync_byte_cache();
        s
    }

    /// Get the raw f32 value.
    pub fn as_f32(&self) -> f32 {
        self.value
    }

    /// Update the cached byte representation (native-endian IEEE 754).
    fn sync_byte_cache(&mut self) {
        self.byte_cache = self.value.to_ne_bytes();
    }

}

impl Default for Comp1Float {
    fn default() -> Self {
        Self::new()
    }
}

impl CobolField for Comp1Float {
    fn category(&self) -> DataCategory {
        DataCategory::Numeric
    }

    fn byte_length(&self) -> usize {
        4
    }

    fn as_bytes(&self) -> &[u8] {
        &self.byte_cache
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.byte_cache
    }

    fn display_bytes(&self) -> Vec<u8> {
        // Format as decimal string without trailing zeros.
        // COBOL DISPLAY of COMP-1 shows the value in readable numeric form.
        let s = format_float_display(f64::from(self.value));
        s.into_bytes()
    }

    fn fill_bytes(&mut self, byte: u8) {
        self.byte_cache = [byte; 4];
        self.value = f32::from_ne_bytes(self.byte_cache);
    }

    fn initialize_default(&mut self) {
        self.value = 0.0;
        self.sync_byte_cache();
    }
}

impl CobolNumeric for Comp1Float {
    fn to_decimal(&self) -> Decimal {
        Decimal::from_f64_retain(f64::from(self.value)).unwrap_or(Decimal::ZERO)
    }

    fn set_decimal(&mut self, value: Decimal) {
        self.value = value.to_f32().unwrap_or(0.0);
        self.sync_byte_cache();
    }

    fn scale(&self) -> u32 {
        // Floats have no fixed scale. Return high value so MOVE engine
        // preserves fractional digits (truncate_decimal uses this as
        // the number of decimal places to keep).
        7
    }

    fn precision(&self) -> u32 {
        // Set high enough that left_truncate_to_precision does not
        // discard integer digits. max_integer_digits = precision - scale = 21.
        28
    }

    fn is_signed(&self) -> bool {
        true
    }

    fn compare_numeric(&self, other: &dyn CobolNumeric) -> Ordering {
        self.to_decimal().cmp(&other.to_decimal())
    }
}

/// Format a float for COBOL DISPLAY output.
///
/// Produces a clean decimal string: no trailing zeros after the decimal
/// point, no scientific notation. If the value is an integer, no decimal
/// point is shown.
pub(crate) fn format_float_display(v: f64) -> String {
    if v.is_nan() {
        return "0".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 {
            "9999999999999999999".to_string()
        } else {
            "-9999999999999999999".to_string()
        };
    }
    // Use Decimal for clean formatting (avoids f64 precision artifacts).
    if let Some(d) = Decimal::from_f64_retain(v) {
        let d = d.normalize();
        
        d.to_string()
    } else {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn comp1_new_default_zero() {
        let c = Comp1Float::new();
        assert_eq!(c.as_f32(), 0.0);
        assert_eq!(c.to_decimal(), dec!(0));
    }

    #[test]
    fn comp1_from_f32() {
        let c = Comp1Float::from_f32(std::f32::consts::PI);
        // f32 precision: roundtrip through f32 storage
        assert!((c.as_f32() - std::f32::consts::PI).abs() < 0.001);
    }

    #[test]
    fn comp1_as_bytes_length() {
        let c = Comp1Float::from_f32(1.0);
        assert_eq!(c.as_bytes().len(), 4);
        assert_eq!(c.byte_length(), 4);
    }

    #[test]
    fn comp1_as_bytes_roundtrip() {
        let c1 = Comp1Float::from_f32(42.5);
        let bytes = c1.as_bytes().to_vec();
        let recovered = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert_eq!(recovered, 42.5);
    }

    #[test]
    fn comp1_display_bytes() {
        let c = Comp1Float::from_f32(123.0);
        let display = String::from_utf8(c.display_bytes()).unwrap();
        assert_eq!(display, "123");
    }

    #[test]
    fn comp1_to_decimal() {
        let c = Comp1Float::from_f32(100.0);
        assert_eq!(c.to_decimal(), dec!(100));
    }

    #[test]
    fn comp1_set_decimal() {
        let mut c = Comp1Float::new();
        c.set_decimal(dec!(250));
        assert_eq!(c.as_f32(), 250.0);
    }

    #[test]
    fn comp1_category_is_numeric() {
        let c = Comp1Float::new();
        assert_eq!(c.category(), DataCategory::Numeric);
    }

    #[test]
    fn comp1_is_signed() {
        let c = Comp1Float::new();
        assert!(c.is_signed());
    }

    #[test]
    fn comp1_precision_and_scale() {
        let c = Comp1Float::new();
        assert_eq!(c.precision(), 28);
        assert_eq!(c.scale(), 7);
    }

    #[test]
    fn comp1_initialize_default() {
        let mut c = Comp1Float::from_f32(999.0);
        c.initialize_default();
        assert_eq!(c.as_f32(), 0.0);
    }

    #[test]
    fn comp1_negative_value() {
        let c = Comp1Float::from_f32(-42.5);
        assert_eq!(c.as_f32(), -42.5);
        let d = c.to_decimal();
        assert!(d < dec!(0));
    }

    #[test]
    fn comp1_compare_numeric() {
        let a = Comp1Float::from_f32(10.0);
        let b = Comp1Float::from_f32(20.0);
        assert_eq!(a.compare_numeric(&b), Ordering::Less);
        assert_eq!(b.compare_numeric(&a), Ordering::Greater);
    }

    #[test]
    fn comp1_fill_bytes() {
        let mut c = Comp1Float::new();
        c.fill_bytes(0xFF);
        assert_eq!(c.as_bytes(), &[0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn comp1_display_fractional() {
        let c = Comp1Float::from_f32(3.5);
        let display = String::from_utf8(c.display_bytes()).unwrap();
        assert_eq!(display, "3.5");
    }

    #[test]
    fn format_float_display_nan() {
        assert_eq!(format_float_display(f64::NAN), "0");
    }

    #[test]
    fn format_float_display_infinity() {
        let s = format_float_display(f64::INFINITY);
        assert!(s.starts_with('9'));
    }
}
