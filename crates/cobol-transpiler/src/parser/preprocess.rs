//! COBOL source preprocessor.
//!
//! Handles fixed-format (columns 1-72) and free-format source.
//! Strips sequence numbers, indicator areas, and comments.
//! Handles continuation lines (indicator `-`).

use crate::error::{Result, TranspileError};

/// Source format detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFormat {
    /// Fixed format: cols 1-6 = sequence, col 7 = indicator, cols 8-72 = code.
    Fixed,
    /// Free format: no column restrictions.
    Free,
}

/// Preprocess fixed-format COBOL source into free-form text suitable for parsing.
///
/// - Strips columns 1-6 (sequence number area).
/// - Interprets column 7 (indicator area):
///   - `*` or `/` = comment line (removed entirely).
///   - `-` = continuation line (appended to previous, leading spaces stripped).
///   - `D` or `d` = debugging line (removed in production mode).
///   - ` ` = normal code line.
/// - Strips columns 73+ (identification area).
/// - Preserves Area A/B indentation (columns 8-72 become the code).
pub fn preprocess_fixed_format(source: &str) -> Result<String> {
    let mut output_lines: Vec<String> = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        let line_1based = line_num + 1;

        // Empty or very short lines pass through as blank
        if line.len() < 7 {
            // If line is too short to have an indicator, treat as blank
            output_lines.push(String::new());
            continue;
        }

        // Get indicator character (column 7, 0-indexed position 6)
        let indicator = line.as_bytes()[6] as char;

        // Extract code area: columns 8-72 (0-indexed 7..72)
        let code_end = line.len().min(72);
        let code_area = &line[7..code_end];

        match indicator {
            // Comment lines and debugging lines -- skip entirely
            '*' | '/' | 'D' | 'd' => {
                output_lines.push(String::new());
            }
            // Continuation line -- append to previous non-empty line
            '-' => {
                let continued_text = code_area.trim_start();
                if continued_text.is_empty() {
                    continue;
                }

                // Find the last non-empty output line and append
                if let Some(last) = output_lines.iter_mut().rev().find(|l| !l.is_empty()) {
                    // If previous line ended with an open quote, strip the
                    // opening quote from the continuation
                    let trimmed = strip_continuation_quote(last, continued_text);
                    last.push_str(trimmed);
                } else {
                    return Err(TranspileError::Preprocess {
                        line: line_1based,
                        message: "continuation line with no preceding line".to_string(),
                    });
                }
            }
            // Normal code line (space or any other indicator)
            _ => {
                output_lines.push(code_area.to_string());
            }
        }
    }

    Ok(output_lines.join("\n"))
}

/// Preprocess free-format COBOL source.
///
/// Free-format has no column restrictions. Comments start with `*>`.
/// Lines starting with `>>` are compiler directives.
pub fn preprocess_free_format(source: &str) -> String {
    let mut output_lines: Vec<String> = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim_start();

        // Full-line comment: starts with *>
        if trimmed.starts_with("*>") {
            output_lines.push(String::new());
            continue;
        }

        // Compiler directive: starts with >>
        if trimmed.starts_with(">>") {
            output_lines.push(String::new());
            continue;
        }

        // Inline comment: strip everything after *>
        if let Some(pos) = line.find("*>") {
            output_lines.push(line[..pos].to_string());
        } else {
            output_lines.push(line.to_string());
        }
    }

    output_lines.join("\n")
}

/// Detect source format from content heuristics.
///
/// If most lines have sequence numbers in columns 1-6 (all digits/spaces)
/// and column 7 is a valid indicator, treat as fixed format.
pub fn detect_source_format(source: &str) -> SourceFormat {
    let mut fixed_score = 0;
    let mut total_checked = 0;

    for line in source.lines().take(50) {
        if line.len() < 7 {
            continue;
        }

        total_checked += 1;

        let seq_area = &line[..6];
        let indicator = line.as_bytes()[6] as char;

        // Check if columns 1-6 look like sequence numbers (digits/spaces)
        let seq_valid = seq_area
            .bytes()
            .all(|b| b.is_ascii_digit() || b == b' ');

        // Check if column 7 is a valid indicator
        let ind_valid = matches!(indicator, ' ' | '*' | '/' | '-' | 'D' | 'd');

        if seq_valid && ind_valid {
            fixed_score += 1;
        }
    }

    if total_checked == 0 {
        return SourceFormat::Free;
    }

    // If >70% of lines look fixed-format, assume fixed
    if fixed_score * 100 / total_checked > 70 {
        SourceFormat::Fixed
    } else {
        SourceFormat::Free
    }
}

/// Auto-detect format and preprocess accordingly.
pub fn preprocess(source: &str) -> Result<String> {
    match detect_source_format(source) {
        SourceFormat::Fixed => preprocess_fixed_format(source),
        SourceFormat::Free => Ok(preprocess_free_format(source)),
    }
}

/// Handle continuation of quoted strings.
///
/// In COBOL, when a string literal is continued on the next line,
/// the continuation starts with a hyphen in column 7 and the
/// continued text starts with the same quote character.
/// We strip the leading quote from the continuation to join properly.
fn strip_continuation_quote<'a>(prev_line: &str, continuation: &'a str) -> &'a str {
    // Check if previous line has an unclosed quote
    let single_count = prev_line.chars().filter(|&c| c == '\'').count();
    let double_count = prev_line.chars().filter(|&c| c == '"').count();

    let in_single_quote = single_count % 2 != 0;
    let in_double_quote = double_count % 2 != 0;

    if (in_single_quote && continuation.starts_with('\''))
        || (in_double_quote && continuation.starts_with('"'))
    {
        &continuation[1..]
    } else {
        continuation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_format_basic() {
        // Columns: 123456 7 890...
        let source = "\
000100 IDENTIFICATION DIVISION.                                         \n\
000200 PROGRAM-ID. TEST1.                                               \n\
000300*THIS IS A COMMENT                                                \n\
000400 DATA DIVISION.                                                   ";

        let result = preprocess_fixed_format(source).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines[0].contains("IDENTIFICATION DIVISION."));
        assert!(lines[1].contains("PROGRAM-ID. TEST1."));
        // Comment line should be blank
        assert!(lines[2].is_empty());
        assert!(lines[3].contains("DATA DIVISION."));
    }

    #[test]
    fn fixed_format_continuation() {
        let source = "\
000100       DISPLAY 'HELLO                                             \n\
000200-             'WORLD'.                                            ";

        let result = preprocess_fixed_format(source).unwrap();
        // Continuation should join the strings
        assert!(result.contains("HELLO"));
        assert!(result.contains("WORLD"));
    }

    #[test]
    fn free_format_comments() {
        let source = "\
*> This is a comment\n\
IDENTIFICATION DIVISION.\n\
PROGRAM-ID. TEST1. *> inline comment";

        let result = preprocess_free_format(source);
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines[0].is_empty());
        assert_eq!(lines[1], "IDENTIFICATION DIVISION.");
        assert_eq!(lines[2].trim(), "PROGRAM-ID. TEST1.");
    }

    #[test]
    fn detect_fixed_format() {
        let source = "\
000100 IDENTIFICATION DIVISION.\n\
000200 PROGRAM-ID. TEST1.\n\
000300 DATA DIVISION.\n\
000400 WORKING-STORAGE SECTION.";

        assert_eq!(detect_source_format(source), SourceFormat::Fixed);
    }

    #[test]
    fn detect_free_format() {
        let source = "\
IDENTIFICATION DIVISION.\n\
PROGRAM-ID. TEST1.\n\
DATA DIVISION.\n\
WORKING-STORAGE SECTION.";

        assert_eq!(detect_source_format(source), SourceFormat::Free);
    }

    #[test]
    fn debug_lines_stripped() {
        let source = "\
000100 IDENTIFICATION DIVISION.                                         \n\
000200D    DISPLAY 'DEBUG INFO'.                                        \n\
000300 PROGRAM-ID. TEST1.                                               ";

        let result = preprocess_fixed_format(source).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines[0].contains("IDENTIFICATION DIVISION."));
        assert!(lines[1].is_empty()); // debug line stripped
        assert!(lines[2].contains("PROGRAM-ID. TEST1."));
    }

    #[test]
    fn short_lines_handled() {
        let source = "\n\n      \n";
        let result = preprocess_fixed_format(source).unwrap();
        // Should not panic, just produce blank lines
        assert!(!result.is_empty() || result.is_empty()); // always passes, testing no panic
    }
}
