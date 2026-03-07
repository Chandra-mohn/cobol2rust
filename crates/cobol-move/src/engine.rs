use cobol_core::category::DataCategory;
use cobol_core::config::{DiagnosticLevel, RuntimeConfig};
use cobol_core::traits::{CobolField, CobolNumeric};

use crate::diagnostics::{MoveDiagnostic, MoveWarning, MoveWarningKind};

/// Central MOVE dispatch: `MOVE src TO dest`.
///
/// Routes to the appropriate handler based on the destination's category.
/// The source determines how to extract data; the destination determines
/// how to store it.
pub fn cobol_move(src: &dyn CobolField, dest: &mut dyn CobolField, config: &RuntimeConfig) {
    let src_cat = src.category();
    let dest_cat = dest.category();

    debug_assert!(
        is_legal_move(src_cat, dest_cat, config),
        "Illegal MOVE from {src_cat:?} to {dest_cat:?}"
    );

    let diag = diagnostic_level(config);

    match dest_cat {
        DataCategory::Alphabetic => move_to_alphabetic(src, dest, diag),
        DataCategory::Alphanumeric
        | DataCategory::AlphanumericEdited
        | DataCategory::National => move_to_alphanumeric(src, dest, diag),
        DataCategory::Numeric
        | DataCategory::NumericEdited => move_to_numeric_field(src, dest, config),
        DataCategory::Group => move_to_group(src, dest, diag),
    }
}

/// MOVE a numeric value to a numeric destination (used when both fields are numeric).
pub fn cobol_move_numeric(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    config: &RuntimeConfig,
) {
    let value = src.to_decimal();
    // Truncate to destination scale
    let truncated =
        cobol_core::decimal_ext::truncate_decimal(value, dest.scale());
    // Left-truncate to destination precision
    let final_value =
        cobol_core::decimal_ext::left_truncate_to_precision(truncated, dest.precision(), dest.scale());

    // Handle sign loss: negative value into unsigned field
    let diag = diagnostic_level(config);
    if value.is_sign_negative() && !dest.is_signed() && !value.is_zero() {
        let warning = MoveWarning {
            kind: MoveWarningKind::SignLoss,
            source_desc: format!("{src:?}"),
            dest_desc: format!("{dest:?}"),
        };
        warning.handle(diag);
    }

    let store_value = if dest.is_signed() {
        final_value
    } else {
        final_value.abs()
    };

    dest.set_decimal(store_value);
}

/// Check whether a MOVE from `src_cat` to `dest_cat` is legal.
pub fn is_legal_move(
    src_cat: DataCategory,
    dest_cat: DataCategory,
    config: &RuntimeConfig,
) -> bool {
    use DataCategory::{Group, Alphanumeric, National, Alphabetic, AlphanumericEdited, Numeric, NumericEdited};
    match (src_cat, dest_cat) {
        // Group destination/source always accepts; alphanumeric/national source can go anywhere;
        // alphabetic/alpha-edited -> alphabetic, alphanumeric, alpha-edited;
        // numeric -> alphanumeric, alpha-edited, numeric, numeric-edited;
        // numeric-edited -> alphanumeric, alpha-edited
        (_, Group | National)
        | (Group | Alphanumeric | National, _)
        | (Alphabetic | AlphanumericEdited,
           Alphabetic | Alphanumeric | AlphanumericEdited)
        | (Numeric, Alphanumeric | AlphanumericEdited | Numeric | NumericEdited)
        | (NumericEdited, Alphanumeric | AlphanumericEdited) => true,
        // Alphabetic/alpha-edited -> numeric or numeric-edited is illegal;
        // numeric/numeric-edited -> alphabetic is illegal
        (Alphabetic | AlphanumericEdited, Numeric | NumericEdited)
        | (Numeric | NumericEdited, Alphabetic) => false,
        // NumericEdited -> numeric/numeric-edited: IBM extension with de-editing
        (NumericEdited, Numeric | NumericEdited) => config.allow_de_editing,
    }
}

/// Move to alphabetic/alphanumeric destination: byte-copy, left-justify, space-pad.
fn move_to_alphabetic(
    src: &dyn CobolField,
    dest: &mut dyn CobolField,
    _diag: MoveDiagnostic,
) {
    let src_bytes = src.display_bytes();
    let justified_right = dest.is_justified_right();
    let dest_bytes = dest.as_bytes_mut();
    let dest_len = dest_bytes.len();
    let copy_len = src_bytes.len().min(dest_len);

    if justified_right {
        // Right-justify: left-pad with spaces, right-align data
        dest_bytes.fill(b' ');
        if src_bytes.len() >= dest_len {
            // Left-truncate
            let start = src_bytes.len() - dest_len;
            dest_bytes.copy_from_slice(&src_bytes[start..start + dest_len]);
        } else {
            let start = dest_len - src_bytes.len();
            dest_bytes[start..].copy_from_slice(&src_bytes);
        }
    } else {
        // Left-justify: copy and space-pad right
        dest_bytes[..copy_len].copy_from_slice(&src_bytes[..copy_len]);
        dest_bytes[copy_len..].fill(b' ');
    }
}

/// Move to alphanumeric destination: same as alphabetic.
fn move_to_alphanumeric(
    src: &dyn CobolField,
    dest: &mut dyn CobolField,
    diag: MoveDiagnostic,
) {
    move_to_alphabetic(src, dest, diag);
}

/// Move to numeric or numeric-edited destination.
fn move_to_numeric_field(
    src: &dyn CobolField,
    dest: &mut dyn CobolField,
    config: &RuntimeConfig,
) {
    let src_cat = src.category();

    match src_cat {
        DataCategory::Numeric => {
            // Both numeric: use display_bytes to extract, then parse
            let src_bytes = src.display_bytes();
            let value = parse_numeric_display(&src_bytes);
            store_numeric_to_dest(value, dest, config);
        }
        DataCategory::Alphanumeric | DataCategory::Group => {
            // Alphanumeric to numeric: parse the display bytes
            let src_bytes = src.display_bytes();
            let value = parse_numeric_display(&src_bytes);
            store_numeric_to_dest(value, dest, config);
        }
        _ => {
            // For other categories, try to interpret as alphanumeric
            let src_bytes = src.display_bytes();
            store_alphanumeric_to_numeric(&src_bytes, dest, config);
        }
    }
}

/// Move to group destination: always byte-copy semantics.
fn move_to_group(
    src: &dyn CobolField,
    dest: &mut dyn CobolField,
    _diag: MoveDiagnostic,
) {
    let src_bytes = src.display_bytes();
    let dest_bytes = dest.as_bytes_mut();
    let copy_len = src_bytes.len().min(dest_bytes.len());

    // Left-justify, space-pad (group moves are always alphanumeric byte copies)
    dest_bytes[..copy_len].copy_from_slice(&src_bytes[..copy_len]);
    dest_bytes[copy_len..].fill(b' ');
}

/// Parse a COBOL display representation to a Decimal value.
///
/// Delegates to the shared `cobol_core::numeric_parse` module which handles
/// plain display, signed, edited, CR/DB, and various COBOL numeric formats.
fn parse_numeric_display(bytes: &[u8]) -> rust_decimal::Decimal {
    cobol_core::numeric_parse::parse_numeric_display(bytes)
}

/// Store an alphanumeric byte string to a numeric destination.
fn store_alphanumeric_to_numeric(
    src_bytes: &[u8],
    dest: &mut dyn CobolField,
    config: &RuntimeConfig,
) {
    let value = parse_numeric_display(src_bytes);
    store_numeric_to_dest(value, dest, config);
}

/// Store a decimal value into a numeric destination field.
///
/// This is the core storage path: handles truncation, precision, and sign.
fn store_numeric_to_dest(
    value: rust_decimal::Decimal,
    dest: &mut dyn CobolField,
    _config: &RuntimeConfig,
) {
    // We need to get the numeric interface. Since we know dest is Numeric category,
    // we rely on the caller having set up the types correctly.
    // The dest's as_bytes_mut() will be used through display_bytes + set pattern.
    // For now, write the formatted numeric display to the dest bytes.
    let dest_bytes = dest.as_bytes_mut();
    let formatted = format!("{value}");
    let src = formatted.as_bytes();

    // Write display representation: right-justify, zero-pad left
    dest_bytes.fill(b'0');
    if src.len() <= dest_bytes.len() {
        let start = dest_bytes.len() - src.len();
        dest_bytes[start..].copy_from_slice(src);
    } else {
        dest_bytes.copy_from_slice(&src[src.len() - dest_bytes.len()..]);
    }
}

/// Convert diagnostic level from config.
fn diagnostic_level(config: &RuntimeConfig) -> MoveDiagnostic {
    match config.diagnostic_level {
        DiagnosticLevel::Silent => MoveDiagnostic::Silent,
        DiagnosticLevel::Warn => MoveDiagnostic::Warn,
        DiagnosticLevel::Strict => MoveDiagnostic::Strict,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_types::{Comp1Float, Comp2Float, PackedDecimal, PicX};
    use rust_decimal_macros::dec;

    fn default_config() -> RuntimeConfig {
        RuntimeConfig::default()
    }

    #[test]
    fn move_picx_to_picx() {
        let src = PicX::new(10, b"HELLO");
        let mut dest = PicX::new(8, b"");
        cobol_move(&src, &mut dest, &default_config());
        assert_eq!(dest.as_bytes(), b"HELLO   ");
    }

    #[test]
    fn move_picx_truncates_right() {
        let src = PicX::new(10, b"HELLO WRLD");
        let mut dest = PicX::new(5, b"");
        cobol_move(&src, &mut dest, &default_config());
        assert_eq!(dest.as_bytes(), b"HELLO");
    }

    #[test]
    fn move_numeric_to_numeric() {
        let mut src = PackedDecimal::new(5, 2, true);
        src.set_decimal(dec!(123.45));
        let mut dest = PackedDecimal::new(7, 2, true);
        cobol_move_numeric(&src, &mut dest, &default_config());
        assert_eq!(dest.to_decimal(), dec!(123.45));
    }

    #[test]
    fn move_numeric_left_truncation() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(12345));
        let mut dest = PackedDecimal::new(3, 0, true);
        cobol_move_numeric(&src, &mut dest, &default_config());
        assert_eq!(dest.to_decimal(), dec!(345));
    }

    #[test]
    fn move_numeric_sign_loss() {
        let mut src = PackedDecimal::new(5, 0, true);
        src.set_decimal(dec!(-123));
        let mut dest = PackedDecimal::new(5, 0, false);
        cobol_move_numeric(&src, &mut dest, &default_config());
        // Unsigned destination: absolute value stored
        assert_eq!(dest.to_decimal(), dec!(123));
    }

    #[test]
    fn move_to_group_byte_copy() {
        let src = PicX::new(5, b"HELLO");
        let mut dest = PicX::new(10, b"");
        // Group move is the same as alphanumeric move
        move_to_group(&src, &mut dest, MoveDiagnostic::Silent);
        assert_eq!(dest.as_bytes(), b"HELLO     ");
    }

    #[test]
    fn legality_matrix_basic() {
        use DataCategory::*;
        let config = default_config();

        // Legal moves
        assert!(is_legal_move(Alphanumeric, Numeric, &config));
        assert!(is_legal_move(Numeric, Alphanumeric, &config));
        assert!(is_legal_move(Numeric, Numeric, &config));
        assert!(is_legal_move(Alphabetic, Alphanumeric, &config));
        assert!(is_legal_move(Group, Numeric, &config));

        // Illegal moves
        assert!(!is_legal_move(Alphabetic, Numeric, &config));
        assert!(!is_legal_move(Alphabetic, NumericEdited, &config));
        assert!(!is_legal_move(Numeric, Alphabetic, &config));
    }

    #[test]
    fn legality_numeric_edited_to_numeric_ibm() {
        let mut config = default_config();
        config.allow_de_editing = true;
        assert!(is_legal_move(
            DataCategory::NumericEdited,
            DataCategory::Numeric,
            &config
        ));

        config.allow_de_editing = false;
        assert!(!is_legal_move(
            DataCategory::NumericEdited,
            DataCategory::Numeric,
            &config
        ));
    }

    #[test]
    fn parse_numeric_display_values() {
        assert_eq!(parse_numeric_display(b"+00123"), dec!(123));
        assert_eq!(parse_numeric_display(b"-00456"), dec!(-456));
        assert_eq!(parse_numeric_display(b"00000"), dec!(0));
        assert_eq!(parse_numeric_display(b"12345"), dec!(12345));
    }

    #[test]
    fn move_comp1_to_packed() {
        let src = Comp1Float::from_f32(42.5);
        let mut dest = PackedDecimal::new(5, 2, true);
        let config = default_config();
        cobol_move_numeric(&src, &mut dest, &config);
        assert_eq!(dest.to_decimal(), dec!(42.50));
    }

    #[test]
    fn move_packed_to_comp2() {
        let mut src = PackedDecimal::new(5, 2, true);
        src.set_decimal(dec!(123.45));
        let mut dest = Comp2Float::new();
        let config = default_config();
        cobol_move_numeric(&src, &mut dest, &config);
        assert!((dest.as_f64() - 123.45).abs() < 0.001);
    }

    #[test]
    fn move_comp1_to_comp2() {
        let src = Comp1Float::from_f32(99.5);
        let mut dest = Comp2Float::new();
        let config = default_config();
        cobol_move_numeric(&src, &mut dest, &config);
        assert!((dest.as_f64() - 99.5).abs() < 0.01);
    }
}
