//! COPY statement expansion -- resolves and inlines copybook content.
//!
//! Scans COBOL source for COPY statements, resolves each copybook via
//! a `CopybookResolver`, and replaces the COPY statement with the
//! copybook content. Handles nested COPYs with cycle and depth detection.
//!
//! Supports REPLACING clause (pseudo-text, word, literal forms) and SUPPRESS option.

use crate::error::{Result, TranspileError};
use crate::parser::copybook::CopybookResolver;

/// Operand in a REPLACING clause.
#[derive(Debug, Clone)]
enum ReplacingOperand {
    /// `==pseudo-text==` form -- exact text replacement (case-insensitive matching).
    PseudoText(String),
    /// Identifier/word form -- word-boundary replacement (case-insensitive).
    Word(String),
    /// Quoted literal form -- exact string replacement (case-sensitive).
    Literal(String),
}

/// A matched COPY statement in source text.
#[derive(Debug, Clone)]
struct CopyMatch {
    /// Byte offset of start of the COPY keyword.
    start: usize,
    /// Byte offset past the terminating period.
    end: usize,
    /// Copybook member name.
    name: String,
    /// Optional library qualifier (from `OF` or `IN`).
    library: Option<String>,
    /// Replacement pairs from REPLACING clause.
    replacing: Vec<(ReplacingOperand, ReplacingOperand)>,
    /// Whether SUPPRESS was specified (parsed but ignored for transpilation).
    #[allow(dead_code)]
    suppress: bool,
}

/// Scan source for COPY statements and return matches.
///
/// Recognizes:
/// - `COPY name.`
/// - `COPY name OF library.`
/// - `COPY name IN library.`
/// - `COPY 'name'.` and `COPY "name".`
///
/// Returns matches in reverse order (last first) for safe in-place replacement.
fn scan_copy_statements(source: &str) -> Vec<CopyMatch> {
    let mut matches = Vec::new();
    let upper = source.to_uppercase();
    let bytes = upper.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    while pos < len {
        // Skip to next potential COPY keyword
        let remaining = &upper[pos..];
        let Some(offset) = remaining.find("COPY ") else {
            break;
        };
        let copy_start = pos + offset;

        // Verify COPY is at word boundary (not part of a larger word)
        if copy_start > 0 {
            let prev = bytes[copy_start - 1];
            if prev.is_ascii_alphanumeric() || prev == b'_' || prev == b'-' {
                pos = copy_start + 5;
                continue;
            }
        }

        // Check we're not inside a comment line (line starts with *)
        let line_start = source[..copy_start].rfind('\n').map_or(0, |p| p + 1);
        let line_prefix = source[line_start..copy_start].trim_start();
        if line_prefix.starts_with('*') {
            pos = copy_start + 5;
            continue;
        }

        // Parse: COPY <name> [OF|IN <library>] .
        let after_copy = copy_start + 5; // past "COPY "
        let mut cursor = after_copy;

        // Skip whitespace
        while cursor < len && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor >= len {
            pos = cursor;
            continue;
        }

        // Parse name (possibly quoted)
        let (name, name_end) = parse_copy_name(source, cursor);
        if name.is_empty() {
            pos = cursor + 1;
            continue;
        }
        cursor = name_end;

        // Skip whitespace
        while cursor < len && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }

        // Check for OF|IN qualifier
        let mut library = None;
        if cursor + 2 < len {
            let next_word = &upper[cursor..];
            if next_word.starts_with("OF ") || next_word.starts_with("IN ") {
                cursor += 3;
                // Skip whitespace
                while cursor < len && bytes[cursor].is_ascii_whitespace() {
                    cursor += 1;
                }
                let (lib_name, lib_end) = parse_copy_name(source, cursor);
                if !lib_name.is_empty() {
                    library = Some(lib_name);
                    cursor = lib_end;
                }
            }
        }

        // Skip whitespace before SUPPRESS / REPLACING / period
        while cursor < len && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }

        // Check for SUPPRESS keyword
        let mut suppress = false;
        if cursor + 8 <= len && &upper[cursor..cursor + 8] == "SUPPRESS" {
            // Verify word boundary after SUPPRESS
            let after = cursor + 8;
            if after >= len
                || (!bytes[after].is_ascii_alphanumeric()
                    && bytes[after] != b'-'
                    && bytes[after] != b'_')
            {
                suppress = true;
                cursor = after;
                // Skip whitespace
                while cursor < len && bytes[cursor].is_ascii_whitespace() {
                    cursor += 1;
                }
            }
        }

        // Check for REPLACING clause
        let mut replacing = Vec::new();
        if cursor + 9 <= len && &upper[cursor..cursor + 9] == "REPLACING" {
            // Verify word boundary after REPLACING
            let after = cursor + 9;
            if after >= len
                || (!bytes[after].is_ascii_alphanumeric()
                    && bytes[after] != b'-'
                    && bytes[after] != b'_')
            {
                if let Ok((pairs, new_cursor)) =
                    parse_replacing_clause(source, &upper, cursor, &name)
                {
                    replacing = pairs;
                    cursor = new_cursor;
                } else {
                    // Malformed REPLACING -- skip this as not a valid COPY
                    pos = copy_start + 5;
                    continue;
                }
            }
        }

        // Skip whitespace to period
        while cursor < len && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }

        // Expect terminating period
        if cursor < len && bytes[cursor] == b'.' {
            cursor += 1; // past the period
            matches.push(CopyMatch {
                start: copy_start,
                end: cursor,
                name,
                library,
                replacing,
                suppress,
            });
            pos = cursor;
        } else {
            // No period found -- not a valid COPY statement
            pos = copy_start + 5;
        }
    }

    // Reverse for safe in-place replacement
    matches.reverse();
    matches
}

/// Parse a copybook name starting at `pos`, handling quoted and unquoted names.
/// Returns (name, `end_position`).
fn parse_copy_name(source: &str, pos: usize) -> (String, usize) {
    let bytes = source.as_bytes();
    if pos >= bytes.len() {
        return (String::new(), pos);
    }

    let ch = bytes[pos];
    if ch == b'\'' || ch == b'"' {
        // Quoted name
        let quote = ch;
        let start = pos + 1;
        let mut end = start;
        while end < bytes.len() && bytes[end] != quote {
            end += 1;
        }
        let name = source[start..end].to_string();
        let past_quote = if end < bytes.len() { end + 1 } else { end };
        (name, past_quote)
    } else {
        // Unquoted name: word characters (alphanumeric, hyphen, underscore)
        let start = pos;
        let mut end = start;
        while end < bytes.len() {
            let b = bytes[end];
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' {
                end += 1;
            } else {
                break;
            }
        }
        let name = source[start..end].to_string();
        (name, end)
    }
}

/// Parse a single REPLACING operand starting at `cursor`.
///
/// Recognizes:
/// - Pseudo-text: `==text==`
/// - Quoted literal: `'text'` or `"text"`
/// - Identifier/word: alphanumeric with hyphens/underscores
///
/// Returns `(operand, new_cursor)` or `None` if no valid operand found.
fn parse_replacing_operand(
    source: &str,
    upper: &str,
    cursor: usize,
) -> Option<(ReplacingOperand, usize)> {
    let bytes = source.as_bytes();
    let len = bytes.len();
    if cursor >= len {
        return None;
    }

    // Check for pseudo-text delimiter ==
    if cursor + 1 < len && bytes[cursor] == b'=' && bytes[cursor + 1] == b'=' {
        let start = cursor + 2;
        // Find closing ==
        let mut end = start;
        while end + 1 < len {
            if bytes[end] == b'=' && bytes[end + 1] == b'=' {
                let text = source[start..end].to_string();
                return Some((ReplacingOperand::PseudoText(text), end + 2));
            }
            end += 1;
        }
        // Unclosed pseudo-text
        return None;
    }

    // Check for quoted literal
    if bytes[cursor] == b'\'' || bytes[cursor] == b'"' {
        let quote = bytes[cursor];
        let start = cursor + 1;
        let mut end = start;
        while end < len && bytes[end] != quote {
            end += 1;
        }
        if end >= len {
            return None; // Unclosed quote
        }
        let text = source[start..end].to_string();
        return Some((ReplacingOperand::Literal(text), end + 1));
    }

    // Identifier/word: alphanumeric, hyphen, underscore
    if bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'-' || bytes[cursor] == b'_' {
        let start = cursor;
        let mut end = start;
        while end < len {
            let b = bytes[end];
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' {
                end += 1;
            } else {
                break;
            }
        }
        if end > start {
            // Make sure this is not the BY keyword (which terminates old operand in some cases)
            let word = &upper[start..end];
            if word == "BY" || word == "." {
                return None;
            }
            let text = source[start..end].to_string();
            return Some((ReplacingOperand::Word(text), end));
        }
    }

    None
}

/// Parse a REPLACING clause starting at `cursor` (positioned at the 'R' of REPLACING).
///
/// Returns `(replacement_pairs, new_cursor)`.
fn parse_replacing_clause(
    source: &str,
    upper: &str,
    cursor: usize,
    copybook_name: &str,
) -> std::result::Result<(Vec<(ReplacingOperand, ReplacingOperand)>, usize), TranspileError> {
    let bytes = upper.as_bytes();
    let len = bytes.len();
    // Skip past "REPLACING"
    let mut pos = cursor + 9;
    let mut pairs = Vec::new();

    loop {
        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        // Check if we hit period (end of COPY statement)
        if bytes[pos] == b'.' {
            break;
        }

        // Parse old operand
        let (old_op, after_old) =
            parse_replacing_operand(source, upper, pos).ok_or_else(|| {
                TranspileError::CopyReplacingInvalid {
                    copybook: copybook_name.to_string(),
                    message: format!(
                        "expected replacing operand at position {pos}"
                    ),
                }
            })?;
        pos = after_old;

        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }

        // Expect BY keyword
        if pos + 2 > len || &upper[pos..pos + 2] != "BY" {
            return Err(TranspileError::CopyReplacingInvalid {
                copybook: copybook_name.to_string(),
                message: format!("expected BY keyword at position {pos}"),
            });
        }
        pos += 2;

        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }

        // Parse new operand
        let (new_op, after_new) =
            parse_replacing_operand(source, upper, pos).ok_or_else(|| {
                TranspileError::CopyReplacingInvalid {
                    copybook: copybook_name.to_string(),
                    message: format!(
                        "expected replacement operand at position {pos}"
                    ),
                }
            })?;
        pos = after_new;

        pairs.push((old_op, new_op));
    }

    Ok((pairs, pos))
}

/// Check if the character at position is a word boundary for COBOL identifiers.
/// A word boundary means the character is NOT alphanumeric, hyphen, or underscore.
fn is_word_boundary(bytes: &[u8], pos: usize) -> bool {
    if pos >= bytes.len() {
        return true;
    }
    let b = bytes[pos];
    !(b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

/// Apply REPLACING substitutions to copybook content.
///
/// Each replacement pair is applied in order across the entire content.
/// - `PseudoText`: case-insensitive find-and-replace of the exact text.
/// - `Word`: case-insensitive word-boundary-delimited replacement.
/// - `Literal`: case-sensitive exact string replacement.
fn apply_replacements(
    content: &str,
    replacing: &[(ReplacingOperand, ReplacingOperand)],
) -> String {
    let mut result = content.to_string();

    for (old_op, new_op) in replacing {
        let new_text = match new_op {
            ReplacingOperand::PseudoText(s)
            | ReplacingOperand::Word(s)
            | ReplacingOperand::Literal(s) => s.as_str(),
        };

        result = match old_op {
            ReplacingOperand::PseudoText(old_text) => {
                // Case-insensitive replacement of pseudo-text content.
                // Trim leading/trailing spaces from the pseudo-text pattern per COBOL standard.
                let pattern = old_text.trim();
                if pattern.is_empty() {
                    result
                } else {
                    let replacement = new_text.trim();
                    replace_case_insensitive(&result, pattern, replacement)
                }
            }
            ReplacingOperand::Word(old_word) => {
                // Case-insensitive word-boundary replacement
                replace_word_boundary(&result, old_word, new_text)
            }
            ReplacingOperand::Literal(old_lit) => {
                // Case-sensitive exact replacement
                result.replace(old_lit, new_text)
            }
        };
    }

    result
}

/// Case-insensitive string replacement (all occurrences).
fn replace_case_insensitive(haystack: &str, needle: &str, replacement: &str) -> String {
    if needle.is_empty() {
        return haystack.to_string();
    }
    let hay_upper = haystack.to_uppercase();
    let needle_upper = needle.to_uppercase();
    let needle_len = needle.len();
    let mut result = String::with_capacity(haystack.len());
    let mut start = 0;

    while let Some(pos) = hay_upper[start..].find(&needle_upper) {
        let abs_pos = start + pos;
        result.push_str(&haystack[start..abs_pos]);
        result.push_str(replacement);
        start = abs_pos + needle_len;
    }
    result.push_str(&haystack[start..]);
    result
}

/// Word-boundary-delimited case-insensitive replacement (all occurrences).
fn replace_word_boundary(haystack: &str, word: &str, replacement: &str) -> String {
    if word.is_empty() {
        return haystack.to_string();
    }
    let hay_upper = haystack.to_uppercase();
    let word_upper = word.to_uppercase();
    let word_len = word.len();
    let hay_bytes = haystack.as_bytes();
    let mut result = String::with_capacity(haystack.len());
    let mut start = 0;

    while let Some(pos) = hay_upper[start..].find(&word_upper) {
        let abs_pos = start + pos;
        // Check word boundaries
        let before_ok = abs_pos == 0 || is_word_boundary(hay_bytes, abs_pos - 1);
        let after_ok = is_word_boundary(hay_bytes, abs_pos + word_len);
        if before_ok && after_ok {
            result.push_str(&haystack[start..abs_pos]);
            result.push_str(replacement);
            start = abs_pos + word_len;
        } else {
            // Not a word boundary match -- include the character and advance
            result.push_str(&haystack[start..=abs_pos]);
            start = abs_pos + 1;
        }
    }
    result.push_str(&haystack[start..]);
    result
}

/// Expands COPY statements in COBOL source by inlining copybook content.
///
/// Handles nested COPYs (copybooks containing COPY statements) up to `max_depth`.
/// Detects cyclic dependencies and reports them as errors.
pub struct CopyExpander {
    resolver: Box<dyn CopybookResolver>,
    max_depth: usize,
}

impl std::fmt::Debug for CopyExpander {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CopyExpander")
            .field("max_depth", &self.max_depth)
            .finish_non_exhaustive()
    }
}

impl CopyExpander {
    pub fn new(resolver: Box<dyn CopybookResolver>, max_depth: usize) -> Self {
        Self {
            resolver,
            max_depth,
        }
    }

    /// Expand all COPY statements in `source`, resolving nested copies.
    ///
    /// # Errors
    ///
    /// Returns `TranspileError::CopyNotFound` if a copybook cannot be resolved,
    /// `CopyCyclic` on circular dependencies, or `CopyDepthExceeded` on too-deep nesting.
    pub fn expand(&self, source: &str) -> Result<String> {
        let mut stack: Vec<String> = Vec::new();
        self.expand_recursive(source, &mut stack, 0)
    }

    fn expand_recursive(
        &self,
        source: &str,
        stack: &mut Vec<String>,
        depth: usize,
    ) -> Result<String> {
        if depth > self.max_depth {
            return Err(TranspileError::CopyDepthExceeded {
                depth,
                max: self.max_depth,
            });
        }

        let copy_matches = scan_copy_statements(source);
        if copy_matches.is_empty() {
            return Ok(source.to_string());
        }

        let mut result = source.to_string();

        // Matches are in reverse order, so replacements don't shift offsets
        for m in &copy_matches {
            let name_upper = m.name.to_uppercase();

            // Cycle detection
            if stack.iter().any(|s| s.to_uppercase() == name_upper) {
                let mut chain: Vec<String> = stack.clone();
                chain.push(m.name.clone());
                return Err(TranspileError::CopyCyclic { chain });
            }

            // Resolve copybook content
            let content = self.resolver.resolve(&m.name, m.library.as_deref())?;

            // Apply REPLACING substitutions before recursive expansion
            let content = if m.replacing.is_empty() {
                content
            } else {
                apply_replacements(&content, &m.replacing)
            };

            // Recursively expand nested COPYs
            stack.push(m.name.clone());
            let expanded = self.expand_recursive(&content, stack, depth + 1)?;
            stack.pop();

            // Replace COPY statement with expanded content
            result.replace_range(m.start..m.end, &expanded);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::copybook::InMemoryResolver;

    #[test]
    fn single_copy() {
        let resolver = InMemoryResolver::new()
            .add("ACCTFILE", "01 ACCT-REC PIC X(80).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "WORKING-STORAGE SECTION.\nCOPY ACCTFILE.\n01 OTHER PIC X.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("ACCT-REC"));
        assert!(result.contains("OTHER"));
        assert!(!result.contains("COPY"));
    }

    #[test]
    fn nested_copy() {
        let resolver = InMemoryResolver::new()
            .add("OUTER", "COPY INNER.\n01 OUTER-FLD PIC X.\n")
            .add("INNER", "01 INNER-FLD PIC 9(5).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY OUTER.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("INNER-FLD"));
        assert!(result.contains("OUTER-FLD"));
        assert!(!result.contains("COPY"));
    }

    #[test]
    fn cycle_detection() {
        let resolver = InMemoryResolver::new()
            .add("A", "COPY B.\n")
            .add("B", "COPY A.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY A.\n";
        let result = expander.expand(source);
        assert!(result.is_err());
        match result.unwrap_err() {
            TranspileError::CopyCyclic { chain } => {
                assert!(chain.contains(&"A".to_string()));
                assert!(chain.contains(&"B".to_string()));
            }
            other => panic!("expected CopyCyclic, got {other:?}"),
        }
    }

    #[test]
    fn depth_limit() {
        // Chain: D0 -> D1 -> D2 -> ... -> D11 (exceeds max 10)
        let mut resolver = InMemoryResolver::new();
        for i in 0..12 {
            let next = format!("COPY D{}.\n", i + 1);
            resolver = resolver.add(&format!("D{i}"), &next);
        }
        resolver = resolver.add("D12", "01 DEEP PIC X.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY D0.\n";
        let result = expander.expand(source);
        assert!(result.is_err());
        match result.unwrap_err() {
            TranspileError::CopyDepthExceeded { depth, max } => {
                assert!(depth > max);
            }
            other => panic!("expected CopyDepthExceeded, got {other:?}"),
        }
    }

    #[test]
    fn library_qualifier() {
        let resolver = InMemoryResolver::new()
            .add("COMMON", "01 WS-COMMON PIC X(10).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY COMMON OF MYLIB.\n";
        let result = expander.expand(source).unwrap();
        assert!(result.contains("WS-COMMON"));
    }

    #[test]
    fn case_insensitive_copy() {
        let resolver = InMemoryResolver::new()
            .add("FIELDS", "01 WS-FLD PIC X.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "copy fields.\n";
        let result = expander.expand(source).unwrap();
        assert!(result.contains("WS-FLD"));
    }

    #[test]
    fn no_copy_passthrough() {
        let resolver = InMemoryResolver::new();
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "01 WS-FIELD PIC X(10).\n";
        let result = expander.expand(source).unwrap();
        assert_eq!(result, source);
    }

    #[test]
    fn multiple_copies_in_source() {
        let resolver = InMemoryResolver::new()
            .add("HEADER", "01 HDR PIC X(10).\n")
            .add("DETAIL", "01 DTL PIC X(20).\n")
            .add("TRAILER", "01 TRL PIC X(5).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY HEADER.\nCOPY DETAIL.\nCOPY TRAILER.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("HDR"));
        assert!(result.contains("DTL"));
        assert!(result.contains("TRL"));
        assert!(!result.contains("COPY"));
    }

    // --- Session 29: REPLACING + SUPPRESS tests ---

    #[test]
    fn replacing_pseudo_text() {
        let resolver = InMemoryResolver::new()
            .add("ACCTFILE", "01 ACCT-REC PIC X(80).\n05 ACCT-NAME PIC X(30).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY ACCTFILE REPLACING ==ACCT-REC== BY ==NEW-ACCT-REC==.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("NEW-ACCT-REC"));
        assert!(!result.contains("COPY"));
        // ACCT-NAME should be unchanged (not a match for ACCT-REC)
        assert!(result.contains("ACCT-NAME"));
    }

    #[test]
    fn replacing_word() {
        let resolver = InMemoryResolver::new()
            .add("FIELDS", "01 WS-OLD PIC X(10).\n05 WS-OLD-FLAG PIC 9.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY FIELDS REPLACING WS-OLD BY WS-NEW.\n";
        let result = expander.expand(source).unwrap();

        // WS-OLD replaced with WS-NEW (word boundary)
        assert!(result.contains("WS-NEW"));
        // WS-OLD-FLAG should NOT be replaced (WS-OLD is not at word boundary in WS-OLD-FLAG)
        assert!(result.contains("WS-OLD-FLAG"));
    }

    #[test]
    fn replacing_literal() {
        let resolver = InMemoryResolver::new()
            .add("MSG", "01 WS-MSG PIC X(20) VALUE 'OLD MESSAGE'.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY MSG REPLACING \"OLD MESSAGE\" BY \"NEW MESSAGE\".\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("NEW MESSAGE"));
        assert!(!result.contains("OLD MESSAGE"));
    }

    #[test]
    fn replacing_multiple_pairs() {
        let resolver = InMemoryResolver::new()
            .add("REC", "01 OLD-REC.\n05 OLD-FIELD PIC X.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source =
            "COPY REC REPLACING ==OLD-REC== BY ==NEW-REC== ==OLD-FIELD== BY ==NEW-FIELD==.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("NEW-REC"));
        assert!(result.contains("NEW-FIELD"));
        assert!(!result.contains("OLD-REC"));
        assert!(!result.contains("OLD-FIELD"));
    }

    #[test]
    fn replacing_case_insensitive() {
        let resolver = InMemoryResolver::new()
            .add("MIXED", "01 ws-field PIC X(10).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        // Word replacement is case-insensitive
        let source = "COPY MIXED REPLACING WS-FIELD BY WS-RENAMED.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("WS-RENAMED"));
    }

    #[test]
    fn replacing_word_boundary() {
        let resolver = InMemoryResolver::new()
            .add("BOUND", "01 WS-OLD PIC X.\n05 WS-OLDER PIC X.\n05 MY-WS-OLD PIC X.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY BOUND REPLACING WS-OLD BY WS-NEW.\n";
        let result = expander.expand(source).unwrap();

        // WS-OLD (standalone) replaced
        assert!(result.contains("WS-NEW"));
        // WS-OLDER should NOT be replaced (OLD is part of a larger word)
        assert!(result.contains("WS-OLDER"));
        // MY-WS-OLD should NOT be replaced (OLD not at word start boundary)
        assert!(result.contains("MY-WS-OLD"));
    }

    #[test]
    fn replacing_empty_pseudo_text() {
        let resolver = InMemoryResolver::new()
            .add("STRIP", "01 PREFIX-FIELD PIC X.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        // Empty replacement == == deletes matched text
        let source = "COPY STRIP REPLACING ==PREFIX-== BY == ==.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("FIELD"));
        assert!(!result.contains("PREFIX-"));
    }

    #[test]
    fn replacing_with_nested_copy() {
        let resolver = InMemoryResolver::new()
            .add("OUTER", "COPY INNER.\n01 OUTER-FLD PIC X.\n")
            .add("INNER", "01 INNER-FLD PIC 9(5).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        // REPLACING on OUTER should apply before nested COPY INNER is expanded
        let source = "COPY OUTER REPLACING ==OUTER-FLD== BY ==RENAMED-FLD==.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("RENAMED-FLD"));
        assert!(result.contains("INNER-FLD"));
        assert!(!result.contains("OUTER-FLD"));
    }

    #[test]
    fn replacing_preserves_non_matching() {
        let resolver = InMemoryResolver::new()
            .add("SAFE", "01 KEEP-ME PIC X.\n05 ALSO-KEEP PIC 9.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY SAFE REPLACING ==NONEXISTENT== BY ==SOMETHING==.\n";
        let result = expander.expand(source).unwrap();

        // Nothing matched, content should be unchanged
        assert!(result.contains("KEEP-ME"));
        assert!(result.contains("ALSO-KEEP"));
    }

    #[test]
    fn suppress_parsed_and_ignored() {
        let resolver = InMemoryResolver::new()
            .add("SUPPTEST", "01 WS-SUP PIC X.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY SUPPTEST SUPPRESS.\n";
        let result = expander.expand(source).unwrap();

        // SUPPRESS is parsed but ignored -- content still included
        assert!(result.contains("WS-SUP"));
        assert!(!result.contains("COPY"));
    }

    #[test]
    fn replacing_and_suppress() {
        let resolver = InMemoryResolver::new()
            .add("COMBO", "01 OLD-NAME PIC X.\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY COMBO SUPPRESS REPLACING ==OLD-NAME== BY ==NEW-NAME==.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("NEW-NAME"));
        assert!(!result.contains("OLD-NAME"));
    }

    #[test]
    fn no_replacing_backward_compat() {
        // Verify existing COPY without REPLACING still works after Session 29 changes
        let resolver = InMemoryResolver::new()
            .add("COMPAT", "01 WS-COMPAT PIC X(10).\n");
        let expander = CopyExpander::new(Box::new(resolver), 10);

        let source = "COPY COMPAT.\n";
        let result = expander.expand(source).unwrap();

        assert!(result.contains("WS-COMPAT"));
        assert!(!result.contains("COPY"));
    }
}
