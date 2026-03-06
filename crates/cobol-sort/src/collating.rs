//! Collating sequence support for SORT/MERGE key comparison.
//!
//! COBOL programs may specify a COLLATING SEQUENCE for sort operations,
//! which controls how alphanumeric characters are ordered. This module
//! provides translation tables for ASCII (native) and EBCDIC (CP037).

/// A 256-byte translation table for collating sequence comparison.
///
/// Each byte position maps an input byte to its collating-sequence weight.
/// Two bytes are compared by comparing their translated weights.
#[derive(Debug, Clone)]
pub struct CollatingTable {
    table: [u8; 256],
}

impl CollatingTable {
    /// Create a new `CollatingTable` from a 256-byte mapping.
    #[must_use]
    pub fn new(table: [u8; 256]) -> Self {
        Self { table }
    }

    /// Translate a byte to its collating weight.
    #[inline]
    pub fn translate(&self, byte: u8) -> u8 {
        self.table[byte as usize]
    }

    /// Compare two byte slices using this collating sequence.
    ///
    /// Compares byte-by-byte using translated weights.
    /// Shorter slice is padded with spaces (0x20) for comparison.
    pub fn compare(&self, a: &[u8], b: &[u8]) -> std::cmp::Ordering {
        let max_len = a.len().max(b.len());
        for i in 0..max_len {
            let ba = if i < a.len() { a[i] } else { b' ' };
            let bb = if i < b.len() { b[i] } else { b' ' };
            let wa = self.translate(ba);
            let wb = self.translate(bb);
            let ord = wa.cmp(&wb);
            if ord != std::cmp::Ordering::Equal {
                return ord;
            }
        }
        std::cmp::Ordering::Equal
    }

    /// Native ASCII collating table (identity mapping).
    #[must_use]
    pub fn native_ascii() -> Self {
        let mut table = [0u8; 256];
        #[allow(clippy::cast_possible_truncation)]
        for (i, slot) in table.iter_mut().enumerate() {
            *slot = i as u8; // 0..256 always fits in u8
        }
        Self { table }
    }

    /// EBCDIC CP037 collating table.
    ///
    /// Maps ASCII byte values to EBCDIC sort weights so that comparison
    /// of ASCII data produces EBCDIC ordering. Key differences from ASCII:
    /// - Lowercase letters sort before uppercase
    /// - Digits sort after letters
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn ebcdic_cp037() -> Self {
        // Build a weight table: for each ASCII code point, assign
        // its EBCDIC ordinal position so that sorting by weight
        // produces EBCDIC collating order.
        let mut table = [0u8; 256];

        // ASCII -> EBCDIC code point mapping for printable range.
        // We use the EBCDIC code point value directly as the weight.
        #[rustfmt::skip]
        let ascii_to_ebcdic: [u8; 256] = {
            let mut map = [0x00u8; 256];
            // Control chars and non-printable map to low weights (0x00)
            // Space
            map[0x20] = 0x40;
            // Punctuation and symbols (selected common ones)
            map[b'.' as usize] = 0x4B;
            map[b'<' as usize] = 0x4C;
            map[b'(' as usize] = 0x4D;
            map[b'+' as usize] = 0x4E;
            map[b'|' as usize] = 0x4F;
            map[b'&' as usize] = 0x50;
            map[b'!' as usize] = 0x5A;
            map[b'$' as usize] = 0x5B;
            map[b'*' as usize] = 0x5C;
            map[b')' as usize] = 0x5D;
            map[b';' as usize] = 0x5E;
            map[b'-' as usize] = 0x60;
            map[b'/' as usize] = 0x61;
            map[b',' as usize] = 0x6B;
            map[b'%' as usize] = 0x6C;
            map[b'_' as usize] = 0x6D;
            map[b'>' as usize] = 0x6E;
            map[b'?' as usize] = 0x6F;
            map[b':' as usize] = 0x7A;
            map[b'#' as usize] = 0x7B;
            map[b'@' as usize] = 0x7C;
            map[b'\'' as usize] = 0x7D;
            map[b'=' as usize] = 0x7E;
            map[b'"' as usize] = 0x7F;
            // Lowercase letters (EBCDIC: a-i = 0x81-0x89, j-r = 0x91-0x99, s-z = 0xA2-0xA9)
            for (i, ch) in (b'a'..=b'i').enumerate() {
                map[ch as usize] = 0x81 + i as u8;
            }
            for (i, ch) in (b'j'..=b'r').enumerate() {
                map[ch as usize] = 0x91 + i as u8;
            }
            for (i, ch) in (b's'..=b'z').enumerate() {
                map[ch as usize] = 0xA2 + i as u8;
            }
            // Uppercase letters (EBCDIC: A-I = 0xC1-0xC9, J-R = 0xD1-0xD9, S-Z = 0xE2-0xE9)
            for (i, ch) in (b'A'..=b'I').enumerate() {
                map[ch as usize] = 0xC1 + i as u8;
            }
            for (i, ch) in (b'J'..=b'R').enumerate() {
                map[ch as usize] = 0xD1 + i as u8;
            }
            for (i, ch) in (b'S'..=b'Z').enumerate() {
                map[ch as usize] = 0xE2 + i as u8;
            }
            // Digits (EBCDIC: 0-9 = 0xF0-0xF9)
            for (i, ch) in (b'0'..=b'9').enumerate() {
                map[ch as usize] = 0xF0 + i as u8;
            }
            map
        };

        for (i, slot) in table.iter_mut().enumerate() {
            *slot = ascii_to_ebcdic[i];
        }
        Self { table }
    }
}
