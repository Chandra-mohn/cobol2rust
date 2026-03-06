//! INSPECT verb runtime implementation.
//!
//! Supports all three INSPECT forms:
//! - TALLYING: count occurrences of patterns in the target
//! - REPLACING: replace occurrences of patterns in the target
//! - CONVERTING: character-by-character translation (like `tr`)
//!
//! All forms support BEFORE/AFTER INITIAL qualifiers to restrict
//! the scanning range within the target.

// ---------------------------------------------------------------------------
// Specification types
// ---------------------------------------------------------------------------

/// What to count or replace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectWhat {
    /// CHARACTERS -- every byte position.
    Characters,
    /// ALL pattern -- every non-overlapping occurrence.
    All(Vec<u8>),
    /// LEADING pattern -- consecutive leading occurrences only.
    Leading(Vec<u8>),
    /// FIRST pattern -- first occurrence only.
    First(Vec<u8>),
}

/// BEFORE/AFTER INITIAL qualifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeforeAfter {
    /// true = BEFORE INITIAL, false = AFTER INITIAL.
    pub is_before: bool,
    /// The delimiter pattern to search for.
    pub value: Vec<u8>,
}

/// Specification for one TALLYING clause.
#[derive(Debug, Clone)]
pub struct TallyingSpec {
    /// What to count.
    pub what: InspectWhat,
    /// BEFORE/AFTER INITIAL qualifiers (0, 1, or 2).
    pub bounds: Vec<BeforeAfter>,
}

/// Specification for one REPLACING clause.
#[derive(Debug, Clone)]
pub struct ReplacingSpec {
    /// What to replace.
    pub what: InspectWhat,
    /// Replacement bytes.
    pub by: Vec<u8>,
    /// BEFORE/AFTER INITIAL qualifiers.
    pub bounds: Vec<BeforeAfter>,
}

// ---------------------------------------------------------------------------
// Scanning range computation
// ---------------------------------------------------------------------------

/// Compute the effective scanning range [start, end) within `target`
/// based on BEFORE/AFTER INITIAL qualifiers.
fn compute_range(target: &[u8], bounds: &[BeforeAfter]) -> (usize, usize) {
    let mut start = 0usize;
    let mut end = target.len();

    for bound in bounds {
        if let Some(pos) = find_substring(target, &bound.value) {
            if bound.is_before {
                // BEFORE INITIAL: scan only up to the delimiter position
                end = end.min(pos);
            } else {
                // AFTER INITIAL: scan only after the delimiter
                start = start.max(pos + bound.value.len());
            }
        } else {
            // Delimiter not found:
            // BEFORE INITIAL with no match -> scan entire range (no restriction)
            // AFTER INITIAL with no match -> scan nothing
            if !bound.is_before {
                return (0, 0); // empty range
            }
        }
    }

    if start > end {
        (0, 0)
    } else {
        (start, end)
    }
}

/// Find the first occurrence of `needle` in `haystack`.
fn find_substring(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}

// ---------------------------------------------------------------------------
// TALLYING
// ---------------------------------------------------------------------------

/// Execute INSPECT TALLYING.
///
/// For each `TallyingSpec`, counts occurrences in `target` and returns the
/// count. The caller adds these counts to the appropriate COBOL counter fields.
///
/// Returns a vector of counts, one per spec (in order).
#[must_use]
pub fn cobol_inspect_tallying(target: &[u8], specs: &[TallyingSpec]) -> Vec<usize> {
    specs.iter().map(|spec| tally_one(target, spec)).collect()
}

/// Count for a single tallying spec.
fn tally_one(target: &[u8], spec: &TallyingSpec) -> usize {
    let (start, end) = compute_range(target, &spec.bounds);
    if start >= end {
        return 0;
    }
    let region = &target[start..end];

    match &spec.what {
        InspectWhat::Characters => region.len(),
        InspectWhat::All(pattern) => count_all(region, pattern),
        InspectWhat::Leading(pattern) => count_leading(region, pattern),
        InspectWhat::First(pattern) => {
            usize::from(find_substring(region, pattern).is_some())
        }
    }
}

/// Count all non-overlapping occurrences of `pattern` in `region`.
fn count_all(region: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut pos = 0;
    while pos + pattern.len() <= region.len() {
        if &region[pos..pos + pattern.len()] == pattern {
            count += 1;
            pos += pattern.len(); // non-overlapping
        } else {
            pos += 1;
        }
    }
    count
}

/// Count consecutive leading occurrences of `pattern` at the start of `region`.
fn count_leading(region: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut pos = 0;
    while pos + pattern.len() <= region.len() {
        if &region[pos..pos + pattern.len()] == pattern {
            count += 1;
            pos += pattern.len();
        } else {
            break; // not leading anymore
        }
    }
    count
}

// ---------------------------------------------------------------------------
// REPLACING
// ---------------------------------------------------------------------------

/// Execute INSPECT REPLACING.
///
/// Modifies `target` in-place according to the replacing specs.
/// Specs are applied in order; each byte position is only affected by the
/// first matching spec (COBOL rule: first match wins, left to right).
pub fn cobol_inspect_replacing(target: &mut [u8], specs: &[ReplacingSpec]) {
    // Track which positions have been replaced (to avoid double-replace)
    let mut replaced = vec![false; target.len()];

    for spec in specs {
        let (start, end) = compute_range(target, &spec.bounds);
        if start >= end {
            continue;
        }

        match &spec.what {
            InspectWhat::Characters => {
                // CHARACTERS BY: replace each unreplaced character in range
                if spec.by.is_empty() {
                    continue;
                }
                let replacement_byte = spec.by[0];
                for i in start..end {
                    if !replaced[i] {
                        target[i] = replacement_byte;
                        replaced[i] = true;
                    }
                }
            }
            InspectWhat::All(pattern) => {
                replace_pattern(target, &mut replaced, pattern, &spec.by, start, end, ReplaceMode::All);
            }
            InspectWhat::Leading(pattern) => {
                replace_pattern(target, &mut replaced, pattern, &spec.by, start, end, ReplaceMode::Leading);
            }
            InspectWhat::First(pattern) => {
                replace_pattern(target, &mut replaced, pattern, &spec.by, start, end, ReplaceMode::First);
            }
        }
    }
}

/// How many occurrences to replace.
#[derive(Clone, Copy)]
enum ReplaceMode {
    /// Replace all occurrences.
    All,
    /// Replace consecutive leading occurrences only.
    Leading,
    /// Replace only the first occurrence.
    First,
}

/// Replace pattern occurrences in `target[start..end]`.
fn replace_pattern(
    target: &mut [u8],
    replaced: &mut [bool],
    pattern: &[u8],
    replacement: &[u8],
    start: usize,
    end: usize,
    mode: ReplaceMode,
) {
    if pattern.is_empty() || replacement.is_empty() {
        return;
    }
    // For REPLACING ALL/LEADING/FIRST: pattern and replacement must be same length
    // (COBOL requirement for non-CHARACTERS replacing)
    let rep_len = pattern.len().min(replacement.len());

    let mut pos = start;
    while pos + pattern.len() <= end {
        // Check if position is already replaced
        let already = replaced[pos..pos + pattern.len()].iter().any(|&r| r);
        if already {
            pos += 1;
            continue;
        }

        if &target[pos..pos + pattern.len()] == pattern {
            // Replace
            target[pos..pos + rep_len].copy_from_slice(&replacement[..rep_len]);
            for flag in &mut replaced[pos..pos + pattern.len()] {
                *flag = true;
            }

            if matches!(mode, ReplaceMode::First) {
                return;
            }

            pos += pattern.len();
        } else {
            if matches!(mode, ReplaceMode::Leading) {
                return; // stop at first non-match
            }
            pos += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// CONVERTING
// ---------------------------------------------------------------------------

/// Execute INSPECT CONVERTING.
///
/// Builds a 256-byte translation table from `from` -> `to` (positional 1:1 mapping)
/// and applies it to each byte in the target within the BEFORE/AFTER range.
///
/// `from` and `to` must be the same length.
pub fn cobol_inspect_converting(
    target: &mut [u8],
    from: &[u8],
    to: &[u8],
    bounds: &[BeforeAfter],
) {
    if from.len() != to.len() || from.is_empty() {
        return;
    }

    // Build translation table (identity by default)
    let mut table = [0u8; 256];
    for (i, slot) in table.iter_mut().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        {
            *slot = i as u8;
        }
    }
    for (&f, &t) in from.iter().zip(to.iter()) {
        table[f as usize] = t;
    }

    let (start, end) = compute_range(target, bounds);

    for byte in &mut target[start..end] {
        *byte = table[*byte as usize];
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- TALLYING ----

    #[test]
    fn test_tally_characters_all() {
        let target = b"HELLO WORLD";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Characters,
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 11);
    }

    #[test]
    fn test_tally_characters_before_initial() {
        let target = b"HELLO WORLD";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Characters,
            bounds: vec![BeforeAfter {
                is_before: true,
                value: b" ".to_vec(),
            }],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 5); // "HELLO"
    }

    #[test]
    fn test_tally_characters_after_initial() {
        let target = b"HELLO WORLD";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Characters,
            bounds: vec![BeforeAfter {
                is_before: false,
                value: b" ".to_vec(),
            }],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 5); // "WORLD"
    }

    #[test]
    fn test_tally_all_pattern() {
        let target = b"ABCABCABC";
        let specs = vec![TallyingSpec {
            what: InspectWhat::All(b"ABC".to_vec()),
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 3);
    }

    #[test]
    fn test_tally_all_single_char() {
        let target = b"BANANA";
        let specs = vec![TallyingSpec {
            what: InspectWhat::All(b"A".to_vec()),
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 3);
    }

    #[test]
    fn test_tally_leading_zeros() {
        let target = b"00012300";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Leading(b"0".to_vec()),
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 3); // three leading zeros
    }

    #[test]
    fn test_tally_leading_no_leading() {
        let target = b"12300";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Leading(b"0".to_vec()),
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 0);
    }

    #[test]
    fn test_tally_first_found() {
        let target = b"ABCABC";
        let specs = vec![TallyingSpec {
            what: InspectWhat::First(b"BC".to_vec()),
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 1);
    }

    #[test]
    fn test_tally_first_not_found() {
        let target = b"ABCABC";
        let specs = vec![TallyingSpec {
            what: InspectWhat::First(b"XY".to_vec()),
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 0);
    }

    #[test]
    fn test_tally_multiple_specs() {
        let target = b"00ABC00";
        let specs = vec![
            TallyingSpec {
                what: InspectWhat::Leading(b"0".to_vec()),
                bounds: vec![],
            },
            TallyingSpec {
                what: InspectWhat::All(b"0".to_vec()),
                bounds: vec![],
            },
        ];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 2); // leading zeros
        assert_eq!(counts[1], 4); // all zeros
    }

    #[test]
    fn test_tally_empty_target() {
        let target: &[u8] = b"";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Characters,
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 0);
    }

    #[test]
    fn test_tally_pattern_longer_than_target() {
        let target = b"AB";
        let specs = vec![TallyingSpec {
            what: InspectWhat::All(b"ABCDE".to_vec()),
            bounds: vec![],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 0);
    }

    #[test]
    fn test_tally_after_initial_not_found() {
        // AFTER INITIAL with no match -> scan nothing
        let target = b"HELLO";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Characters,
            bounds: vec![BeforeAfter {
                is_before: false,
                value: b"X".to_vec(),
            }],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 0);
    }

    #[test]
    fn test_tally_before_initial_not_found() {
        // BEFORE INITIAL with no match -> scan entire range
        let target = b"HELLO";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Characters,
            bounds: vec![BeforeAfter {
                is_before: true,
                value: b"X".to_vec(),
            }],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        assert_eq!(counts[0], 5);
    }

    // ---- REPLACING ----

    #[test]
    fn test_replace_characters_by() {
        let mut target = b"HELLO".to_vec();
        let specs = vec![ReplacingSpec {
            what: InspectWhat::Characters,
            by: b"*".to_vec(),
            bounds: vec![],
        }];
        cobol_inspect_replacing(&mut target, &specs);
        assert_eq!(target, b"*****");
    }

    #[test]
    fn test_replace_all_pattern() {
        let mut target = b"ABCABCABC".to_vec();
        let specs = vec![ReplacingSpec {
            what: InspectWhat::All(b"ABC".to_vec()),
            by: b"XYZ".to_vec(),
            bounds: vec![],
        }];
        cobol_inspect_replacing(&mut target, &specs);
        assert_eq!(target, b"XYZXYZXYZ");
    }

    #[test]
    fn test_replace_leading_zeros() {
        let mut target = b"000123".to_vec();
        let specs = vec![ReplacingSpec {
            what: InspectWhat::Leading(b"0".to_vec()),
            by: b" ".to_vec(),
            bounds: vec![],
        }];
        cobol_inspect_replacing(&mut target, &specs);
        assert_eq!(target, b"   123");
    }

    #[test]
    fn test_replace_first_only() {
        let mut target = b"ABCABCABC".to_vec();
        let specs = vec![ReplacingSpec {
            what: InspectWhat::First(b"ABC".to_vec()),
            by: b"XYZ".to_vec(),
            bounds: vec![],
        }];
        cobol_inspect_replacing(&mut target, &specs);
        assert_eq!(target, b"XYZABCABC");
    }

    #[test]
    fn test_replace_with_before_initial() {
        let mut target = b"000.123".to_vec();
        let specs = vec![ReplacingSpec {
            what: InspectWhat::Leading(b"0".to_vec()),
            by: b" ".to_vec(),
            bounds: vec![BeforeAfter {
                is_before: true,
                value: b".".to_vec(),
            }],
        }];
        cobol_inspect_replacing(&mut target, &specs);
        assert_eq!(target, b"   .123");
    }

    #[test]
    fn test_replace_no_match() {
        let mut target = b"HELLO".to_vec();
        let specs = vec![ReplacingSpec {
            what: InspectWhat::All(b"XYZ".to_vec()),
            by: b"ABC".to_vec(),
            bounds: vec![],
        }];
        cobol_inspect_replacing(&mut target, &specs);
        assert_eq!(target, b"HELLO");
    }

    #[test]
    fn test_replace_characters_with_after() {
        let mut target = b"HELLO WORLD".to_vec();
        let specs = vec![ReplacingSpec {
            what: InspectWhat::Characters,
            by: b"*".to_vec(),
            bounds: vec![BeforeAfter {
                is_before: false,
                value: b" ".to_vec(),
            }],
        }];
        cobol_inspect_replacing(&mut target, &specs);
        assert_eq!(target, b"HELLO *****");
    }

    // ---- CONVERTING ----

    #[test]
    fn test_converting_lowercase_to_uppercase() {
        let mut target = b"Hello World".to_vec();
        let from = b"abcdefghijklmnopqrstuvwxyz";
        let to = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        cobol_inspect_converting(&mut target, from, to, &[]);
        assert_eq!(target, b"HELLO WORLD");
    }

    #[test]
    fn test_converting_digits_to_stars() {
        let mut target = b"ABC123DEF".to_vec();
        let from = b"0123456789";
        let to = b"**********";
        cobol_inspect_converting(&mut target, from, to, &[]);
        assert_eq!(target, b"ABC***DEF");
    }

    #[test]
    fn test_converting_with_before_initial() {
        let mut target = b"aaa.bbb".to_vec();
        let from = b"abcdefghijklmnopqrstuvwxyz";
        let to = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let bounds = vec![BeforeAfter {
            is_before: true,
            value: b".".to_vec(),
        }];
        cobol_inspect_converting(&mut target, from, to, &bounds);
        assert_eq!(target, b"AAA.bbb"); // only before the dot
    }

    #[test]
    fn test_converting_with_after_initial() {
        let mut target = b"aaa.bbb".to_vec();
        let from = b"abcdefghijklmnopqrstuvwxyz";
        let to = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let bounds = vec![BeforeAfter {
            is_before: false,
            value: b".".to_vec(),
        }];
        cobol_inspect_converting(&mut target, from, to, &bounds);
        assert_eq!(target, b"aaa.BBB"); // only after the dot
    }

    #[test]
    fn test_converting_no_match_chars() {
        let mut target = b"HELLO".to_vec();
        let from = b"xyz";
        let to = b"XYZ";
        cobol_inspect_converting(&mut target, from, to, &[]);
        assert_eq!(target, b"HELLO"); // no change
    }

    #[test]
    fn test_converting_empty_from_to() {
        let mut target = b"HELLO".to_vec();
        cobol_inspect_converting(&mut target, b"", b"", &[]);
        assert_eq!(target, b"HELLO"); // no change
    }

    #[test]
    fn test_converting_mismatched_lengths() {
        let mut target = b"HELLO".to_vec();
        // from and to different lengths -> no conversion
        cobol_inspect_converting(&mut target, b"HE", b"X", &[]);
        assert_eq!(target, b"HELLO");
    }

    // ---- Combined / edge cases ----

    #[test]
    fn test_tally_before_and_after() {
        // BEFORE "." AND AFTER " "
        let target = b"XX YY.ZZ";
        let specs = vec![TallyingSpec {
            what: InspectWhat::Characters,
            bounds: vec![
                BeforeAfter {
                    is_before: false,
                    value: b" ".to_vec(),
                },
                BeforeAfter {
                    is_before: true,
                    value: b".".to_vec(),
                },
            ],
        }];
        let counts = cobol_inspect_tallying(target, &specs);
        // AFTER " " -> start at 3, BEFORE "." -> end at 5
        // region = "YY" = 2 characters
        assert_eq!(counts[0], 2);
    }

    #[test]
    fn test_replace_multiple_specs() {
        let mut target = b"00ABC00".to_vec();
        let specs = vec![
            ReplacingSpec {
                what: InspectWhat::Leading(b"0".to_vec()),
                by: b" ".to_vec(),
                bounds: vec![],
            },
            ReplacingSpec {
                what: InspectWhat::All(b"0".to_vec()),
                by: b"*".to_vec(),
                bounds: vec![],
            },
        ];
        cobol_inspect_replacing(&mut target, &specs);
        // Leading zeros -> spaces, then remaining zeros -> stars
        assert_eq!(target, b"  ABC**");
    }
}
