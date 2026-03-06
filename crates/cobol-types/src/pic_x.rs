use cobol_core::category::DataCategory;
use cobol_core::traits::CobolField;

/// PIC X(N): Fixed-length alphanumeric field.
///
/// COBOL string semantics:
/// - Left-justified by default (JUSTIFIED RIGHT overrides)
/// - Space-padded on right
/// - Right-truncated on assignment if source is longer
#[derive(Clone)]
pub struct PicX {
    data: Vec<u8>,
    justified_right: bool,
}

impl std::fmt::Debug for PicX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PicX")
            .field("value", &self.as_str())
            .field("length", &self.data.len())
            .finish()
    }
}

impl PicX {
    /// Create a new `PicX` of the given length, initialized from the provided bytes.
    ///
    /// The value is left-justified and space-padded on the right.
    /// If `initial` is longer than `length`, it is right-truncated.
    pub fn new(length: usize, initial: &[u8]) -> Self {
        let mut data = vec![b' '; length];
        let copy_len = initial.len().min(length);
        data[..copy_len].copy_from_slice(&initial[..copy_len]);
        Self {
            data,
            justified_right: false,
        }
    }

    /// Create an empty (all spaces) `PicX` of the given length.
    pub fn spaces(length: usize) -> Self {
        Self {
            data: vec![b' '; length],
            justified_right: false,
        }
    }

    /// Create a `PicX` with JUSTIFIED RIGHT clause.
    pub fn new_justified_right(length: usize, initial: &[u8]) -> Self {
        let mut data = vec![b' '; length];
        let copy_len = initial.len().min(length);
        // Right-justify: place content at the end
        let start = length - copy_len;
        data[start..].copy_from_slice(&initial[..copy_len]);
        Self {
            data,
            justified_right: true,
        }
    }

    /// Set the value from a byte slice.
    ///
    /// Left-justified (or right-justified if JUSTIFIED RIGHT), space-padded, truncated.
    pub fn set(&mut self, value: &[u8]) {
        let length = self.data.len();
        self.data.fill(b' ');
        let copy_len = value.len().min(length);

        if self.justified_right {
            // Right-justify: left-truncate if too long, left-pad with spaces
            if value.len() > length {
                // Left-truncation for JUSTIFIED RIGHT
                self.data
                    .copy_from_slice(&value[value.len() - length..]);
            } else {
                let start = length - copy_len;
                self.data[start..].copy_from_slice(&value[..copy_len]);
            }
        } else {
            // Left-justify: right-truncate if too long, right-pad with spaces
            self.data[..copy_len].copy_from_slice(&value[..copy_len]);
        }
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
            .map_or(0, |p| p + 1);
        &self.data[..end]
    }

    /// Get the field length.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the field is all spaces.
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&b| b == b' ')
    }
}

impl CobolField for PicX {
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

    fn is_justified_right(&self) -> bool {
        self.justified_right
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
        let f = PicX::new(10, b"HELLO");
        assert_eq!(f.as_bytes(), b"HELLO     ");
    }

    #[test]
    fn new_right_truncates() {
        let f = PicX::new(3, b"HELLO");
        assert_eq!(f.as_bytes(), b"HEL");
    }

    #[test]
    fn spaces_creates_all_spaces() {
        let f = PicX::spaces(5);
        assert_eq!(f.as_bytes(), b"     ");
        assert!(f.is_empty());
    }

    #[test]
    fn set_replaces_value() {
        let mut f = PicX::new(10, b"OLD VALUE");
        f.set(b"NEW");
        assert_eq!(f.as_bytes(), b"NEW       ");
    }

    #[test]
    fn set_truncates() {
        let mut f = PicX::new(3, b"");
        f.set(b"ABCDEFG");
        assert_eq!(f.as_bytes(), b"ABC");
    }

    #[test]
    fn trimmed_removes_trailing_spaces() {
        let f = PicX::new(10, b"HELLO");
        assert_eq!(f.trimmed(), b"HELLO");
    }

    #[test]
    fn trimmed_empty() {
        let f = PicX::spaces(5);
        assert_eq!(f.trimmed(), b"");
    }

    #[test]
    fn justified_right() {
        let f = PicX::new_justified_right(10, b"HI");
        assert_eq!(f.as_bytes(), b"        HI");
    }

    #[test]
    fn justified_right_set() {
        let mut f = PicX::new_justified_right(5, b"");
        f.set(b"AB");
        assert_eq!(f.as_bytes(), b"   AB");
    }

    #[test]
    fn justified_right_truncates_left() {
        let mut f = PicX::new_justified_right(3, b"");
        f.set(b"ABCDE");
        assert_eq!(f.as_bytes(), b"CDE");
    }

    #[test]
    fn category_is_alphanumeric() {
        let f = PicX::new(5, b"TEST");
        assert_eq!(f.category(), DataCategory::Alphanumeric);
    }

    #[test]
    fn fill_bytes_works() {
        let mut f = PicX::new(5, b"HELLO");
        f.fill_bytes(b'X');
        assert_eq!(f.as_bytes(), b"XXXXX");
    }

    #[test]
    fn initialize_default_fills_spaces() {
        let mut f = PicX::new(5, b"HELLO");
        f.initialize_default();
        assert_eq!(f.as_bytes(), b"     ");
    }

    #[test]
    fn as_str_works() {
        let f = PicX::new(5, b"HELLO");
        assert_eq!(f.as_str(), "HELLO");
    }

    #[test]
    fn byte_length_correct() {
        let f = PicX::new(10, b"");
        assert_eq!(f.byte_length(), 10);
    }

    #[test]
    fn trait_object() {
        let f = PicX::new(5, b"TEST");
        let field: &dyn CobolField = &f;
        assert_eq!(field.category(), DataCategory::Alphanumeric);
        assert_eq!(field.byte_length(), 5);
    }
}
