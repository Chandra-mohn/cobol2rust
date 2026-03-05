/// Data category of a COBOL data item.
///
/// Determines MOVE legality, comparison rules, and INITIALIZE behavior.
/// Category is derived from the PICTURE clause and USAGE clause per standard rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataCategory {
    /// PIC contains only A and B
    Alphabetic,
    /// PIC contains only X, or combination of A/X/9 without editing symbols
    Alphanumeric,
    /// PIC contains X/A with B, 0, / insertion characters
    AlphanumericEdited,
    /// PIC contains only 9, S, V, P (no editing symbols), any USAGE
    Numeric,
    /// PIC contains 9 with editing symbols (Z, *, $, +, -, CR, DB, etc.)
    NumericEdited,
    /// PIC contains N or G (DBCS) -- Phase 3
    National,
    /// No PICTURE, has subordinates
    Group,
}
