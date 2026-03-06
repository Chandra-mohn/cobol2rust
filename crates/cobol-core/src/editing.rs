/// Parsed PIC editing symbols (for numeric-edited and alpha-edited types).
///
/// These represent the individual elements of a PIC editing mask after
/// the PIC string has been parsed and expanded (e.g., Z(3) -> [`ZeroSuppress`; 3]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditSymbol {
    /// 9 -- always display digit
    Digit,
    /// Z (space fill, byte=b' ') or * (asterisk fill, byte=b'*')
    ZeroSuppress(u8),
    /// $ (or other currency symbol) -- floating
    FloatCurrency(u8),
    /// + floating plus/minus
    FloatPlus,
    /// - floating minus/space
    FloatMinus,
    /// + at fixed position
    FixedPlus,
    /// - at fixed position
    FixedMinus,
    /// $ at fixed position
    FixedCurrency(u8),
    /// . decimal point
    Period,
    /// , thousands separator
    Comma,
    /// B insertion (space)
    InsertSpace,
    /// 0 insertion
    InsertZero,
    /// / insertion
    InsertSlash,
    /// CR (2-char suffix for credit)
    CreditRight,
    /// DB (2-char suffix for debit)
    DebitRight,
    /// S (sign position indicator)
    SignPosition,
    /// X or A in alpha-edited PIC
    AlphaChar,
}
