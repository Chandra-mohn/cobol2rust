//! COBOL UNSTRING verb runtime implementation.
//!
//! The UNSTRING verb splits a source string into multiple target fields based
//! on one or more delimiters, with optional pointer, tallying, and delimiter
//! capture.
//!
//! # COBOL Syntax
//!
//! ```text
//! UNSTRING source
//!     DELIMITED BY [ALL] delim-1 [OR [ALL] delim-2] ...
//!     INTO target-1 [DELIMITER IN delim-save-1] [COUNT IN count-1]
//!          target-2 [DELIMITER IN delim-save-2] [COUNT IN count-2]
//!          ...
//!     [WITH POINTER pointer-field]
//!     [TALLYING IN tally-field]
//!     [ON OVERFLOW imperative-statement]
//!     [NOT ON OVERFLOW imperative-statement]
//! END-UNSTRING
//! ```

/// Delimiter specification for UNSTRING.
#[derive(Debug, Clone)]
pub struct UnstringDelimSpec {
    /// The delimiter pattern bytes.
    pub pattern: Vec<u8>,
    /// Whether ALL consecutive occurrences are treated as one delimiter.
    pub all: bool,
}

impl UnstringDelimSpec {
    /// Create a delimiter spec.
    #[must_use]
    pub fn new(pattern: &[u8], all: bool) -> Self {
        Self {
            pattern: pattern.to_vec(),
            all,
        }
    }

    /// Create a simple (non-ALL) delimiter.
    #[must_use]
    pub fn simple(pattern: &[u8]) -> Self {
        Self::new(pattern, false)
    }

    /// Create an ALL delimiter.
    #[must_use]
    pub fn all(pattern: &[u8]) -> Self {
        Self::new(pattern, true)
    }
}

/// Result of an UNSTRING operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnstringResult {
    /// Operation completed without overflow.
    Success,
    /// Overflow: either source exhausted before all targets filled,
    /// or pointer was out of range, or more data than targets available.
    Overflow,
}

/// Execute the COBOL UNSTRING verb.
///
/// Scans `source` starting at `pointer` (1-based), looking for delimiters.
/// For each segment found between delimiters:
/// - Copies the segment bytes into the next `into` target (space-padded right)
/// - If `delimiter_into` is provided, stores the matched delimiter
/// - If `count_into` is provided, stores the byte count transferred
///
/// When ALL is specified for a delimiter, consecutive occurrences are consumed
/// as a single delimiter. Multiple delimiter patterns (OR) are checked in
/// order at each position.
///
/// # Arguments
///
/// * `source` - The source bytes to split
/// * `delimiters` - Delimiter specifications (checked in order at each position)
/// * `into` - Target buffers to fill (mutable slices, space-padded)
/// * `delimiter_into` - Optional delimiter capture buffers (one per target)
/// * `count_into` - Optional byte count captures (one per target)
/// * `pointer` - 1-based position pointer (advanced during operation)
/// * `tallying` - Optional counter of targets filled
///
/// # Returns
///
/// `UnstringResult::Success` or `UnstringResult::Overflow`.
#[allow(clippy::too_many_arguments)]
pub fn cobol_unstring(
    source: &[u8],
    delimiters: &[UnstringDelimSpec],
    into: &mut [&mut [u8]],
    delimiter_into: &mut [Option<&mut Vec<u8>>],
    count_into: &mut [Option<&mut usize>],
    pointer: &mut usize,
    tallying: Option<&mut usize>,
) -> UnstringResult {
    // Validate pointer (1-based)
    if *pointer < 1 || *pointer > source.len() {
        return UnstringResult::Overflow;
    }

    let mut pos = *pointer - 1; // Convert to 0-based
    let mut target_idx = 0;
    let mut overflow = false;

    while pos < source.len() && target_idx < into.len() {
        // Find the next delimiter from current position
        let (segment_end, delim_len, matched_delim) =
            find_next_delimiter(source, pos, delimiters);

        // Extract segment
        let segment = &source[pos..segment_end];

        // Copy segment to current target (space-padded right)
        let target = &mut into[target_idx];
        let copy_len = segment.len().min(target.len());
        target[..copy_len].copy_from_slice(&segment[..copy_len]);
        // Space-pad the rest
        for byte in &mut target[copy_len..] {
            *byte = b' ';
        }

        // Store delimiter if capture buffer provided
        if target_idx < delimiter_into.len() {
            if let Some(ref mut delim_buf) = delimiter_into[target_idx] {
                delim_buf.clear();
                if let Some(ref d) = matched_delim {
                    delim_buf.extend_from_slice(d);
                }
            }
        }

        // Store count if capture buffer provided
        if target_idx < count_into.len() {
            if let Some(ref mut count) = count_into[target_idx] {
                **count = segment.len();
            }
        }

        target_idx += 1;

        if delim_len > 0 {
            // Skip past the delimiter (and consume ALL consecutive if needed)
            pos = segment_end + delim_len;

            // Handle ALL: consume consecutive occurrences of the same delimiter
            if let Some(ref d) = matched_delim {
                // Find which delimiter spec matched
                for spec in delimiters {
                    if spec.pattern == *d && spec.all {
                        // Consume consecutive occurrences
                        while pos + spec.pattern.len() <= source.len()
                            && &source[pos..pos + spec.pattern.len()] == d.as_slice()
                        {
                            pos += spec.pattern.len();
                        }
                        break;
                    }
                }
            }
        } else {
            // No delimiter found -- rest of source consumed
            pos = source.len();
        }
    }

    // Check overflow conditions:
    // 1. More data remaining but no more targets
    if pos < source.len() && target_idx >= into.len() {
        overflow = true;
    }

    // Update pointer (1-based)
    *pointer = pos + 1;

    // Update tallying
    if let Some(tally) = tallying {
        *tally += target_idx;
    }

    if overflow {
        UnstringResult::Overflow
    } else {
        UnstringResult::Success
    }
}

/// Find the next delimiter in `source` starting at `start_pos`.
///
/// Returns `(segment_end, delimiter_length, matched_delimiter_bytes)`.
/// If no delimiter found, returns `(source.len(), 0, None)`.
fn find_next_delimiter(
    source: &[u8],
    start_pos: usize,
    delimiters: &[UnstringDelimSpec],
) -> (usize, usize, Option<Vec<u8>>) {
    let mut pos = start_pos;
    while pos < source.len() {
        // Check each delimiter pattern (in order -- first match wins)
        for spec in delimiters {
            if spec.pattern.is_empty() {
                continue;
            }
            if pos + spec.pattern.len() <= source.len()
                && &source[pos..pos + spec.pattern.len()] == spec.pattern.as_slice()
            {
                return (pos, spec.pattern.len(), Some(spec.pattern.clone()));
            }
        }
        pos += 1;
    }
    (source.len(), 0, None)
}

/// Simplified UNSTRING for common case: no DELIMITER IN, no COUNT IN.
///
/// # Returns
///
/// `UnstringResult::Success` or `UnstringResult::Overflow`.
pub fn cobol_unstring_simple(
    source: &[u8],
    delimiters: &[UnstringDelimSpec],
    into: &mut [&mut [u8]],
    pointer: &mut usize,
    tallying: Option<&mut usize>,
) -> UnstringResult {
    let mut delimiter_into: Vec<Option<&mut Vec<u8>>> = Vec::new();
    let mut count_into: Vec<Option<&mut usize>> = Vec::new();

    // Pad to match target count
    for _ in 0..into.len() {
        delimiter_into.push(None);
        count_into.push(None);
    }

    cobol_unstring(
        source,
        delimiters,
        into,
        &mut delimiter_into,
        &mut count_into,
        pointer,
        tallying,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unstring_single_delimiter() {
        let source = b"HELLO WORLD";
        let delims = vec![UnstringDelimSpec::simple(b" ")];
        let mut t1 = [0u8; 10];
        let mut t2 = [0u8; 10];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(&t1[..5], b"HELLO");
        assert_eq!(t1[5], b' '); // space-padded
        assert_eq!(&t2[..5], b"WORLD");
    }

    #[test]
    fn test_unstring_multiple_delimiters_or() {
        let source = b"A,B;C";
        let delims = vec![
            UnstringDelimSpec::simple(b","),
            UnstringDelimSpec::simple(b";"),
        ];
        let mut t1 = [0u8; 5];
        let mut t2 = [0u8; 5];
        let mut t3 = [0u8; 5];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2, &mut t3];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(t1[0], b'A');
        assert_eq!(t2[0], b'B');
        assert_eq!(t3[0], b'C');
    }

    #[test]
    fn test_unstring_all_delimiter() {
        let source = b"HELLO   WORLD";
        let delims = vec![UnstringDelimSpec::all(b" ")];
        let mut t1 = [0u8; 10];
        let mut t2 = [0u8; 10];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(&t1[..5], b"HELLO");
        assert_eq!(&t2[..5], b"WORLD");
    }

    #[test]
    fn test_unstring_with_pointer() {
        let source = b"SKIP:HELLO:WORLD";
        let delims = vec![UnstringDelimSpec::simple(b":")];
        let mut t1 = [0u8; 10];
        let mut t2 = [0u8; 10];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2];
        let mut pointer = 6; // Start after "SKIP:"

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(&t1[..5], b"HELLO");
        assert_eq!(&t2[..5], b"WORLD");
    }

    #[test]
    fn test_unstring_with_tallying() {
        let source = b"A,B,C";
        let delims = vec![UnstringDelimSpec::simple(b",")];
        let mut t1 = [0u8; 5];
        let mut t2 = [0u8; 5];
        let mut t3 = [0u8; 5];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2, &mut t3];
        let mut pointer = 1;
        let mut tally = 0usize;

        let result =
            cobol_unstring_simple(source, &delims, &mut into, &mut pointer, Some(&mut tally));

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(tally, 3);
    }

    #[test]
    fn test_unstring_delimiter_in_and_count_in() {
        let source = b"HELLO,WORLD";
        let delims = vec![UnstringDelimSpec::simple(b",")];
        let mut t1 = [0u8; 10];
        let mut t2 = [0u8; 10];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2];
        let mut d1 = Vec::new();
        let mut d2 = Vec::new();
        let mut c1 = 0usize;
        let mut c2 = 0usize;
        let mut delimiter_into: Vec<Option<&mut Vec<u8>>> =
            vec![Some(&mut d1), Some(&mut d2)];
        let mut count_into: Vec<Option<&mut usize>> = vec![Some(&mut c1), Some(&mut c2)];
        let mut pointer = 1;

        let result = cobol_unstring(
            source,
            &delims,
            &mut into,
            &mut delimiter_into,
            &mut count_into,
            &mut pointer,
            None,
        );

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(&t1[..5], b"HELLO");
        assert_eq!(&t2[..5], b"WORLD");
        assert_eq!(d1, b",");
        assert!(d2.is_empty()); // Last segment has no trailing delimiter
        assert_eq!(c1, 5);
        assert_eq!(c2, 5);
    }

    #[test]
    fn test_unstring_overflow_more_data_than_targets() {
        let source = b"A,B,C,D";
        let delims = vec![UnstringDelimSpec::simple(b",")];
        let mut t1 = [0u8; 5];
        let mut t2 = [0u8; 5];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Overflow);
        assert_eq!(t1[0], b'A');
        assert_eq!(t2[0], b'B');
    }

    #[test]
    fn test_unstring_overflow_pointer_out_of_range() {
        let source = b"HELLO";
        let delims = vec![UnstringDelimSpec::simple(b",")];
        let mut t1 = [0u8; 5];
        let mut into: Vec<&mut [u8]> = vec![&mut t1];
        let mut pointer = 10; // Beyond source

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Overflow);
    }

    #[test]
    fn test_unstring_no_delimiter_found() {
        let source = b"HELLO WORLD";
        let delims = vec![UnstringDelimSpec::simple(b",")]; // comma not in source
        let mut t1 = [0u8; 15];
        let mut t2 = [0u8; 10];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(&t1[..11], b"HELLO WORLD");
        // t2 should not be touched -- only 1 segment extracted
    }

    #[test]
    fn test_unstring_empty_segments() {
        // Consecutive delimiters produce empty segments (when not ALL)
        let source = b"A,,B";
        let delims = vec![UnstringDelimSpec::simple(b",")];
        let mut t1 = [0u8; 5];
        let mut t2 = [0u8; 5];
        let mut t3 = [0u8; 5];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2, &mut t3];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(t1[0], b'A');
        assert_eq!(t2[0], b' '); // empty segment -> space-padded
        assert_eq!(t3[0], b'B');
    }

    #[test]
    fn test_unstring_multi_char_delimiter() {
        let source = b"HELLO::WORLD::END";
        let delims = vec![UnstringDelimSpec::simple(b"::")];
        let mut t1 = [0u8; 10];
        let mut t2 = [0u8; 10];
        let mut t3 = [0u8; 10];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2, &mut t3];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(&t1[..5], b"HELLO");
        assert_eq!(&t2[..5], b"WORLD");
        assert_eq!(&t3[..3], b"END");
    }

    #[test]
    fn test_unstring_pointer_zero_overflow() {
        let source = b"HELLO";
        let delims = vec![UnstringDelimSpec::simple(b",")];
        let mut t1 = [0u8; 5];
        let mut into: Vec<&mut [u8]> = vec![&mut t1];
        let mut pointer = 0;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Overflow);
    }

    #[test]
    fn test_unstring_all_with_mixed_delimiters() {
        // ALL spaces, single comma
        let source = b"A   B,C";
        let delims = vec![
            UnstringDelimSpec::all(b" "),
            UnstringDelimSpec::simple(b","),
        ];
        let mut t1 = [0u8; 5];
        let mut t2 = [0u8; 5];
        let mut t3 = [0u8; 5];
        let mut into: Vec<&mut [u8]> = vec![&mut t1, &mut t2, &mut t3];
        let mut pointer = 1;

        let result = cobol_unstring_simple(source, &delims, &mut into, &mut pointer, None);

        assert_eq!(result, UnstringResult::Success);
        assert_eq!(t1[0], b'A');
        assert_eq!(t2[0], b'B');
        assert_eq!(t3[0], b'C');
    }
}
