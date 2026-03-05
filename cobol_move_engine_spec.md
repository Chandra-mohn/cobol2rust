# COBOL MOVE Engine: Complete Specification

## Purpose

This document specifies the complete semantics of COBOL's MOVE statement and
all related data transfer operations for the cobol2rust transpiler runtime
library. The MOVE engine is the most pervasive mechanism in COBOL -- virtually
every data assignment, arithmetic result store, and I/O operation passes through
MOVE semantics. Getting this right is critical to semantic fidelity.

This spec covers: MOVE, MOVE CORRESPONDING, INITIALIZE, implicit MOVE contexts
(COMPUTE targets, READ INTO, WRITE FROM, parameter passing), de-editing,
figurative constants, and the complete source-to-destination conversion matrix
with Rust implementation pseudocode.

---

## Table of Contents

1. [Fundamental Principle](#1-fundamental-principle)
2. [Data Categories](#2-data-categories)
3. [MOVE Legality Matrix](#3-move-legality-matrix)
4. [Conversion Rules by Destination Category](#4-conversion-rules-by-destination-category)
   - 4.1 [Receiving Alphabetic](#41-receiving-alphabetic)
   - 4.2 [Receiving Alphanumeric](#42-receiving-alphanumeric)
   - 4.3 [Receiving Alphanumeric-Edited](#43-receiving-alphanumeric-edited)
   - 4.4 [Receiving Numeric (Integer and Fixed-Point)](#44-receiving-numeric-integer-and-fixed-point)
   - 4.5 [Receiving Numeric-Edited](#45-receiving-numeric-edited)
   - 4.6 [Receiving Group Item](#46-receiving-group-item)
5. [Special MOVE Operations](#5-special-move-operations)
   - 5.1 [MOVE CORRESPONDING](#51-move-corresponding)
   - 5.2 [De-editing (Numeric-Edited to Numeric)](#52-de-editing-numeric-edited-to-numeric)
   - 5.3 [JUSTIFIED RIGHT](#53-justified-right)
   - 5.4 [BLANK WHEN ZERO](#54-blank-when-zero)
   - 5.5 [Reference Modification](#55-reference-modification)
   - 5.6 [Subscripting and Indexing](#56-subscripting-and-indexing)
6. [Figurative Constants](#6-figurative-constants)
7. [INITIALIZE Verb](#7-initialize-verb)
8. [Implicit MOVE Contexts](#8-implicit-move-contexts)
9. [Rust Implementation Architecture](#9-rust-implementation-architecture)
10. [Worked Examples](#10-worked-examples)
11. [Dialect Variations](#11-dialect-variations)

---

## 1. Fundamental Principle

**The destination determines the conversion rules.**

COBOL's MOVE statement is not a byte copy. It is a type-aware, size-adjusting,
format-converting data transfer operation. The process is:

1. Classify the source into a data category
2. Classify the destination into a data category
3. Check legality of the source-destination combination
4. Extract the logical value from the source
5. Apply conversion rules specific to the destination category
6. Store the result, applying padding or truncation as needed

The source contributes its **value**; the destination contributes the
**format, size, and alignment rules**.

Exception: GROUP moves always use alphanumeric (byte-copy) semantics regardless
of the categories of subordinate items.

---

## 2. Data Categories

COBOL classifies every data item into exactly one of these categories based on
its PICTURE string and USAGE clause:

### 2.1 Category Table

| Category | PIC Pattern | Examples | Description |
|---|---|---|---|
| **Alphabetic** | Only `A`, `B` | `PIC A(10)`, `PIC AAABAA` | Letters and spaces only |
| **Alphanumeric** | Only `X`, or mix of `A`+`9`+`X` | `PIC X(20)`, `PIC X(5)9(3)` | Any character |
| **Alphanumeric-Edited** | `X`/`A`/`9` with `B`,`0`,`/` insertions | `PIC X(5)BX(5)`, `PIC 99/99/99` | Alphanumeric with insertions |
| **Numeric** | Only `9`,`S`,`V`,`P` (no edit chars) | `PIC 9(5)`, `PIC S9(7)V99`, `PIC S9(4) COMP-3` | Pure numeric, any USAGE |
| **Numeric-Edited** | `9` with `Z`,`*`,`$`,`+`,`-`,`,`,`.`,`CR`,`DB`,`B`,`0`,`/` | `PIC $ZZ,ZZ9.99`, `PIC -(5)9.99` | Numeric with editing |
| **National** | `N` or `G` | `PIC N(10)` | Double-byte characters (Phase 3) |
| **Group** | No PICTURE (has subordinates) | `01 CUSTOMER-REC.` | Treated as alphanumeric for MOVE |

### 2.2 Category Determination Rules

- If a data item has **no PICTURE** and has subordinates -> **Group**
- If PICTURE contains only `A` and `B` -> **Alphabetic**
- If PICTURE contains only `X`, or a combination of `A`, `X`, `9` without
  editing symbols -> **Alphanumeric**
- If PICTURE contains `X`/`A` combined with `B`, `0`, `/` insertion
  characters -> **Alphanumeric-Edited**
- If PICTURE contains only `9`, `S`, `V`, `P` (no editing symbols) ->
  **Numeric**, regardless of USAGE (DISPLAY, COMP, COMP-3, etc.)
- If PICTURE contains `9` with editing symbols (`Z`, `*`, `$`, `+`, `-`,
  `,`, `.`, `CR`, `DB`) -> **Numeric-Edited**
- If PICTURE contains `N` or `G` -> **National** (DBCS)

### 2.3 Literals and Figurative Constants

These are classified contextually when used as MOVE sources:

| Source | Category for MOVE | Notes |
|---|---|---|
| Non-numeric literal `"HELLO"` | Alphanumeric | Always |
| Numeric literal `12345` | Numeric (integer) | Treated as unsigned integer |
| Numeric literal `123.45` | Numeric (fixed-point) | Implied 2 decimal places |
| `SPACES` / `SPACE` | Alphanumeric | `X"20"` repeated |
| `ZEROS` / `ZEROES` / `ZERO` | **Depends on context** | Alphanumeric `"0"` or numeric `0` |
| `HIGH-VALUES` / `HIGH-VALUE` | Alphanumeric | `X"FF"` repeated (ASCII), `X"FF"` (EBCDIC varies) |
| `LOW-VALUES` / `LOW-VALUE` | Alphanumeric | `X"00"` repeated |
| `QUOTES` / `QUOTE` | Alphanumeric | `"` character repeated |
| `ALL "literal"` | Alphanumeric | Repeating pattern to fill dest length |
| `NULL` / `NULLS` | Special | Pointer value, not for MOVE to non-pointer |

**ZEROS context rule**: When ZEROS is moved to a numeric destination, it acts
as numeric zero. When moved to an alphanumeric destination, it acts as the
character `"0"` repeated to fill the field.

---

## 3. MOVE Legality Matrix

This matrix shows whether a MOVE from source (rows) to destination (columns)
is legal per the COBOL standard. IBM extensions are marked with `[IBM]`.

### 3.1 Standard Legality Matrix

| Source \ Dest | Alphabetic | Alphanumeric | Alpha-Edited | Numeric | Numeric-Edited | Group |
|---|---|---|---|---|---|---|
| **Alphabetic** | Yes | Yes | Yes | **NO** | **NO** | Yes |
| **Alphanumeric** | Yes | Yes | Yes | Yes(1) | Yes(1) | Yes |
| **Alpha-Edited** | Yes | Yes | Yes | **NO** | **NO** | Yes |
| **Numeric (integer)** | **NO** | Yes | Yes | Yes | Yes | Yes |
| **Numeric (non-int)** | **NO** | Yes | Yes | Yes | Yes | Yes |
| **Numeric-Edited** | **NO** | Yes | Yes | **NO**(2) | **NO**(2) | Yes |
| **Group** | Yes | Yes | Yes | Yes(3) | Yes(3) | Yes |

Notes:
1. Alphanumeric to Numeric: The source must contain a valid numeric string
   (digits, optional sign, optional decimal point). If not, results are
   undefined (no runtime error in standard COBOL -- data corruption).
2. Numeric-Edited to Numeric/Numeric-Edited: **Illegal in ANSI standard**.
   IBM Enterprise COBOL allows this as an extension with de-editing. See
   Section 5.2.
3. Group to Numeric/Numeric-Edited: Legal because GROUP is treated as
   alphanumeric, but the programmer is responsible for ensuring the group
   contains valid numeric data.

### 3.2 Figurative Constants Legality

All figurative constants (SPACES, ZEROS, HIGH-VALUES, LOW-VALUES, ALL literal)
can be moved to any destination category:

- SPACES to numeric: sets to zero (IBM) or undefined (standard)
- ZEROS to alphabetic: fills with character `"0"`
- HIGH-VALUES/LOW-VALUES to numeric: treated as alphanumeric byte fill
- ALL literal to any: repeating pattern, treated as alphanumeric

### 3.3 Illegal MOVE Summary

The illegal combinations follow a logical pattern:

- **Alphabetic/Alpha-Edited -> Numeric/Numeric-Edited**: Letters cannot be
  converted to numbers
- **Numeric-Edited -> Numeric/Numeric-Edited**: The editing characters make
  numeric extraction ambiguous (without de-editing extension)
- **Numeric (non-integer) -> Alphabetic**: Decimal points are not alphabetic
  characters

---

## 4. Conversion Rules by Destination Category

### 4.1 Receiving Alphabetic

**Alignment**: Left-justified
**Padding**: Space-filled on right
**Truncation**: Right-truncated if source is longer

**Valid sources**: Alphabetic, Alphanumeric, Alphanumeric-Edited, Group,
figurative constants

**Rule**: The source's display representation is copied left-to-right. If the
source is shorter, remaining positions are filled with spaces. If longer,
excess characters on the right are discarded.

```
Source: "HELLO WORLD" (11 bytes)
Dest:  PIC A(8)
Result: "HELLO WO" (right-truncated)

Source: "HI" (2 bytes)
Dest:  PIC A(8)
Result: "HI      " (space-padded right)
```

**Rust pseudocode**:

```rust
fn move_to_alphabetic(src: &dyn CobolField, dest: &mut AlphabeticField) {
    let src_bytes = src.as_display_bytes();
    let dest_len = dest.byte_length();
    for i in 0..dest_len {
        if i < src_bytes.len() {
            dest.data[i] = src_bytes[i];
        } else {
            dest.data[i] = b' '; // space-pad right
        }
    }
    // Right-truncation is implicit: we stop at dest_len
}
```

### 4.2 Receiving Alphanumeric

**Alignment**: Left-justified (unless JUSTIFIED RIGHT specified)
**Padding**: Space-filled on right (or left if JUSTIFIED RIGHT)
**Truncation**: Right-truncated (or left if JUSTIFIED RIGHT)

**Valid sources**: ALL categories (alphanumeric is the universal receiver)

**Rule by source category**:

| Source Category | Behavior |
|---|---|
| Alphabetic | Direct byte copy, left-justified, space-padded right |
| Alphanumeric | Direct byte copy, left-justified, space-padded right |
| Alpha-Edited | Direct byte copy (editing chars included), left-justified |
| Numeric (DISPLAY) | Move display representation (with sign if SEPARATE), left-justified |
| Numeric (COMP/COMP-3) | Convert to display format first, then left-justified |
| Numeric-Edited | Direct byte copy of edited representation, left-justified |
| Group | Byte copy (group treated as alphanumeric) |

**Numeric source detail**: When a numeric source is moved to alphanumeric,
the source's **display representation** is used. This means:

- For `PIC S9(5)` with value -12345 and TRAILING SEPARATE sign:
  display = `"12345-"` (6 bytes moved)
- For `PIC 9(5)` with value 345:
  display = `"00345"` (5 bytes, leading zeros included)
- For `PIC S9(5) COMP-3` with value +12345:
  converted to display = `"12345"` (5 bytes, sign not in display unless
  SEPARATE CHARACTER specified on the destination -- but destination is
  alphanumeric, so sign handling depends on implementation)

**IBM note**: When COMP/COMP-3 is moved to alphanumeric, IBM converts to the
DISPLAY equivalent as if the source were `PIC [S]9(n)[V9(m)]` DISPLAY. The
sign appears as an overpunch on the last byte (zone nibble encoding) unless
SIGN IS SEPARATE.

```rust
fn move_to_alphanumeric(src: &dyn CobolField, dest: &mut AlphanumericField) {
    let src_bytes = match src.category() {
        Category::Numeric => src.to_display_representation(),
        _ => src.as_display_bytes().to_vec(),
    };
    let dest_len = dest.byte_length();
    let justified_right = dest.is_justified_right();

    if justified_right {
        // Right-justified, space-pad left, left-truncate
        let start = if src_bytes.len() >= dest_len {
            src_bytes.len() - dest_len // left-truncate
        } else {
            0
        };
        let pad_count = dest_len.saturating_sub(src_bytes.len());
        for i in 0..pad_count {
            dest.data[i] = b' ';
        }
        for i in 0..(dest_len - pad_count) {
            dest.data[pad_count + i] = src_bytes[start + i];
        }
    } else {
        // Left-justified, space-pad right, right-truncate
        for i in 0..dest_len {
            if i < src_bytes.len() {
                dest.data[i] = src_bytes[i];
            } else {
                dest.data[i] = b' ';
            }
        }
    }
}
```

### 4.3 Receiving Alphanumeric-Edited

**Alignment**: Left-justified
**Padding**: Space-filled on right
**Truncation**: Right-truncated

**Valid sources**: Alphabetic, Alphanumeric, Alphanumeric-Edited, Numeric
(integer only), Numeric-Edited, Group

**Rule**: The source value is treated as an alphanumeric string and placed into
the destination. Insertion characters (`B`, `0`, `/`) in the destination PICTURE
are applied at their defined positions.

Insertion characters in the destination:
- `B` inserts a space at that position
- `0` inserts a zero character at that position
- `/` inserts a slash at that position

The source characters fill the non-insertion positions left-to-right.

```
Source: "ABCDEF"
Dest:   PIC X(3)BX(3)    (mask: ___B___)
Result: "ABC DEF"

Source: "12345678"
Dest:   PIC 99/99/9999
Result: "12/34/5678"
```

```rust
fn move_to_alpha_edited(src: &dyn CobolField, dest: &mut AlphaEditedField) {
    let src_bytes = match src.category() {
        Category::Numeric => src.to_display_representation(),
        _ => src.as_display_bytes().to_vec(),
    };
    let mut src_idx = 0;
    for (dest_idx, pic_char) in dest.pic_mask().iter().enumerate() {
        match pic_char {
            PicChar::B => dest.data[dest_idx] = b' ',
            PicChar::Zero => dest.data[dest_idx] = b'0',
            PicChar::Slash => dest.data[dest_idx] = b'/',
            PicChar::X | PicChar::A | PicChar::Nine => {
                if src_idx < src_bytes.len() {
                    dest.data[dest_idx] = src_bytes[src_idx];
                    src_idx += 1;
                } else {
                    dest.data[dest_idx] = b' '; // space-pad
                }
            }
        }
    }
}
```

### 4.4 Receiving Numeric (Integer and Fixed-Point)

This is the most complex conversion. The destination is a pure numeric field
(any USAGE: DISPLAY, COMP, COMP-3, COMP-5).

**Alignment**: Decimal-point aligned (right-justified for integers)
**Padding**: Zero-filled on both sides of decimal point
**Truncation**: LEFT-truncated for integer part, RIGHT-truncated for decimal part

This is the critical behavior: **COBOL truncates from the LEFT (high-order),
not the right.** This means overflow silently drops the most significant digits.

**Valid sources**: Alphanumeric (must contain valid numeric string), Numeric,
Group (treated as alphanumeric), ZEROS, figurative constants

#### 4.4.1 Numeric to Numeric

The core case. Decimal points are aligned, then padding/truncation applied.

```
Source: PIC 9(5)V99   value = 12345.67
Dest:   PIC 9(3)V9
Step 1: Align decimal points
        Source: [1][2][3][4][5] . [6][7]
        Dest:   [_][_][_]       . [_]
Step 2: Decimal part: copy left-to-right, truncate right
        Decimal: [6] (7 is truncated)
Step 3: Integer part: copy right-to-left, truncate left
        Integer: [3][4][5] (1,2 are truncated)
Result: 345.6
```

```
Source: PIC 9(2)V9    value = 12.3
Dest:   PIC 9(5)V999
Step 1: Align decimal points
Step 2: Decimal: [3][0][0] (zero-padded right)
Step 3: Integer: [0][0][0][1][2] (zero-padded left)
Result: 00012.300
```

**Sign handling**:

| Source Sign | Dest Sign | Result |
|---|---|---|
| Signed (+) | Signed | Positive sign stored |
| Signed (-) | Signed | Negative sign stored |
| Signed (+/-) | Unsigned | Sign is **lost** -- absolute value stored |
| Unsigned | Signed | Treated as positive |
| Unsigned | Unsigned | No sign processing |

```rust
fn move_numeric_to_numeric(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
) {
    let src_value = src.to_decimal(); // rust_decimal::Decimal

    // Handle sign
    let value = if !dest.is_signed() {
        src_value.abs()
    } else {
        src_value
    };

    // Decimal alignment and truncation
    let dest_scale = dest.scale();
    let dest_precision = dest.precision(); // total digits (integer + decimal)
    let dest_integer_digits = dest_precision - dest_scale;

    // Rescale to destination's decimal places (truncates right of decimal)
    // Note: COBOL truncates, does NOT round (unless ROUNDED specified)
    let scaled = value.trunc_to_scale(dest_scale);

    // Left-truncation: keep only the rightmost dest_integer_digits of integer part
    let max_integer_value = 10_i128.pow(dest_integer_digits)
        - 1; // e.g., 999 for 3 integer digits
    let integer_part = scaled.trunc().abs();
    if integer_part > Decimal::from(max_integer_value) {
        // Left-truncate: take modulo 10^dest_integer_digits
        let modulus = Decimal::from(10_i128.pow(dest_integer_digits));
        let truncated_int = integer_part % modulus;
        let decimal_part = scaled.fract();
        let sign = if value.is_sign_negative() {
            Decimal::NEGATIVE_ONE
        } else {
            Decimal::ONE
        };
        let result = (truncated_int + decimal_part.abs()) * sign;
        dest.set_decimal(result);
    } else {
        dest.set_decimal(scaled);
    }
}
```

#### 4.4.2 Alphanumeric to Numeric

The source is treated as a string of characters that must represent a valid
numeric value. The conversion:

1. Strip leading/trailing spaces
2. Parse sign (leading `-`, `+`, or trailing sign character)
3. Parse digits and optional decimal point
4. Apply same decimal-alignment rules as numeric-to-numeric
5. If source contains non-numeric characters -> **undefined behavior**
   (no runtime error in standard COBOL; IBM may set data to zero or leave
   as-is depending on compiler options)

**IBM behavior on invalid data**: When `NUMPROC(NOPFD)` is active, IBM does
minimal validation. When `NUMPROC(PFD)` is active, all data is assumed valid.
Neither mode raises a runtime error for invalid moves -- the data is simply
corrupted.

```rust
fn move_alphanumeric_to_numeric(
    src: &dyn CobolField,
    dest: &mut dyn CobolNumeric,
) {
    let src_bytes = src.as_display_bytes();

    // Attempt to parse a numeric value from the alphanumeric source
    match parse_cobol_numeric_string(src_bytes) {
        Some(value) => {
            // Apply normal numeric-to-numeric rules
            move_numeric_value_to_numeric(value, dest);
        }
        None => {
            // Undefined behavior -- match IBM: set to zero
            dest.set_decimal(Decimal::ZERO);
            // TODO: Optionally emit a diagnostic/warning
        }
    }
}

fn parse_cobol_numeric_string(bytes: &[u8]) -> Option<Decimal> {
    // Strip leading/trailing spaces
    let trimmed = trim_spaces(bytes);
    if trimmed.is_empty() {
        return Some(Decimal::ZERO); // all spaces = zero
    }

    // Check for sign
    let (sign, digits) = match trimmed[0] {
        b'+' => (Sign::Positive, &trimmed[1..]),
        b'-' => (Sign::Negative, &trimmed[1..]),
        _ => {
            // Check trailing sign
            match trimmed[trimmed.len() - 1] {
                b'+' => (Sign::Positive, &trimmed[..trimmed.len() - 1]),
                b'-' => (Sign::Negative, &trimmed[..trimmed.len() - 1]),
                _ => (Sign::Positive, trimmed),
            }
        }
    };

    // Parse digits and optional decimal point
    let mut integer_part = Vec::new();
    let mut decimal_part = Vec::new();
    let mut seen_decimal = false;
    for &b in digits {
        match b {
            b'0'..=b'9' => {
                if seen_decimal {
                    decimal_part.push(b);
                } else {
                    integer_part.push(b);
                }
            }
            b'.' => {
                if seen_decimal {
                    return None; // two decimal points
                }
                seen_decimal = true;
            }
            b' ' => {} // ignore embedded spaces (IBM allows this)
            _ => return None, // invalid character
        }
    }

    // Build Decimal from parsed parts
    build_decimal(sign, &integer_part, &decimal_part)
}
```

#### 4.4.3 Group to Numeric

Since a GROUP item is always treated as alphanumeric for MOVE purposes, this
follows the Alphanumeric-to-Numeric rules in Section 4.4.2. The raw bytes of
the group are interpreted as a character string.

### 4.5 Receiving Numeric-Edited

**Alignment**: Determined by the editing mask
**Padding**: Fill characters per mask (spaces, asterisks, etc.)
**Truncation**: Overflow may be masked by fill characters

**Valid sources**: Numeric, Alphanumeric (must be valid numeric string),
Group (treated as alphanumeric)

**Rule**: The MOVE to a numeric-edited field is a two-step process:

1. **Extract numeric value** from the source (using numeric conversion rules)
2. **Apply the editing mask** to produce the display representation

The editing mask (PIC string) controls how the numeric value is formatted:

### 4.5.1 Editing Symbols Reference

| Symbol | Name | Behavior |
|---|---|---|
| `Z` | Zero-suppress | Replace leading zero with space |
| `*` | Check-protect | Replace leading zero with asterisk |
| `$` | Currency | Float or fixed currency symbol |
| `+` | Plus sign | Float or fixed: show `+` or `-` |
| `-` | Minus sign | Float or fixed: show `-` or space |
| `,` | Comma | Insert comma (suppressed with leading zeros) |
| `.` | Period | Decimal point (actual position in output) |
| `B` | Space insertion | Insert space at this position |
| `0` | Zero insertion | Insert `0` at this position |
| `/` | Slash insertion | Insert `/` at this position |
| `CR` | Credit | Show `CR` if negative, two spaces if positive |
| `DB` | Debit | Show `DB` if negative, two spaces if positive |

### 4.5.2 Editing Process

The editing process scans the PIC mask from left to right:

**Simple insertion** (`B`, `0`, `/`): Always insert the character at that
position. Not affected by zero suppression.

**Fixed insertion** (`.`, `,`): Insert at the position. Comma is suppressed
(replaced by space or `*`) if all digits to its left are suppressed zeros.
Period is the decimal point and is always shown.

**Zero suppression** (`Z`, `*`): Replace digit position with space (for `Z`)
or asterisk (for `*`) if the digit and all higher-order digits are zero.
Suppression stops at the first non-zero digit or at the decimal point.

**Floating signs** (`+`, `-`, `$`): When repeated, they form a floating
string. The sign/currency appears in the rightmost suppressed position. All
positions to the left become spaces.

**Fixed signs**: A single `+`, `-`, or `$` at the leftmost position is fixed --
it always appears there.

**CR/DB**: Appear at the rightmost two positions. If value is negative, `CR`
or `DB` is shown. If positive, two spaces replace them.

### 4.5.3 Editing Examples

```
PIC ZZ,ZZ9.99
Value: 1234.56    -> " 1,234.56"
Value: 0.50       -> "     0.50"
Value: 0          -> "     0.00"

PIC **,**9.99
Value: 1234.56    -> "*1,234.56"
Value: 0.50       -> "****0.50"
Value: 0          -> "****0.00"

PIC $$$,$$9.99
Value: 1234.56    -> " $1,234.56"
Value: 0.50       -> "     $0.50"

PIC -(5)9.99
Value: -12345.67  -> "-12345.67"
Value: 12345.67   -> " 12345.67"
Value: -0.50      -> "    -0.50"

PIC 9(5).99CR
Value: -12345.67  -> "12345.67CR"
Value: 12345.67   -> "12345.67  "
```

```rust
fn move_to_numeric_edited(
    src: &dyn CobolField,
    dest: &mut NumericEditedField,
) {
    // Step 1: Extract numeric value from source
    let value = match src.category() {
        Category::Numeric => src.to_numeric_value().unwrap(),
        Category::Alphanumeric | Category::Group => {
            parse_cobol_numeric_string(src.as_display_bytes())
                .unwrap_or(NumericValue::ZERO)
        }
        _ => panic!("Illegal MOVE to numeric-edited from {:?}", src.category()),
    };

    // Step 2: Split into sign, integer digits, decimal digits
    let is_negative = value.is_negative();
    let abs_value = value.abs();
    let integer_digits = extract_integer_digits(&abs_value, dest.integer_positions());
    let decimal_digits = extract_decimal_digits(&abs_value, dest.decimal_positions());

    // Step 3: Apply editing mask
    let mut output = vec![0u8; dest.byte_length()];
    let mask = dest.pic_mask();

    // Track zero suppression state
    let mut suppressing = true;  // start suppressing
    let mut src_int_idx = 0;     // index into integer_digits
    let mut src_dec_idx = 0;     // index into decimal_digits
    let mut past_decimal = false;
    let mut float_sign_placed = false;

    for (pos, sym) in mask.iter().enumerate() {
        match sym {
            // Simple insertion -- always insert
            EditSym::B => output[pos] = b' ',
            EditSym::InsertZero => output[pos] = b'0',
            EditSym::Slash => output[pos] = b'/',

            // Decimal point -- stop suppression for decimal part
            EditSym::Period => {
                output[pos] = b'.';
                suppressing = false;
                past_decimal = true;
            }

            // Comma -- suppress if still in leading zeros
            EditSym::Comma => {
                if suppressing {
                    output[pos] = dest.suppress_char(); // space or *
                } else {
                    output[pos] = b',';
                }
            }

            // Digit position with zero suppression (Z or *)
            EditSym::ZeroSuppress(fill) => {
                let digit = if past_decimal {
                    get_or_zero(&decimal_digits, &mut src_dec_idx)
                } else {
                    get_or_zero(&integer_digits, &mut src_int_idx)
                };
                if suppressing && digit == b'0' && !past_decimal {
                    output[pos] = *fill; // b' ' for Z, b'*' for *
                } else {
                    suppressing = false;
                    output[pos] = digit;
                }
            }

            // Plain digit (9)
            EditSym::Digit => {
                suppressing = false;
                let digit = if past_decimal {
                    get_or_zero(&decimal_digits, &mut src_dec_idx)
                } else {
                    get_or_zero(&integer_digits, &mut src_int_idx)
                };
                output[pos] = digit;
            }

            // Floating sign (+, -, $)
            EditSym::FloatSign(sign_char) => {
                let digit = if past_decimal {
                    get_or_zero(&decimal_digits, &mut src_dec_idx)
                } else {
                    get_or_zero(&integer_digits, &mut src_int_idx)
                };
                if suppressing && digit == b'0' && !past_decimal {
                    output[pos] = b' '; // suppress position
                } else {
                    if !float_sign_placed {
                        // Place sign in rightmost suppressed position
                        // (backfill: handled by post-processing)
                        suppressing = false;
                        float_sign_placed = true;
                    }
                    output[pos] = digit;
                }
            }

            // CR/DB: two-character sign indicator
            EditSym::CR => {
                if is_negative {
                    output[pos] = b'C';
                    output[pos + 1] = b'R';
                } else {
                    output[pos] = b' ';
                    output[pos + 1] = b' ';
                }
            }
            EditSym::DB => {
                if is_negative {
                    output[pos] = b'D';
                    output[pos + 1] = b'B';
                } else {
                    output[pos] = b' ';
                    output[pos + 1] = b' ';
                }
            }

            // Fixed sign at position 0
            EditSym::FixedPlus => {
                output[pos] = if is_negative { b'-' } else { b'+' };
            }
            EditSym::FixedMinus => {
                output[pos] = if is_negative { b'-' } else { b' ' };
            }
            EditSym::FixedCurrency(ch) => {
                output[pos] = *ch;
            }
        }
    }

    // Post-process: place floating sign in correct position
    if !float_sign_placed && mask.has_floating_sign() {
        place_floating_sign(&mut output, &mask, is_negative);
    }

    dest.set_bytes(&output);
}
```

### 4.6 Receiving Group Item

**Alignment**: Left-justified
**Padding**: Space-filled on right
**Truncation**: Right-truncated

**Rule**: **GROUP receiving ALWAYS uses alphanumeric semantics.** The source
is treated as a flat byte string regardless of its actual category. The bytes
are copied left-to-right into the group's storage.

This is true even if the group contains numeric subordinate items. The
individual subordinate items are NOT individually converted -- the raw bytes
are overlaid.

```
Source: "12345ABCDE" (10 bytes)
Dest:  01 MY-GROUP.          (total 15 bytes)
         05 FIELD-A PIC 9(5).
         05 FIELD-B PIC X(5).
         05 FIELD-C PIC 9(5).

Result: FIELD-A = "12345" (valid numeric -- lucky)
        FIELD-B = "ABCDE"
        FIELD-C = "     " (spaces -- INVALID numeric data!)
```

This is a major source of data corruption bugs in COBOL programs.

```rust
fn move_to_group(src: &dyn CobolField, dest: &mut GroupField) {
    let src_bytes = match src.category() {
        Category::Numeric => src.to_display_representation(),
        _ => src.as_display_bytes().to_vec(),
    };
    let dest_bytes = dest.as_mut_bytes();
    let dest_len = dest_bytes.len();

    for i in 0..dest_len {
        if i < src_bytes.len() {
            dest_bytes[i] = src_bytes[i];
        } else {
            dest_bytes[i] = b' '; // space-pad right
        }
    }
    // Right-truncation is implicit: we stop at dest_len
}
```

---

## 5. Special MOVE Operations

### 5.1 MOVE CORRESPONDING

**Syntax**: `MOVE CORRESPONDING source-group TO dest-group`
(also `MOVE CORR`)

**Semantics**: Match elementary items by name between the two group items and
perform individual MOVE operations for each matched pair.

#### 5.1.1 Matching Rules

1. An item in the source group is matched to an item in the destination group
   if they have **the same data-name** (case-insensitive)
2. Only **elementary items** are matched (not sub-groups, unless both sides
   have the sub-group and the elementary items within match)
3. The following are **excluded** from matching:
   - FILLER items (unnamed)
   - Items subordinate to a REDEFINES entry
   - Items subordinate to a RENAMES (level 66) entry
   - Items described with USAGE INDEX
   - Items with no corresponding name in the other group
4. Each matched pair follows the **normal MOVE rules** for that
   source-destination category combination
5. Matching is based on name only -- not on level number, position, or type

#### 5.1.2 Nested Matching

IBM Enterprise COBOL supports matching through nested group levels. If both
source and destination have a sub-group with the same name, elementary items
within those sub-groups can match:

```cobol
01  SOURCE-REC.
    05  CUSTOMER.
        10  NAME     PIC X(20).
        10  ID       PIC 9(5).
    05  AMOUNT       PIC 9(7)V99.

01  DEST-REC.
    05  CUSTOMER.
        10  NAME     PIC X(30).
        10  PHONE    PIC X(10).
    05  AMOUNT       PIC 9(5)V99.
    05  DATE-FIELD   PIC 9(8).

MOVE CORRESPONDING SOURCE-REC TO DEST-REC.
```

Result:
- CUSTOMER/NAME: Matched -- `PIC X(20)` moved to `PIC X(30)`, space-padded
- CUSTOMER/ID: NOT matched (no ID in dest CUSTOMER)
- CUSTOMER/PHONE: NOT matched (no PHONE in source CUSTOMER)
- AMOUNT: Matched -- `PIC 9(7)V99` moved to `PIC 9(5)V99`, left-truncated
- DATE-FIELD: NOT matched (no DATE-FIELD in source)

```rust
fn move_corresponding(
    src: &GroupField,
    dest: &mut GroupField,
) {
    let src_fields = src.elementary_fields_by_name();
    let dest_fields = dest.elementary_fields_by_name_mut();

    for (name, dest_field) in dest_fields.iter_mut() {
        if let Some(src_field) = src_fields.get(name) {
            if !src_field.is_excluded_from_corr()
                && !dest_field.is_excluded_from_corr()
            {
                cobol_move(src_field.as_ref(), dest_field.as_mut());
            }
        }
    }
}

// Exclusion check
fn is_excluded_from_corr(field: &dyn CobolField) -> bool {
    field.is_filler()
        || field.is_redefines_subordinate()
        || field.is_renames_subordinate()
        || field.is_index()
}
```

### 5.2 De-editing (Numeric-Edited to Numeric)

**Standard**: ILLEGAL in ANSI/ISO COBOL
**IBM Enterprise COBOL**: SUPPORTED as an extension

When IBM de-editing is enabled, a MOVE from a numeric-edited field to a numeric
field strips all editing characters and recovers the numeric value.

#### 5.2.1 De-editing Process

1. Scan the source's display bytes
2. Extract digits (0-9)
3. Determine sign from: `-`, `CR`, `DB` presence (negative) or `+` (positive)
4. Locate decimal point position
5. Reconstruct numeric value
6. Apply normal numeric-to-numeric MOVE rules to destination

```
Source: PIC $ZZ,ZZ9.99   value: " $1,234.56"
De-edit result: +1234.56

Source: PIC -(5)9.99CR   value: " -1234.56CR"
De-edit result: -1234.56
```

#### 5.2.2 De-editing Edge Cases

- All-spaces or all-asterisks: treated as zero
- Currency symbol only (e.g., `"$      0.00"`): value is zero
- `CR`/`DB` present: negative sign
- Both leading `-` and trailing `CR`: sign from trailing indicator

```rust
fn de_edit_numeric(src: &NumericEditedField) -> Decimal {
    let bytes = src.as_display_bytes();
    let mut digits = Vec::new();
    let mut decimal_pos: Option<usize> = None;
    let mut is_negative = false;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'0'..=b'9' => {
                digits.push(b - b'0');
            }
            b'.' => {
                decimal_pos = Some(digits.len());
            }
            b'-' => {
                is_negative = true;
            }
            b'C' => {
                // Check for CR
                if i + 1 < bytes.len() && bytes[i + 1] == b'R' {
                    is_negative = true;
                }
            }
            b'D' => {
                // Check for DB
                if i + 1 < bytes.len() && bytes[i + 1] == b'B' {
                    is_negative = true;
                }
            }
            // Skip: spaces, asterisks, commas, currency symbols, +, B, 0, /
            _ => {}
        }
    }

    if digits.is_empty() {
        return Decimal::ZERO;
    }

    let scale = match decimal_pos {
        Some(pos) => (digits.len() - pos) as u32,
        None => 0,
    };

    let mut value = Decimal::ZERO;
    for &d in &digits {
        value = value * Decimal::TEN + Decimal::from(d);
    }
    if scale > 0 {
        value = value / Decimal::from(10_i64.pow(scale));
    }
    if is_negative {
        value = -value;
    }

    value
}
```

### 5.3 JUSTIFIED RIGHT

**Applies to**: Alphabetic and Alphanumeric destinations only
**Effect**: Reverses the alignment rules

Without JUSTIFIED RIGHT (default):
- Left-justified
- Space-padded on right
- Right-truncated when source is longer

With JUSTIFIED RIGHT:
- Right-justified
- Space-padded on left
- **Left-truncated** when source is longer

```
JUSTIFIED RIGHT example:
Source: "HELLO"
Dest:   PIC X(8) JUSTIFIED RIGHT
Result: "   HELLO"

Source: "HELLO WORLD"
Dest:   PIC X(8) JUSTIFIED RIGHT
Result: "LO WORLD"  (left-truncated, NOT right-truncated)
```

JUSTIFIED RIGHT does NOT apply to numeric destinations -- numeric fields
always use decimal-point alignment.

### 5.4 BLANK WHEN ZERO

**Applies to**: Numeric and Numeric-Edited destinations
**Effect**: If the resulting value is zero after the MOVE, the entire field
is filled with spaces

```
Source: PIC 9(3) value 0
Dest:   PIC 9(5) BLANK WHEN ZERO
Result: "     " (5 spaces, not "00000")

Source: PIC 9(3)V99 value 0.00
Dest:   PIC ZZ9.99 BLANK WHEN ZERO
Result: "      " (6 spaces, not "  0.00")
```

**Implementation note**: BLANK WHEN ZERO is applied as a post-processing step
after the normal MOVE conversion completes. Check if the stored numeric value
is zero; if so, fill the display bytes with spaces.

```rust
fn apply_blank_when_zero(field: &mut dyn CobolField) {
    if field.has_blank_when_zero() {
        if let Some(num) = field.to_numeric_value() {
            if num.is_zero() {
                let bytes = field.as_mut_bytes();
                for b in bytes.iter_mut() {
                    *b = b' ';
                }
            }
        }
    }
}
```

### 5.5 Reference Modification

**Syntax**: `MOVE identifier-1(start:length) TO identifier-2`
or `MOVE identifier-1 TO identifier-2(start:length)`

**Effect**: Reference modification creates a temporary alphanumeric view into
a field. The referenced portion is always treated as alphanumeric regardless
of the original field's category.

**Rules**:
- `start` is 1-based (COBOL indexing)
- `length` is optional; defaults to remaining length from start
- The reference-modified item is category Alphanumeric
- When used on the source: treated as alphanumeric source
- When used on the destination: treated as alphanumeric destination
  (partial update of the field)

```cobol
01  FULL-NAME  PIC X(30) VALUE "JOHN SMITH".
01  FIRST-NAME PIC X(10).
01  LAST-NAME  PIC X(20).

MOVE FULL-NAME(1:4) TO FIRST-NAME.   *> "JOHN      "
MOVE FULL-NAME(6:5) TO LAST-NAME.    *> "SMITH               "
```

```rust
fn move_with_ref_mod(
    src: &dyn CobolField,
    src_start: Option<usize>,  // 1-based, None = no ref-mod
    src_length: Option<usize>, // None = rest of field
    dest: &mut dyn CobolField,
    dest_start: Option<usize>,
    dest_length: Option<usize>,
) {
    // Extract source bytes with ref-mod
    let src_bytes = src.as_display_bytes();
    let src_slice = apply_ref_mod(src_bytes, src_start, src_length);

    // Apply to destination with ref-mod
    if let (Some(d_start), d_len) = (dest_start, dest_length) {
        // Partial update -- only modify the referenced portion
        let dest_bytes = dest.as_mut_bytes();
        let d_start_0 = d_start - 1; // convert to 0-based
        let d_len = d_len.unwrap_or(dest_bytes.len() - d_start_0);
        for i in 0..d_len {
            if i < src_slice.len() {
                dest_bytes[d_start_0 + i] = src_slice[i];
            } else {
                dest_bytes[d_start_0 + i] = b' ';
            }
        }
    } else {
        // Full destination -- treat source slice as alphanumeric
        // and apply normal alphanumeric move rules
        let temp_src = AlphanumericSlice::new(src_slice);
        cobol_move(&temp_src, dest);
    }
}

fn apply_ref_mod(
    bytes: &[u8],
    start: Option<usize>,
    length: Option<usize>,
) -> &[u8] {
    match start {
        None => bytes,
        Some(s) => {
            let s0 = s - 1; // 1-based to 0-based
            let len = length.unwrap_or(bytes.len() - s0);
            &bytes[s0..s0 + len]
        }
    }
}
```

### 5.6 Subscripting and Indexing

MOVE with subscripted (table) items follows all the same rules. The subscript
merely identifies which occurrence to use:

```cobol
01  TABLE-A.
    05  ITEM-A PIC X(10) OCCURS 5 TIMES.
01  WS-FIELD PIC X(10).

MOVE ITEM-A(3) TO WS-FIELD.   *> Move 3rd occurrence
MOVE WS-FIELD TO ITEM-A(1).   *> Move to 1st occurrence
```

**COBOL indexing is 1-based.** The transpiler must subtract 1 for Rust's
0-based array access:

```rust
// COBOL: MOVE ITEM-A(IDX) TO WS-FIELD
// Rust:
cobol_move(&table_a.item_a[idx - 1], &mut ws_field);
```

---

## 6. Figurative Constants

Figurative constants are special source values that adapt their length to
match the destination field:

### 6.1 SPACES / SPACE

- Fills destination with space characters (`0x20` in ASCII, `0x40` in EBCDIC)
- Works with all destination categories
- For numeric destinations: IBM treats as zero; standard is undefined

### 6.2 ZEROS / ZEROES / ZERO

- **Context-dependent**: acts as numeric zero for numeric destinations,
  character `"0"` for alphanumeric destinations
- Numeric destination: sets value to 0 (all digit positions become 0)
- Alphanumeric destination: fills with `"0"` characters
- Numeric-edited destination: sets value to 0, then applies editing mask

### 6.3 HIGH-VALUES / HIGH-VALUE

- Fills destination with the highest value in the collating sequence
- ASCII: `0xFF` bytes
- EBCDIC: `0xFF` bytes
- For numeric destinations: treated as alphanumeric byte fill (produces
  invalid numeric data -- programmer beware)

### 6.4 LOW-VALUES / LOW-VALUE

- Fills destination with the lowest value in the collating sequence
- ASCII: `0x00` bytes
- EBCDIC: `0x00` bytes
- Same caveats as HIGH-VALUES for numeric destinations

### 6.5 ALL literal

- Repeats the literal to fill the destination length
- `MOVE ALL "AB" TO PIC X(7)` -> `"ABABABA"` (repeating pattern, truncated)
- `MOVE ALL ZEROS TO field` is equivalent to `MOVE ZEROS TO field`
- Single-character ALL is equivalent to the character repeated

### 6.6 QUOTE / QUOTES

- Fills with quotation mark character (`"`)
- Behavior equivalent to `ALL '"'`

```rust
fn move_figurative_constant(
    fig: FigurativeConstant,
    dest: &mut dyn CobolField,
) {
    match fig {
        FigurativeConstant::Spaces => {
            match dest.category() {
                Category::Numeric | Category::NumericEdited => {
                    // IBM: set to zero
                    if let Some(num) = dest.as_numeric_mut() {
                        num.set_decimal(Decimal::ZERO);
                    }
                }
                _ => {
                    dest.fill_bytes(b' ');
                }
            }
        }
        FigurativeConstant::Zeros => {
            match dest.category() {
                Category::Numeric => {
                    dest.as_numeric_mut().unwrap().set_decimal(Decimal::ZERO);
                }
                Category::NumericEdited => {
                    // Set value to zero, apply editing mask
                    move_to_numeric_edited_value(Decimal::ZERO, dest);
                }
                _ => {
                    dest.fill_bytes(b'0');
                }
            }
        }
        FigurativeConstant::HighValues => {
            dest.fill_bytes(0xFF);
        }
        FigurativeConstant::LowValues => {
            dest.fill_bytes(0x00);
        }
        FigurativeConstant::AllLiteral(pattern) => {
            let bytes = dest.as_mut_bytes();
            for i in 0..bytes.len() {
                bytes[i] = pattern[i % pattern.len()];
            }
        }
        FigurativeConstant::Quotes => {
            dest.fill_bytes(b'"');
        }
    }
}
```

---

## 7. INITIALIZE Verb

INITIALIZE is a structured MOVE that sets all elementary items within a group
to category-appropriate default values.

### 7.1 Basic Syntax

```cobol
INITIALIZE group-name
INITIALIZE group-name REPLACING
    ALPHANUMERIC DATA BY "DEFAULT"
    NUMERIC DATA BY 0
```

### 7.2 Default Values

Without REPLACING phrase:

| Category | Default Value |
|---|---|
| Alphabetic | SPACES |
| Alphanumeric | SPACES |
| Alphanumeric-Edited | SPACES |
| Numeric | ZEROS |
| Numeric-Edited | ZEROS (then editing applied) |
| National | SPACES (national) |

### 7.3 Items Excluded from INITIALIZE

The following items are NOT initialized:

- FILLER items (standard behavior; IBM has compiler option to include them)
- Items subordinate to a REDEFINES (only the first definition is initialized)
- INDEX data items
- Items with USAGE POINTER
- Items with OCCURS DEPENDING ON object (the object itself is excluded,
  but the table entries are initialized)

### 7.4 REPLACING Phrase

REPLACING allows overriding the default value for specific categories:

```cobol
INITIALIZE CUSTOMER-REC
    REPLACING ALPHANUMERIC DATA BY ALL "*"
              NUMERIC DATA BY -1
              ALPHABETIC DATA BY ALL "?"
```

Each REPLACING clause applies to all elementary items of that category
within the group.

### 7.5 INITIALIZE with VALUE clause interaction

COBOL 2002+ introduced `INITIALIZE ... WITH FILLER` and
`INITIALIZE ... ALL TO VALUE` which interacts with VALUE clauses:

- `INITIALIZE ... ALL TO VALUE`: sets items to their VALUE clause values
  instead of category defaults
- `INITIALIZE ... WITH FILLER`: includes FILLER items in initialization

IBM Enterprise COBOL supports these with the `INITLR` compiler option.

```rust
fn initialize(
    group: &mut GroupField,
    replacing: &[(DataCategory, CobolValueSource)],
    include_filler: bool,
    use_value_clauses: bool,
) {
    for field in group.elementary_fields_mut() {
        // Skip excluded items
        if field.is_index() || field.is_pointer() {
            continue;
        }
        if field.is_filler() && !include_filler {
            continue;
        }
        if field.is_redefines_subordinate() {
            continue;
        }

        // Check for REPLACING override
        let category = field.category();
        let has_replacing = replacing.iter().find(|(cat, _)| *cat == category);

        if let Some((_, value_source)) = has_replacing {
            // Use REPLACING value
            cobol_move_from_source(value_source, field);
        } else if use_value_clauses && field.has_value_clause() {
            // Use VALUE clause
            field.set_to_value_clause();
        } else {
            // Use category default
            match category {
                Category::Alphabetic
                | Category::Alphanumeric
                | Category::AlphanumericEdited => {
                    field.fill_bytes(b' ');
                }
                Category::Numeric
                | Category::NumericEdited => {
                    field.as_numeric_mut().unwrap().set_decimal(Decimal::ZERO);
                    if category == Category::NumericEdited {
                        // Apply editing mask to show zero
                        apply_editing_mask(field);
                    }
                }
                Category::National => {
                    field.fill_national_spaces();
                }
                Category::Group => {
                    // Should not reach here -- INITIALIZE works on elementary items
                    unreachable!("INITIALIZE should only process elementary items");
                }
            }
        }
    }
}
```

---

## 8. Implicit MOVE Contexts

Many COBOL statements perform implicit MOVEs. The transpiler must recognize
these and apply the same conversion rules.

### 8.1 Arithmetic Result Storage

When an arithmetic verb (ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE) stores a
result into a destination field, the storage step follows numeric-to-numeric
MOVE rules, UNLESS the ROUNDED phrase is specified.

```cobol
COMPUTE WS-RESULT = WS-A + WS-B.
*> Implicit: MOVE (intermediate result) TO WS-RESULT
*> Follows numeric MOVE rules: decimal alignment, left-truncation
```

Without ROUNDED: decimal part is truncated (not rounded)
With ROUNDED: result is rounded before storage per the rounding mode

**Key difference from explicit MOVE**: The source is the intermediate
arithmetic result (full precision), not a stored field. The intermediate
result may have more decimal places than either operand.

See `cobol_arithmetic_operations.md` for full intermediate result rules.

### 8.2 READ INTO

```cobol
READ FILE-A INTO WS-RECORD.
*> Equivalent to:
*>   READ FILE-A.
*>   MOVE record-area TO WS-RECORD.
```

This is a GROUP-to-GROUP move (alphanumeric semantics). The record area bytes
are copied left-to-right with space padding or right-truncation.

### 8.3 WRITE FROM

```cobol
WRITE REC-OUT FROM WS-RECORD.
*> Equivalent to:
*>   MOVE WS-RECORD TO REC-OUT.
*>   WRITE REC-OUT.
```

Same GROUP-to-GROUP semantics.

### 8.4 CALL BY CONTENT

```cobol
CALL "SUBPROG" USING BY CONTENT WS-FIELD.
*> Creates a temporary copy of WS-FIELD.
*> The subprogram receives the copy, not the original.
```

BY CONTENT creates a copy. This is a MOVE to a temporary area with the
same PICTURE as the source. No conversion is needed -- it's a byte copy.

BY REFERENCE passes the actual storage address -- no MOVE involved.

### 8.5 STRING/UNSTRING Receiving Fields

```cobol
STRING "HELLO" DELIMITED BY SIZE
       " "    DELIMITED BY SIZE
       "WORLD" DELIMITED BY SIZE
    INTO WS-RESULT
    WITH POINTER WS-PTR.
```

STRING concatenates into the destination using alphanumeric rules from the
pointer position onward. Each source is moved as an alphanumeric value.

UNSTRING splits a source string and moves each delimited portion to its
receiving field using alphanumeric-to-destination-category rules.

### 8.6 INSPECT REPLACING/CONVERTING

```cobol
INSPECT WS-FIELD REPLACING ALL "A" BY "B".
```

INSPECT operates on individual bytes within a field. When REPLACING or
CONVERTING, each matched byte is replaced in-place. This is NOT a MOVE --
it is a byte-level substitution. However, the field's category constraints
still apply (e.g., replacing in a numeric field must produce valid numeric
data).

### 8.7 ACCEPT and DISPLAY

```cobol
ACCEPT WS-FIELD FROM CONSOLE.
DISPLAY WS-FIELD.
```

ACCEPT performs an implicit MOVE from the input data (treated as
alphanumeric) to the receiving field.

DISPLAY converts the field to its display representation (similar to
moving to an alphanumeric field for output).

---

## 9. Rust Implementation Architecture

### 9.1 Design Approach: Category-Dispatched Central Engine

Based on analysis in Section 4, the MOVE engine uses a central dispatch
function that delegates to destination-category-specific handlers:

```rust
/// Central MOVE dispatch -- the heart of the engine
pub fn cobol_move(src: &dyn CobolField, dest: &mut dyn CobolField) {
    // Step 1: Check legality
    let src_cat = src.category();
    let dest_cat = dest.category();
    if !is_legal_move(src_cat, dest_cat) {
        // Transpiler should catch illegal moves at compile time.
        // At runtime, this is a programming error.
        panic!(
            "Illegal MOVE from {:?} to {:?}",
            src_cat, dest_cat
        );
    }

    // Step 2: Dispatch by destination category
    match dest_cat {
        Category::Alphabetic => move_to_alphabetic(src, dest),
        Category::Alphanumeric => move_to_alphanumeric(src, dest),
        Category::AlphanumericEdited => move_to_alpha_edited(src, dest),
        Category::Numeric => move_to_numeric(src, dest),
        Category::NumericEdited => move_to_numeric_edited(src, dest),
        Category::Group => move_to_group(src, dest),
        Category::National => move_to_national(src, dest), // Phase 3
    }

    // Step 3: Apply post-processing
    if dest.has_blank_when_zero() {
        apply_blank_when_zero(dest);
    }
}
```

### 9.2 Trait Requirements

The MOVE engine depends on these traits from the type system
(defined in `cobol_to_rust_datatype_mapping.md` Section 8):

```rust
/// Every COBOL data item implements this
pub trait CobolField {
    /// The item's data category
    fn category(&self) -> DataCategory;

    /// Raw storage bytes
    fn as_bytes(&self) -> &[u8];
    fn as_mut_bytes(&mut self) -> &mut [u8];

    /// Display representation (for alphanumeric moves)
    /// For numeric items, this is the DISPLAY-format string
    fn as_display_bytes(&self) -> Vec<u8>;

    /// Byte length of storage
    fn byte_length(&self) -> usize;

    /// JUSTIFIED RIGHT flag
    fn is_justified_right(&self) -> bool { false }

    /// BLANK WHEN ZERO flag
    fn has_blank_when_zero(&self) -> bool { false }

    /// Fill all bytes with a single value
    fn fill_bytes(&mut self, byte: u8);
}

/// Numeric items additionally implement this
pub trait CobolNumeric: CobolField {
    fn to_decimal(&self) -> Decimal;
    fn set_decimal(&mut self, value: Decimal);
    fn scale(&self) -> u32;       // decimal places
    fn precision(&self) -> u32;   // total digit count
    fn is_signed(&self) -> bool;
}

/// Numeric-Edited items additionally implement this
pub trait CobolNumericEdited: CobolField {
    fn pic_mask(&self) -> &[EditSymbol];
    fn integer_positions(&self) -> u32;
    fn decimal_positions(&self) -> u32;
    fn suppress_char(&self) -> u8; // b' ' for Z, b'*' for *
    fn has_floating_sign(&self) -> bool;
}

/// Group items additionally implement this
pub trait CobolGroup: CobolField {
    fn elementary_fields(&self) -> Vec<&dyn CobolField>;
    fn elementary_fields_mut(&mut self) -> Vec<&mut dyn CobolField>;
    fn elementary_fields_by_name(&self) -> HashMap<&str, &dyn CobolField>;
    fn elementary_fields_by_name_mut(&mut self)
        -> HashMap<&str, &mut dyn CobolField>;
}
```

### 9.3 DataCategory Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataCategory {
    Alphabetic,
    Alphanumeric,
    AlphanumericEdited,
    Numeric,
    NumericEdited,
    National,
    Group,
}
```

### 9.4 Legality Check

```rust
pub fn is_legal_move(src: DataCategory, dest: DataCategory) -> bool {
    use DataCategory::*;
    match (src, dest) {
        // Alphabetic source: cannot go to Numeric or Numeric-Edited
        (Alphabetic, Numeric) => false,
        (Alphabetic, NumericEdited) => false,

        // Alpha-Edited source: same restrictions as Alphabetic
        (AlphanumericEdited, Numeric) => false,
        (AlphanumericEdited, NumericEdited) => false,

        // Numeric-Edited source: illegal to Numeric/Numeric-Edited
        // (unless IBM de-editing extension is enabled)
        (NumericEdited, Numeric) => false,     // [IBM: true]
        (NumericEdited, NumericEdited) => false, // [IBM: true]

        // Numeric with decimal places: cannot go to Alphabetic
        // (Note: this is checked at the value level, not here,
        //  because we don't know if it has decimals from category alone)

        // Everything else is legal
        _ => true,
    }
}
```

For IBM dialect support, the legality function takes a dialect parameter:

```rust
pub fn is_legal_move_dialect(
    src: DataCategory,
    dest: DataCategory,
    dialect: CobolDialect,
) -> bool {
    use DataCategory::*;
    match (src, dest) {
        (NumericEdited, Numeric) | (NumericEdited, NumericEdited) => {
            matches!(dialect, CobolDialect::IbmEnterprise)
        }
        _ => is_legal_move(src, dest),
    }
}
```

### 9.5 NumericValue Intermediate Representation

For numeric moves, an intermediate representation avoids repeated
parsing:

```rust
/// Intermediate numeric value for MOVE processing
#[derive(Debug, Clone)]
pub struct NumericValue {
    /// The decimal value
    pub value: Decimal,
    /// Original precision (total digits in source)
    pub source_precision: u32,
    /// Original scale (decimal places in source)
    pub source_scale: u32,
    /// Whether source was signed
    pub source_signed: bool,
}

impl NumericValue {
    pub fn is_negative(&self) -> bool {
        self.value.is_sign_negative()
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    pub fn abs(&self) -> Decimal {
        self.value.abs()
    }
}
```

### 9.6 Padding and Truncation Utilities

```rust
/// Left-truncate an integer to fit within max_digits
pub fn left_truncate_integer(value: i128, max_digits: u32) -> i128 {
    let modulus = 10_i128.pow(max_digits);
    let abs_val = value.unsigned_abs();
    let truncated = (abs_val % modulus as u128) as i128;
    if value < 0 { -truncated } else { truncated }
}

/// Right-truncate a decimal by reducing scale (no rounding)
pub fn right_truncate_decimal(value: Decimal, target_scale: u32) -> Decimal {
    // Truncate (floor toward zero), not round
    let factor = Decimal::from(10_i64.pow(target_scale));
    let shifted = value * factor;
    let truncated = if value.is_sign_negative() {
        shifted.ceil() // ceil for negative = toward zero
    } else {
        shifted.floor() // floor for positive = toward zero
    };
    truncated / factor
}

/// Copy bytes with left-justification, space-padding right
pub fn copy_left_justified(src: &[u8], dest: &mut [u8]) {
    let copy_len = src.len().min(dest.len());
    dest[..copy_len].copy_from_slice(&src[..copy_len]);
    for b in dest[copy_len..].iter_mut() {
        *b = b' ';
    }
}

/// Copy bytes with right-justification, space-padding left
pub fn copy_right_justified(src: &[u8], dest: &mut [u8]) {
    let dest_len = dest.len();
    let src_len = src.len();
    if src_len >= dest_len {
        // Left-truncate: take rightmost dest_len bytes
        let start = src_len - dest_len;
        dest.copy_from_slice(&src[start..]);
    } else {
        // Space-pad left
        let pad = dest_len - src_len;
        for b in dest[..pad].iter_mut() {
            *b = b' ';
        }
        dest[pad..].copy_from_slice(src);
    }
}
```

### 9.7 Error and Diagnostic Handling

COBOL MOVE never raises a runtime error (in standard COBOL). Data corruption
from illegal moves, invalid numeric strings, or overflow is silent. However,
for the transpiler runtime we may want optional diagnostics:

```rust
/// Move diagnostic level
#[derive(Debug, Clone, Copy)]
pub enum MoveDiagnostic {
    /// No diagnostics (production mode -- matches COBOL behavior)
    Silent,
    /// Log warnings for data loss (development/testing mode)
    Warn,
    /// Panic on data loss (strict testing mode)
    Strict,
}

/// Emitted when MOVE causes data loss
#[derive(Debug)]
pub struct MoveWarning {
    pub kind: MoveWarningKind,
    pub source_desc: String,
    pub dest_desc: String,
}

#[derive(Debug)]
pub enum MoveWarningKind {
    LeftTruncation,       // Integer digits lost
    RightTruncation,      // Decimal digits lost
    SignLoss,             // Signed -> unsigned, value was negative
    InvalidNumericData,   // Alphanumeric source contained non-numeric chars
    GroupOverlay,         // Group move may corrupt subordinate numeric fields
}
```

---

## 10. Worked Examples

### 10.1 Simple Numeric Move with Truncation

```cobol
01  WS-SOURCE  PIC 9(7)V99  VALUE 1234567.89.
01  WS-DEST    PIC 9(4)V9.
MOVE WS-SOURCE TO WS-DEST.
```

**Step-by-step**:
1. Source category: Numeric, value = 1234567.89
2. Dest category: Numeric, 4 integer digits, 1 decimal place
3. Legal: Yes (Numeric -> Numeric)
4. Decimal alignment:
   - Source decimal: .89
   - Dest decimal:   .9  (1 place -> right-truncate: .8)
5. Integer alignment:
   - Source integer: 1234567 (7 digits)
   - Dest integer:  4 digits -> left-truncate: 4567
6. Result: 4567.8

```rust
// Rust equivalent:
let src_value = Decimal::new(123456789, 2); // 1234567.89
let dest_scale = 1u32;
let dest_int_digits = 4u32;
// Right-truncate decimal: 1234567.89 -> 1234567.8
let truncated = right_truncate_decimal(src_value, dest_scale); // 1234567.8
// Left-truncate integer: 1234567 -> 4567
let result = left_truncate_to_fit(truncated, dest_int_digits, dest_scale);
// result = 4567.8
dest.set_decimal(result);
```

### 10.2 Alphanumeric to Numeric with Decimal

```cobol
01  WS-ALPHA   PIC X(8)  VALUE "01234.56".
01  WS-NUM     PIC 9(5)V99.
MOVE WS-ALPHA TO WS-NUM.
```

**Step-by-step**:
1. Source category: Alphanumeric, bytes = "01234.56"
2. Dest category: Numeric, 5 integer, 2 decimal
3. Legal: Yes (Alphanumeric -> Numeric, if valid numeric string)
4. Parse alphanumeric: value = 1234.56
5. Decimal alignment: .56 -> .56 (fits, no truncation)
6. Integer alignment: 1234 -> 01234 (zero-padded left, fits)
7. Result: 01234.56

### 10.3 MOVE CORRESPONDING with Mismatched Types

```cobol
01  SRC-REC.
    05  AMOUNT      PIC 9(7)V99   VALUE 9876543.21.
    05  NAME        PIC X(20)     VALUE "JOHN DOE".
    05  CODE        PIC 9(3)      VALUE 456.
    05  FILLER      PIC X(5)      VALUE "XXXXX".

01  DEST-REC.
    05  NAME        PIC X(15).
    05  AMOUNT      PIC 9(5)V9.
    05  STATUS      PIC X(10).

MOVE CORRESPONDING SRC-REC TO DEST-REC.
```

**Step-by-step**:
1. Match by name:
   - AMOUNT matches AMOUNT
   - NAME matches NAME
   - CODE: no match in DEST-REC
   - FILLER: excluded (unnamed)
   - STATUS: no match in SRC-REC
2. Execute individual MOVEs:
   - `MOVE AMOUNT(src) TO AMOUNT(dest)`:
     9876543.21 (PIC 9(7)V99) -> PIC 9(5)V9
     Left-truncate integer: 76543, right-truncate decimal: .2
     Result: 76543.2
   - `MOVE NAME(src) TO NAME(dest)`:
     "JOHN DOE" (PIC X(20)) -> PIC X(15)
     Right-truncate: "JOHN DOE       " -> "JOHN DOE       " (fits, 8 < 15)
     Result: "JOHN DOE       "
3. Untouched: STATUS retains its previous value, CODE is not moved

### 10.4 Numeric-Edited Formatting

```cobol
01  WS-AMOUNT  PIC 9(5)V99  VALUE 1234.50.
01  WS-EDIT    PIC $$$,$$9.99.
MOVE WS-AMOUNT TO WS-EDIT.
```

**Step-by-step**:
1. Source: Numeric, value = 01234.50
2. Dest: Numeric-Edited, mask = `$$$,$$9.99`
3. Extract numeric: +1234.50
4. Integer digits needed: 6 positions (3 float-$, comma, 2 more)
   Actual integer: 001234
5. Decimal digits: 50
6. Apply mask:
   ```
   Position: $ $ $ , $ $ 9 . 9 9
   Digits:   0 0 1   2 3 4   5 0
   Float $:  suppress, suppress, place $
   Result:       $ 1 , 2 3 4 . 5 0
   Display: "  $1,234.50"
   ```

### 10.5 Group Move Causing Data Corruption

```cobol
01  SRC-GROUP.
    05  FILLER PIC X(5) VALUE "HELLO".
    05  FILLER PIC X(5) VALUE "WORLD".

01  DEST-GROUP.
    05  NUM-FIELD PIC 9(5).
    05  ALPHA-FIELD PIC X(5).

MOVE SRC-GROUP TO DEST-GROUP.
```

**Step-by-step**:
1. Source: Group (treated as alphanumeric), bytes = "HELLOWORLD"
2. Dest: Group, 10 bytes total
3. Byte copy: "HELLOWORLD" -> DEST-GROUP storage
4. Result:
   - NUM-FIELD contains "HELLO" -> **INVALID numeric data!**
   - ALPHA-FIELD contains "WORLD" -> valid
5. Any subsequent arithmetic on NUM-FIELD will produce undefined results

This is why GROUP moves are dangerous and why the diagnostic system
(Section 9.7) flags `MoveWarningKind::GroupOverlay`.

---

## 11. Dialect Variations

### 11.1 IBM Enterprise COBOL

| Feature | Behavior |
|---|---|
| De-editing | Supported: MOVE numeric-edited TO numeric is legal |
| NUMPROC option | Controls sign normalization on MOVE |
| MOVE SPACES to numeric | Sets to zero (not standard) |
| INITIALIZE WITH FILLER | Supported via INITLR compiler option |
| GROUP MOVE of OCCURS DEPENDING | Moves based on current ODO value |
| Reference modification | Fully supported on any item |
| National (PIC N) MOVE | Full DBCS/SBCS conversion support |

IBM-specific NUMPROC interaction with MOVE:
- `NUMPROC(NOPFD)`: Sign is corrected on MOVE (if invalid sign nibble, it is
  fixed to the preferred sign)
- `NUMPROC(PFD)`: No sign correction -- assumes all data is valid
- `NUMPROC(MIG)`: Migration mode -- minimal processing

### 11.2 GnuCOBOL

| Feature | Behavior |
|---|---|
| De-editing | Supported (more permissive than IBM) |
| MOVE SPACES to numeric | Sets to zero |
| Figurative constants to numeric | More permissive than standard |
| Reference modification | Fully supported |
| Implicit conversion | More lenient on type mismatches |
| MOVE of Boolean (level 88) | Not directly supported (use SET) |

GnuCOBOL is generally more permissive with MOVE operations, allowing
combinations that are technically illegal per the standard. The transpiler
should warn about non-portable moves when targeting GnuCOBOL source.

### 11.3 Micro Focus Visual COBOL

| Feature | Behavior |
|---|---|
| De-editing | Supported via DEEDIT directive |
| MOVE SPACES to numeric | Implementation-defined |
| Extended MOVE | Supports MOVE of POINTER types |
| DISPLAY-1 (DBCS) | Special MOVE rules for DBCS fields |
| Reference modification | Fully supported |

### 11.4 Dialect Configuration for Transpiler

```rust
#[derive(Debug, Clone)]
pub struct MoveConfig {
    /// Target COBOL dialect
    pub dialect: CobolDialect,

    /// Allow de-editing (numeric-edited -> numeric)
    pub allow_de_editing: bool,

    /// NUMPROC behavior (IBM-specific)
    pub numproc: NumProc,

    /// How to handle MOVE SPACES to numeric
    pub spaces_to_numeric: SpacesToNumeric,

    /// Diagnostic level
    pub diagnostics: MoveDiagnostic,
}

#[derive(Debug, Clone, Copy)]
pub enum CobolDialect {
    IbmEnterprise,
    GnuCobol,
    MicroFocus,
    AnsiStandard,
}

#[derive(Debug, Clone, Copy)]
pub enum NumProc {
    Nopfd,  // Correct signs on MOVE
    Pfd,    // Assume valid signs
    Mig,    // Migration mode
}

#[derive(Debug, Clone, Copy)]
pub enum SpacesToNumeric {
    SetToZero,  // IBM behavior
    Undefined,  // Standard behavior (runtime undefined)
}

impl Default for MoveConfig {
    fn default() -> Self {
        Self {
            dialect: CobolDialect::IbmEnterprise,
            allow_de_editing: true,
            numproc: NumProc::Nopfd,
            spaces_to_numeric: SpacesToNumeric::SetToZero,
            diagnostics: MoveDiagnostic::Silent,
        }
    }
}
```

---

## Appendix A: Complete Move Flow Diagram

```
MOVE src TO dest
    |
    v
+-------------------+
| Classify src      |-----> DataCategory
| Classify dest     |-----> DataCategory
+-------------------+
    |
    v
+-------------------+
| Legal? (matrix)   |-----> NO: compile-time error (transpiler)
+-------------------+         or runtime panic (strict mode)
    |
    YES
    v
+-------------------+
| dest is Group?    |-----> YES: byte copy, left-just, space-pad
+-------------------+
    |
    NO
    v
+-------------------+
| dest is Numeric?  |-----> YES: extract numeric value from src
+-------------------+         |    decimal-align
    |                         |    zero-pad / left-truncate / right-truncate
    NO                        |    handle sign
    v                         v
+-------------------+    +-------------------+
| dest is Num-Ed?   |--->| Extract numeric   |
+-------------------+    | Apply edit mask   |
    |                    +-------------------+
    NO
    v
+-------------------+
| dest is Alpha/    |-----> Get display bytes from src
| Alphanumeric/     |       Left-justify (or right if JUST RIGHT)
| Alpha-Edited?     |       Space-pad / truncate
+-------------------+       Insert editing chars if Alpha-Edited
    |
    v
+-------------------+
| BLANK WHEN ZERO?  |-----> If zero, fill with spaces
+-------------------+
    |
    v
  DONE
```

---

## Appendix B: Quick Reference -- Alignment and Truncation Summary

| Destination | Alignment | Padding | Truncation |
|---|---|---|---|
| Alphabetic | Left | Space right | Right |
| Alphanumeric | Left (or Right if JUST) | Space right (or left) | Right (or Left if JUST) |
| Alpha-Edited | Left | Space right | Right |
| Numeric | Decimal point | Zero both sides | LEFT (integer), RIGHT (decimal) |
| Numeric-Edited | Per edit mask | Per edit mask | Per edit mask |
| Group | Left | Space right | Right |

**Critical reminder**: Numeric truncation is LEFT (high-order). This is the
opposite of what most programmers expect from languages like C, Java, or Rust
where integer overflow wraps around the low-order bits.

---

## Appendix C: Cross-Reference to Other Specifications

| Topic | Document | Section |
|---|---|---|
| Type definitions and traits | `cobol_to_rust_datatype_mapping.md` | Sections 2-5, 8 |
| Arithmetic result storage | `cobol_arithmetic_operations.md` | Section 5 |
| PERFORM VARYING loop variables | `cobol_control_flow_constructs.md` | PERFORM section |
| File I/O (READ INTO, WRITE FROM) | (future: File I/O spec) | -- |
| EBCDIC/ASCII encoding | `cobol_to_rust_datatype_mapping.md` | Section 10 (encoding) |
