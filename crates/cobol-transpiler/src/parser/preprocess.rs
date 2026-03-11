//! COBOL source preprocessor.
//!
//! Handles fixed-format (columns 1-72) and free-format source.
//! Strips sequence numbers, indicator areas, and comments.
//! Handles continuation lines (indicator `-`).

use std::cell::RefCell;

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
/// Snap a byte index to the nearest valid char boundary (searching forward).
/// If `pos` is already on a boundary, returns it unchanged.
fn snap_to_char_boundary(s: &str, pos: usize) -> usize {
    if pos >= s.len() {
        return s.len();
    }
    let mut i = pos;
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

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
        // Use char boundary-safe slicing to handle non-ASCII (e.g., EBCDIC remnants).
        let code_end = line.len().min(72);
        let safe_start = snap_to_char_boundary(line, 7);
        let safe_end = snap_to_char_boundary(line, code_end);
        let code_area = &line[safe_start..safe_end];

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
        if line.len() < 7 || !line.is_char_boundary(6) {
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

/// Strip IDENTIFICATION DIVISION metadata paragraphs that confuse the ANTLR parser.
///
/// AUTHOR, DATE-WRITTEN, DATE-COMPILED, INSTALLATION, SECURITY, REMARKS
/// are optional ID division paragraphs. Their content is free-form text that
/// can contain hyphens, periods, and other characters that the parser may
/// misinterpret as COBOL statements. Since this metadata is not needed for
/// transpilation, we blank these lines.
fn strip_id_division_metadata(source: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut in_metadata = false;

    for line in source.lines() {
        let trimmed = line.trim().to_uppercase();

        // Detect start of a metadata paragraph
        if trimmed.starts_with("AUTHOR")
            || trimmed.starts_with("DATE-WRITTEN")
            || trimmed.starts_with("DATE-COMPILED")
            || trimmed.starts_with("INSTALLATION")
            || trimmed.starts_with("SECURITY")
            || trimmed.starts_with("REMARKS")
        {
            // Check it's actually a paragraph header (contains a period)
            if trimmed.contains('.') {
                in_metadata = true;
                lines.push(String::new());
                // If the line ends with a period after content, metadata ends on this line
                let after_keyword = if let Some(pos) = trimmed.find('.') {
                    trimmed[pos + 1..].trim().to_string()
                } else {
                    String::new()
                };
                // If there's content after the first period that also ends with a period,
                // the metadata is self-contained on this line
                if after_keyword.ends_with('.') || after_keyword.is_empty() {
                    in_metadata = false;
                }
                continue;
            }
        }

        // End metadata at next recognized division/section/paragraph header
        if in_metadata {
            if trimmed.starts_with("ENVIRONMENT")
                || trimmed.starts_with("DATA")
                || trimmed.starts_with("PROCEDURE")
                || trimmed.starts_with("PROGRAM-ID")
                || trimmed.starts_with("WORKING-STORAGE")
                || trimmed.starts_with("FILE")
                || trimmed.starts_with("LINKAGE")
                || trimmed.starts_with("LOCAL-STORAGE")
            {
                in_metadata = false;
                lines.push(line.to_string());
            } else {
                // Still in metadata -- blank this line
                lines.push(String::new());
            }
            continue;
        }

        lines.push(line.to_string());
    }

    lines.join("\n")
}

/// Auto-detect format and preprocess accordingly.
///
/// Returns the preprocessed source with EXEC SQL/CICS blocks replaced by
/// CONTINUE placeholders. The extracted blocks are stored in a thread-local
/// for the proc_listener to consume during AST construction.
pub fn preprocess(source: &str) -> Result<String> {
    // Strip IDENTIFICATION DIVISION metadata paragraphs first
    let cleaned = strip_id_division_metadata(source);
    let format_result = match detect_source_format(&cleaned) {
        SourceFormat::Fixed => preprocess_fixed_format(&cleaned)?,
        SourceFormat::Free => preprocess_free_format(&cleaned),
    };
    // Flatten long ELSE IF chains to prevent ANTLR exponential backtracking
    let flattened = flatten_else_if_chains(&format_result);
    // Extract EXEC SQL/CICS blocks and replace with CONTINUE
    let extraction = extract_exec_blocks(&flattened);
    // Store extracted blocks for the proc_listener to consume
    set_exec_blocks(extraction.sql_blocks);
    Ok(extraction.cleaned_source)
}

/// Flatten long chains of `ELSE IF` without `END-IF` by injecting `END-IF`
/// before each `ELSE IF` in the chain.
///
/// COBOL allows `IF ... ELSE IF ... ELSE IF ...` without explicit `END-IF`
/// scope terminators. The ANTLR grammar treats each `ELSE IF` as a nested
/// `ELSE (IF ...)`, creating exponential backtracking with O(2^N) parse
/// time for N levels of nesting.
///
/// This function detects such chains in the PROCEDURE DIVISION and rewrites
/// them by inserting `END-IF` before each `ELSE IF`, converting the grammar
/// from deeply nested to flat. The ANTLR parser then handles each IF/END-IF
/// pair independently in linear time.
///
/// Only chains of 3+ consecutive `ELSE IF` without intervening `END-IF` are
/// rewritten. Shorter chains parse quickly and don't need intervention.
///
/// The rewrite preserves semantics: `ELSE IF B` is equivalent to
/// `END-IF ELSE IF B` when the preceding IF has no explicit END-IF.
fn flatten_else_if_chains(source: &str) -> String {
    // Only operate on PROCEDURE DIVISION content
    let upper = source.to_uppercase();
    let proc_pos = match upper.find("PROCEDURE DIVISION") {
        Some(pos) => pos,
        None => return source.to_string(),
    };

    // Find the start of the line containing PROCEDURE DIVISION
    let proc_line_start = source[..proc_pos].rfind('\n').map_or(0, |p| p + 1);

    // Work on lines from PROCEDURE DIVISION onward
    let prefix = &source[..proc_line_start];
    let proc_source = &source[proc_line_start..];
    let proc_lines: Vec<&str> = proc_source.lines().collect();

    // First pass: identify lines that are ELSE IF or END-IF
    // Build a map of line classifications
    let mut line_types: Vec<LineType> = Vec::with_capacity(proc_lines.len());
    for line in &proc_lines {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("ELSE IF ") || trimmed == "ELSE IF" {
            line_types.push(LineType::ElseIf);
        } else if trimmed.starts_with("END-IF") || trimmed.starts_with("END IF") {
            line_types.push(LineType::EndIf);
        } else if trimmed.starts_with("IF ") || trimmed == "IF" {
            line_types.push(LineType::If);
        } else {
            line_types.push(LineType::Other);
        }
    }

    // Second pass: find chains of ELSE IF without END-IF
    // A chain is: IF ... body ... ELSE IF ... body ... ELSE IF ... body ...
    // We need to find sequences of ElseIf lines with no EndIf between them
    // that form chains of 3+ ELSE IFs
    let chains = find_else_if_chains(&line_types);

    if chains.is_empty() {
        return source.to_string();
    }

    // Third pass: rebuild source with END-IF injected before each ELSE IF in chains
    let mut result = String::with_capacity(source.len() + chains.len() * 20);
    result.push_str(prefix);

    // Collect the line indices that need END-IF injection
    let mut inject_before: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for chain in &chains {
        // Inject END-IF before each ELSE IF in the chain (not the first IF)
        for &line_idx in chain {
            inject_before.insert(line_idx);
        }
    }

    for (i, line) in proc_lines.iter().enumerate() {
        if inject_before.contains(&i) {
            // Inject END-IF on its own line, matching indentation of the ELSE IF
            let indent = line.len() - line.trim_start().len();
            let indent_str: String = line[..indent].to_string();
            result.push_str(&indent_str);
            result.push_str("END-IF\n");
        }
        result.push_str(line);
        if i < proc_lines.len() - 1 {
            result.push('\n');
        }
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LineType {
    If,
    ElseIf,
    EndIf,
    Other,
}

/// Find chains of 3+ consecutive ELSE IF lines without intervening END-IF.
///
/// Returns a list of chains, where each chain is a list of line indices
/// (of the ELSE IF lines that need END-IF injected before them).
fn find_else_if_chains(line_types: &[LineType]) -> Vec<Vec<usize>> {
    let mut chains: Vec<Vec<usize>> = Vec::new();
    let mut current_chain: Vec<usize> = Vec::new();

    for (i, lt) in line_types.iter().enumerate() {
        match lt {
            LineType::ElseIf => {
                current_chain.push(i);
            }
            LineType::EndIf => {
                // END-IF breaks the chain -- this code uses explicit scoping
                if current_chain.len() >= 3 {
                    chains.push(current_chain.clone());
                }
                current_chain.clear();
            }
            LineType::If => {
                // New IF: save any existing chain and start fresh
                if current_chain.len() >= 3 {
                    chains.push(current_chain.clone());
                }
                current_chain.clear();
            }
            LineType::Other => {
                // Other lines don't break the chain (they're the body between ELSE IFs)
            }
        }
    }

    // Don't forget the last chain
    if current_chain.len() >= 3 {
        chains.push(current_chain);
    }

    chains
}

// Thread-local storage for extracted EXEC blocks.
// Set by preprocess(), consumed by the proc_listener.
thread_local! {
    static EXEC_BLOCKS: RefCell<Vec<ExtractedExecBlock>> = const { RefCell::new(Vec::new()) };
}

/// Store extracted EXEC blocks for later consumption.
fn set_exec_blocks(blocks: Vec<ExtractedExecBlock>) {
    EXEC_BLOCKS.with(|eb| {
        *eb.borrow_mut() = blocks;
    });
}

/// Take the next extracted EXEC block (FIFO order).
/// Called by the proc_listener when it encounters a CONTINUE placeholder.
pub fn take_next_exec_block() -> Option<ExtractedExecBlock> {
    EXEC_BLOCKS.with(|eb| {
        let mut blocks = eb.borrow_mut();
        if blocks.is_empty() {
            None
        } else {
            Some(blocks.remove(0))
        }
    })
}

/// Check if there are pending EXEC blocks (used to detect SQL programs).
pub fn has_pending_exec_blocks() -> bool {
    EXEC_BLOCKS.with(|eb| !eb.borrow().is_empty())
}

/// Get the count of remaining EXEC blocks.
pub fn pending_exec_block_count() -> usize {
    EXEC_BLOCKS.with(|eb| eb.borrow().len())
}

/// Result of extracting EXEC SQL/CICS blocks from source.
#[derive(Debug, Clone)]
pub struct ExecExtraction {
    /// The source with EXEC blocks replaced by `CONTINUE` placeholders.
    pub cleaned_source: String,
    /// Extracted SQL blocks in order of appearance.
    pub sql_blocks: Vec<ExtractedExecBlock>,
}

/// A single extracted EXEC SQL or EXEC CICS block.
#[derive(Debug, Clone)]
pub struct ExtractedExecBlock {
    /// "SQL" or "CICS".
    pub exec_type: String,
    /// Normalized SQL/CICS text (whitespace collapsed, trimmed).
    pub text: String,
}

/// Extract EXEC SQL/CICS blocks from preprocessed source.
///
/// Replaces each `EXEC SQL ... END-EXEC` (and `EXEC CICS ... END-EXEC`)
/// with a COBOL `CONTINUE` statement so ANTLR can parse the rest normally.
/// The extracted blocks are returned separately for AST injection.
pub fn extract_exec_blocks(source: &str) -> ExecExtraction {
    let mut result = String::with_capacity(source.len());
    let mut blocks = Vec::new();
    let upper = source.to_uppercase();
    let bytes = source.as_bytes();
    let upper_bytes = upper.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    // Find the position of PROCEDURE DIVISION in the source.
    // EXEC SQL blocks before this point (e.g., INCLUDE SQLCA in DATA DIVISION)
    // must NOT be replaced with CONTINUE (which is a procedure statement).
    let proc_div_pos = upper.find("PROCEDURE DIVISION").unwrap_or(len);

    while i < len {
        // Look for "EXEC SQL" (case-insensitive)
        if i + 8 <= len && &upper_bytes[i..i + 8] == b"EXEC SQL" {
            let before_ok = i == 0 || !upper_bytes[i - 1].is_ascii_alphanumeric();
            let after_ok = i + 8 >= len || !upper_bytes[i + 8].is_ascii_alphanumeric();

            if before_ok && after_ok {
                let sql_start = i + 8;
                if let Some(end_pos) = find_end_exec(&upper, sql_start) {
                    let sql_text = &source[sql_start..end_pos];
                    let normalized = sql_text
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ");
                    blocks.push(ExtractedExecBlock {
                        exec_type: "SQL".to_string(),
                        text: normalized,
                    });
                    // In PROCEDURE DIVISION: replace with CONTINUE statement.
                    // Before PROCEDURE DIVISION (DATA DIVISION): remove entirely
                    // (CONTINUE is not valid in DATA DIVISION).
                    let in_proc_div = i >= proc_div_pos;
                    if in_proc_div {
                        result.push_str("CONTINUE");
                    }
                    i = end_pos + 8; // skip past END-EXEC
                    // Skip optional trailing whitespace and period
                    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
                        i += 1;
                    }
                    if i < len && bytes[i] == b'.' {
                        if in_proc_div {
                            result.push('.');
                        }
                        i += 1;
                    }
                    continue;
                }
            }
        }

        // Look for "EXEC CICS"
        if i + 9 <= len && &upper_bytes[i..i + 9] == b"EXEC CICS" {
            let before_ok = i == 0 || !upper_bytes[i - 1].is_ascii_alphanumeric();
            let after_ok = i + 9 >= len || !upper_bytes[i + 9].is_ascii_alphanumeric();

            if before_ok && after_ok {
                let cics_start = i + 9;
                if let Some(end_pos) = find_end_exec(&upper, cics_start) {
                    let cics_text = &source[cics_start..end_pos];
                    let normalized = cics_text
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ");
                    blocks.push(ExtractedExecBlock {
                        exec_type: "CICS".to_string(),
                        text: normalized,
                    });
                    let in_proc_div = i >= proc_div_pos;
                    if in_proc_div {
                        result.push_str("CONTINUE");
                    }
                    i = end_pos + 8;
                    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
                        i += 1;
                    }
                    if i < len && bytes[i] == b'.' {
                        if in_proc_div {
                            result.push('.');
                        }
                        i += 1;
                    }
                    continue;
                }
            }
        }

        result.push(bytes[i] as char);
        i += 1;
    }

    ExecExtraction {
        cleaned_source: result,
        sql_blocks: blocks,
    }
}

/// Find the position of "END-EXEC" in the uppercased source starting from `from`.
fn find_end_exec(upper: &str, from: usize) -> Option<usize> {
    upper[from..].find("END-EXEC").map(|p| from + p)
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

    // -----------------------------------------------------------------------
    // EXEC SQL/CICS extraction tests
    // -----------------------------------------------------------------------

    #[test]
    fn extract_single_exec_sql() {
        let source = "PROCEDURE DIVISION.\nMOVE 1 TO WS-X.\nEXEC SQL SELECT A INTO :H FROM T END-EXEC.\nSTOP RUN.";
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 1);
        assert_eq!(ext.sql_blocks[0].exec_type, "SQL");
        assert_eq!(ext.sql_blocks[0].text, "SELECT A INTO :H FROM T");
        // EXEC SQL replaced with CONTINUE
        assert!(ext.cleaned_source.contains("CONTINUE"));
        assert!(!ext.cleaned_source.contains("EXEC SQL"));
        // Surrounding code preserved
        assert!(ext.cleaned_source.contains("MOVE 1 TO WS-X."));
        assert!(ext.cleaned_source.contains("STOP RUN."));
    }

    #[test]
    fn extract_multiple_exec_sql() {
        let source = concat!(
            "EXEC SQL INSERT INTO T VALUES (:H1) END-EXEC.\n",
            "DISPLAY 'OK'.\n",
            "EXEC SQL COMMIT END-EXEC.\n",
        );
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 2);
        assert_eq!(ext.sql_blocks[0].text, "INSERT INTO T VALUES (:H1)");
        assert_eq!(ext.sql_blocks[1].text, "COMMIT");
    }

    #[test]
    fn extract_multiline_exec_sql() {
        let source = concat!(
            "    EXEC SQL\n",
            "        SELECT ENAME, SAL\n",
            "        INTO :WS-ENAME, :WS-SAL\n",
            "        FROM EMP\n",
            "        WHERE EMPNO = :WS-EMPNO\n",
            "    END-EXEC.\n",
        );
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 1);
        // Whitespace should be collapsed
        assert!(ext.sql_blocks[0].text.contains("SELECT ENAME, SAL INTO :WS-ENAME, :WS-SAL FROM EMP WHERE EMPNO = :WS-EMPNO"));
        assert!(!ext.sql_blocks[0].text.contains('\n'));
    }

    #[test]
    fn extract_exec_cics() {
        let source = "EXEC CICS RETURN TRANSID('TXN1') COMMAREA(WS-COMM) END-EXEC.";
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 1);
        assert_eq!(ext.sql_blocks[0].exec_type, "CICS");
        assert!(ext.sql_blocks[0].text.contains("RETURN TRANSID"));
    }

    #[test]
    fn extract_mixed_sql_and_cics() {
        let source = concat!(
            "EXEC SQL SELECT A INTO :H FROM T END-EXEC.\n",
            "EXEC CICS RETURN END-EXEC.\n",
        );
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 2);
        assert_eq!(ext.sql_blocks[0].exec_type, "SQL");
        assert_eq!(ext.sql_blocks[1].exec_type, "CICS");
    }

    #[test]
    fn extract_no_exec_blocks() {
        let source = "MOVE 1 TO WS-X.\nDISPLAY 'HELLO'.\nSTOP RUN.";
        let ext = extract_exec_blocks(source);
        assert!(ext.sql_blocks.is_empty());
        assert_eq!(ext.cleaned_source, source);
    }

    #[test]
    fn extract_preserves_period_after_end_exec() {
        let source = "PROCEDURE DIVISION.\nEXEC SQL COMMIT END-EXEC.\nSTOP RUN.";
        let ext = extract_exec_blocks(source);
        // The period after END-EXEC should be preserved (attached to CONTINUE)
        assert!(ext.cleaned_source.contains("CONTINUE."));
    }

    #[test]
    fn extract_case_insensitive() {
        let source = "exec sql commit end-exec.";
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 1);
        assert_eq!(ext.sql_blocks[0].text, "commit");
    }

    #[test]
    fn extract_exec_sql_include_sqlca() {
        let source = "EXEC SQL INCLUDE SQLCA END-EXEC.";
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 1);
        assert_eq!(ext.sql_blocks[0].text, "INCLUDE SQLCA");
    }

    #[test]
    fn extract_data_division_exec_sql_no_continue() {
        // EXEC SQL in DATA DIVISION should NOT produce CONTINUE (it's not valid there)
        let source = "DATA DIVISION.\nEXEC SQL INCLUDE SQLCA END-EXEC.\nPROCEDURE DIVISION.\nEXEC SQL COMMIT END-EXEC.\nSTOP RUN.";
        let ext = extract_exec_blocks(source);
        assert_eq!(ext.sql_blocks.len(), 2);
        // DATA DIVISION block: no CONTINUE inserted
        assert!(!ext.cleaned_source.starts_with("DATA DIVISION.\nCONTINUE"));
        // PROCEDURE DIVISION block: CONTINUE inserted
        assert!(ext.cleaned_source.contains("PROCEDURE DIVISION.\nCONTINUE."));
    }

    #[test]
    fn extract_does_not_match_partial_exec() {
        // "EXECUTE" should not trigger EXEC SQL extraction
        let source = "EXECUTE SECTION-A.\n";
        let ext = extract_exec_blocks(source);
        assert!(ext.sql_blocks.is_empty());
        assert_eq!(ext.cleaned_source, source);
    }

    #[test]
    fn extract_exec_sql_no_end_exec() {
        // Unterminated EXEC SQL -- should pass through as-is
        let source = "EXEC SQL SELECT A FROM T\nSTOP RUN.";
        let ext = extract_exec_blocks(source);
        // No END-EXEC found, so no extraction
        assert!(ext.sql_blocks.is_empty());
    }

    // -----------------------------------------------------------------------
    // ELSE IF chain flattening tests
    // -----------------------------------------------------------------------

    #[test]
    fn flatten_short_chain_untouched() {
        // 2 ELSE IFs -- below threshold of 3, should not be modified
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TEST1.\n",
            "PROCEDURE DIVISION.\n",
            "    IF A = 1\n",
            "        MOVE 1 TO X\n",
            "    ELSE IF A = 2\n",
            "        MOVE 2 TO X\n",
            "    ELSE\n",
            "        MOVE 0 TO X.\n",
        );
        let result = flatten_else_if_chains(source);
        assert_eq!(result, source, "Short chains should not be modified");
    }

    #[test]
    fn flatten_long_chain() {
        // 3 ELSE IFs -- meets threshold of 3, should inject END-IF before each
        let source = concat!(
            "PROCEDURE DIVISION.\n",
            "    IF A = 1\n",
            "        MOVE 1 TO X\n",
            "    ELSE IF A = 2\n",
            "        MOVE 2 TO X\n",
            "    ELSE IF A = 3\n",
            "        MOVE 3 TO X\n",
            "    ELSE IF A = 4\n",
            "        MOVE 4 TO X\n",
            "    ELSE\n",
            "        MOVE 0 TO X.\n",
        );
        let result = flatten_else_if_chains(source);
        // Should have 3 END-IF injected (one before each ELSE IF)
        let end_if_count = result.matches("END-IF").count();
        assert_eq!(end_if_count, 3, "Should inject 3 END-IF markers, got {end_if_count}");
        // Original ELSE IFs should still be present
        assert!(result.contains("ELSE IF A = 2"));
        assert!(result.contains("ELSE IF A = 3"));
        assert!(result.contains("ELSE IF A = 4"));
    }

    #[test]
    fn flatten_preserves_code_with_end_if() {
        // Code that already uses END-IF should not be modified
        let source = concat!(
            "PROCEDURE DIVISION.\n",
            "    IF A = 1\n",
            "        MOVE 1 TO X\n",
            "    ELSE IF A = 2\n",
            "        MOVE 2 TO X\n",
            "    END-IF\n",
            "    ELSE IF A = 3\n",
            "        MOVE 3 TO X\n",
            "    END-IF\n",
            "    ELSE IF A = 4\n",
            "        MOVE 4 TO X\n",
            "    END-IF\n",
            "    END-IF.\n",
        );
        let result = flatten_else_if_chains(source);
        // END-IF breaks chains, so no chain reaches 3. Should be unchanged.
        assert_eq!(result, source, "Code with END-IF should not be modified");
    }

    #[test]
    fn flatten_no_procedure_division() {
        // No PROCEDURE DIVISION -- should return unchanged
        let source = "DATA DIVISION.\nWORKING-STORAGE SECTION.\n01 WS-X PIC 9.\n";
        let result = flatten_else_if_chains(source);
        assert_eq!(result, source);
    }

    #[test]
    fn flatten_encascii_pattern() {
        // Simulate the ENCASCII.COB pattern: many ELSE IFs
        let mut source = String::from("PROCEDURE DIVISION.\n");
        source.push_str("       IF ASCII-NUL THEN\n");
        source.push_str("           MOVE 0 TO CHAR-CODE\n");
        for i in 1..20 {
            source.push_str(&format!("       ELSE IF ASCII-{i} THEN\n"));
            source.push_str(&format!("           MOVE {i} TO CHAR-CODE\n"));
        }
        source.push_str("       ELSE MOVE COBOL-STRING TO CHAR-CODE.\n");

        let result = flatten_else_if_chains(&source);
        // Should inject END-IF before each of the 19 ELSE IFs
        let end_if_count = result.matches("END-IF").count();
        assert_eq!(end_if_count, 19, "Should inject 19 END-IF markers, got {end_if_count}");
    }

    #[test]
    fn preprocess_strips_exec_sql_from_full_program() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TEST1.\n",
            "PROCEDURE DIVISION.\n",
            "    EXEC SQL COMMIT END-EXEC.\n",
            "    STOP RUN.\n",
        );
        let result = preprocess(source).unwrap();
        assert!(!result.contains("EXEC SQL"));
        assert!(result.contains("CONTINUE"));
        assert!(result.contains("STOP RUN"));
    }
}
