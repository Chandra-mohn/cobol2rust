use cobol_core::category::DataCategory;
use cobol_core::traits::CobolField;

/// PIC A(N): Fixed-length alphabetic field.
///
/// Same storage semantics as PicX but category is Alphabetic.
/// Only allows letters and spaces (enforcement is at transpiler/MOVE level).
#[derive(Clone)]
pub struct PicA {
    data: Vec<u8>,
}

impl std::fmt::Debug for PicA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PicA")
            .field("value", &self.as_str())
            .field("length", &self.data.len())
            .finish()
    }
}

impl PicA {
    /// Create a new `PicA` of the given length, initialized from the provided bytes.
    pub fn new(length: usize, initial: &[u8]) -> Self {
        let mut data = vec![b' '; length];
        let copy_len = initial.len().min(length);
        data[..copy_len].copy_from_slice(&initial[..copy_len]);
        Self { data }
    }

    /// Create an empty (all spaces) `PicA` of the given length.
    pub fn spaces(length: usize) -> Self {
        Self {
            data: vec![b' '; length],
        }
    }

    /// Set the value from a byte slice (left-justified, space-padded, right-truncated).
    pub fn set(&mut self, value: &[u8]) {
        let length = self.data.len();
        self.data.fill(b' ');
        let copy_len = value.len().min(length);
        self.data[..copy_len].copy_from_slice(&value[..copy_len]);
    }

    /// Get the value as a UTF-8 string (best-effort).
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.data).unwrap_or("<non-utf8>")
    }

    /// Get the value with trailing spaces removed.
    pub fn trimmed(&self) -> &[u8] {
        let end = self
            .data
            .iter()
            .rposition(|&b| b != b' ')
            .map(|p| p + 1)
            .unwrap_or(0);
        &self.data[..end]
    }
}

impl CobolField for PicA {
    fn category(&self) -> DataCategory {
        DataCategory::Alphabetic
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_space_pads() {
        let f = PicA::new(10, b"HELLO");
        assert_eq!(f.as_bytes(), b"HELLO     ");
    }

    #[test]
    fn new_right_truncates() {
        let f = PicA::new(3, b"HELLO");
        assert_eq!(f.as_bytes(), b"HEL");
    }

    #[test]
    fn category_is_alphabetic() {
        let f = PicA::new(5, b"TEST");
        assert_eq!(f.category(), DataCategory::Alphabetic);
    }

    #[test]
    fn set_replaces_value() {
        let mut f = PicA::new(10, b"OLD");
        f.set(b"NEW");
        assert_eq!(f.as_bytes(), b"NEW       ");
    }

    #[test]
    fn trimmed_works() {
        let f = PicA::new(10, b"HELLO");
        assert_eq!(f.trimmed(), b"HELLO");
    }

    #[test]
    fn initialize_fills_spaces() {
        let mut f = PicA::new(5, b"HELLO");
        f.initialize_default();
        assert_eq!(f.as_bytes(), b"     ");
    }
}
