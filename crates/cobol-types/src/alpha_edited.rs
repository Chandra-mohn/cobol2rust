//! `AlphanumericEdited`: PIC with B, 0, / insertion characters.
//!
//! Alphanumeric editing is much simpler than numeric editing:
//! - Only 3 insertion symbols: B (space), 0 (zero), / (slash)
//! - No suppression, no floating symbols, no sign handling
//! - Source data characters fill the Data positions left-to-right
//! - Insertion symbols are placed at their fixed positions

use cobol_core::category::DataCategory;
use cobol_core::traits::CobolField;

/// Edit symbol for alphanumeric-edited PIC patterns.
///
/// Only 3 insertion types exist for alphanumeric editing (plus Data for pass-through).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaEditSymbol {
    /// X or A -- data position (pass through from source)
    Data,
    /// B -- insert space (0x20)
    Space,
    /// 0 -- insert zero (0x30)
    Zero,
    /// / -- insert slash (0x2F)
    Slash,
}

impl AlphaEditSymbol {
    /// The byte value to insert for this symbol, or None for Data positions.
    pub fn insertion_byte(&self) -> Option<u8> {
        match self {
            AlphaEditSymbol::Data => None,
            AlphaEditSymbol::Space => Some(b' '),
            AlphaEditSymbol::Zero => Some(b'0'),
            AlphaEditSymbol::Slash => Some(b'/'),
        }
    }
}

/// Alphanumeric-Edited field (PIC with B, 0, / insertion characters).
///
/// The display buffer holds the fully-edited string representation.
/// Data positions receive source characters left-to-right; insertion
/// positions always contain their fixed character.
#[derive(Clone)]
pub struct AlphanumericEdited {
    /// Edited display buffer (the "value" of an alphanumeric-edited field).
    data: Vec<u8>,
    /// Parsed editing pattern from PIC clause.
    edit_pattern: Vec<AlphaEditSymbol>,
}

impl std::fmt::Debug for AlphanumericEdited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = String::from_utf8_lossy(&self.data);
        f.debug_struct("AlphanumericEdited")
            .field("display", &display.as_ref())
            .field("length", &self.data.len())
            .finish()
    }
}

impl AlphanumericEdited {
    /// Create a new `AlphanumericEdited` field from an edit pattern.
    ///
    /// The field is initialized with spaces in data positions and
    /// insertion characters at their fixed positions.
    pub fn new(edit_pattern: Vec<AlphaEditSymbol>) -> Self {
        let data = edit_pattern
            .iter()
            .map(|sym| sym.insertion_byte().unwrap_or(b' '))
            .collect();
        Self { data, edit_pattern }
    }

    /// Create a new `AlphanumericEdited` and immediately set from source bytes.
    pub fn from_bytes(edit_pattern: Vec<AlphaEditSymbol>, source: &[u8]) -> Self {
        let mut field = Self::new(edit_pattern);
        field.set(source);
        field
    }

    /// Set the value from source bytes by applying the edit pattern.
    ///
    /// Source characters fill Data positions left-to-right.
    /// If source is shorter, remaining data positions are space-padded.
    /// If source is longer, excess characters are truncated.
    /// Insertion positions always get their fixed character.
    pub fn set(&mut self, source: &[u8]) {
        let mut src_idx = 0;
        for (i, sym) in self.edit_pattern.iter().enumerate() {
            match sym.insertion_byte() {
                Some(ins_byte) => {
                    // Insertion position: always the fixed character
                    self.data[i] = ins_byte;
                }
                None => {
                    // Data position: take from source or space-pad
                    if src_idx < source.len() {
                        self.data[i] = source[src_idx];
                        src_idx += 1;
                    } else {
                        self.data[i] = b' ';
                    }
                }
            }
        }
    }

    /// De-edit: extract only the data position characters (strip insertions).
    ///
    /// Returns the raw alphanumeric data without insertion characters.
    pub fn de_edit(&self) -> Vec<u8> {
        self.edit_pattern
            .iter()
            .zip(self.data.iter())
            .filter_map(|(sym, &byte)| {
                if *sym == AlphaEditSymbol::Data {
                    Some(byte)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get the number of data positions in the edit pattern.
    pub fn data_positions(&self) -> usize {
        self.edit_pattern
            .iter()
            .filter(|s| **s == AlphaEditSymbol::Data)
            .count()
    }

    /// Get the total display length (including insertion positions).
    pub fn display_length(&self) -> usize {
        self.data.len()
    }

    /// Get the value as a UTF-8 string (best-effort).
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.data).unwrap_or("<non-utf8>")
    }

    /// Get the edit pattern.
    pub fn edit_pattern(&self) -> &[AlphaEditSymbol] {
        &self.edit_pattern
    }
}

impl CobolField for AlphanumericEdited {
    fn category(&self) -> DataCategory {
        DataCategory::AlphanumericEdited
    }

    fn byte_length(&self) -> usize {
        self.data.len()
    }

    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn display_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn fill_bytes(&mut self, byte: u8) {
        self.data.fill(byte);
    }

    fn initialize_default(&mut self) {
        // INITIALIZE sets alphanumeric-edited to spaces with insertion characters
        for (i, sym) in self.edit_pattern.iter().enumerate() {
            self.data[i] = sym.insertion_byte().unwrap_or(b' ');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pattern(spec: &str) -> Vec<AlphaEditSymbol> {
        spec.chars()
            .map(|c| match c {
                'X' | 'A' => AlphaEditSymbol::Data,
                'B' => AlphaEditSymbol::Space,
                '0' => AlphaEditSymbol::Zero,
                '/' => AlphaEditSymbol::Slash,
                _ => panic!("unknown spec char: {c}"),
            })
            .collect()
    }

    #[test]
    fn alpha_edited_new() {
        // PIC X(3)BX(3) = XXXBXXX
        let pat = make_pattern("XXXBXXX");
        let field = AlphanumericEdited::new(pat);
        assert_eq!(field.display_length(), 7);
        assert_eq!(field.data_positions(), 6);
        // Default: spaces in data, space in insertion (B is also space)
        assert_eq!(field.as_bytes(), b"       ");
    }

    #[test]
    fn alpha_edited_edit_space() {
        // PIC X(3)BX(3) -> insert space at position 3
        let pat = make_pattern("XXXBXXX");
        let field = AlphanumericEdited::from_bytes(pat, b"ABCDEF");
        assert_eq!(field.as_str(), "ABC DEF");
    }

    #[test]
    fn alpha_edited_edit_zero() {
        // PIC X(3)0X(3) -> insert zero at position 3
        let pat = make_pattern("XXX0XXX");
        let field = AlphanumericEdited::from_bytes(pat, b"ABCDEF");
        assert_eq!(field.as_str(), "ABC0DEF");
    }

    #[test]
    fn alpha_edited_edit_slash() {
        // PIC X(2)/X(2)/X(4) -> date-like formatting
        let pat = make_pattern("XX/XX/XXXX");
        let field = AlphanumericEdited::from_bytes(pat, b"03062026");
        assert_eq!(field.as_str(), "03/06/2026");
    }

    #[test]
    fn alpha_edited_display() {
        let pat = make_pattern("XXXBXXX");
        let field = AlphanumericEdited::from_bytes(pat, b"HIWRLD");
        let display = field.display_bytes();
        assert_eq!(&display, b"HIW RLD");
    }

    #[test]
    fn alpha_edited_as_bytes() {
        let pat = make_pattern("XX0XX");
        let field = AlphanumericEdited::from_bytes(pat, b"ABCD");
        assert_eq!(field.as_bytes(), b"AB0CD");
    }

    #[test]
    fn alpha_edited_fill() {
        let pat = make_pattern("XXXBXXX");
        let mut field = AlphanumericEdited::new(pat);
        field.fill_bytes(b'*');
        assert_eq!(field.as_bytes(), b"*******");
    }

    #[test]
    fn alpha_edited_de_edit() {
        let pat = make_pattern("XX/XX/XXXX");
        let field = AlphanumericEdited::from_bytes(pat, b"03062026");
        let raw = field.de_edit();
        assert_eq!(&raw, b"03062026");
    }

    #[test]
    fn alpha_edited_set_from_short() {
        // Source shorter than data positions -> space-pad remaining
        let pat = make_pattern("XXXBXXX");
        let field = AlphanumericEdited::from_bytes(pat, b"AB");
        assert_eq!(field.as_str(), "AB     ");
    }

    #[test]
    fn alpha_edited_set_from_long() {
        // Source longer than data positions -> truncate
        let pat = make_pattern("XXXBXXX");
        let field = AlphanumericEdited::from_bytes(pat, b"ABCDEFGHIJ");
        // Only 6 data positions, so only ABCDEF fit
        assert_eq!(field.as_str(), "ABC DEF");
    }

    #[test]
    fn alpha_edited_initialize() {
        let pat = make_pattern("XX/XX/XXXX");
        let mut field = AlphanumericEdited::from_bytes(pat, b"03062026");
        assert_eq!(field.as_str(), "03/06/2026");
        field.initialize_default();
        // After INITIALIZE: spaces in data positions, slashes in insertion positions
        assert_eq!(field.as_str(), "  /  /    ");
    }

    #[test]
    fn alpha_edited_category() {
        let pat = make_pattern("XXXBXXX");
        let field = AlphanumericEdited::new(pat);
        assert_eq!(field.category(), DataCategory::AlphanumericEdited);
    }
}
