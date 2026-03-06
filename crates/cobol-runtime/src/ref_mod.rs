//! Reference modification runtime helpers.
//!
//! COBOL reference modification allows accessing a substring of a field:
//!   `field-name(offset:length)` where offset is 1-based and length is optional.
//!
//! These helpers implement read (extract) and write (overlay) operations
//! with COBOL's 1-based indexing convention.

use cobol_core::CobolField;

/// Read a substring from a field (reference modification read).
///
/// `offset` is 1-based (COBOL convention). `length` is the byte count.
/// Returns the extracted bytes as a Vec.
///
/// # Panics
/// Panics if offset is 0, or if offset + length - 1 exceeds the field length.
pub fn ref_mod_read(field: &dyn CobolField, offset: usize, length: usize) -> Vec<u8> {
    assert!(offset >= 1, "Reference modification offset must be >= 1, got {offset}");
    let bytes = field.as_bytes();
    let start = offset - 1; // Convert to 0-based
    assert!(
        start + length <= bytes.len(),
        "Reference modification out of bounds: offset={offset}, length={length}, field_len={}",
        bytes.len()
    );
    bytes[start..start + length].to_vec()
}

/// Read from offset to end of field (when length is omitted).
///
/// `offset` is 1-based (COBOL convention).
/// Returns bytes from offset position to the end of the field.
///
/// # Panics
/// Panics if offset is 0 or exceeds the field length.
pub fn ref_mod_read_to_end(field: &dyn CobolField, offset: usize) -> Vec<u8> {
    assert!(offset >= 1, "Reference modification offset must be >= 1, got {offset}");
    let bytes = field.as_bytes();
    let start = offset - 1;
    assert!(
        start <= bytes.len(),
        "Reference modification offset {offset} exceeds field length {}",
        bytes.len()
    );
    bytes[start..].to_vec()
}

/// Write source bytes into a target field at a specific byte range.
///
/// `offset` is 1-based, `length` is the byte count of the target window.
/// Source bytes are left-justified in the target window:
/// - If source is shorter than length, remaining bytes are space-padded.
/// - If source is longer than length, source is truncated.
///
/// # Panics
/// Panics if offset is 0 or if the target window exceeds the field length.
pub fn ref_mod_write(
    source: &[u8],
    target: &mut dyn CobolField,
    offset: usize,
    length: usize,
) {
    assert!(offset >= 1, "Reference modification offset must be >= 1, got {offset}");
    let start = offset - 1;
    let target_bytes = target.as_bytes_mut();
    assert!(
        start + length <= target_bytes.len(),
        "Reference modification write out of bounds: offset={offset}, length={length}, field_len={}",
        target_bytes.len()
    );

    // Copy source into target window, left-justified with space padding
    for i in 0..length {
        if i < source.len() {
            target_bytes[start + i] = source[i];
        } else {
            target_bytes[start + i] = b' '; // Space-pad
        }
    }
}

/// Write source bytes into a target field from offset to end (length omitted).
///
/// `offset` is 1-based. The write window extends from offset to the end of the field.
/// Source bytes are left-justified with space padding if shorter.
///
/// # Panics
/// Panics if offset is 0 or exceeds the field length.
pub fn ref_mod_write_to_end(
    source: &[u8],
    target: &mut dyn CobolField,
    offset: usize,
) {
    assert!(offset >= 1, "Reference modification offset must be >= 1, got {offset}");
    let target_bytes = target.as_bytes_mut();
    let start = offset - 1;
    assert!(
        start <= target_bytes.len(),
        "Reference modification write offset {offset} exceeds field length {}",
        target_bytes.len()
    );

    let length = target_bytes.len() - start;
    for i in 0..length {
        if i < source.len() {
            target_bytes[start + i] = source[i];
        } else {
            target_bytes[start + i] = b' ';
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_types::PicX;

    #[test]
    fn read_first_n_bytes() {
        let field = PicX::new(10, b"ABCDEFGHIJ");
        let result = ref_mod_read(&field, 1, 3);
        assert_eq!(result, b"ABC");
    }

    #[test]
    fn read_middle_bytes() {
        let field = PicX::new(10, b"ABCDEFGHIJ");
        let result = ref_mod_read(&field, 3, 5);
        assert_eq!(result, b"CDEFG");
    }

    #[test]
    fn read_last_byte() {
        let field = PicX::new(10, b"ABCDEFGHIJ");
        let result = ref_mod_read(&field, 10, 1);
        assert_eq!(result, b"J");
    }

    #[test]
    fn read_entire_field() {
        let field = PicX::new(5, b"HELLO");
        let result = ref_mod_read(&field, 1, 5);
        assert_eq!(result, b"HELLO");
    }

    #[test]
    fn read_to_end_from_start() {
        let field = PicX::new(10, b"ABCDEFGHIJ");
        let result = ref_mod_read_to_end(&field, 1);
        assert_eq!(result, b"ABCDEFGHIJ");
    }

    #[test]
    fn read_to_end_from_offset() {
        let field = PicX::new(10, b"ABCDEFGHIJ");
        let result = ref_mod_read_to_end(&field, 4);
        assert_eq!(result, b"DEFGHIJ");
    }

    #[test]
    fn read_to_end_last_byte() {
        let field = PicX::new(5, b"HELLO");
        let result = ref_mod_read_to_end(&field, 5);
        assert_eq!(result, b"O");
    }

    #[test]
    fn write_into_middle() {
        let mut field = PicX::new(10, b"ABCDEFGHIJ");
        ref_mod_write(b"XY", &mut field, 3, 2);
        assert_eq!(field.as_bytes(), b"ABXYEFGHIJ");
    }

    #[test]
    fn write_at_start() {
        let mut field = PicX::new(10, b"ABCDEFGHIJ");
        ref_mod_write(b"ZZ", &mut field, 1, 2);
        assert_eq!(field.as_bytes(), b"ZZCDEFGHIJ");
    }

    #[test]
    fn write_with_space_padding() {
        let mut field = PicX::new(10, b"ABCDEFGHIJ");
        ref_mod_write(b"X", &mut field, 3, 4);
        assert_eq!(field.as_bytes(), b"ABX   GHIJ");
    }

    #[test]
    fn write_with_truncation() {
        let mut field = PicX::new(10, b"ABCDEFGHIJ");
        ref_mod_write(b"VWXYZ", &mut field, 3, 2);
        assert_eq!(field.as_bytes(), b"ABVWEFGHIJ");
    }

    #[test]
    fn write_to_end_from_offset() {
        let mut field = PicX::new(10, b"ABCDEFGHIJ");
        ref_mod_write_to_end(b"XYZ", &mut field, 8);
        assert_eq!(field.as_bytes(), b"ABCDEFGXYZ");
    }

    #[test]
    fn write_to_end_with_padding() {
        let mut field = PicX::new(10, b"ABCDEFGHIJ");
        ref_mod_write_to_end(b"X", &mut field, 8);
        assert_eq!(field.as_bytes(), b"ABCDEFGX  ");
    }

    #[test]
    #[should_panic(expected = "offset must be >= 1")]
    fn read_zero_offset_panics() {
        let field = PicX::new(5, b"HELLO");
        ref_mod_read(&field, 0, 1);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn read_exceeds_length_panics() {
        let field = PicX::new(5, b"HELLO");
        ref_mod_read(&field, 3, 5);
    }

    #[test]
    #[should_panic(expected = "exceeds field length")]
    fn read_to_end_offset_too_large_panics() {
        let field = PicX::new(5, b"HELLO");
        ref_mod_read_to_end(&field, 7);
    }

    #[test]
    #[should_panic(expected = "offset must be >= 1")]
    fn write_zero_offset_panics() {
        let mut field = PicX::new(5, b"HELLO");
        ref_mod_write(b"A", &mut field, 0, 1);
    }

    #[test]
    #[should_panic(expected = "write out of bounds")]
    fn write_exceeds_length_panics() {
        let mut field = PicX::new(5, b"HELLO");
        ref_mod_write(b"A", &mut field, 4, 5);
    }
}
