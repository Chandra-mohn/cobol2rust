//! EBCDIC-to-ASCII and ASCII-to-EBCDIC translation tables.
//!
//! Supports three IBM code pages commonly used on z/OS mainframes:
//! - **CP037** (US/Canada) -- the most common EBCDIC code page
//! - **CP1140** (Euro variant of CP037) -- identical except position 0x9F
//! - **CP500** (International) -- used in Belgium, Switzerland, French-Canada
//!
//! Tables are sourced from the Unicode Consortium's official mapping files:
//! <https://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/EBCDIC/>
//!
//! ## Design Notes
//!
//! EBCDIC bytes map to Unicode code points, many of which are above U+007F
//! (accented characters, special symbols). For the COBOL transpiler, we only
//! need lossless roundtrip of the printable COBOL character set (digits,
//! uppercase/lowercase letters, common symbols, space). Characters with
//! Unicode code points > 0xFF are represented as their low byte.
//!
//! Key collating sequence difference:
//! - EBCDIC: space < special < lowercase < uppercase < digits
//! - ASCII:  space < special < digits < uppercase < lowercase

/// Supported EBCDIC code pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum CodePage {
    /// IBM CP037 -- US/Canada, most common mainframe code page.
    #[default]
    CP037,
    /// IBM CP1140 -- Euro variant of CP037 (0x9F = Euro sign instead of
    /// currency sign). All other positions identical to CP037.
    CP1140,
    /// IBM CP500 -- International (Belgium, Switzerland, French-Canada).
    CP500,
}


// ---------------------------------------------------------------------------
// CP037 EBCDIC -> Unicode/ASCII mapping (256 entries)
// Source: https://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/EBCDIC/CP037.TXT
//
// Each entry: CP037_TO_UNICODE[ebcdic_byte] = unicode_code_point (truncated to u8
// for code points <= 0xFF, or substitution char 0x1A for unmapped/control).
//
// For code points in 0x00..=0xFF this is lossless. For COBOL's printable set
// (letters, digits, common punctuation) all code points are in 0x00..=0x7F.
// ---------------------------------------------------------------------------

/// CP037 EBCDIC -> byte mapping. Index by EBCDIC byte, get the Unicode
/// code point's low byte. Lossless for code points 0x00..=0xFF.
#[rustfmt::skip]
const CP037_TO_UNICODE: [u8; 256] = [
    // 0x00-0x0F
    0x00, 0x01, 0x02, 0x03, 0x9C, 0x09, 0x86, 0x7F,
    0x97, 0x8D, 0x8E, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    // 0x10-0x1F
    0x10, 0x11, 0x12, 0x13, 0x9D, 0x85, 0x08, 0x87,
    0x18, 0x19, 0x92, 0x8F, 0x1C, 0x1D, 0x1E, 0x1F,
    // 0x20-0x2F
    0x80, 0x81, 0x82, 0x83, 0x84, 0x0A, 0x17, 0x1B,
    0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x05, 0x06, 0x07,
    // 0x30-0x3F
    0x90, 0x91, 0x16, 0x93, 0x94, 0x95, 0x96, 0x04,
    0x98, 0x99, 0x9A, 0x9B, 0x14, 0x15, 0x9E, 0x1A,
    // 0x40-0x4F  (0x40=space, 0x4B='.', 0x4C='<', 0x4D='(', 0x4E='+', 0x4F='|')
    0x20, 0xA0, 0xE2, 0xE4, 0xE0, 0xE1, 0xE3, 0xE5,
    0xE7, 0xF1, 0xA2, 0x2E, 0x3C, 0x28, 0x2B, 0x7C,
    // 0x50-0x5F  (0x50='&', 0x5A='!', 0x5B='$', 0x5C='*', 0x5D=')', 0x5E=';')
    0x26, 0xE9, 0xEA, 0xEB, 0xE8, 0xED, 0xEE, 0xEF,
    0xEC, 0xDF, 0x21, 0x24, 0x2A, 0x29, 0x3B, 0xAC,
    // 0x60-0x6F  (0x60='-', 0x61='/', 0x6B=',', 0x6C='%', 0x6D='_', 0x6E='>', 0x6F='?')
    0x2D, 0x2F, 0xC2, 0xC4, 0xC0, 0xC1, 0xC3, 0xC5,
    0xC7, 0xD1, 0xA6, 0x2C, 0x25, 0x5F, 0x3E, 0x3F,
    // 0x70-0x7F  (0x79='`', 0x7A=':', 0x7B='#', 0x7C='@', 0x7D='\'', 0x7E='=', 0x7F='"')
    0xF8, 0xC9, 0xCA, 0xCB, 0xC8, 0xCD, 0xCE, 0xCF,
    0xCC, 0x60, 0x3A, 0x23, 0x40, 0x27, 0x3D, 0x22,
    // 0x80-0x8F  (0x81-0x89 = 'a'-'i')
    0xD8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
    0x68, 0x69, 0xAB, 0xBB, 0xF0, 0xFD, 0xFE, 0xB1,
    // 0x90-0x9F  (0x91-0x99 = 'j'-'r', 0x9F = currency sign 0xA4)
    0xB0, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70,
    0x71, 0x72, 0xAA, 0xBA, 0xE6, 0xB8, 0xC6, 0xA4,
    // 0xA0-0xAF  (0xA1='~', 0xA2-0xA9 = 's'-'z')
    0xB5, 0x7E, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
    0x79, 0x7A, 0xA1, 0xBF, 0xD0, 0xDD, 0xDE, 0xAE,
    // 0xB0-0xBF  (0xB0='^', 0xBA='[', 0xBB=']')
    0x5E, 0xA3, 0xA5, 0xB7, 0xA9, 0xA7, 0xB6, 0xBC,
    0xBD, 0xBE, 0x5B, 0x5D, 0xAF, 0xA8, 0xB4, 0xD7,
    // 0xC0-0xCF  (0xC0='{', 0xC1-0xC9 = 'A'-'I')
    0x7B, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
    0x48, 0x49, 0xAD, 0xF4, 0xF6, 0xF2, 0xF3, 0xF5,
    // 0xD0-0xDF  (0xD0='}', 0xD1-0xD9 = 'J'-'R')
    0x7D, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
    0x51, 0x52, 0xB9, 0xFB, 0xFC, 0xF9, 0xFA, 0xFF,
    // 0xE0-0xEF  (0xE0='\\', 0xE2-0xE9 = 'S'-'Z')
    0x5C, 0xF7, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
    0x59, 0x5A, 0xB2, 0xD4, 0xD6, 0xD2, 0xD3, 0xD5,
    // 0xF0-0xFF  (0xF0-0xF9 = '0'-'9')
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x38, 0x39, 0xB3, 0xDB, 0xDC, 0xD9, 0xDA, 0x9F,
];

/// CP500 EBCDIC -> byte mapping.
/// Source: <https://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/EBCDIC/CP500.TXT>
///
/// Differences from CP037 are concentrated in the symbol/punctuation area:
/// 0x4A='[', 0x4F='!', 0x5A=']', 0x5F='^', 0xB0=cent, 0xBA=not, 0xBB='|'
#[rustfmt::skip]
const CP500_TO_UNICODE: [u8; 256] = [
    // 0x00-0x0F  (identical to CP037)
    0x00, 0x01, 0x02, 0x03, 0x9C, 0x09, 0x86, 0x7F,
    0x97, 0x8D, 0x8E, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    // 0x10-0x1F
    0x10, 0x11, 0x12, 0x13, 0x9D, 0x85, 0x08, 0x87,
    0x18, 0x19, 0x92, 0x8F, 0x1C, 0x1D, 0x1E, 0x1F,
    // 0x20-0x2F
    0x80, 0x81, 0x82, 0x83, 0x84, 0x0A, 0x17, 0x1B,
    0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x05, 0x06, 0x07,
    // 0x30-0x3F
    0x90, 0x91, 0x16, 0x93, 0x94, 0x95, 0x96, 0x04,
    0x98, 0x99, 0x9A, 0x9B, 0x14, 0x15, 0x9E, 0x1A,
    // 0x40-0x4F  (0x4A='[' differs from CP037's cent sign)
    0x20, 0xA0, 0xE2, 0xE4, 0xE0, 0xE1, 0xE3, 0xE5,
    0xE7, 0xF1, 0x5B, 0x2E, 0x3C, 0x28, 0x2B, 0x21,
    // 0x50-0x5F  (0x5A=']', 0x5F='^' differ from CP037)
    0x26, 0xE9, 0xEA, 0xEB, 0xE8, 0xED, 0xEE, 0xEF,
    0xEC, 0xDF, 0x5D, 0x24, 0x2A, 0x29, 0x3B, 0x5E,
    // 0x60-0x6F  (same as CP037)
    0x2D, 0x2F, 0xC2, 0xC4, 0xC0, 0xC1, 0xC3, 0xC5,
    0xC7, 0xD1, 0xA6, 0x2C, 0x25, 0x5F, 0x3E, 0x3F,
    // 0x70-0x7F
    0xF8, 0xC9, 0xCA, 0xCB, 0xC8, 0xCD, 0xCE, 0xCF,
    0xCC, 0x60, 0x3A, 0x23, 0x40, 0x27, 0x3D, 0x22,
    // 0x80-0x8F  (same as CP037)
    0xD8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
    0x68, 0x69, 0xAB, 0xBB, 0xF0, 0xFD, 0xFE, 0xB1,
    // 0x90-0x9F  (same as CP037)
    0xB0, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70,
    0x71, 0x72, 0xAA, 0xBA, 0xE6, 0xB8, 0xC6, 0xA4,
    // 0xA0-0xAF  (same as CP037)
    0xB5, 0x7E, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
    0x79, 0x7A, 0xA1, 0xBF, 0xD0, 0xDD, 0xDE, 0xAE,
    // 0xB0-0xBF  (0xB0=cent, 0xBA=not-sign, 0xBB='|' differ from CP037)
    0xA2, 0xA3, 0xA5, 0xB7, 0xA9, 0xA7, 0xB6, 0xBC,
    0xBD, 0xBE, 0xAC, 0x7C, 0xAF, 0xA8, 0xB4, 0xD7,
    // 0xC0-0xCF  (same as CP037)
    0x7B, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
    0x48, 0x49, 0xAD, 0xF4, 0xF6, 0xF2, 0xF3, 0xF5,
    // 0xD0-0xDF  (same as CP037)
    0x7D, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
    0x51, 0x52, 0xB9, 0xFB, 0xFC, 0xF9, 0xFA, 0xFF,
    // 0xE0-0xEF  (same as CP037)
    0x5C, 0xF7, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
    0x59, 0x5A, 0xB2, 0xD4, 0xD6, 0xD2, 0xD3, 0xD5,
    // 0xF0-0xFF  (same as CP037)
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x38, 0x39, 0xB3, 0xDB, 0xDC, 0xD9, 0xDA, 0x9F,
];

// ---------------------------------------------------------------------------
// Reverse tables: Unicode/ASCII -> EBCDIC
// Built at compile time from the forward tables.
// ---------------------------------------------------------------------------

/// Build the reverse mapping table from a forward (EBCDIC->Unicode) table.
const fn build_reverse_table(forward: &[u8; 256]) -> [u8; 256] {
    let mut reverse = [0x3Fu8; 256]; // Default: SUB (0x3F) for unmapped
    let mut i = 0;
    while i < 256 {
        let unicode_byte = forward[i] as usize;
        // Only map if the reverse slot hasn't been filled by an earlier entry
        // (first EBCDIC byte wins for a given Unicode value).
        if reverse[unicode_byte] == 0x3F || i == unicode_byte {
            reverse[unicode_byte] = i as u8;
        }
        i += 1;
    }
    // Ensure NUL maps to NUL
    reverse[0x00] = 0x00;
    reverse
}

/// CP037: Unicode byte -> EBCDIC byte.
const ASCII_TO_CP037: [u8; 256] = build_reverse_table(&CP037_TO_UNICODE);

/// CP500: Unicode byte -> EBCDIC byte.
const ASCII_TO_CP500: [u8; 256] = build_reverse_table(&CP500_TO_UNICODE);

// ---------------------------------------------------------------------------
// Public conversion API
// ---------------------------------------------------------------------------

/// Get the forward (EBCDIC -> Unicode) table for a code page.
#[must_use]
pub fn forward_table(cp: CodePage) -> &'static [u8; 256] {
    match cp {
        CodePage::CP037 | CodePage::CP1140 => &CP037_TO_UNICODE,
        CodePage::CP500 => &CP500_TO_UNICODE,
    }
}

/// Get the reverse (Unicode -> EBCDIC) table for a code page.
#[must_use]
pub fn reverse_table(cp: CodePage) -> &'static [u8; 256] {
    match cp {
        CodePage::CP037 | CodePage::CP1140 => &ASCII_TO_CP037,
        CodePage::CP500 => &ASCII_TO_CP500,
    }
}

/// Convert a single EBCDIC byte to its Unicode/ASCII equivalent.
#[inline]
#[must_use]
pub fn ebcdic_to_ascii_byte(cp: CodePage, byte: u8) -> u8 {
    // CP1140 special case: position 0x9F is Euro sign (U+20AC).
    // Since we can't represent U+20AC in a single byte, we use 0x80
    // (which is unused in ASCII) as a marker for the Euro sign.
    if cp == CodePage::CP1140 && byte == 0x9F {
        return 0x80; // Euro sign marker
    }
    forward_table(cp)[byte as usize]
}

/// Convert a single Unicode/ASCII byte to its EBCDIC equivalent.
#[inline]
#[must_use]
pub fn ascii_to_ebcdic_byte(cp: CodePage, byte: u8) -> u8 {
    if cp == CodePage::CP1140 && byte == 0x80 {
        return 0x9F; // Euro sign -> EBCDIC 0x9F
    }
    reverse_table(cp)[byte as usize]
}

/// Convert a buffer of EBCDIC bytes to ASCII/Unicode in-place.
pub fn ebcdic_to_ascii(cp: CodePage, buf: &mut [u8]) {
    let table = forward_table(cp);
    let is_cp1140 = cp == CodePage::CP1140;
    for byte in buf.iter_mut() {
        if is_cp1140 && *byte == 0x9F {
            *byte = 0x80; // Euro sign marker
        } else {
            *byte = table[*byte as usize];
        }
    }
}

/// Convert a buffer of ASCII/Unicode bytes to EBCDIC in-place.
pub fn ascii_to_ebcdic(cp: CodePage, buf: &mut [u8]) {
    let table = reverse_table(cp);
    let is_cp1140 = cp == CodePage::CP1140;
    for byte in buf.iter_mut() {
        if is_cp1140 && *byte == 0x80 {
            *byte = 0x9F; // Euro sign marker -> EBCDIC 0x9F
        } else {
            *byte = table[*byte as usize];
        }
    }
}

/// Convert EBCDIC bytes to ASCII, writing to a separate output buffer.
///
/// # Panics
/// Panics if `dst.len() < src.len()`.
pub fn ebcdic_to_ascii_copy(cp: CodePage, src: &[u8], dst: &mut [u8]) {
    assert!(
        dst.len() >= src.len(),
        "destination buffer too small: {} < {}",
        dst.len(),
        src.len()
    );
    let table = forward_table(cp);
    let is_cp1140 = cp == CodePage::CP1140;
    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        if is_cp1140 && s == 0x9F {
            *d = 0x80;
        } else {
            *d = table[s as usize];
        }
    }
}

/// Convert ASCII bytes to EBCDIC, writing to a separate output buffer.
///
/// # Panics
/// Panics if `dst.len() < src.len()`.
pub fn ascii_to_ebcdic_copy(cp: CodePage, src: &[u8], dst: &mut [u8]) {
    assert!(
        dst.len() >= src.len(),
        "destination buffer too small: {} < {}",
        dst.len(),
        src.len()
    );
    let table = reverse_table(cp);
    let is_cp1140 = cp == CodePage::CP1140;
    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        if is_cp1140 && s == 0x80 {
            *d = 0x9F;
        } else {
            *d = table[s as usize];
        }
    }
}

// ---------------------------------------------------------------------------
// EBCDIC Collating Sequence Table
//
// This table assigns a weight to each ASCII byte value such that comparing
// by weight produces EBCDIC collating order. The weight is simply the
// EBCDIC code point for that character (higher EBCDIC code = higher weight).
//
// EBCDIC order: space(0x40) < specials < lowercase(0x81+) < uppercase(0xC1+) < digits(0xF0+)
// ---------------------------------------------------------------------------

/// Build an EBCDIC collating weight table for ASCII data.
///
/// For each ASCII byte value, the weight is its corresponding EBCDIC code
/// point. Comparing ASCII strings by these weights produces EBCDIC ordering.
#[must_use]
pub fn ebcdic_collating_weights(cp: CodePage) -> [u8; 256] {
    *reverse_table(cp)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Roundtrip tests --

    #[test]
    fn cp037_roundtrip_printable_ascii() {
        // All printable ASCII chars (0x20..=0x7E) must survive roundtrip.
        for ascii in 0x20u8..=0x7E {
            let ebcdic = ascii_to_ebcdic_byte(CodePage::CP037, ascii);
            let back = ebcdic_to_ascii_byte(CodePage::CP037, ebcdic);
            assert_eq!(
                back, ascii,
                "CP037 roundtrip failed for ASCII 0x{ascii:02X}: EBCDIC 0x{ebcdic:02X} -> 0x{back:02X}"
            );
        }
    }

    #[test]
    fn cp500_roundtrip_printable_ascii() {
        for ascii in 0x20u8..=0x7E {
            let ebcdic = ascii_to_ebcdic_byte(CodePage::CP500, ascii);
            let back = ebcdic_to_ascii_byte(CodePage::CP500, ebcdic);
            assert_eq!(
                back, ascii,
                "CP500 roundtrip failed for ASCII 0x{ascii:02X}: EBCDIC 0x{ebcdic:02X} -> 0x{back:02X}"
            );
        }
    }

    #[test]
    fn cp1140_roundtrip_printable_ascii() {
        for ascii in 0x20u8..=0x7E {
            let ebcdic = ascii_to_ebcdic_byte(CodePage::CP1140, ascii);
            let back = ebcdic_to_ascii_byte(CodePage::CP1140, ebcdic);
            assert_eq!(
                back, ascii,
                "CP1140 roundtrip failed for ASCII 0x{ascii:02X}: EBCDIC 0x{ebcdic:02X} -> 0x{back:02X}"
            );
        }
    }

    // -- Specific character mapping tests --

    #[test]
    fn cp037_digits() {
        // EBCDIC digits: 0xF0-0xF9 -> ASCII '0'-'9'
        for digit in 0..10u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP037, 0xF0 + digit),
                b'0' + digit,
                "digit {digit} mapping failed"
            );
        }
    }

    #[test]
    fn cp037_uppercase_letters() {
        // A-I: EBCDIC 0xC1-0xC9 -> ASCII 0x41-0x49
        for i in 0..9u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP037, 0xC1 + i),
                b'A' + i
            );
        }
        // J-R: EBCDIC 0xD1-0xD9 -> ASCII 0x4A-0x52
        for i in 0..9u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP037, 0xD1 + i),
                b'J' + i
            );
        }
        // S-Z: EBCDIC 0xE2-0xE9 -> ASCII 0x53-0x5A
        for i in 0..8u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP037, 0xE2 + i),
                b'S' + i
            );
        }
    }

    #[test]
    fn cp037_lowercase_letters() {
        // a-i: EBCDIC 0x81-0x89 -> ASCII 0x61-0x69
        for i in 0..9u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP037, 0x81 + i),
                b'a' + i
            );
        }
        // j-r: EBCDIC 0x91-0x99 -> ASCII 0x6A-0x72
        for i in 0..9u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP037, 0x91 + i),
                b'j' + i
            );
        }
        // s-z: EBCDIC 0xA2-0xA9 -> ASCII 0x73-0x7A
        for i in 0..8u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP037, 0xA2 + i),
                b's' + i
            );
        }
    }

    #[test]
    fn cp037_common_symbols() {
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x40), b' ');  // space
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x4B), b'.');  // period
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x6B), b',');  // comma
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x60), b'-');  // hyphen
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x61), b'/');  // slash
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x5B), b'$');  // dollar
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x4D), b'(');  // open paren
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x5D), b')');  // close paren
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x7D), b'\''); // apostrophe
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x7E), b'=');  // equals
    }

    // -- CP500 differences from CP037 --

    #[test]
    fn cp500_bracket_differences() {
        // CP500 has [ at 0x4A (CP037 has cent-sign there)
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP500, 0x4A), b'[');
        // CP500 has ] at 0x5A (CP037 has ! there)
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP500, 0x5A), b']');
        // CP500 has ! at 0x4F (CP037 has | there)
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP500, 0x4F), b'!');
        // CP500 has ^ at 0x5F (CP037 has not-sign there)
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP500, 0x5F), b'^');
        // CP500 has | at 0xBB (CP037 has ] there)
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP500, 0xBB), b'|');
    }

    #[test]
    fn cp500_digits_same_as_cp037() {
        for digit in 0..10u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP500, 0xF0 + digit),
                b'0' + digit
            );
        }
    }

    #[test]
    fn cp500_letters_same_as_cp037() {
        // Letters occupy same positions in CP037 and CP500
        for i in 0..9u8 {
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP500, 0xC1 + i),
                ebcdic_to_ascii_byte(CodePage::CP037, 0xC1 + i)
            );
        }
    }

    // -- CP1140 Euro sign --

    #[test]
    fn cp1140_euro_sign_difference() {
        // CP037 at 0x9F -> currency sign (0xA4)
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP037, 0x9F), 0xA4);
        // CP1140 at 0x9F -> Euro sign marker (0x80)
        assert_eq!(ebcdic_to_ascii_byte(CodePage::CP1140, 0x9F), 0x80);
    }

    #[test]
    fn cp1140_euro_sign_roundtrip() {
        let ebcdic = ascii_to_ebcdic_byte(CodePage::CP1140, 0x80);
        assert_eq!(ebcdic, 0x9F);
        let back = ebcdic_to_ascii_byte(CodePage::CP1140, ebcdic);
        assert_eq!(back, 0x80);
    }

    #[test]
    fn cp1140_other_positions_same_as_cp037() {
        // All positions except 0x9F should be identical
        for i in 0..256u16 {
            if i == 0x9F {
                continue;
            }
            assert_eq!(
                ebcdic_to_ascii_byte(CodePage::CP1140, i as u8),
                ebcdic_to_ascii_byte(CodePage::CP037, i as u8),
                "CP1140 vs CP037 differ at position 0x{i:02X}"
            );
        }
    }

    // -- Bulk conversion tests --

    #[test]
    fn bulk_ebcdic_to_ascii() {
        // "HELLO" in CP037 EBCDIC
        let mut buf = [0xC8, 0xC5, 0xD3, 0xD3, 0xD6]; // H, E, L, L, O
        ebcdic_to_ascii(CodePage::CP037, &mut buf);
        assert_eq!(&buf, b"HELLO");
    }

    #[test]
    fn bulk_ascii_to_ebcdic() {
        let mut buf = *b"HELLO";
        ascii_to_ebcdic(CodePage::CP037, &mut buf);
        assert_eq!(buf, [0xC8, 0xC5, 0xD3, 0xD3, 0xD6]);
    }

    #[test]
    fn bulk_roundtrip() {
        let original = b"Hello, World! 12345 @#$%".to_vec();
        let mut buf = original.clone();
        ascii_to_ebcdic(CodePage::CP037, &mut buf);
        // Should be different from original
        assert_ne!(buf, original.as_slice());
        ebcdic_to_ascii(CodePage::CP037, &mut buf);
        assert_eq!(buf, original.as_slice());
    }

    #[test]
    fn copy_conversion() {
        let ebcdic_data = [0xF1, 0xF2, 0xF3]; // "123" in EBCDIC
        let mut ascii_buf = [0u8; 3];
        ebcdic_to_ascii_copy(CodePage::CP037, &ebcdic_data, &mut ascii_buf);
        assert_eq!(&ascii_buf, b"123");

        let mut back_buf = [0u8; 3];
        ascii_to_ebcdic_copy(CodePage::CP037, &ascii_buf, &mut back_buf);
        assert_eq!(back_buf, ebcdic_data);
    }

    // -- Collating sequence tests --

    #[test]
    fn ebcdic_collating_lowercase_before_uppercase() {
        let weights = ebcdic_collating_weights(CodePage::CP037);
        // In EBCDIC, 'a' (0x81) < 'A' (0xC1) -- lowercase before uppercase
        assert!(
            weights[b'a' as usize] < weights[b'A' as usize],
            "EBCDIC: 'a' weight ({}) should be < 'A' weight ({})",
            weights[b'a' as usize],
            weights[b'A' as usize]
        );
    }

    #[test]
    fn ebcdic_collating_uppercase_before_digits() {
        let weights = ebcdic_collating_weights(CodePage::CP037);
        // In EBCDIC, 'Z' (0xE9) < '0' (0xF0) -- letters before digits
        assert!(
            weights[b'Z' as usize] < weights[b'0' as usize],
            "EBCDIC: 'Z' weight ({}) should be < '0' weight ({})",
            weights[b'Z' as usize],
            weights[b'0' as usize]
        );
    }

    #[test]
    fn ebcdic_collating_space_lowest() {
        let weights = ebcdic_collating_weights(CodePage::CP037);
        // Space (0x40 in EBCDIC) should have a weight > NUL but < all printable
        let space_weight = weights[b' ' as usize];
        assert!(
            space_weight < weights[b'a' as usize],
            "space weight ({}) should be < 'a' weight ({})",
            space_weight,
            weights[b'a' as usize]
        );
        assert!(
            space_weight < weights[b'A' as usize],
            "space weight ({}) should be < 'A' weight ({})",
            space_weight,
            weights[b'A' as usize]
        );
        assert!(
            space_weight < weights[b'0' as usize],
            "space weight ({}) should be < '0' weight ({})",
            space_weight,
            weights[b'0' as usize]
        );
    }

    #[test]
    fn ebcdic_collating_digit_order_preserved() {
        let weights = ebcdic_collating_weights(CodePage::CP037);
        for i in 0..9u8 {
            assert!(
                weights[(b'0' + i) as usize] < weights[(b'0' + i + 1) as usize],
                "digit {} should sort before digit {}",
                i,
                i + 1
            );
        }
    }

    #[test]
    fn ebcdic_collating_letter_order_preserved() {
        let weights = ebcdic_collating_weights(CodePage::CP037);
        // Uppercase letter order should be preserved (A < B < ... < Z)
        for i in 0..25u8 {
            assert!(
                weights[(b'A' + i) as usize] < weights[(b'A' + i + 1) as usize],
                "letter {} should sort before letter {}",
                (b'A' + i) as char,
                (b'A' + i + 1) as char
            );
        }
    }

    // -- Default --

    #[test]
    fn default_code_page_is_cp037() {
        assert_eq!(CodePage::default(), CodePage::CP037);
    }

    // -- Empty buffer --

    #[test]
    fn empty_buffer_conversion() {
        let mut buf: [u8; 0] = [];
        ebcdic_to_ascii(CodePage::CP037, &mut buf);
        ascii_to_ebcdic(CodePage::CP037, &mut buf);
        // Should not panic
    }
}
