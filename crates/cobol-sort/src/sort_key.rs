//! Sort key comparison infrastructure.
//!
//! Builds multi-key comparators from COBOL SORT key specifications.
//! Supports alphanumeric, zoned decimal, packed decimal, and binary key types
//! with ascending/descending order and collating sequence.

use std::cmp::Ordering;

use rust_decimal::Decimal;

use crate::collating::CollatingTable;

/// Boxed comparator function for sort key comparison.
pub type RecordComparator = Box<dyn Fn(&[u8], &[u8]) -> Ordering + Send + Sync>;

/// Arc-wrapped comparator for shared use in merge operations.
pub type SharedComparator = std::sync::Arc<dyn Fn(&[u8], &[u8]) -> Ordering + Send + Sync>;

/// Type of data stored in a sort key field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKeyType {
    /// `PIC X` / `PIC A` -- compared byte-by-byte using collating sequence.
    Alphanumeric,
    /// `PIC 9 USAGE DISPLAY` -- zoned decimal (one byte per digit).
    ZonedDecimal {
        precision: u32,
        scale: u32,
        signed: bool,
    },
    /// `COMP-3` -- packed BCD (two digits per byte, trailing sign nibble).
    PackedDecimal {
        precision: u32,
        scale: u32,
        signed: bool,
    },
    /// `COMP` / `COMP-4` / `COMP-5` -- binary integer (2, 4, or 8 bytes).
    Binary {
        /// Storage size: 2, 4, or 8.
        size: usize,
        signed: bool,
    },
}

/// Specification for a single sort key.
#[derive(Debug, Clone)]
pub struct SortKeySpec {
    /// Byte offset within the record where this key starts.
    pub offset: usize,
    /// Length in bytes of the key field.
    pub length: usize,
    /// true = ASCENDING KEY, false = DESCENDING KEY.
    pub ascending: bool,
    /// The data type of the key field.
    pub data_type: SortKeyType,
}

impl SortKeySpec {
    /// Create a new sort key specification.
    #[must_use]
    pub fn new(offset: usize, length: usize, ascending: bool, data_type: SortKeyType) -> Self {
        Self {
            offset,
            length,
            ascending,
            data_type,
        }
    }

    /// Create an alphanumeric sort key (ascending).
    #[must_use]
    pub fn alphanumeric(offset: usize, length: usize) -> Self {
        Self::new(offset, length, true, SortKeyType::Alphanumeric)
    }

    /// Create a packed decimal sort key (ascending).
    #[must_use]
    pub fn packed(offset: usize, length: usize, precision: u32, scale: u32, signed: bool) -> Self {
        Self::new(
            offset,
            length,
            true,
            SortKeyType::PackedDecimal {
                precision,
                scale,
                signed,
            },
        )
    }
}

/// Extract key bytes from a record.
fn extract_key_bytes<'a>(record: &'a [u8], key: &SortKeySpec) -> &'a [u8] {
    let end = (key.offset + key.length).min(record.len());
    let start = key.offset.min(end);
    &record[start..end]
}

/// Compare two key fields based on data type.
fn compare_key(
    a: &[u8],
    b: &[u8],
    key_type: &SortKeyType,
    collating: &CollatingTable,
) -> Ordering {
    match key_type {
        SortKeyType::Alphanumeric => collating.compare(a, b),
        SortKeyType::ZonedDecimal {
            precision,
            scale,
            signed,
        } => {
            let da = decode_zoned(a, *precision, *scale, *signed);
            let db = decode_zoned(b, *precision, *scale, *signed);
            da.cmp(&db)
        }
        SortKeyType::PackedDecimal {
            precision,
            scale,
            signed,
        } => {
            let da = decode_packed(a, *precision, *scale, *signed);
            let db = decode_packed(b, *precision, *scale, *signed);
            da.cmp(&db)
        }
        SortKeyType::Binary { size, signed } => {
            let ia = decode_binary(a, *size, *signed);
            let ib = decode_binary(b, *size, *signed);
            ia.cmp(&ib)
        }
    }
}

/// Decode a zoned decimal from raw bytes into a Decimal.
///
/// Each byte: high nibble = zone (0xF for unsigned/positive digits),
/// low nibble = digit (0-9).
/// Sign is in the zone nibble of the last byte: 0xD = negative, 0xC/0xF = positive.
fn decode_zoned(bytes: &[u8], _precision: u32, scale: u32, signed: bool) -> Decimal {
    if bytes.is_empty() {
        return Decimal::ZERO;
    }
    let mut result: i64 = 0;
    let mut negative = false;

    for (i, &b) in bytes.iter().enumerate() {
        let digit = i64::from(b & 0x0F);
        result = result * 10 + digit;

        // Check sign nibble on last byte
        if signed && i == bytes.len() - 1 {
            let zone = (b >> 4) & 0x0F;
            if zone == 0x0D {
                negative = true;
            }
        }
    }

    if negative {
        result = -result;
    }

    let mut d = Decimal::new(result, scale);
    d.normalize_assign();
    d
}

/// Decode packed decimal (COMP-3) from raw bytes into a Decimal.
///
/// Layout: pairs of BCD digits, last nibble is sign (0xC=+, 0xD=-, 0xF=unsigned).
fn decode_packed(bytes: &[u8], _precision: u32, scale: u32, signed: bool) -> Decimal {
    if bytes.is_empty() {
        return Decimal::ZERO;
    }
    let mut result: i64 = 0;
    let mut negative = false;

    for (i, &b) in bytes.iter().enumerate() {
        let high = i64::from((b >> 4) & 0x0F);
        let low = i64::from(b & 0x0F);

        if i == bytes.len() - 1 {
            // Last byte: high nibble is digit, low nibble is sign
            result = result * 10 + high;
            if signed && low == 0x0D {
                negative = true;
            }
        } else {
            result = result * 10 + high;
            result = result * 10 + low;
        }
    }

    if negative {
        result = -result;
    }

    let mut d = Decimal::new(result, scale);
    d.normalize_assign();
    d
}

/// Decode binary (COMP/COMP-5) from big-endian bytes.
fn decode_binary(bytes: &[u8], size: usize, signed: bool) -> i64 {
    let actual = bytes.len().min(size);
    if actual == 0 {
        return 0;
    }
    // Pad to 8 bytes (big-endian)
    let mut buf = [0u8; 8];
    let offset = 8 - actual;

    // Sign extension for signed values
    if signed && actual > 0 && (bytes[0] & 0x80) != 0 {
        buf.fill(0xFF);
    }

    buf[offset..offset + actual].copy_from_slice(&bytes[..actual]);
    i64::from_be_bytes(buf)
}

/// Build a comparator function from a list of sort key specifications.
///
/// The returned closure compares two records (byte slices) according to the
/// multi-key specification. Keys are applied in order; the first key that
/// produces a non-equal comparison determines the result.
///
/// The `collating` table is used for alphanumeric key comparison.
#[must_use]
pub fn build_comparator(
    keys: &[SortKeySpec],
    collating: &CollatingTable,
) -> RecordComparator {
    let keys = keys.to_vec();
    let collating = collating.clone();

    Box::new(move |a: &[u8], b: &[u8]| -> Ordering {
        for key in &keys {
            let ka = extract_key_bytes(a, key);
            let kb = extract_key_bytes(b, key);

            let ord = compare_key(ka, kb, &key.data_type, &collating);
            let ord = if key.ascending { ord } else { ord.reverse() };

            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphanumeric_ascending() {
        let keys = vec![SortKeySpec::alphanumeric(0, 5)];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        assert_eq!(cmp(b"AAAAA", b"BBBBB"), Ordering::Less);
        assert_eq!(cmp(b"BBBBB", b"AAAAA"), Ordering::Greater);
        assert_eq!(cmp(b"HELLO", b"HELLO"), Ordering::Equal);
    }

    #[test]
    fn test_alphanumeric_descending() {
        let keys = vec![SortKeySpec::new(0, 5, false, SortKeyType::Alphanumeric)];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        assert_eq!(cmp(b"AAAAA", b"BBBBB"), Ordering::Greater);
        assert_eq!(cmp(b"BBBBB", b"AAAAA"), Ordering::Less);
    }

    #[test]
    fn test_multi_key() {
        // Key 1: bytes 0-2 ascending, Key 2: bytes 3-5 descending
        let keys = vec![
            SortKeySpec::new(0, 3, true, SortKeyType::Alphanumeric),
            SortKeySpec::new(3, 3, false, SortKeyType::Alphanumeric),
        ];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        // Same first key, different second key -- descending
        assert_eq!(cmp(b"AAAAAA", b"AAABBB"), Ordering::Greater);
        // Different first key takes priority
        assert_eq!(cmp(b"BBBAAA", b"AAABBB"), Ordering::Greater);
    }

    #[test]
    fn test_packed_decimal_key() {
        // Pack 12345 (unsigned): 0x01 0x23 0x45 0x0F -> but let's use correct encoding
        // PIC 9(5) COMP-3 = 3 bytes: 0x12 0x34 0x5F (5 digits + sign)
        let keys = vec![SortKeySpec::packed(0, 3, 5, 0, false)];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        let rec_a = [0x01u8, 0x23, 0x4F]; // 01234 unsigned
        let rec_b = [0x05u8, 0x67, 0x8F]; // 05678 unsigned
        assert_eq!(cmp(&rec_a, &rec_b), Ordering::Less);
    }

    #[test]
    fn test_packed_decimal_signed() {
        let keys = vec![SortKeySpec::packed(0, 3, 5, 0, true)];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        let positive = [0x01u8, 0x23, 0x4C]; // +01234
        let negative = [0x01u8, 0x23, 0x4D]; // -01234
        assert_eq!(cmp(&negative, &positive), Ordering::Less);
    }

    #[test]
    fn test_binary_key_signed() {
        let keys = vec![SortKeySpec::new(
            0,
            4,
            true,
            SortKeyType::Binary {
                size: 4,
                signed: true,
            },
        )];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        let val_100 = 100i32.to_be_bytes();
        let val_neg = (-50i32).to_be_bytes();
        assert_eq!(cmp(&val_neg, &val_100), Ordering::Less);
    }

    #[test]
    fn test_binary_key_unsigned() {
        let keys = vec![SortKeySpec::new(
            0,
            2,
            true,
            SortKeyType::Binary {
                size: 2,
                signed: false,
            },
        )];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        let val_a = 100u16.to_be_bytes();
        let val_b = 200u16.to_be_bytes();
        assert_eq!(cmp(&val_a, &val_b), Ordering::Less);
    }

    #[test]
    fn test_zoned_decimal_key() {
        // Zoned: PIC 9(3), each byte = 0xFd where d is digit
        // "123" in zoned = [0xF1, 0xF2, 0xF3]
        let keys = vec![SortKeySpec::new(
            0,
            3,
            true,
            SortKeyType::ZonedDecimal {
                precision: 3,
                scale: 0,
                signed: false,
            },
        )];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        let rec_a = [0xF1u8, 0xF2, 0xF3]; // 123
        let rec_b = [0xF4u8, 0xF5, 0xF6]; // 456
        assert_eq!(cmp(&rec_a, &rec_b), Ordering::Less);
    }

    #[test]
    fn test_zoned_decimal_signed_negative() {
        let keys = vec![SortKeySpec::new(
            0,
            3,
            true,
            SortKeyType::ZonedDecimal {
                precision: 3,
                scale: 0,
                signed: true,
            },
        )];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        let positive = [0xF1u8, 0xF2, 0xC3]; // +123 (zone C = positive)
        let negative = [0xF1u8, 0xF2, 0xD3]; // -123 (zone D = negative)
        assert_eq!(cmp(&negative, &positive), Ordering::Less);
    }

    #[test]
    fn test_empty_record() {
        let keys = vec![SortKeySpec::alphanumeric(0, 5)];
        let collating = CollatingTable::native_ascii();
        let cmp = build_comparator(&keys, &collating);

        // Empty vs non-empty: empty padded with spaces
        assert_eq!(cmp(b"", b"AAAAA"), Ordering::Less);
    }
}
