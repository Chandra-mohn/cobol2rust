//! COBOL REDEFINES support -- byte-level storage sharing.
//!
//! In COBOL, REDEFINES allows two or more data items to share the same
//! physical storage. For example:
//!
//! ```text
//! 05 WS-DATE        PIC X(8).
//! 05 WS-DATE-PARTS  REDEFINES WS-DATE.
//!    10 WS-YEAR     PIC 9(4).
//!    10 WS-MONTH    PIC 9(2).
//!    10 WS-DAY      PIC 9(2).
//! ```
//!
//! `RedefinesGroup` wraps a shared byte buffer. Both the original field
//! and the redefining field(s) operate on the same bytes. Typed views
//! are created on demand via accessor helpers.

use cobol_core::category::DataCategory;
use cobol_core::traits::CobolField;

/// Shared byte storage for REDEFINES.
///
/// Holds a flat byte buffer representing the physical storage. Both the
/// original data item and its REDEFINES overlays read/write to this buffer.
#[derive(Clone)]
pub struct RedefinesGroup {
    /// The shared byte storage.
    data: Vec<u8>,
}

impl std::fmt::Debug for RedefinesGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedefinesGroup")
            .field("len", &self.data.len())
            .field("data", &self.data)
            .finish()
    }
}

impl RedefinesGroup {
    /// Create a new shared storage of the given size, initialized to spaces.
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![b' '; size],
        }
    }

    /// Create from existing bytes.
    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    /// Get a byte slice view at the given offset and length.
    ///
    /// Panics if `offset + length > self.data.len()`.
    pub fn view(&self, offset: usize, length: usize) -> &[u8] {
        &self.data[offset..offset + length]
    }

    /// Get a mutable byte slice view at the given offset and length.
    pub fn view_mut(&mut self, offset: usize, length: usize) -> &mut [u8] {
        &mut self.data[offset..offset + length]
    }

    /// Copy bytes into the storage at a given offset.
    ///
    /// Source bytes are truncated or padded with `pad` to fill `length`.
    pub fn set_bytes(&mut self, offset: usize, length: usize, src: &[u8], pad: u8) {
        let target = &mut self.data[offset..offset + length];
        let copy_len = src.len().min(length);
        target[..copy_len].copy_from_slice(&src[..copy_len]);
        if copy_len < length {
            for byte in &mut target[copy_len..] {
                *byte = pad;
            }
        }
    }

    /// Total storage size in bytes.
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl CobolField for RedefinesGroup {
    fn category(&self) -> DataCategory {
        DataCategory::Alphanumeric
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

    fn initialize_default(&mut self) {
        self.data.fill(b' ');
    }
}

/// Helper to copy bytes from a typed field into a `RedefinesGroup` at an offset.
///
/// Used by generated code when writing to a REDEFINES overlay field.
pub fn sync_field_to_redefines(group: &mut RedefinesGroup, offset: usize, field: &dyn CobolField) {
    let bytes = field.as_bytes();
    let length = field.byte_length();
    let target = group.view_mut(offset, length);
    let copy_len = bytes.len().min(length);
    target[..copy_len].copy_from_slice(&bytes[..copy_len]);
}

/// Helper to copy bytes from a `RedefinesGroup` into a typed field.
///
/// Used by generated code when reading from a REDEFINES overlay field.
pub fn sync_redefines_to_field(
    group: &RedefinesGroup,
    offset: usize,
    field: &mut dyn CobolField,
) {
    let length = field.byte_length();
    let src = group.view(offset, length);
    let target = field.as_bytes_mut();
    let copy_len = src.len().min(target.len());
    target[..copy_len].copy_from_slice(&src[..copy_len]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PicX;

    #[test]
    fn test_new_initializes_to_spaces() {
        let rg = RedefinesGroup::new(8);
        assert_eq!(rg.as_bytes(), b"        ");
        assert_eq!(rg.size(), 8);
    }

    #[test]
    fn test_from_bytes() {
        let rg = RedefinesGroup::from_bytes(b"20241225");
        assert_eq!(rg.as_bytes(), b"20241225");
    }

    #[test]
    fn test_view_slicing() {
        let rg = RedefinesGroup::from_bytes(b"20241225");
        // View as: YYYY (0..4), MM (4..6), DD (6..8)
        assert_eq!(rg.view(0, 4), b"2024");
        assert_eq!(rg.view(4, 2), b"12");
        assert_eq!(rg.view(6, 2), b"25");
    }

    #[test]
    fn test_view_mut_modification() {
        let mut rg = RedefinesGroup::from_bytes(b"20241225");

        // Change the month
        rg.view_mut(4, 2).copy_from_slice(b"06");
        assert_eq!(rg.as_bytes(), b"20240625");

        // The full view reflects the change
        assert_eq!(rg.view(0, 8), b"20240625");
    }

    #[test]
    fn test_set_bytes_exact() {
        let mut rg = RedefinesGroup::new(8);
        rg.set_bytes(0, 4, b"2024", b' ');
        assert_eq!(rg.as_bytes(), b"2024    ");
    }

    #[test]
    fn test_set_bytes_with_padding() {
        let mut rg = RedefinesGroup::new(8);
        rg.set_bytes(0, 8, b"HI", b' ');
        assert_eq!(rg.as_bytes(), b"HI      ");
    }

    #[test]
    fn test_set_bytes_truncation() {
        let mut rg = RedefinesGroup::new(8);
        rg.set_bytes(0, 3, b"HELLO", b' ');
        assert_eq!(&rg.as_bytes()[..3], b"HEL");
    }

    #[test]
    fn test_fill_bytes() {
        let mut rg = RedefinesGroup::new(5);
        rg.fill_bytes(b'*');
        assert_eq!(rg.as_bytes(), b"*****");
    }

    #[test]
    fn test_initialize_default_spaces() {
        let mut rg = RedefinesGroup::from_bytes(b"HELLO");
        rg.initialize_default();
        assert_eq!(rg.as_bytes(), b"     ");
    }

    #[test]
    fn test_category_is_alphanumeric() {
        let rg = RedefinesGroup::new(4);
        assert_eq!(rg.category(), DataCategory::Alphanumeric);
    }

    #[test]
    fn test_sync_field_to_redefines() {
        let mut rg = RedefinesGroup::new(8);
        let field = PicX::new(4, b"2024");
        sync_field_to_redefines(&mut rg, 0, &field);
        assert_eq!(rg.view(0, 4), b"2024");
    }

    #[test]
    fn test_sync_redefines_to_field() {
        let rg = RedefinesGroup::from_bytes(b"20241225");
        let mut field = PicX::new(4, b"    ");
        sync_redefines_to_field(&rg, 0, &mut field);
        assert_eq!(field.as_bytes(), b"2024");
    }

    #[test]
    fn test_redefines_roundtrip() {
        // Simulates: WS-DATE PIC X(8) / WS-DATE-PARTS REDEFINES WS-DATE
        let mut rg = RedefinesGroup::new(8);

        // Write as the full date string
        rg.set_bytes(0, 8, b"20241225", b' ');

        // Read the year portion (offset 0, length 4)
        assert_eq!(rg.view(0, 4), b"2024");

        // Modify the day portion (offset 6, length 2)
        rg.view_mut(6, 2).copy_from_slice(b"31");

        // Verify the full date changed
        assert_eq!(rg.as_bytes(), b"20241231");
    }

    #[test]
    fn test_multiple_overlapping_views() {
        // Simulates multiple REDEFINES on same storage
        let mut rg = RedefinesGroup::new(10);

        // View 1: PIC X(10) -- full record
        rg.set_bytes(0, 10, b"ABCDEFGHIJ", b' ');

        // View 2: REDEFINES as PIC X(5) PIC X(5)
        assert_eq!(rg.view(0, 5), b"ABCDE");
        assert_eq!(rg.view(5, 5), b"FGHIJ");

        // Modify through view 2 affects view 1
        rg.view_mut(0, 5).copy_from_slice(b"12345");
        assert_eq!(rg.as_bytes(), b"12345FGHIJ");
    }
}
