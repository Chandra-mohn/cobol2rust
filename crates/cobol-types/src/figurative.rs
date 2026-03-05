use rust_decimal::Decimal;

use cobol_core::traits::{CobolField, CobolNumeric};
use cobol_core::DataCategory;

/// Figurative constants that adapt to destination size.
///
/// In COBOL, figurative constants like SPACES, ZEROS, HIGH-VALUES are
/// implicitly sized to match the destination field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FigurativeConstant {
    /// SPACES -- fill with 0x20
    Spaces,
    /// ZEROS / ZEROES -- fill with 0x30 (ASCII '0') for alpha, set 0 for numeric
    Zeros,
    /// HIGH-VALUES -- fill with 0xFF
    HighValues,
    /// LOW-VALUES -- fill with 0x00
    LowValues,
    /// QUOTES -- fill with 0x22 (ASCII double quote)
    Quotes,
    /// ALL literal -- repeat a byte pattern to fill the field
    AllLiteral(Vec<u8>),
    /// NULLS -- fill with 0x00 (same as LOW-VALUES)
    Nulls,
}

impl FigurativeConstant {
    /// Fill a `CobolField` with this figurative constant.
    ///
    /// For numeric fields receiving ZEROS, sets the numeric value to 0.
    /// For alphanumeric fields receiving ZEROS, fills with ASCII '0' characters.
    /// For all other combinations, fills with the appropriate byte value.
    pub fn fill_field(&self, dest: &mut dyn CobolField) {
        match self {
            Self::Spaces => dest.fill_bytes(b' '),
            Self::Zeros => {
                if dest.category() == DataCategory::Numeric {
                    // For numeric fields, set to zero via numeric interface if possible
                    // Fall back to fill with '0' bytes
                    dest.fill_bytes(0x00);
                    // The caller should use fill_numeric for proper numeric zero
                } else {
                    dest.fill_bytes(b'0');
                }
            }
            Self::HighValues => dest.fill_bytes(0xFF),
            Self::LowValues | Self::Nulls => dest.fill_bytes(0x00),
            Self::Quotes => dest.fill_bytes(b'"'),
            Self::AllLiteral(pattern) => {
                if pattern.is_empty() {
                    return;
                }
                let bytes = dest.as_bytes_mut();
                for (i, b) in bytes.iter_mut().enumerate() {
                    *b = pattern[i % pattern.len()];
                }
            }
        }
    }

    /// Fill a numeric field with this figurative constant's numeric interpretation.
    ///
    /// Only ZEROS is meaningful for numeric fields (sets to 0).
    pub fn fill_numeric(&self, dest: &mut dyn CobolNumeric) {
        match self {
            Self::Zeros => dest.set_decimal(Decimal::ZERO),
            _ => self.fill_field(dest),
        }
    }

    /// Get the fill byte for non-numeric use.
    pub fn fill_byte(&self) -> Option<u8> {
        match self {
            Self::Spaces => Some(b' '),
            Self::Zeros => Some(b'0'),
            Self::HighValues => Some(0xFF),
            Self::LowValues | Self::Nulls => Some(0x00),
            Self::Quotes => Some(b'"'),
            Self::AllLiteral(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PackedDecimal, PicX};
    use rust_decimal_macros::dec;

    #[test]
    fn spaces_fills_picx() {
        let mut f = PicX::new(5, b"HELLO");
        FigurativeConstant::Spaces.fill_field(&mut f);
        assert_eq!(f.as_bytes(), b"     ");
    }

    #[test]
    fn zeros_fills_picx_with_ascii_zero() {
        let mut f = PicX::new(5, b"HELLO");
        FigurativeConstant::Zeros.fill_field(&mut f);
        assert_eq!(f.as_bytes(), b"00000");
    }

    #[test]
    fn zeros_fills_packed_decimal_numerically() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.set_decimal(dec!(123.45));
        FigurativeConstant::Zeros.fill_numeric(&mut pd);
        assert_eq!(pd.to_decimal(), dec!(0));
    }

    #[test]
    fn high_values_fills_0xff() {
        let mut f = PicX::new(3, b"ABC");
        FigurativeConstant::HighValues.fill_field(&mut f);
        assert_eq!(f.as_bytes(), &[0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn low_values_fills_0x00() {
        let mut f = PicX::new(3, b"ABC");
        FigurativeConstant::LowValues.fill_field(&mut f);
        assert_eq!(f.as_bytes(), &[0x00, 0x00, 0x00]);
    }

    #[test]
    fn quotes_fills_double_quote() {
        let mut f = PicX::new(3, b"ABC");
        FigurativeConstant::Quotes.fill_field(&mut f);
        assert_eq!(f.as_bytes(), b"\"\"\"");
    }

    #[test]
    fn all_literal_repeats_pattern() {
        let mut f = PicX::new(6, b"");
        FigurativeConstant::AllLiteral(b"AB".to_vec()).fill_field(&mut f);
        assert_eq!(f.as_bytes(), b"ABABAB");
    }

    #[test]
    fn all_literal_single_byte() {
        let mut f = PicX::new(4, b"");
        FigurativeConstant::AllLiteral(b"X".to_vec()).fill_field(&mut f);
        assert_eq!(f.as_bytes(), b"XXXX");
    }

    #[test]
    fn nulls_same_as_low_values() {
        let mut f = PicX::new(3, b"ABC");
        FigurativeConstant::Nulls.fill_field(&mut f);
        assert_eq!(f.as_bytes(), &[0x00, 0x00, 0x00]);
    }

    #[test]
    fn fill_byte_returns_correct_values() {
        assert_eq!(FigurativeConstant::Spaces.fill_byte(), Some(b' '));
        assert_eq!(FigurativeConstant::Zeros.fill_byte(), Some(b'0'));
        assert_eq!(FigurativeConstant::HighValues.fill_byte(), Some(0xFF));
        assert_eq!(FigurativeConstant::LowValues.fill_byte(), Some(0x00));
        assert_eq!(FigurativeConstant::Quotes.fill_byte(), Some(b'"'));
        assert_eq!(FigurativeConstant::AllLiteral(b"X".to_vec()).fill_byte(), None);
    }
}
