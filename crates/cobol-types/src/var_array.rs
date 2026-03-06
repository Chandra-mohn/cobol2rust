//! COBOL variable-length array (OCCURS DEPENDING ON).
//!
//! `CobolVarArray<T>` has a maximum capacity but an active length
//! controlled by a "depending on" counter. Only the first `active_len`
//! elements participate in MOVE, DISPLAY, and I/O operations.
//!
//! # COBOL Syntax
//!
//! ```text
//! 05 WS-TABLE OCCURS 1 TO 100 TIMES
//!     DEPENDING ON WS-COUNT.
//! ```

use cobol_core::category::DataCategory;
use cobol_core::traits::CobolField;

/// COBOL variable-length array with 1-based indexing.
///
/// Unlike `CobolArray` (fixed OCCURS), the active element count is
/// dynamic and determined by an external counter field. All elements
/// up to `max_occurs` are allocated, but only `active_len` elements
/// are "visible" to COBOL operations.
#[derive(Clone)]
pub struct CobolVarArray<T: CobolField> {
    elements: Vec<T>,
    max_occurs: usize,
    active_len: usize,
}

impl<T: CobolField> std::fmt::Debug for CobolVarArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CobolVarArray")
            .field("max_occurs", &self.max_occurs)
            .field("active_len", &self.active_len)
            .field("elements", &self.elements)
            .finish()
    }
}

impl<T: CobolField> CobolVarArray<T> {
    /// Create from a vector of elements with a maximum capacity.
    ///
    /// `active_len` defaults to `elements.len()` (all active).
    /// Panics if `elements.len() > max_occurs`.
    pub fn new(elements: Vec<T>, max_occurs: usize) -> Self {
        assert!(
            elements.len() <= max_occurs,
            "elements.len() ({}) exceeds max_occurs ({})",
            elements.len(),
            max_occurs,
        );
        let active_len = elements.len();
        Self {
            elements,
            max_occurs,
            active_len,
        }
    }

    /// Create with explicit active length.
    ///
    /// Panics if `active_len > elements.len()` or `elements.len() > max_occurs`.
    pub fn with_active_len(elements: Vec<T>, max_occurs: usize, active_len: usize) -> Self {
        assert!(
            elements.len() <= max_occurs,
            "elements.len() ({}) exceeds max_occurs ({})",
            elements.len(),
            max_occurs,
        );
        assert!(
            active_len <= elements.len(),
            "active_len ({}) exceeds elements.len() ({})",
            active_len,
            elements.len(),
        );
        Self {
            elements,
            max_occurs,
            active_len,
        }
    }

    /// Maximum number of elements (OCCURS n TIMES).
    pub fn max_occurs(&self) -> usize {
        self.max_occurs
    }

    /// Current active length (from DEPENDING ON field).
    pub fn active_len(&self) -> usize {
        self.active_len
    }

    /// Set the active length (called when DEPENDING ON field changes).
    ///
    /// Panics if `len > self.elements.len()`.
    pub fn set_active_len(&mut self, len: usize) {
        assert!(
            len <= self.elements.len(),
            "active_len ({}) exceeds allocated elements ({})",
            len,
            self.elements.len(),
        );
        self.active_len = len;
    }

    /// Total allocated elements (may be > `active_len`).
    pub fn allocated_len(&self) -> usize {
        self.elements.len()
    }

    /// Access by 1-based COBOL subscript.
    ///
    /// Panics if subscript is 0 or exceeds the active length.
    pub fn get(&self, subscript: usize) -> &T {
        assert!(
            subscript >= 1,
            "COBOL subscript must be >= 1, got {subscript}"
        );
        assert!(
            subscript <= self.active_len,
            "subscript ({subscript}) exceeds active length ({})",
            self.active_len
        );
        &self.elements[subscript - 1]
    }

    /// Access by 1-based COBOL subscript (mutable).
    pub fn get_mut(&mut self, subscript: usize) -> &mut T {
        assert!(
            subscript >= 1,
            "COBOL subscript must be >= 1, got {subscript}"
        );
        assert!(
            subscript <= self.active_len,
            "subscript ({subscript}) exceeds active length ({})",
            self.active_len
        );
        &mut self.elements[subscript - 1]
    }

    /// Iterate over active elements only.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements[..self.active_len].iter()
    }

    /// Iterate over active elements (mutable).
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elements[..self.active_len].iter_mut()
    }

    /// Whether the active portion is empty.
    pub fn is_empty(&self) -> bool {
        self.active_len == 0
    }
}

impl<T: CobolField + Clone> CobolVarArray<T> {
    /// Concatenate active element bytes into a contiguous vector.
    pub fn to_bytes_vec(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.byte_length());
        for elem in &self.elements[..self.active_len] {
            result.extend_from_slice(elem.as_bytes());
        }
        result
    }
}

impl<T: CobolField + Clone> CobolField for CobolVarArray<T> {
    fn category(&self) -> DataCategory {
        DataCategory::Group
    }

    fn byte_length(&self) -> usize {
        // Only active elements contribute to byte length
        self.elements[..self.active_len]
            .iter()
            .map(CobolField::byte_length)
            .sum()
    }

    fn as_bytes(&self) -> &[u8] {
        // Cannot return a contiguous slice of non-contiguous elements.
        // Callers should use to_bytes_vec() for I/O serialization.
        &[]
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut []
    }

    fn display_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.byte_length());
        for elem in &self.elements[..self.active_len] {
            result.extend_from_slice(&elem.display_bytes());
        }
        result
    }

    fn fill_bytes(&mut self, byte: u8) {
        for elem in &mut self.elements[..self.active_len] {
            elem.fill_bytes(byte);
        }
    }

    fn initialize_default(&mut self) {
        for elem in &mut self.elements[..self.active_len] {
            elem.initialize_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PicX;

    fn make_picx_array(count: usize, max: usize) -> CobolVarArray<PicX> {
        let elements: Vec<PicX> = (0..count)
            .map(|_| PicX::new(5, b"     "))
            .collect();
        CobolVarArray::new(elements, max)
    }

    #[test]
    fn test_new_defaults_active_to_len() {
        let arr = make_picx_array(3, 5);
        assert_eq!(arr.allocated_len(), 3);
        assert_eq!(arr.active_len(), 3);
        assert_eq!(arr.max_occurs(), 5);
    }

    #[test]
    fn test_with_active_len() {
        let elements: Vec<PicX> = (0..5)
            .map(|_| PicX::new(5, b"ABCDE"))
            .collect();
        let arr = CobolVarArray::with_active_len(elements, 10, 3);
        assert_eq!(arr.allocated_len(), 5);
        assert_eq!(arr.active_len(), 3);
        assert_eq!(arr.max_occurs(), 10);
    }

    #[test]
    fn test_get_within_active_range() {
        let elements: Vec<PicX> = vec![
            PicX::new(3, b"AAA"),
            PicX::new(3, b"BBB"),
            PicX::new(3, b"CCC"),
        ];
        let arr = CobolVarArray::new(elements, 5);
        assert_eq!(arr.get(1).as_bytes(), b"AAA");
        assert_eq!(arr.get(2).as_bytes(), b"BBB");
        assert_eq!(arr.get(3).as_bytes(), b"CCC");
    }

    #[test]
    #[should_panic(expected = "subscript (3) exceeds active length")]
    fn test_get_beyond_active_panics() {
        let elements: Vec<PicX> = vec![
            PicX::new(3, b"AAA"),
            PicX::new(3, b"BBB"),
            PicX::new(3, b"CCC"),
        ];
        let arr = CobolVarArray::with_active_len(elements, 5, 2);
        let _ = arr.get(3); // active_len is 2, subscript 3 is out of active range
    }

    #[test]
    #[should_panic(expected = "COBOL subscript must be >= 1")]
    fn test_get_zero_subscript_panics() {
        let arr = make_picx_array(3, 5);
        let _ = arr.get(0);
    }

    #[test]
    fn test_set_active_len() {
        let elements: Vec<PicX> = (0..5)
            .map(|_| PicX::new(3, b"XXX"))
            .collect();
        let mut arr = CobolVarArray::new(elements, 10);
        assert_eq!(arr.active_len(), 5);

        arr.set_active_len(2);
        assert_eq!(arr.active_len(), 2);

        // Can still allocate all 5 but only 2 are active
        assert_eq!(arr.allocated_len(), 5);
    }

    #[test]
    #[should_panic(expected = "active_len (6) exceeds allocated elements")]
    fn test_set_active_len_too_large_panics() {
        let mut arr = make_picx_array(3, 5);
        arr.set_active_len(6); // Only 3 allocated
    }

    #[test]
    fn test_byte_length_active_only() {
        let elements: Vec<PicX> = (0..5)
            .map(|_| PicX::new(4, b"TEST"))
            .collect();
        let mut arr = CobolVarArray::new(elements, 10);

        // All 5 active: 5 * 4 = 20
        assert_eq!(arr.byte_length(), 20);

        // Set active to 3: 3 * 4 = 12
        arr.set_active_len(3);
        assert_eq!(arr.byte_length(), 12);

        // Set active to 0: 0 bytes
        arr.set_active_len(0);
        assert_eq!(arr.byte_length(), 0);
    }

    #[test]
    fn test_to_bytes_vec_active_only() {
        let elements: Vec<PicX> = vec![
            PicX::new(2, b"AB"),
            PicX::new(2, b"CD"),
            PicX::new(2, b"EF"),
        ];
        let mut arr = CobolVarArray::new(elements, 5);

        assert_eq!(arr.to_bytes_vec(), b"ABCDEF");

        arr.set_active_len(2);
        assert_eq!(arr.to_bytes_vec(), b"ABCD");

        arr.set_active_len(0);
        assert!(arr.to_bytes_vec().is_empty());
    }

    #[test]
    fn test_display_bytes_active_only() {
        let elements: Vec<PicX> = vec![
            PicX::new(2, b"AB"),
            PicX::new(2, b"CD"),
            PicX::new(2, b"EF"),
        ];
        let mut arr = CobolVarArray::new(elements, 5);

        assert_eq!(arr.display_bytes(), b"ABCDEF");
        arr.set_active_len(1);
        assert_eq!(arr.display_bytes(), b"AB");
    }

    #[test]
    fn test_fill_bytes_active_only() {
        let elements: Vec<PicX> = vec![
            PicX::new(2, b"AB"),
            PicX::new(2, b"CD"),
            PicX::new(2, b"EF"),
        ];
        let mut arr = CobolVarArray::with_active_len(elements, 5, 2);

        arr.fill_bytes(b'*');
        // Only first 2 should be filled
        assert_eq!(arr.get(1).as_bytes(), b"**");
        assert_eq!(arr.get(2).as_bytes(), b"**");

        // Third element should be unchanged (not active)
        arr.set_active_len(3);
        assert_eq!(arr.get(3).as_bytes(), b"EF");
    }

    #[test]
    fn test_initialize_default_active_only() {
        let elements: Vec<PicX> = vec![
            PicX::new(3, b"ABC"),
            PicX::new(3, b"DEF"),
            PicX::new(3, b"GHI"),
        ];
        let mut arr = CobolVarArray::with_active_len(elements, 5, 2);

        arr.initialize_default();
        // PicX initializes to spaces
        assert_eq!(arr.get(1).as_bytes(), b"   ");
        assert_eq!(arr.get(2).as_bytes(), b"   ");

        // Third should be unchanged
        arr.set_active_len(3);
        assert_eq!(arr.get(3).as_bytes(), b"GHI");
    }

    #[test]
    fn test_iter_active_only() {
        let elements: Vec<PicX> = vec![
            PicX::new(1, b"A"),
            PicX::new(1, b"B"),
            PicX::new(1, b"C"),
        ];
        let arr = CobolVarArray::with_active_len(elements, 5, 2);

        let bytes: Vec<&[u8]> = arr.iter().map(cobol_core::CobolField::as_bytes).collect();
        assert_eq!(bytes, vec![b"A" as &[u8], b"B"]);
    }

    #[test]
    fn test_is_empty() {
        let elements: Vec<PicX> = vec![PicX::new(1, b"A")];
        let mut arr = CobolVarArray::new(elements, 5);

        assert!(!arr.is_empty());
        arr.set_active_len(0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_get_mut_modifies_element() {
        let elements: Vec<PicX> = vec![
            PicX::new(3, b"AAA"),
            PicX::new(3, b"BBB"),
        ];
        let mut arr = CobolVarArray::new(elements, 5);

        arr.get_mut(1).fill_bytes(b'X');
        assert_eq!(arr.get(1).as_bytes(), b"XXX");
        assert_eq!(arr.get(2).as_bytes(), b"BBB");
    }

    #[test]
    fn test_category_is_group() {
        let arr = make_picx_array(2, 5);
        assert_eq!(arr.category(), DataCategory::Group);
    }

    #[test]
    #[should_panic(expected = "elements.len() (6) exceeds max_occurs (5)")]
    fn test_new_exceeds_max_panics() {
        let elements: Vec<PicX> = (0..6)
            .map(|_| PicX::new(1, b"A"))
            .collect();
        let _ = CobolVarArray::new(elements, 5);
    }
}
