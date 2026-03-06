//! PIC clause parser.
//!
//! Parses raw PIC strings like "S9(5)V99", "X(10)", "ZZZ,ZZ9.99"
//! into structured `PicClause` with category, digit counts, and edit symbols.

use crate::ast::{EditSymbolEntry, PicCategory, PicClause};
use crate::error::{Result, TranspileError};

/// Parse a raw PIC string into a structured `PicClause`.
///
/// # Examples of PIC strings
///
/// - `X(10)` -- 10-byte alphanumeric
/// - `9(5)` -- 5-digit numeric (display)
/// - `S9(5)V99` -- signed, 5 integer digits + 2 decimal digits
/// - `ZZZ,ZZ9.99` -- numeric edited with zero suppression
/// - `A(20)` -- 20-byte alphabetic
///
/// # Errors
///
/// Returns `TranspileError::InvalidPic` if the clause is malformed.
pub fn parse_pic(raw: &str) -> Result<PicClause> {
    let upper = raw.trim().to_uppercase();
    if upper.is_empty() {
        return Err(TranspileError::InvalidPic {
            clause: raw.to_string(),
            reason: "empty PIC clause".to_string(),
        });
    }

    // Expand repetition notation: 9(5) -> 99999, X(10) -> XXXXXXXXXX
    let expanded = expand_repetitions(&upper)?;

    // Parse character by character
    let mut signed = false;
    let mut integer_digits: u32 = 0;
    let mut decimal_digits: u32 = 0;
    let mut after_v = false;
    let mut has_alpha = false;
    let mut has_x = false;
    let mut has_digit_9 = false;
    let mut has_edit_symbols = false;
    let mut display_length: u32 = 0;
    let mut edit_symbols: Vec<EditSymbolEntry> = Vec::new();

    // Track consecutive edit symbol runs
    let mut last_edit: Option<char> = None;
    let mut edit_count: u32 = 0;

    let flush_edit = |symbols: &mut Vec<EditSymbolEntry>, ch: Option<char>, count: u32| {
        if let Some(c) = ch {
            if count > 0 {
                symbols.push(EditSymbolEntry {
                    symbol: c,
                    count,
                });
            }
        }
    };

    let chars: Vec<char> = expanded.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        match ch {
            'S' => {
                // S must be first character
                if i == 0 {
                    signed = true;
                } else {
                    return Err(TranspileError::InvalidPic {
                        clause: raw.to_string(),
                        reason: format!("S must be first character, found at position {i}"),
                    });
                }
            }
            'V' => {
                // Implied decimal point
                if after_v {
                    return Err(TranspileError::InvalidPic {
                        clause: raw.to_string(),
                        reason: "duplicate V in PIC clause".to_string(),
                    });
                }
                after_v = true;
                // V does not contribute to display length
            }
            '9' => {
                has_digit_9 = true;
                if after_v {
                    decimal_digits += 1;
                } else {
                    integer_digits += 1;
                }
                display_length += 1;
            }
            'A' => {
                has_alpha = true;
                display_length += 1;
            }
            'X' => {
                has_x = true;
                display_length += 1;
            }
            'Z' => {
                has_edit_symbols = true;
                has_digit_9 = true; // Z represents a digit position
                if after_v {
                    decimal_digits += 1;
                } else {
                    integer_digits += 1;
                }
                display_length += 1;
                if last_edit == Some('Z') {
                    edit_count += 1;
                } else {
                    flush_edit(&mut edit_symbols, last_edit, edit_count);
                    last_edit = Some('Z');
                    edit_count = 1;
                }
            }
            '*' => {
                has_edit_symbols = true;
                has_digit_9 = true;
                if after_v {
                    decimal_digits += 1;
                } else {
                    integer_digits += 1;
                }
                display_length += 1;
                if last_edit == Some('*') {
                    edit_count += 1;
                } else {
                    flush_edit(&mut edit_symbols, last_edit, edit_count);
                    last_edit = Some('*');
                    edit_count = 1;
                }
            }
            '$' => {
                has_edit_symbols = true;
                display_length += 1;
                if last_edit == Some('$') {
                    // Floating $ -- each additional $ is a digit position
                    edit_count += 1;
                    if after_v {
                        decimal_digits += 1;
                    } else {
                        integer_digits += 1;
                    }
                } else {
                    flush_edit(&mut edit_symbols, last_edit, edit_count);
                    last_edit = Some('$');
                    edit_count = 1;
                    // First $ is fixed insertion, not a digit position
                }
            }
            '+' => {
                has_edit_symbols = true;
                display_length += 1;
                if last_edit == Some('+') {
                    edit_count += 1;
                    if after_v {
                        decimal_digits += 1;
                    } else {
                        integer_digits += 1;
                    }
                } else {
                    flush_edit(&mut edit_symbols, last_edit, edit_count);
                    last_edit = Some('+');
                    edit_count = 1;
                }
            }
            '-' => {
                has_edit_symbols = true;
                display_length += 1;
                if last_edit == Some('-') {
                    edit_count += 1;
                    if after_v {
                        decimal_digits += 1;
                    } else {
                        integer_digits += 1;
                    }
                } else {
                    flush_edit(&mut edit_symbols, last_edit, edit_count);
                    last_edit = Some('-');
                    edit_count = 1;
                }
            }
            '.' => {
                // Decimal point (editing symbol, also marks decimal position)
                has_edit_symbols = true;
                display_length += 1;
                after_v = true; // Actual decimal point implies V
                flush_edit(&mut edit_symbols, last_edit, edit_count);
                last_edit = Some('.');
                edit_count = 1;
            }
            ',' => {
                // Comma insertion
                has_edit_symbols = true;
                display_length += 1;
                flush_edit(&mut edit_symbols, last_edit, edit_count);
                last_edit = Some(',');
                edit_count = 1;
            }
            'B' => {
                // Blank insertion
                has_edit_symbols = true;
                display_length += 1;
                flush_edit(&mut edit_symbols, last_edit, edit_count);
                last_edit = Some('B');
                edit_count = 1;
            }
            '0' => {
                // Zero insertion
                has_edit_symbols = true;
                display_length += 1;
                flush_edit(&mut edit_symbols, last_edit, edit_count);
                last_edit = Some('0');
                edit_count = 1;
            }
            '/' => {
                // Slash insertion
                has_edit_symbols = true;
                display_length += 1;
                flush_edit(&mut edit_symbols, last_edit, edit_count);
                last_edit = Some('/');
                edit_count = 1;
            }
            'C' => {
                // CR (credit)
                if i + 1 < chars.len() && chars[i + 1] == 'R' {
                    has_edit_symbols = true;
                    display_length += 2;
                    flush_edit(&mut edit_symbols, last_edit, edit_count);
                    edit_symbols.push(EditSymbolEntry {
                        symbol: 'C',
                        count: 1,
                    });
                    last_edit = None;
                    edit_count = 0;
                    i += 1; // Skip R
                }
            }
            'D' => {
                // DB (debit)
                if i + 1 < chars.len() && chars[i + 1] == 'B' {
                    has_edit_symbols = true;
                    display_length += 2;
                    flush_edit(&mut edit_symbols, last_edit, edit_count);
                    edit_symbols.push(EditSymbolEntry {
                        symbol: 'D',
                        count: 1,
                    });
                    last_edit = None;
                    edit_count = 0;
                    i += 1; // Skip B
                }
            }
            'P' => {
                // P-scaling (assumed decimal positions)
                // P before V: positions to the right
                // P after V: positions to the left
                if after_v {
                    decimal_digits += 1;
                } else {
                    integer_digits += 1;
                }
                // P does not contribute to display length
            }
            _ => {
                return Err(TranspileError::InvalidPic {
                    clause: raw.to_string(),
                    reason: format!("unexpected character '{ch}' in PIC clause"),
                });
            }
        }
        i += 1;
    }

    // Flush remaining edit symbols
    flush_edit(&mut edit_symbols, last_edit, edit_count);

    // Determine category
    let category = determine_category(has_alpha, has_x, has_digit_9, has_edit_symbols);

    let total_digits = integer_digits + decimal_digits;

    // For non-numeric types, display_length is the byte count
    // For signed display numeric, add 1 for sign if SIGN SEPARATE
    // (handled at DataEntry level, not here)

    Ok(PicClause {
        raw: raw.to_string(),
        category,
        total_digits,
        scale: decimal_digits,
        signed,
        display_length,
        edit_symbols,
    })
}

/// Expand repetition notation in a PIC string.
///
/// `9(5)` -> `99999`, `X(10)` -> `XXXXXXXXXX`, etc.
fn expand_repetitions(pic: &str) -> Result<String> {
    let mut result = String::with_capacity(pic.len());
    let chars: Vec<char> = pic.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if i + 1 < chars.len() && chars[i + 1] == '(' {
            // Look for closing paren and extract count
            if let Some(close) = chars[i + 2..].iter().position(|&c| c == ')') {
                let count_str: String = chars[i + 2..i + 2 + close].iter().collect();
                let count: u32 = count_str.parse().map_err(|_| TranspileError::InvalidPic {
                    clause: pic.to_string(),
                    reason: format!("invalid repetition count '{count_str}'"),
                })?;
                for _ in 0..count {
                    result.push(ch);
                }
                i = i + 2 + close + 1; // Skip past ')'
            } else {
                return Err(TranspileError::InvalidPic {
                    clause: pic.to_string(),
                    reason: "unclosed parenthesis in PIC clause".to_string(),
                });
            }
        } else {
            result.push(ch);
            i += 1;
        }
    }

    Ok(result)
}

/// Determine PIC category from character types present.
fn determine_category(
    has_alpha: bool,
    has_x: bool,
    has_digit_9: bool,
    has_edit_symbols: bool,
) -> PicCategory {
    if has_edit_symbols && has_digit_9 {
        return PicCategory::NumericEdited;
    }
    if has_edit_symbols && (has_alpha || has_x) {
        return PicCategory::AlphanumericEdited;
    }
    if has_alpha && !has_x && !has_digit_9 {
        return PicCategory::Alphabetic;
    }
    if has_digit_9 && !has_alpha && !has_x {
        return PicCategory::Numeric;
    }
    if has_x || (has_alpha && has_digit_9) {
        return PicCategory::Alphanumeric;
    }
    // Default to alphanumeric for anything else
    PicCategory::Alphanumeric
}

/// Compute the storage size in bytes for a PIC + USAGE combination.
///
/// Used during layout computation.
pub fn compute_storage_size(pic: &PicClause, usage: crate::ast::Usage) -> u32 {
    match usage {
        crate::ast::Usage::Display => {
            // Display: 1 byte per character position
            pic.display_length
        }
        crate::ast::Usage::Comp3 => {
            // COMP-3: (digits / 2) + 1 bytes
            (pic.total_digits / 2) + 1
        }
        crate::ast::Usage::Comp | crate::ast::Usage::Comp5 => {
            // COMP/COMP-5: storage based on digit count
            match pic.total_digits {
                0..=4 => 2,  // halfword
                5..=9 => 4,  // fullword
                _ => 8,      // doubleword
            }
        }
        crate::ast::Usage::Comp1 => 4,  // single-precision float
        crate::ast::Usage::Comp2 => 8,  // double-precision float
        crate::ast::Usage::Index => 4,  // index
        crate::ast::Usage::Pointer => 8, // pointer (64-bit)
    }
}

/// Build an alpha edit pattern from an AlphanumericEdited PIC clause.
///
/// Returns a vector of pattern characters where:
/// - 'X' = data position (source character pass-through)
/// - 'B' = space insertion
/// - '0' = zero insertion
/// - '/' = slash insertion
///
/// Codegen uses this to emit `AlphaEditSymbol` constructors in the generated code.
///
/// Only valid for `PicCategory::AlphanumericEdited` clauses. Returns `None`
/// if the clause is not an alphanumeric-edited type.
pub fn build_alpha_edit_pattern(pic: &PicClause) -> Option<Vec<char>> {
    if pic.category != PicCategory::AlphanumericEdited {
        return None;
    }

    let upper = pic.raw.trim().to_uppercase();
    let expanded = expand_repetitions(&upper).ok()?;

    let pattern: Vec<char> = expanded
        .chars()
        .filter_map(|ch| match ch {
            'X' | 'A' | '9' => Some('X'), // All data positions normalized to 'X'
            'B' => Some('B'),
            '0' => Some('0'),
            '/' => Some('/'),
            _ => None, // Skip S, V, and other non-position chars
        })
        .collect();

    Some(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_simple() {
        assert_eq!(expand_repetitions("9(5)").unwrap(), "99999");
        assert_eq!(expand_repetitions("X(3)").unwrap(), "XXX");
    }

    #[test]
    fn expand_mixed() {
        assert_eq!(expand_repetitions("S9(5)V9(2)").unwrap(), "S99999V99");
    }

    #[test]
    fn expand_no_parens() {
        assert_eq!(expand_repetitions("99V99").unwrap(), "99V99");
    }

    #[test]
    fn parse_alphanumeric() {
        let pic = parse_pic("X(10)").unwrap();
        assert_eq!(pic.category, PicCategory::Alphanumeric);
        assert_eq!(pic.display_length, 10);
        assert_eq!(pic.total_digits, 0);
        assert!(!pic.signed);
    }

    #[test]
    fn parse_alphabetic() {
        let pic = parse_pic("A(20)").unwrap();
        assert_eq!(pic.category, PicCategory::Alphabetic);
        assert_eq!(pic.display_length, 20);
    }

    #[test]
    fn parse_numeric_unsigned() {
        let pic = parse_pic("9(5)").unwrap();
        assert_eq!(pic.category, PicCategory::Numeric);
        assert_eq!(pic.total_digits, 5);
        assert_eq!(pic.scale, 0);
        assert!(!pic.signed);
        assert_eq!(pic.display_length, 5);
    }

    #[test]
    fn parse_numeric_signed_with_decimal() {
        let pic = parse_pic("S9(5)V99").unwrap();
        assert_eq!(pic.category, PicCategory::Numeric);
        assert_eq!(pic.total_digits, 7);
        assert_eq!(pic.scale, 2);
        assert!(pic.signed);
        assert_eq!(pic.display_length, 7); // S and V don't add to display
    }

    #[test]
    fn parse_numeric_edited() {
        let pic = parse_pic("ZZZ,ZZ9.99").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
        assert_eq!(pic.total_digits, 8); // 5 Zs + 1 nine + 2 nines
        assert_eq!(pic.scale, 2);
        assert!(!pic.edit_symbols.is_empty());
    }

    #[test]
    fn parse_zero_suppress() {
        let pic = parse_pic("Z(4)9").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
        assert_eq!(pic.total_digits, 5); // 4 Zs + 1 nine
        assert_eq!(pic.display_length, 5);
    }

    #[test]
    fn parse_cr_db() {
        let pic = parse_pic("9(5).99CR").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
        assert_eq!(pic.display_length, 10); // 5 + 1(.) + 2 + 2(CR)
    }

    #[test]
    fn parse_empty_fails() {
        assert!(parse_pic("").is_err());
    }

    #[test]
    fn parse_unclosed_paren_fails() {
        assert!(parse_pic("9(5").is_err());
    }

    #[test]
    fn compute_storage_display() {
        let pic = parse_pic("S9(5)V99").unwrap();
        assert_eq!(compute_storage_size(&pic, crate::ast::Usage::Display), 7);
    }

    #[test]
    fn compute_storage_comp3() {
        let pic = parse_pic("S9(5)V99").unwrap();
        assert_eq!(compute_storage_size(&pic, crate::ast::Usage::Comp3), 4); // 7/2 + 1 = 4
    }

    #[test]
    fn compute_storage_comp() {
        let pic4 = parse_pic("9(4)").unwrap();
        assert_eq!(compute_storage_size(&pic4, crate::ast::Usage::Comp), 2);

        let pic9 = parse_pic("9(9)").unwrap();
        assert_eq!(compute_storage_size(&pic9, crate::ast::Usage::Comp), 4);

        let pic18 = parse_pic("9(18)").unwrap();
        assert_eq!(compute_storage_size(&pic18, crate::ast::Usage::Comp), 8);
    }

    // --- Edge case tests ---

    #[test]
    fn parse_invalid_character_fails() {
        assert!(parse_pic("9(3)Q").is_err());
    }

    #[test]
    fn parse_invalid_repetition_count_fails() {
        assert!(parse_pic("9(abc)").is_err());
    }

    #[test]
    fn parse_single_digit() {
        let pic = parse_pic("9").unwrap();
        assert_eq!(pic.category, PicCategory::Numeric);
        assert_eq!(pic.total_digits, 1);
        assert_eq!(pic.display_length, 1);
    }

    #[test]
    fn parse_single_x() {
        let pic = parse_pic("X").unwrap();
        assert_eq!(pic.category, PicCategory::Alphanumeric);
        assert_eq!(pic.display_length, 1);
    }

    #[test]
    fn parse_p_scaling_before_v() {
        // PP999 means 2 assumed decimal positions before the digits
        let pic = parse_pic("PP999").unwrap();
        assert_eq!(pic.category, PicCategory::Numeric);
        assert_eq!(pic.total_digits, 5); // 2 Ps (as integer digits) + 3 nines
    }

    #[test]
    fn parse_p_scaling_after_v() {
        // 999VPP means 2 assumed decimal positions after V
        let pic = parse_pic("999VPP").unwrap();
        assert_eq!(pic.category, PicCategory::Numeric);
        assert_eq!(pic.total_digits, 5); // 3 nines + 2 Ps (as decimal digits)
        assert_eq!(pic.scale, 2);
    }

    #[test]
    fn parse_asterisk_check_protect() {
        let pic = parse_pic("**,**9.99").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
        assert!(!pic.edit_symbols.is_empty());
    }

    #[test]
    fn parse_dollar_sign_fixed() {
        let pic = parse_pic("$9(5).99").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
        assert_eq!(pic.display_length, 9); // $ + 5 nines + . + 2 nines
    }

    #[test]
    fn parse_plus_sign_edited() {
        let pic = parse_pic("+9(3).99").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
    }

    #[test]
    fn parse_minus_sign_edited() {
        let pic = parse_pic("-9(3).99").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
    }

    #[test]
    fn parse_db_suffix() {
        let pic = parse_pic("9(5).99DB").unwrap();
        assert_eq!(pic.category, PicCategory::NumericEdited);
        assert_eq!(pic.display_length, 10); // 5 + . + 2 + DB(2)
    }

    #[test]
    fn parse_large_repetition() {
        let pic = parse_pic("X(200)").unwrap();
        assert_eq!(pic.category, PicCategory::Alphanumeric);
        assert_eq!(pic.display_length, 200);
    }

    #[test]
    fn parse_mixed_no_parens() {
        let pic = parse_pic("99V99").unwrap();
        assert_eq!(pic.category, PicCategory::Numeric);
        assert_eq!(pic.total_digits, 4);
        assert_eq!(pic.scale, 2);
        assert_eq!(pic.display_length, 4);
    }

    #[test]
    fn parse_sign_only_at_start() {
        let pic = parse_pic("S9(7)V99").unwrap();
        assert!(pic.signed);
        assert_eq!(pic.total_digits, 9);
        assert_eq!(pic.scale, 2);
    }

    #[test]
    fn expand_nested_repetitions() {
        assert_eq!(expand_repetitions("9(2)V9(3)").unwrap(), "99V999");
    }

    #[test]
    fn alpha_edit_pattern_space() {
        // PIC X(3)BX(3) -> XXXBXXX
        let pic = parse_pic("X(3)BX(3)").unwrap();
        assert_eq!(pic.category, PicCategory::AlphanumericEdited);
        let pat = build_alpha_edit_pattern(&pic).unwrap();
        assert_eq!(pat, vec!['X', 'X', 'X', 'B', 'X', 'X', 'X']);
    }

    #[test]
    fn alpha_edit_pattern_slash() {
        // PIC X(2)/X(2)/X(4) -> date format
        let pic = parse_pic("X(2)/X(2)/X(4)").unwrap();
        assert_eq!(pic.category, PicCategory::AlphanumericEdited);
        let pat = build_alpha_edit_pattern(&pic).unwrap();
        assert_eq!(pat, vec!['X', 'X', '/', 'X', 'X', '/', 'X', 'X', 'X', 'X']);
    }

    #[test]
    fn alpha_edit_pattern_zero() {
        // PIC X(3)0X(3) -> zero insertion
        let pic = parse_pic("X(3)0X(3)").unwrap();
        assert_eq!(pic.category, PicCategory::AlphanumericEdited);
        let pat = build_alpha_edit_pattern(&pic).unwrap();
        assert_eq!(pat, vec!['X', 'X', 'X', '0', 'X', 'X', 'X']);
    }

    #[test]
    fn alpha_edit_pattern_mixed() {
        // PIC X(5)BB -> mixed with trailing spaces
        let pic = parse_pic("X(5)BB").unwrap();
        assert_eq!(pic.category, PicCategory::AlphanumericEdited);
        let pat = build_alpha_edit_pattern(&pic).unwrap();
        assert_eq!(pat, vec!['X', 'X', 'X', 'X', 'X', 'B', 'B']);
    }

    #[test]
    fn alpha_edit_pattern_non_edited_returns_none() {
        // PIC X(10) is plain alphanumeric, not edited
        let pic = parse_pic("X(10)").unwrap();
        assert_eq!(pic.category, PicCategory::Alphanumeric);
        assert!(build_alpha_edit_pattern(&pic).is_none());
    }
}
