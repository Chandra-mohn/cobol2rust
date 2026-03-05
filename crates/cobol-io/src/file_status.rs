//! COBOL FILE STATUS codes.
//!
//! Every COBOL I/O operation sets a 2-byte file status code.
//! The first byte is the status class, the second provides detail.
//! Programs check these after every I/O verb (IF FILE-STATUS = "00").

use cobol_core::FileError;

/// A 2-byte COBOL FILE STATUS code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStatusCode {
    /// Status byte 1 (class).
    pub byte1: u8,
    /// Status byte 2 (detail).
    pub byte2: u8,
}

impl FileStatusCode {
    /// Create from two ASCII digit characters.
    pub const fn new(b1: u8, b2: u8) -> Self {
        Self { byte1: b1, byte2: b2 }
    }

    /// Successful completion.
    pub const SUCCESS: Self = Self::new(b'0', b'0');
    /// Successful, duplicate key exists (informational).
    pub const SUCCESS_DUPLICATE: Self = Self::new(b'0', b'2');
    /// Successful, record length mismatch.
    pub const SUCCESS_LENGTH: Self = Self::new(b'0', b'4');
    /// Successful, optional file not present.
    pub const SUCCESS_OPTIONAL: Self = Self::new(b'0', b'5');

    /// End of file (AT END).
    pub const AT_END: Self = Self::new(b'1', b'0');
    /// AT END on sequential read, out-of-key-range.
    pub const AT_END_KEY: Self = Self::new(b'1', b'4');

    /// Invalid key: sequence error.
    pub const SEQUENCE_ERROR: Self = Self::new(b'2', b'1');
    /// Invalid key: duplicate key.
    pub const DUPLICATE_KEY: Self = Self::new(b'2', b'2');
    /// Invalid key: record not found.
    pub const RECORD_NOT_FOUND: Self = Self::new(b'2', b'3');
    /// Invalid key: boundary violation.
    pub const KEY_BOUNDARY: Self = Self::new(b'2', b'4');

    /// Permanent error: no further info.
    pub const PERM_ERROR: Self = Self::new(b'3', b'0');
    /// Permanent error: inconsistent filename.
    pub const PERM_FILENAME: Self = Self::new(b'3', b'5');

    /// Logic error: file already open.
    pub const ALREADY_OPEN: Self = Self::new(b'4', b'1');
    /// Logic error: file not open.
    pub const NOT_OPEN: Self = Self::new(b'4', b'2');
    /// Logic error: no valid prior READ.
    pub const NO_PRIOR_READ: Self = Self::new(b'4', b'3');
    /// Logic error: boundary violation on write.
    pub const WRITE_BOUNDARY: Self = Self::new(b'4', b'4');
    /// Logic error: prior READ incomplete (already read).
    pub const READ_INCOMPLETE: Self = Self::new(b'4', b'6');
    /// Logic error: operation incompatible with open mode.
    pub const BAD_OPEN_MODE: Self = Self::new(b'4', b'7');

    /// Returns true if status indicates success (class 0).
    pub fn is_success(self) -> bool {
        self.byte1 == b'0'
    }

    /// Returns the 2-byte display string (e.g., "00", "10", "23").
    pub fn as_str(&self) -> [u8; 2] {
        [self.byte1, self.byte2]
    }

    /// Convert a FileError to the appropriate status code.
    pub fn from_error(err: &FileError) -> Self {
        match err {
            FileError::EndOfFile => Self::AT_END,
            FileError::NotFound(_) => Self::RECORD_NOT_FOUND,
            FileError::AlreadyOpen(_) => Self::ALREADY_OPEN,
            FileError::NotOpen(_) => Self::NOT_OPEN,
            FileError::DuplicateKey => Self::DUPLICATE_KEY,
            FileError::RecordNotFound => Self::RECORD_NOT_FOUND,
            FileError::SequenceError => Self::SEQUENCE_ERROR,
            FileError::Io(_) => Self::PERM_ERROR,
        }
    }
}

impl std::fmt::Display for FileStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.byte1 as char, self.byte2 as char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_code() {
        assert!(FileStatusCode::SUCCESS.is_success());
        assert_eq!(FileStatusCode::SUCCESS.as_str(), [b'0', b'0']);
    }

    #[test]
    fn at_end_not_success() {
        assert!(!FileStatusCode::AT_END.is_success());
        assert_eq!(FileStatusCode::AT_END.as_str(), [b'1', b'0']);
    }

    #[test]
    fn from_error_eof() {
        let code = FileStatusCode::from_error(&FileError::EndOfFile);
        assert_eq!(code, FileStatusCode::AT_END);
    }

    #[test]
    fn from_error_duplicate() {
        let code = FileStatusCode::from_error(&FileError::DuplicateKey);
        assert_eq!(code, FileStatusCode::DUPLICATE_KEY);
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", FileStatusCode::SUCCESS), "00");
        assert_eq!(format!("{}", FileStatusCode::RECORD_NOT_FOUND), "23");
    }
}
