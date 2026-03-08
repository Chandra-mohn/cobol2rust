use std::ops::{Index, IndexMut};

use cobol_core::category::DataCategory;
use cobol_core::traits::CobolField;

/// COBOL array with 1-based indexing (OCCURS clause).
///
/// Wraps a `Vec<T>` and translates COBOL 1-based subscripts to 0-based.
/// Panics on out-of-bounds access (COBOL has no bounds-checking; we add it).
#[derive(Clone)]
pub struct CobolArray<T: CobolField> {
    elements: Vec<T>,
}

impl<T: CobolField> std::fmt::Debug for CobolArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CobolArray")
            .field("length", &self.elements.len())
            .field("elements", &self.elements)
            .finish()
    }
}

impl<T: CobolField> CobolArray<T> {
    /// Create from a vector of elements.
    pub fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Access by 1-based COBOL subscript.
    ///
    /// Panics if subscript is 0 or exceeds the array length.
    pub fn get(&self, subscript: usize) -> &T {
        assert!(subscript >= 1, "COBOL subscript must be >= 1, got {subscript}");
        &self.elements[subscript - 1]
    }

    /// Access by 1-based COBOL subscript (mutable).
    pub fn get_mut(&mut self, subscript: usize) -> &mut T {
        assert!(subscript >= 1, "COBOL subscript must be >= 1, got {subscript}");
        &mut self.elements[subscript - 1]
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Whether the array is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Iterate over elements.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    /// Iterate over elements (mutable).
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elements.iter_mut()
    }

}

impl<T: CobolField> Index<usize> for CobolArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.elements[index]
    }
}

impl<T: CobolField> IndexMut<usize> for CobolArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.elements[index]
    }
}

impl<T: CobolField + Clone> CobolArray<T> {
    /// Concatenate all element bytes into a contiguous vector.
    ///
    /// Use this for I/O serialization. The `as_bytes()` method on `CobolField`
    /// cannot return a contiguous slice for arrays, so this method provides
    /// the correct byte representation.
    pub fn to_bytes_vec(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.byte_length());
        for elem in &self.elements {
            result.extend_from_slice(elem.as_bytes());
        }
        result
    }
}

impl<T: CobolField + Clone> CobolField for CobolArray<T> {
    fn category(&self) -> DataCategory {
        DataCategory::Group
    }

    fn byte_length(&self) -> usize {
        self.elements.iter().map(CobolField::byte_length).sum()
    }

    fn as_bytes(&self) -> &[u8] {
        // Cannot return a contiguous slice of non-contiguous elements.
        // For GROUP I/O, callers should concatenate element bytes.
        &[]
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut []
    }

    fn display_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.byte_length());
        for elem in &self.elements {
            result.extend_from_slice(&elem.display_bytes());
        }
        result
    }

    fn fill_bytes(&mut self, byte: u8) {
        for elem in &mut self.elements {
            elem.fill_bytes(byte);
        }
    }

    fn initialize_default(&mut self) {
        for elem in &mut self.elements {
            elem.initialize_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PicX;

    #[test]
    fn one_based_indexing() {
        let arr = CobolArray::new(vec![
            PicX::new(5, b"ONE"),
            PicX::new(5, b"TWO"),
            PicX::new(5, b"THREE"),
        ]);
        assert_eq!(arr.get(1).as_bytes(), b"ONE  ");
        assert_eq!(arr.get(2).as_bytes(), b"TWO  ");
        assert_eq!(arr.get(3).as_bytes(), b"THREE");
    }

    #[test]
    #[should_panic(expected = "COBOL subscript must be >= 1")]
    fn zero_subscript_panics() {
        let arr = CobolArray::new(vec![PicX::new(5, b"TEST")]);
        let _ = arr.get(0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn out_of_bounds_panics() {
        let arr = CobolArray::new(vec![PicX::new(5, b"TEST")]);
        let _ = arr.get(2);
    }

    #[test]
    fn len_correct() {
        let arr = CobolArray::new(vec![
            PicX::new(3, b"A"),
            PicX::new(3, b"B"),
        ]);
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn byte_length_is_sum() {
        let arr = CobolArray::new(vec![
            PicX::new(5, b"A"),
            PicX::new(5, b"B"),
            PicX::new(5, b"C"),
        ]);
        assert_eq!(arr.byte_length(), 15);
    }

    #[test]
    fn display_bytes_concatenates() {
        let arr = CobolArray::new(vec![
            PicX::new(3, b"AB"),
            PicX::new(3, b"CD"),
        ]);
        assert_eq!(arr.display_bytes(), b"AB CD ");
    }

    #[test]
    fn get_mut_modifies() {
        let mut arr = CobolArray::new(vec![
            PicX::new(5, b"OLD"),
        ]);
        arr.get_mut(1).set(b"NEW");
        assert_eq!(arr.get(1).as_bytes(), b"NEW  ");
    }

    #[test]
    fn initialize_default_resets_all() {
        let mut arr = CobolArray::new(vec![
            PicX::new(3, b"ABC"),
            PicX::new(3, b"DEF"),
        ]);
        arr.initialize_default();
        assert_eq!(arr.get(1).as_bytes(), b"   ");
        assert_eq!(arr.get(2).as_bytes(), b"   ");
    }

    #[test]
    fn to_bytes_vec_concatenates() {
        let arr = CobolArray::new(vec![
            PicX::new(3, b"AB"),
            PicX::new(3, b"CD"),
        ]);
        assert_eq!(arr.to_bytes_vec(), b"AB CD ");
    }
}
