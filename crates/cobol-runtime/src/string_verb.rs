//! COBOL STRING verb runtime implementation.
//!
//! The STRING verb concatenates multiple sources into a single target field,
//! optionally delimited by specified values, with pointer-based positioning.
//!
//! # COBOL Syntax
//!
//! ```text
//! STRING source-1 DELIMITED BY {SIZE | delimiter-1}
//!        source-2 DELIMITED BY {SIZE | delimiter-2}
//!        ...
//!        INTO target
//!        [WITH POINTER pointer-field]
//!        [ON OVERFLOW imperative-statement]
//!        [NOT ON OVERFLOW imperative-statement]
//! END-STRING
//! ```

/// How a STRING source is delimited.
#[derive(Debug, Clone)]
pub enum StringDelimiter {
    /// DELIMITED BY SIZE -- use the entire source.
    Size,
    /// DELIMITED BY literal -- use bytes before the first occurrence of the delimiter.
    Literal(Vec<u8>),
}

/// A single source in a STRING statement.
#[derive(Debug, Clone)]
pub struct StringSourceSpec {
    /// The source data bytes.
    pub data: Vec<u8>,
    /// How the source is delimited.
    pub delimiter: StringDelimiter,
}

impl StringSourceSpec {
    /// Create a source delimited by SIZE.
    #[must_use]
    pub fn by_size(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            delimiter: StringDelimiter::Size,
        }
    }

    /// Create a source delimited by a literal value.
    #[must_use]
    pub fn by_literal(data: &[u8], delimiter: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            delimiter: StringDelimiter::Literal(delimiter.to_vec()),
        }
    }
}

/// Result of a STRING operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringResult {
    /// Operation completed without overflow.
    Success,
    /// Pointer went beyond the target length (ON OVERFLOW).
    Overflow,
}

/// Execute the COBOL STRING verb.
///
/// Concatenates bytes from each source into `target`, starting at position
/// `pointer` (1-based, COBOL convention). For each source:
/// - DELIMITED BY SIZE: all source bytes are used
/// - DELIMITED BY literal: bytes before the first occurrence of the delimiter
///
/// The pointer is advanced after each source. If the pointer exceeds the
/// target length, the operation stops and returns `Overflow`.
///
/// # Arguments
///
/// * `sources` - The source specifications (data + delimiter)
/// * `target` - The target byte buffer (mutable, fixed-size)
/// * `pointer` - 1-based position pointer (advanced during operation)
///
/// # Returns
///
/// `StringResult::Success` if all sources fit, `StringResult::Overflow` if
/// the pointer exceeds the target length.
pub fn cobol_string(
    sources: &[StringSourceSpec],
    target: &mut [u8],
    pointer: &mut usize,
) -> StringResult {
    // COBOL pointer is 1-based
    // If pointer is already out of range, overflow immediately
    if *pointer < 1 || *pointer > target.len() {
        return StringResult::Overflow;
    }

    // Convert to 0-based index
    let mut pos = *pointer - 1;

    for source in sources {
        // Determine effective bytes from this source
        let effective = match &source.delimiter {
            StringDelimiter::Size => &source.data[..],
            StringDelimiter::Literal(delim) => {
                if delim.is_empty() {
                    &source.data[..]
                } else {
                    // Find first occurrence of delimiter in source data
                    match find_delimiter(&source.data, delim) {
                        Some(idx) => &source.data[..idx],
                        None => &source.data[..], // no delimiter found -> use all
                    }
                }
            }
        };

        // Copy effective bytes into target
        for &byte in effective {
            if pos >= target.len() {
                // Overflow: pointer exceeds target
                *pointer = pos + 1; // Update pointer (1-based)
                return StringResult::Overflow;
            }
            target[pos] = byte;
            pos += 1;
        }
    }

    // Update pointer (1-based, points to next available position)
    *pointer = pos + 1;
    StringResult::Success
}

/// Find the first occurrence of `delimiter` in `data`.
fn find_delimiter(data: &[u8], delimiter: &[u8]) -> Option<usize> {
    if delimiter.is_empty() || delimiter.len() > data.len() {
        return None;
    }
    for i in 0..=data.len() - delimiter.len() {
        if &data[i..i + delimiter.len()] == delimiter {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_by_size_single_source() {
        let mut target = [b' '; 20];
        let mut pointer = 1;

        let sources = vec![StringSourceSpec::by_size(b"HELLO")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[..5], b"HELLO");
        assert_eq!(pointer, 6); // next position (1-based)
    }

    #[test]
    fn test_string_by_size_multiple_sources() {
        let mut target = [b' '; 20];
        let mut pointer = 1;

        let sources = vec![
            StringSourceSpec::by_size(b"HELLO"),
            StringSourceSpec::by_size(b" "),
            StringSourceSpec::by_size(b"WORLD"),
        ];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[..11], b"HELLO WORLD");
        assert_eq!(pointer, 12);
    }

    #[test]
    fn test_string_delimited_by_literal() {
        let mut target = [b' '; 20];
        let mut pointer = 1;

        // Source "HELLO WORLD" delimited by " " -> uses "HELLO"
        let sources = vec![StringSourceSpec::by_literal(b"HELLO WORLD", b" ")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[..5], b"HELLO");
        assert_eq!(pointer, 6);
    }

    #[test]
    fn test_string_delimited_no_match() {
        let mut target = [b' '; 20];
        let mut pointer = 1;

        // Delimiter not found -> use entire source
        let sources = vec![StringSourceSpec::by_literal(b"HELLO", b"X")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[..5], b"HELLO");
        assert_eq!(pointer, 6);
    }

    #[test]
    fn test_string_with_pointer_offset() {
        let mut target = [b' '; 20];
        let mut pointer = 6; // Start at position 6

        let sources = vec![StringSourceSpec::by_size(b"WORLD")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[5..10], b"WORLD");
        assert_eq!(pointer, 11);
    }

    #[test]
    fn test_string_overflow() {
        let mut target = [b' '; 5];
        let mut pointer = 1;

        // Source is longer than target
        let sources = vec![StringSourceSpec::by_size(b"HELLO WORLD")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Overflow);
        assert_eq!(&target[..5], b"HELLO"); // Partial fill
    }

    #[test]
    fn test_string_overflow_pointer_out_of_range() {
        let mut target = [b' '; 10];
        let mut pointer = 15; // Already beyond target

        let sources = vec![StringSourceSpec::by_size(b"HELLO")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Overflow);
    }

    #[test]
    fn test_string_overflow_pointer_zero() {
        let mut target = [b' '; 10];
        let mut pointer = 0; // Invalid (COBOL is 1-based)

        let sources = vec![StringSourceSpec::by_size(b"HELLO")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Overflow);
    }

    #[test]
    fn test_string_mixed_delimiters() {
        let mut target = [b' '; 30];
        let mut pointer = 1;

        let sources = vec![
            StringSourceSpec::by_literal(b"JOHN DOE", b" "), // "JOHN"
            StringSourceSpec::by_size(b", "),                 // ", "
            StringSourceSpec::by_literal(b"123-456-7890", b"-"), // "123"
        ];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[..9], b"JOHN, 123");
        assert_eq!(pointer, 10);
    }

    #[test]
    fn test_string_empty_source() {
        let mut target = [b' '; 10];
        let mut pointer = 1;

        let sources = vec![
            StringSourceSpec::by_size(b""),
            StringSourceSpec::by_size(b"HI"),
        ];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[..2], b"HI");
        assert_eq!(pointer, 3);
    }

    #[test]
    fn test_string_exact_fit() {
        let mut target = [b' '; 5];
        let mut pointer = 1;

        let sources = vec![StringSourceSpec::by_size(b"EXACT")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target, b"EXACT");
        assert_eq!(pointer, 6); // One past end -- still success
    }

    #[test]
    fn test_string_multi_char_delimiter() {
        let mut target = [b' '; 20];
        let mut pointer = 1;

        let sources = vec![StringSourceSpec::by_literal(b"HELLO::WORLD", b"::")];
        let result = cobol_string(&sources, &mut target, &mut pointer);

        assert_eq!(result, StringResult::Success);
        assert_eq!(&target[..5], b"HELLO");
        assert_eq!(pointer, 6);
    }
}
