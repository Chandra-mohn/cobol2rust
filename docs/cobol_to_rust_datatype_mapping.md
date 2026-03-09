# COBOL-to-Rust Data Type Mapping: Foundation Reference

## Purpose

This document defines the complete mapping from COBOL data types to Rust
types for the cobol2rust transpiler runtime library. It serves as the
specification for implementing the `cobol-runtime` crate -- the foundational
layer that all transpiled Rust code depends on.

---

## Table of Contents

1. [Design Principles](#1-design-principles)
2. [Numeric Types](#2-numeric-types)
   - 2.1 [DISPLAY (Zoned Decimal)](#21-display-zoned-decimal)
   - 2.2 [COMP / COMP-4 (PIC-Limited Binary)](#22-comp--comp-4-pic-limited-binary)
   - 2.3 [COMP-3 / PACKED-DECIMAL](#23-comp-3--packed-decimal)
   - 2.4 [COMP-5 / BINARY (Native Binary)](#24-comp-5--binary-native-binary)
   - 2.5 [COMP-1 (Single-Precision Float)](#25-comp-1-single-precision-float)
   - 2.6 [COMP-2 (Double-Precision Float)](#26-comp-2-double-precision-float)
3. [Alphanumeric Types](#3-alphanumeric-types)
   - 3.1 [PIC X (Alphanumeric)](#31-pic-x-alphanumeric)
   - 3.2 [PIC A (Alphabetic)](#32-pic-a-alphabetic)
   - 3.3 [PIC N / PIC G (National / DBCS)](#33-pic-n--pic-g-national--dbcs)
4. [Edited Types](#4-edited-types)
   - 4.1 [Numeric Edited](#41-numeric-edited)
   - 4.2 [Alphanumeric Edited](#42-alphanumeric-edited)
5. [Structural Constructs](#5-structural-constructs)
   - 5.1 [GROUP Items (Records -> Structs)](#51-group-items-records---structs)
   - 5.2 [REDEFINES (Memory Overlay -> Enum/Union)](#52-redefines-memory-overlay---enumunion)
   - 5.3 [OCCURS (Arrays / Tables)](#53-occurs-arrays--tables)
   - 5.4 [Level 88 (Condition Names -> Enums/Predicates)](#54-level-88-condition-names---enumspredicates)
   - 5.5 [Level 66 RENAMES (Byte-Range Alias -> Slice)](#55-level-66-renames-byte-range-alias---slice)
   - 5.6 [Level 77 (Independent Items)](#56-level-77-independent-items)
   - 5.7 [FILLER (Padding)](#57-filler-padding)
6. [Fixed-Point Arithmetic Details](#6-fixed-point-arithmetic-details)
   - 6.1 [Implied Decimal Point (V)](#61-implied-decimal-point-v)
   - 6.2 [Scaling Position (P)](#62-scaling-position-p)
   - 6.3 [Sign Handling](#63-sign-handling)
7. [Storage Size Reference](#7-storage-size-reference)
8. [Trait Architecture](#8-trait-architecture)
9. [Crate Dependencies](#9-crate-dependencies)
10. [Architectural Decisions](#10-architectural-decisions)
11. [Key Transpiler Challenges](#11-key-transpiler-challenges)
12. [Cross-Dialect Mapping Matrix](#12-cross-dialect-mapping-matrix)
   - 12.1 [Numeric Types -- USAGE Variants](#121-numeric-types----usage-variants)
   - 12.2 [Numeric Storage Sizes by Dialect](#122-numeric-storage-sizes-by-dialect)
   - 12.3 [Alphanumeric and String Types](#123-alphanumeric-and-string-types)
   - 12.4 [Edited Types](#124-edited-types)
   - 12.5 [Sign Handling](#125-sign-handling)
   - 12.6 [Structural Constructs](#126-structural-constructs)
   - 12.7 [Alignment and Storage Modifiers](#127-alignment-and-storage-modifiers)
   - 12.8 [Arithmetic and Overflow Behavior](#128-arithmetic-and-overflow-behavior)
   - 12.9 [Encoding and Character Set](#129-encoding-and-character-set)
   - 12.10 [Dialect-Specific Extensions](#1210-dialect-specific-extensions-no-cross-dialect-equivalent)
   - 12.11 [Feature Support Summary](#1211-feature-support-summary)
   - 12.12 [Transpiler Dialect Configuration](#1212-transpiler-dialect-configuration)

---

## 1. Design Principles

### 1.1 Core Philosophy

The runtime library follows these principles in priority order:

1. **Semantic fidelity** -- COBOL business logic must produce identical results
   in Rust. Decimal arithmetic must be exact. Truncation, padding, and overflow
   must match COBOL behavior precisely.

2. **Type safety** -- Use Rust's type system to prevent errors that COBOL allows
   at runtime (e.g., moving alphanumeric to numeric without conversion). Make
   illegal states unrepresentable where possible.

3. **Zero-cost abstraction** -- Newtypes, const generics, and `#[repr]`
   attributes should produce code with no runtime overhead beyond what the
   equivalent C or COBOL code would incur.

4. **Two-layer architecture** -- Raw byte-level record layout (for I/O
   compatibility) + safe typed representation (for business logic). The
   transpiler generates both layers and the conversion between them.

### 1.2 Encoding Strategy

All internal processing uses ASCII. EBCDIC conversion happens at I/O boundaries
only (when reading/writing mainframe-originated data). This is handled by a
separate encoding layer, not embedded in the data types.

### 1.3 Dialect Scope

The primary target dialect is **IBM Enterprise COBOL for z/OS**. GnuCOBOL
and Micro Focus differences are noted where they affect type mapping. The
transpiler should accept a dialect configuration parameter.

---

## 2. Numeric Types

### 2.1 DISPLAY (Zoned Decimal)

**COBOL:**
```cobol
01 WS-AMOUNT        PIC S9(5)V99.
01 WS-COUNTER       PIC 9(4).
01 WS-RATE          PIC V9(6).
```

**Semantics:** Default USAGE. Each digit occupies one byte. The zone nibble
(upper 4 bits) is 0xF for EBCDIC, 0x3 for ASCII. The sign is encoded in the
zone nibble of the leading or trailing byte (configurable via SIGN clause).

**Storage:**
- Unsigned: 1 byte per digit
- Signed (embedded): 1 byte per digit (sign in zone nibble)
- Signed (SEPARATE): 1 byte per digit + 1 byte for +/- character

| PIC Clause | Digits | Decimals | Bytes (embedded) | Bytes (SEPARATE) |
|---|---|---|---|---|
| `9(4)` | 4 | 0 | 4 | 4 |
| `S9(5)V99` | 7 | 2 | 7 | 8 |
| `V9(6)` | 6 | 6 | 6 | 6 |
| `S9(7)` | 7 | 0 | 7 | 8 |

**Rust type:**
```rust
/// Zoned decimal (USAGE DISPLAY) -- the default COBOL numeric format.
///
/// INT_DIGITS: number of integer digit positions
/// DEC_DIGITS: number of decimal digit positions (after implied V)
/// STORAGE: total byte count (computed by transpiler at codegen time)
///
/// Backed by rust_decimal::Decimal for arithmetic.
/// The raw zoned format is only used during record I/O serialization.
pub struct ZonedDecimal<const INT_DIGITS: usize, const DEC_DIGITS: usize, const STORAGE: usize> {
    value: rust_decimal::Decimal,
    signed: bool,
}
```

**Why not store as raw bytes internally?** Arithmetic on zoned-encoded bytes
requires decoding to a numeric value, performing the operation, then
re-encoding. Since COBOL programs constantly mix arithmetic and data
movement, the internal representation should optimize for arithmetic. The
zoned byte format is generated only during serialization to record buffers.

**Sign encoding variants (affects serialization only):**

| SIGN Clause | Sign Location | Extra Byte? | Encoding (EBCDIC) |
|---|---|---|---|
| (default) | Trailing, embedded | No | Zone 0xC=+, 0xD=- |
| SIGN LEADING | Leading, embedded | No | Zone 0xC=+, 0xD=- |
| SIGN TRAILING | Trailing, embedded | No | Zone 0xC=+, 0xD=- |
| SIGN LEADING SEPARATE | Leading, separate char | Yes (+1) | '+' or '-' character |
| SIGN TRAILING SEPARATE | Trailing, separate char | Yes (+1) | '+' or '-' character |

---

### 2.2 COMP / COMP-4 (PIC-Limited Binary)

**COBOL:**
```cobol
01 WS-BIN-SHORT     PIC S9(4)   COMP.
01 WS-BIN-LONG      PIC S9(9)   COMP.
01 WS-BIN-LONGLONG  PIC S9(18)  COMP.
01 WS-BIN-UNSIGNED  PIC 9(4)    COMP.
```

**Semantics:** Binary integer storage. Big-endian on mainframes. The critical
distinction from COMP-5: the **PIC clause limits the value range**, not the
binary storage capacity. `PIC S9(4) COMP` uses 2 bytes of storage but is
limited to -9999 to +9999, NOT -32768 to +32767.

**Storage:**

| PIC Digits | Bytes | COMP Signed Range (PIC-limited) | COMP-5 Signed Range (full) |
|---|---|---|---|
| 1-4 | 2 | -9(d) to +9(d) | -32768 to +32767 |
| 5-9 | 4 | -9(d) to +9(d) | -2147483648 to +2147483647 |
| 10-18 | 8 | -9(d) to +9(d) | -9223372036854775808 to +... |

**Rust type:**
```rust
/// PIC-limited binary (USAGE COMP / COMP-4).
///
/// Stored internally as the native integer type matching the storage size,
/// but arithmetic operations enforce the PIC-clause range limit.
/// The transpiler generates range-checking wrappers.
///
/// PIC_DIGITS: total digit count from PIC clause (determines range limit)
/// STORAGE: byte size (2, 4, or 8 -- computed by transpiler)
pub struct CompBinary<const PIC_DIGITS: usize, const STORAGE: usize> {
    value: i64,   // always fits in i64 for up to 18 digits
    signed: bool,
}

// For transpiler codegen convenience, type aliases:
// PIC S9(1-4) COMP  -> CompBinary<N, 2>  (backed by i16 semantics)
// PIC S9(5-9) COMP  -> CompBinary<N, 4>  (backed by i32 semantics)
// PIC S9(10-18) COMP -> CompBinary<N, 8> (backed by i64 semantics)
```

**PIC-range enforcement:** On every assignment and arithmetic result, the value
is checked against the maximum determined by PIC digits. If it exceeds the
range, COBOL silently truncates (drops high-order digits). The Rust
implementation must replicate this truncation, not panic.

**Endianness:** Serialization to record buffers uses big-endian (mainframe
format). The `byteorder` crate handles conversion.

---

### 2.3 COMP-3 / PACKED-DECIMAL

**COBOL:**
```cobol
01 WS-PACKED-AMT    PIC S9(7)V99 COMP-3.
01 WS-PACKED-INT    PIC 9(5)     COMP-3.
```

**Semantics:** Binary-Coded Decimal. Each byte stores two decimal digits (one
per nibble), except the last byte which stores one digit and a sign nibble.
This is the most common numeric format in COBOL financial applications.

**Storage formula:** `bytes = ceil((total_digits + 1) / 2)`

The sign nibble occupies the low nibble of the last byte:
- `0xC` = positive
- `0xD` = negative
- `0xF` = unsigned

**Example:** `PIC S9(7)V99 COMP-3` = 9 total digits -> `ceil(10/2)` = 5 bytes

Value +1234567.89 stored as: `01 23 45 67 89 C` (but packed into 5 bytes):
```
Byte 0: 0x01  (digits 0,1)
Byte 1: 0x23  (digits 2,3)
Byte 2: 0x45  (digits 4,5)
Byte 3: 0x67  (digits 6,7)
Byte 4: 0x9C  (digit 8, sign=positive)
```

**Storage size table:**

| Total Digits | Bytes | Formula |
|---|---|---|
| 1 | 1 | ceil(2/2) |
| 2-3 | 2 | ceil((d+1)/2) |
| 4-5 | 3 | " |
| 6-7 | 4 | " |
| 8-9 | 5 | " |
| 10-11 | 6 | " |
| 12-13 | 7 | " |
| 14-15 | 8 | " |
| 16-17 | 9 | " |
| 18 | 10 | " |

**Rust type:**
```rust
/// COMP-3 packed decimal.
///
/// Arithmetic uses rust_decimal::Decimal internally.
/// BCD encoding/decoding happens at record I/O boundaries.
///
/// DIGITS: total digit count (integer + decimal)
/// SCALE: number of implied decimal places (from V in PIC)
/// STORAGE: byte count = ceil((DIGITS + 1) / 2)
pub struct PackedDecimal<const DIGITS: usize, const SCALE: usize, const STORAGE: usize> {
    value: rust_decimal::Decimal,
}

// The runtime library must implement:
// - pack(decimal) -> [u8; STORAGE]    (encode to BCD)
// - unpack([u8; STORAGE]) -> decimal  (decode from BCD)
// These are custom implementations -- no existing crate handles this.
```

**Why rust_decimal internally?** COMP-3 is the workhorse of COBOL financial
arithmetic. It must be exact (no floating-point imprecision). `rust_decimal`
provides:
- Up to 28-29 significant digits (covers COBOL's 18-digit IBM max)
- All rounding modes (matches COBOL's ROUNDED phrase)
- Full arithmetic operator support
- Configurable scale

**Custom BCD codec:** The BCD byte encoding (packing/unpacking nibbles, sign
nibble handling, odd/even digit count alignment) has no adequate Rust crate.
This must be implemented in the runtime library.

---

### 2.4 COMP-5 / BINARY (Native Binary)

**COBOL:**
```cobol
01 WS-NATIVE-BIN   PIC S9(4)   COMP-5.
01 WS-NATIVE-LONG  PIC S9(9)   COMP-5.
```

Also available as COBOL 2002 standard type names:
```cobol
01 WS-BYTE          BINARY-CHAR.
01 WS-SHORT         BINARY-SHORT.
01 WS-LONG          BINARY-LONG.
01 WS-DOUBLE        BINARY-DOUBLE.
```

**Semantics:** Native binary storage using the **full range** of the binary
container, NOT limited by the PIC clause. `PIC S9(4) COMP-5` uses 2 bytes
and allows -32768 to +32767 (the full `i16` range).

**Rust type:**
```rust
/// COMP-5 / native binary -- maps directly to Rust integer primitives.
/// This is the simplest mapping in the entire type system.

// PIC S9(1-2) COMP-5 / BINARY-CHAR  -> i8  (signed) or u8  (unsigned)
// PIC S9(3-4) COMP-5 / BINARY-SHORT -> i16 (signed) or u16 (unsigned)
// PIC S9(5-9) COMP-5 / BINARY-LONG  -> i32 (signed) or u32 (unsigned)
// PIC S9(10-18) COMP-5 / BINARY-DOUBLE -> i64 (signed) or u64 (unsigned)

// No wrapper type needed -- direct primitive mapping.
// The transpiler emits the appropriate Rust primitive type.
```

**Dialect note:** The behavior of plain `BINARY` keyword varies:
- IBM: `BINARY` = COMP (PIC-limited)
- Micro Focus: `BINARY` = COMP-5 (full range)
- GnuCOBOL: configurable

The transpiler must respect the configured dialect.

---

### 2.5 COMP-1 (Single-Precision Float)

**COBOL:**
```cobol
01 WS-FLOAT-S       COMP-1.
```

**Semantics:** 4-byte floating-point. No PIC clause allowed.

**Critical dialect difference:**
- IBM (with FLOAT(HEX) option): IBM hexadecimal floating-point (NOT IEEE 754)
- IBM (with FLOAT(IEEE) option or ARITH(EXTEND)): IEEE 754 single-precision
- GnuCOBOL: IEEE 754 single-precision
- Micro Focus: IEEE 754 single-precision

**Rust type:**
```rust
// IEEE 754 path (GnuCOBOL, Micro Focus, modern IBM):
// COMP-1 -> f32

// IBM HFP path (legacy IBM):
// COMP-1 -> custom HexFloat32 type (requires custom implementation)
// IBM HFP has different precision characteristics than IEEE 754.
// The transpiler should warn when targeting HFP compatibility.
```

**Storage:** 4 bytes, always.

**Note:** Floating-point types are uncommon in COBOL financial programs (which
prefer exact decimal arithmetic). They appear primarily in scientific or
engineering COBOL programs.

---

### 2.6 COMP-2 (Double-Precision Float)

**COBOL:**
```cobol
01 WS-FLOAT-D       COMP-2.
```

**Semantics:** 8-byte floating-point. No PIC clause allowed. Same
IEEE-vs-HFP dialect considerations as COMP-1.

**Rust type:**
```rust
// IEEE 754 path: COMP-2 -> f64
// IBM HFP path:  COMP-2 -> custom HexFloat64 type
```

**Storage:** 8 bytes, always.

---

## 3. Alphanumeric Types

### 3.1 PIC X (Alphanumeric)

**COBOL:**
```cobol
01 WS-NAME          PIC X(30).
01 WS-SINGLE-CHAR   PIC X.
01 WS-CODE          PIC XXX.
```

**Semantics:** Fixed-length character field. 1 byte per character. Right-padded
with spaces. Can hold any character in the program's character set.

**COBOL MOVE behavior:**
- MOVE to shorter PIC X: truncate on the right
- MOVE to longer PIC X: space-pad on the right
- MOVE numeric to PIC X: converts to display character representation

**Maximum length:** ~128 MB (IBM), memory-limited (GnuCOBOL)

**Rust type:**
```rust
/// Fixed-length alphanumeric field (PIC X).
///
/// Stored as a fixed-size byte array. Space-padded on the right.
/// All COBOL string semantics (padding, truncation, comparison)
/// are implemented as methods.
///
/// N: field length in bytes
pub struct PicX<const N: usize> {
    data: [u8; N],
}

// Key methods the runtime must provide:
impl<const N: usize> PicX<N> {
    fn new() -> Self;                          // initialized to spaces
    fn from_str(s: &str) -> Self;             // truncate/pad to fit
    fn as_str(&self) -> &str;                 // view as string (trimmed)
    fn as_bytes(&self) -> &[u8; N];           // raw bytes
    fn move_from<const M: usize>(other: &PicX<M>) -> Self;  // COBOL MOVE
    fn is_spaces(&self) -> bool;              // all spaces?
    fn compare<const M: usize>(&self, other: &PicX<M>) -> Ordering;
    // Comparison: shorter field is logically right-padded with spaces
}
```

**JUSTIFIED RIGHT:** When the JUSTIFIED RIGHT clause is specified, data is
right-justified and left-padded with spaces instead. This affects the
`move_from` method behavior, tracked via a const generic or runtime flag.

---

### 3.2 PIC A (Alphabetic)

**COBOL:**
```cobol
01 WS-ALPHA         PIC A(20).
```

**Semantics:** Restricted to alphabetic characters (A-Z, a-z) and spaces.
Identical storage to PIC X. Rarely used in modern COBOL.

**Rust type:**
```rust
/// Alphabetic field (PIC A). Same as PicX with validation.
/// The validation is a transpiler-time concern; at runtime,
/// PicA is identical to PicX with an optional validation wrapper.
pub type PicA<const N: usize> = PicX<N>;
// Or, if runtime validation is desired:
// pub struct PicA<const N: usize>(PicX<N>);
// with TryFrom<&str> that rejects non-alphabetic characters.
```

---

### 3.3 PIC N / PIC G (National / DBCS)

**COBOL:**
```cobol
01 WS-NATIONAL      PIC N(20) USAGE NATIONAL.
01 WS-DBCS          PIC G(10) USAGE DISPLAY-1.
```

**Semantics:** Double-byte characters. PIC N with USAGE NATIONAL = UTF-16.
PIC G = DBCS (IBM-specific, Shift-JIS or similar). 2 bytes per character.

**Rust type:**
```rust
/// National/DBCS character field.
///
/// Stored internally as UTF-8 String (Rust's native encoding).
/// Conversion to/from UTF-16 or DBCS happens at I/O boundaries.
///
/// CHARS: number of characters (not bytes)
pub struct PicNational<const CHARS: usize> {
    data: String,  // UTF-8 internally, max CHARS characters
}

// Alternatively, for fixed-size UTF-16 layout compatibility:
// pub struct PicNational<const CHARS: usize> {
//     data: [u16; CHARS],  // UTF-16 code units
// }
```

**Relevance:** National/DBCS types are uncommon in the financial COBOL programs
that are the primary migration target. Lower implementation priority.

---

## 4. Edited Types

### 4.1 Numeric Edited

**COBOL:**
```cobol
01 WS-EDIT-AMT      PIC $$$,$$9.99.
01 WS-EDIT-NEG      PIC -(5)9.99.
01 WS-EDIT-CHECK    PIC $**,**9.99CR.
01 WS-EDIT-ZERO     PIC Z(5)9.
01 WS-EDIT-PLUS     PIC ++++9.99.
01 WS-EDIT-BLANK    PIC 9(3)B9(3).
```

**Semantics:** Display-only formatting. CANNOT be used in arithmetic. Values
are formatted during MOVE from a numeric source.

**Editing characters:**

| Char | Meaning |
|---|---|
| `Z` | Replace leading zeros with spaces |
| `*` | Replace leading zeros with asterisks (check protection) |
| `$` | Dollar sign insertion; floating `$` replaces leading zeros |
| `+` | Floating plus; shows `+` for positive, `-` for negative |
| `-` | Floating minus; shows `-` for negative, space for positive |
| `.` | Decimal point insertion (actual period character) |
| `,` | Comma insertion (actual comma character) |
| `B` | Space insertion |
| `0` | Zero insertion |
| `/` | Slash insertion |
| `CR` | Shows "CR" if negative, spaces if positive |
| `DB` | Shows "DB" if negative, spaces if positive |

**Rust type:**
```rust
/// Numeric edited field -- display-only, no arithmetic.
///
/// The PIC editing pattern is stored as a compile-time constant string.
/// The format() method applies the editing rules to produce the display
/// string. The transpiler generates a dedicated formatter for each
/// unique PIC editing pattern encountered in the source.
///
/// N: total byte length of the formatted output
pub struct NumericEdited<const N: usize> {
    data: [u8; N],  // formatted display bytes
}

impl<const N: usize> NumericEdited<N> {
    /// Apply the editing pattern to a numeric value.
    /// This is the core of COBOL's MOVE-to-edited-field behavior.
    fn format(&mut self, value: &rust_decimal::Decimal, pattern: &EditPattern);
}

/// Compile-time representation of a PIC editing pattern.
/// Generated by the transpiler for each unique pattern.
pub struct EditPattern {
    positions: &'static [EditPosition],
    // Tracks: which positions are digit slots, which are insertion
    // characters, where the decimal point goes, sign placement, etc.
}
```

**BLANK WHEN ZERO:** Modifier clause -- when the entire value is zero, the
formatted output is all spaces. Implemented as a check in `format()`.

**DECIMAL-POINT IS COMMA:** Program-level configuration that swaps `.` and `,`
in all numeric editing. Implemented as a runtime configuration flag.

---

### 4.2 Alphanumeric Edited

**COBOL:**
```cobol
01 WS-PHONE         PIC X(3)BX(3)BX(4).
01 WS-DATE-FMT      PIC XX/XX/XXXX.
```

**Semantics:** Alphanumeric with insertion characters (B, 0, /). Display-only.
Values are formatted during MOVE.

**Rust type:**
```rust
/// Alphanumeric edited -- same approach as NumericEdited.
/// Insertion characters are placed at fixed positions during MOVE.
pub struct AlphanumericEdited<const N: usize> {
    data: [u8; N],
}
```

Lower implementation priority than numeric edited.

---

## 5. Structural Constructs

### 5.1 GROUP Items (Records -> Structs)

**COBOL:**
```cobol
01 CUSTOMER-RECORD.
   05 CUST-ID         PIC 9(8).
   05 CUST-NAME.
      10 FIRST-NAME   PIC X(20).
      10 LAST-NAME    PIC X(30).
   05 CUST-BALANCE    PIC S9(7)V99 COMP-3.
   05 CUST-STATUS     PIC X.
```

**Semantics:** GROUP items are hierarchical containers. A group item's length
is the sum of all its children. Group items are treated as alphanumeric
(PIC X equivalent) when used in MOVE or comparison -- the entire byte range
is treated as a flat character string.

**Rust type:**
```rust
/// Each GROUP item becomes a Rust struct.
/// The #[repr(C)] attribute ensures field order matches COBOL layout.
/// Nested groups become nested structs.
#[repr(C)]
pub struct CustomerRecord {
    pub cust_id: ZonedDecimal<8, 0, 8>,
    pub cust_name: CustName,
    pub cust_balance: PackedDecimal<9, 2, 5>,
    pub cust_status: PicX<1>,
}

#[repr(C)]
pub struct CustName {
    pub first_name: PicX<20>,
    pub last_name: PicX<30>,
}

// The transpiler also generates:
// - to_bytes() / from_bytes() for flat record I/O
// - as_alphanumeric() to get the group as a PicX view (for group MOVE)
// - A RECORD_LENGTH constant
```

**Group-as-alphanumeric duality:** When COBOL treats a group item as a sending
or receiving field in a MOVE, it treats the entire byte range as PIC X. The
transpiler must generate a `to_bytes()` / `from_bytes()` method that flattens
the struct to/from a contiguous byte array.

---

### 5.2 REDEFINES (Memory Overlay -> Enum/Union)

**COBOL:**
```cobol
01 TRANSACTION-RECORD.
   05 TRANS-TYPE       PIC X.
   05 TRANS-DATA       PIC X(100).
   05 TRANS-PAYMENT  REDEFINES TRANS-DATA.
      10 PAY-AMOUNT    PIC S9(9)V99 COMP-3.
      10 PAY-METHOD    PIC X(10).
   05 TRANS-REFUND   REDEFINES TRANS-DATA.
      10 REF-AMOUNT    PIC S9(9)V99 COMP-3.
      10 REF-REASON    PIC X(50).
```

**Approach: Safe enum with I/O conversion (recommended)**
```rust
/// Discriminated by TRANS-TYPE, parsed from raw bytes during record read.
enum TransactionData {
    Payment {
        amount: PackedDecimal<11, 2, 6>,
        method: PicX<10>,
    },
    Refund {
        amount: PackedDecimal<11, 2, 6>,
        reason: PicX<50>,
    },
    /// Fallback for unknown TRANS-TYPE values
    Raw([u8; 100]),
}

pub struct TransactionRecord {
    pub trans_type: PicX<1>,
    pub trans_data: TransactionData,
}

// The transpiler generates conversion code:
// - On record read: inspect trans_type, parse trans_data bytes into the
//   appropriate enum variant
// - On record write: serialize the active variant back to 100 bytes
```

**Alternative: Raw union (for byte-level compatibility)**
```rust
/// Use only when binary-exact memory layout is required (e.g., memory-mapped
/// file access where the record is accessed in-place).
#[repr(C)]
pub union TransactionDataRaw {
    pub payment: PaymentData,
    pub refund: RefundData,
    pub raw: [u8; 100],
}
// Accessing fields of a union requires `unsafe`.
```

**Decision:** The safe enum approach is preferred for all business logic. The
raw union approach is available for performance-critical zero-copy scenarios.

---

### 5.3 OCCURS (Arrays / Tables)

**COBOL:**
```cobol
*> Fixed-size array
01 MONTHLY-TOTALS.
   05 MONTH-ENTRY     OCCURS 12 TIMES.
      10 MONTH-AMT    PIC S9(7)V99 COMP-3.
      10 MONTH-COUNT  PIC 9(5) COMP.

*> Variable-length array (OCCURS DEPENDING ON)
01 VARIABLE-TABLE.
   05 ENTRY-COUNT     PIC 9(3).
   05 TABLE-ENTRY     OCCURS 1 TO 500 TIMES
                      DEPENDING ON ENTRY-COUNT.
      10 ENTRY-DATA   PIC X(20).

*> Multi-dimensional
01 MATRIX.
   05 ROW             OCCURS 10 TIMES.
      10 COL          OCCURS 20 TIMES.
         15 CELL-VAL  PIC 9(5).
```

**Rust types:**
```rust
/// Fixed OCCURS -> Rust fixed-size array.
/// COBOL arrays are 1-indexed; the runtime provides a 1-based accessor.
pub struct CobolArray<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> CobolArray<T, N> {
    /// 1-based index access (COBOL convention).
    /// Panics if index is 0 or > N (matches COBOL runtime behavior
    /// when subscript checking is enabled).
    fn get(&self, index: usize) -> &T;
    fn get_mut(&mut self, index: usize) -> &mut T;
}

// Fixed OCCURS example:
pub struct MonthlyTotals {
    pub month_entry: CobolArray<MonthEntry, 12>,
}

/// Variable OCCURS -> Vec<T> with a maximum capacity.
/// The DEPENDING ON identifier controls the active length.
pub struct CobolVarArray<T, const MAX: usize> {
    data: Vec<T>,
    // Invariant: data.len() <= MAX
}

// Variable OCCURS example:
pub struct VariableTable {
    pub entry_count: ZonedDecimal<3, 0, 3>,
    pub table_entry: CobolVarArray<TableEntry, 500>,
    // entry_count.value() determines how many entries are "active"
}

// Multi-dimensional: nested CobolArrays
pub struct Matrix {
    pub row: CobolArray<Row, 10>,
}
pub struct Row {
    pub col: CobolArray<CellVal, 20>,
}
```

**INDEXED BY:** The INDEXED BY clause creates an index variable for use with
SEARCH. In Rust, this becomes a `usize` variable associated with the array.

**SEARCH / SEARCH ALL:** Linear and binary search respectively. Implemented as
methods on `CobolArray`:
```rust
impl<T, const N: usize> CobolArray<T, N> {
    /// SEARCH (linear scan)
    fn search<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<(usize, &T)>;
    /// SEARCH ALL (binary search, requires sorted data)
    fn search_all<F: Fn(&T) -> Ordering>(&self, compare: F) -> Option<(usize, &T)>;
}
```

---

### 5.4 Level 88 (Condition Names -> Enums/Predicates)

**COBOL:**
```cobol
05 CUST-STATUS      PIC X.
   88 ACTIVE        VALUE 'A'.
   88 INACTIVE      VALUE 'I'.
   88 SUSPENDED     VALUE 'S'.

05 TRANS-CODE       PIC 99.
   88 VALID-TRANS   VALUE 10 THRU 50.
   88 SPECIAL-TRANS VALUE 01 03 07.
```

**Strategy: Hybrid approach**

Simple 88-levels (single value, small finite set) -> Rust enum:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustStatus {
    Active,
    Inactive,
    Suspended,
}

impl TryFrom<u8> for CustStatus {
    type Error = InvalidValueError;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'A' => Ok(CustStatus::Active),
            b'I' => Ok(CustStatus::Inactive),
            b'S' => Ok(CustStatus::Suspended),
            _ => Err(InvalidValueError(byte)),
        }
    }
}

impl From<CustStatus> for u8 {
    fn from(status: CustStatus) -> u8 {
        match status {
            CustStatus::Active => b'A',
            CustStatus::Inactive => b'I',
            CustStatus::Suspended => b'S',
        }
    }
}

// SET ACTIVE TO TRUE -> cust_status = CustStatus::Active
// IF ACTIVE -> cust_status == CustStatus::Active
```

Complex 88-levels (ranges, multiple values) -> predicate methods:
```rust
impl TransCode {
    /// 88 VALID-TRANS VALUE 10 THRU 50
    fn is_valid_trans(&self) -> bool {
        let v = self.value();
        v >= 10 && v <= 50
    }

    /// 88 SPECIAL-TRANS VALUE 01 03 07
    fn is_special_trans(&self) -> bool {
        matches!(self.value(), 1 | 3 | 7)
    }
}
```

---

### 5.5 Level 66 RENAMES (Byte-Range Alias -> Slice)

**COBOL:**
```cobol
01 WS-RECORD.
   05 WS-FIELD-A    PIC X(5).
   05 WS-FIELD-B    PIC X(10).
   05 WS-FIELD-C    PIC X(3).
66 WS-PARTIAL       RENAMES WS-FIELD-A THRU WS-FIELD-B.
```

**Rust type:**
```rust
// RENAMES creates a byte-range view over the parent record.
// Implemented as a method that returns a slice.
impl WsRecord {
    /// 66 WS-PARTIAL RENAMES WS-FIELD-A THRU WS-FIELD-B
    /// Returns bytes 0..15 (5 + 10 = 15 bytes)
    fn ws_partial(&self) -> &[u8] {
        let bytes = self.to_bytes();
        &bytes[0..15]
    }

    fn ws_partial_mut(&mut self) -> &mut [u8] {
        let bytes = self.to_bytes_mut();
        &mut bytes[0..15]
    }
}
```

Level 66 RENAMES is uncommon in modern COBOL. Lower implementation priority.

---

### 5.6 Level 77 (Independent Items)

**COBOL:**
```cobol
77 WS-COUNTER       PIC S9(4) COMP.
77 WS-FLAG          PIC X.
```

**Rust type:** Standalone variable of the appropriate type. No special
handling -- identical to a level 01 elementary item.

```rust
let ws_counter: CompBinary<4, 2> = CompBinary::new(0);
let ws_flag: PicX<1> = PicX::new();
```

---

### 5.7 FILLER (Padding)

**COBOL:**
```cobol
01 WS-RECORD.
   05 WS-NAME       PIC X(20).
   05 FILLER         PIC X(5).
   05 WS-CODE        PIC X(3).
```

**Rust type:**
```rust
#[repr(C)]
pub struct WsRecord {
    pub ws_name: PicX<20>,
    pub _filler_1: [u8; 5],   // unnamed padding
    pub ws_code: PicX<3>,
}
```

FILLER items are initialized to spaces by the `INITIALIZE` verb (unless
`INITIALIZE ... WITH FILLER` is specified, which initializes them per type).

---

## 6. Fixed-Point Arithmetic Details

### 6.1 Implied Decimal Point (V)

The `V` in a PIC clause marks the implied decimal position. It is NOT stored
in memory -- it is a compile-time concept.

**Examples:**
- `PIC S9(5)V99` = 5 integer digits, 2 decimal places = 7 total digits
- `PIC V9(6)` = 0 integer digits, 6 decimal places (always < 1)
- `PIC 9(7)` = 7 integer digits, 0 decimal places (integer)

**Arithmetic alignment:** COBOL automatically aligns decimal points during
arithmetic. Adding `PIC S9(5)V99` (scale=2) to `PIC S9(3)V9(4)` (scale=4)
produces a result with the scale of the receiving field.

**Rust implementation:** The scale is encoded in const generics of each type.
`rust_decimal::Decimal` natively tracks scale, so arithmetic alignment is
handled automatically. The transpiler ensures the result is truncated/rounded
to match the receiving field's scale.

---

### 6.2 Scaling Position (P)

**COBOL:**
```cobol
01 WS-MILLIONS      PIC 9(3)P(6).
01 WS-TINY          PIC VP(4)9(3).
```

**Semantics:** `P` represents assumed zeros NOT stored in memory:
- `PIC 9(3)P(6)`: stores 3 digits, value is multiplied by 10^6.
  Storing "123" means 123,000,000. Only 3 bytes storage.
- `PIC VP(4)9(3)`: stores 3 digits, positioned after 4 implied zeros.
  Storing "456" means 0.0000456. Only 3 bytes storage.

**Rust implementation:** The P positions are encoded as an additional scale
offset. The transpiler computes the effective scale:
- `PIC 9(3)P(6)`: effective scale = -6 (multiply stored value by 10^6)
- `PIC VP(4)9(3)`: effective scale = 7 (4 P-positions + 3 digit positions)

```rust
// P is handled as a scale adjustment at the type level.
// The transpiler computes the effective scale and generates the type:
// PIC 9(3)P(6)  -> ScaledDecimal with stored_digits=3, scale_offset=-6
// VP(4)9(3)     -> ScaledDecimal with stored_digits=3, scale_offset=7
```

P-scaling is uncommon. Lower implementation priority.

---

### 6.3 Sign Handling

Sign encoding depends on the combination of USAGE and SIGN clause:

**COMP/COMP-5:** Sign is part of the binary representation (two's complement).
No special handling needed -- Rust signed integers handle this natively.

**COMP-3:** Sign is in the low nibble of the last byte:
- `0xC` = positive
- `0xD` = negative
- `0xF` = unsigned

**DISPLAY (zoned decimal):** Sign is embedded in the zone nibble or stored as
a separate character (see Section 2.1 table).

All sign handling is a serialization concern. Internally, all numeric types
use signed Rust types (`rust_decimal::Decimal` or `i64` etc.), and the sign
encoding only applies when reading from or writing to record buffers.

---

## 7. Storage Size Reference

### 7.1 Complete Size Table

| USAGE | Unsigned Formula | Signed Formula | Example: S9(7)V99 |
|---|---|---|---|
| DISPLAY | digits bytes | digits bytes (+1 if SEPARATE) | 9 bytes |
| COMP/COMP-4 | 2/4/8 by digit count | same | 4 bytes |
| COMP-3 | ceil((d+1)/2) | ceil((d+1)/2) | 5 bytes |
| COMP-5 | 1/2/4/8 by digit count | same | 4 bytes |
| COMP-1 | 4 (always) | N/A | N/A |
| COMP-2 | 8 (always) | N/A | N/A |

### 7.2 Maximum Precision by Dialect

| Feature | IBM Enterprise COBOL | GnuCOBOL | Micro Focus |
|---|---|---|---|
| Max PIC 9 digits | 18 | 38 | 18 (38 optional) |
| Max COMP-3 digits | 18 | 38 | 18 (38 optional) |
| Max COMP/COMP-5 digits | 18 | 18 | 18 |
| Max PIC X length | ~128 MB | memory-limited | ~2 GB |
| Max OCCURS | 134,217,727 | memory-limited | memory-limited |
| Max OCCURS dimensions | 7 | 7+ | 7 |
| Max level depth | 49 | 49 | 49 |

### 7.3 Alignment (SYNCHRONIZED Clause)

IBM Enterprise COBOL alignment when SYNC is specified:

| USAGE | Storage | Alignment Boundary |
|---|---|---|
| COMP (2 bytes) | 2 | halfword (2-byte) |
| COMP (4 bytes) | 4 | fullword (4-byte) |
| COMP (8 bytes) | 8 | doubleword (8-byte) |
| COMP-1 | 4 | fullword (4-byte) |
| COMP-2 | 8 | doubleword (8-byte) |
| INDEX | 4/8 | fullword/doubleword |

When SYNC is specified, the compiler may insert **slack bytes** (implicit
FILLER) before the item to achieve alignment. These slack bytes affect the
total record length and field offsets.

**Rust equivalent:** `#[repr(align(N))]` on the field or containing struct.
The transpiler must compute slack bytes when generating record layouts for
binary-compatible I/O.

---

## 8. Trait Architecture

### 8.1 Trait Hierarchy

```
CobolType (root)
  |
  +-- CobolNumeric
  |     |
  |     +-- CobolDisplay    (PIC 9, zoned decimal)
  |     +-- CobolPacked     (COMP-3)
  |     +-- CobolBinary     (COMP, COMP-4, COMP-5)
  |     +-- CobolFloat      (COMP-1, COMP-2)
  |
  +-- CobolAlphanumeric
  |     |
  |     +-- CobolAlphabetic (PIC A -- validation only)
  |     +-- CobolNational   (PIC N)
  |
  +-- CobolEdited
        |
        +-- CobolNumericEdited
        +-- CobolAlphaEdited
```

### 8.2 Root Trait

```rust
pub trait CobolType {
    /// Storage size in bytes (for record I/O)
    fn byte_length(&self) -> usize;

    /// Serialize to a record buffer at the given offset
    fn to_bytes(&self, buf: &mut [u8]);

    /// Deserialize from a record buffer at the given offset
    fn from_bytes(buf: &[u8]) -> Result<Self, CobolError> where Self: Sized;

    /// COBOL INITIALIZE verb behavior
    fn initialize(&mut self);

    /// COBOL MOVE verb -- assigns from any CobolType with appropriate
    /// conversion rules (numeric-to-numeric, alpha-to-alpha, etc.)
    fn move_from(&mut self, source: &dyn CobolType);
}
```

### 8.3 Numeric Trait

```rust
pub trait CobolNumeric: CobolType {
    /// Get the value as a canonical Decimal
    fn to_decimal(&self) -> rust_decimal::Decimal;

    /// Set the value from a canonical Decimal
    fn set_decimal(&mut self, value: rust_decimal::Decimal);

    /// Number of implied decimal places (V position)
    fn scale(&self) -> u32;

    /// Total digit count
    fn precision(&self) -> u32;

    /// Is this a signed type?
    fn is_signed(&self) -> bool;

    /// COBOL numeric comparison (aligns decimal points)
    fn compare_numeric(&self, other: &dyn CobolNumeric) -> Ordering;
}
```

### 8.4 Required Operator Implementations

Every numeric type must implement:

**Arithmetic** (std::ops): `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`,
plus the `Assign` variants (`AddAssign`, etc.)

**Comparison**: `PartialEq`, `Eq`, `PartialOrd`, `Ord`

**Conversion**: `From<i32>`, `From<i64>`, `From<&str>`, `TryFrom<Decimal>`,
`FromStr`

**Display**: `Display` (for COBOL DISPLAY verb), `Debug`

---

## 9. Crate Dependencies

### 9.1 Required (high confidence)

| Crate | Purpose | Why |
|---|---|---|
| `rust_decimal` | Decimal arithmetic | COMP-3 and DISPLAY numeric arithmetic, exact decimal math |
| `byteorder` | Endianness handling | Mainframe data is big-endian, target hardware is little-endian |
| `serde` | Serialization | Record I/O framework |
| `num-traits` | Numeric trait abstractions | Zero, One, Num traits for generic numeric code |

### 9.2 Recommended

| Crate | Purpose | Why |
|---|---|---|
| `zerocopy` | Zero-copy record parsing | Efficient buffer-to-struct conversion for simple layouts |
| `derive_more` | Auto-derive traits | Reduces boilerplate on newtype wrappers |
| `thiserror` | Error type derivation | Clean error types for the runtime |

### 9.3 Consider

| Crate | Purpose | When |
|---|---|---|
| `packed_struct` | Derive-based binary layouts | If derive macros for record packing are needed |
| `memmap2` | Memory-mapped file I/O | Large sequential file processing |
| `num-bigint` | Arbitrary precision | GnuCOBOL 38-digit numeric support |

### 9.4 Must Build Custom (no adequate crate exists)

| Component | What It Does |
|---|---|
| **COMP-3 BCD codec** | Pack/unpack nibbles, sign nibble (C/D/F), odd/even alignment |
| **Zoned decimal codec** | Zone nibble encoding (EBCDIC: 0xFn, ASCII: 0x3n), sign embedding |
| **EBCDIC translation tables** | Per-code-page 256-byte lookup tables (CP037, CP1140, CP500) |
| **COBOL MOVE engine** | Type coercion rules, truncation, padding, numeric-to-alpha conversion |
| **PIC editing formatter** | Z, *, $, +, -, comma insertion, CR/DB, BLANK WHEN ZERO |
| **COBOL arithmetic semantics** | Left-truncation on overflow, ROUNDED phrase, ON SIZE ERROR |

---

## 10. Architectural Decisions

### Decision 1: Internal Numeric Representation

| Option | Pros | Cons | Verdict |
|---|---|---|---|
| `rust_decimal` wrapped | Battle-tested, 28 digits, all rounding modes | 28-digit limit, runtime scale | **Selected** for 99% case |
| Custom `i128` + scale | 38 digits, compile-time scale | Manual arithmetic implementation | Fallback for 31+ digit fields |
| `num-bigint` | Unlimited precision | Heap allocation, slow | Only for extreme edge cases |

### Decision 2: Record Layout Strategy

| Option | Pros | Cons | Verdict |
|---|---|---|---|
| `#[repr(C, packed)]` structs | Memory-compatible, zero-copy | Unaligned access issues | Wire format only |
| Safe typed structs | Safe, idiomatic Rust | Copy overhead on I/O | **Selected** for business logic |
| Byte buffer + accessors | Closest to COBOL | No type safety | Rejected |

**Two-layer approach:** Raw `#[repr(C, packed)]` for I/O serialization + safe
typed struct for business logic. The transpiler generates both and the
conversion code.

### Decision 3: String Representation

| Option | Pros | Cons | Verdict |
|---|---|---|---|
| `PicX<N>([u8; N])` newtype | Fixed-size, stack, COBOL semantics | Const generics boilerplate | **Selected** |
| `[u8; N]` bare | No wrapper overhead | No methods, no type safety | Rejected |
| `String` | Idiomatic Rust | Heap, variable-length, wrong semantics | Rejected |

### Decision 4: REDEFINES Strategy

| Option | Pros | Cons | Verdict |
|---|---|---|---|
| Safe `enum` (tagged union) | Pattern matching, exhaustive | Not memory-layout-compatible | **Selected** for business logic |
| `union` (raw) | Memory-compatible | Unsafe access | Available for zero-copy I/O |

Parse raw bytes -> safe enum on read, serialize enum -> raw bytes on write.

### Decision 5: 88-Level Strategy

| Option | When Applied | Verdict |
|---|---|---|
| Rust `enum` + TryFrom/From | Single values, small finite set | **Selected** for simple cases |
| Predicate methods | Ranges (THRU), large/multiple value sets | **Selected** for complex cases |

### Decision 6: Const Generics Limitation Workaround

Rust's `generic_const_exprs` feature (computing `[u8; (DIGITS+2)/2]` from
const generic parameters) is NOT stabilized. Workaround:

```rust
// The transpiler computes STORAGE at code generation time
// and passes it as a separate const generic parameter.
struct PackedDecimal<const DIGITS: usize, const SCALE: usize, const STORAGE: usize> {
    value: rust_decimal::Decimal,
}
// Example: PIC S9(7)V99 COMP-3
//   DIGITS=9, SCALE=2, STORAGE=5 (computed by transpiler: ceil(10/2)=5)
```

---

## 11. Key Transpiler Challenges

### 11.1 Arithmetic Fidelity

COBOL arithmetic has specific behaviors that differ from Rust defaults:

| Behavior | COBOL | Rust Default | Resolution |
|---|---|---|---|
| Integer overflow | Silent left-truncation | Panic (debug) / wrap (release) | Custom truncation |
| Decimal overflow | Silent left-truncation | N/A (decimals don't overflow the same way) | Truncate to PIC range |
| Division by zero | ON SIZE ERROR or undefined | Panic | Match ON SIZE ERROR |
| Rounding | ROUNDED phrase per operation | No default rounding | Use rust_decimal rounding modes |
| Intermediate precision | Compiler-dependent (often 18+ digits) | Full precision | Configurable |

### 11.2 MOVE Semantics

COBOL MOVE is the most complex single operation to implement:

| Source -> Dest | Behavior |
|---|---|
| Numeric -> Numeric | Decimal alignment, truncation, sign propagation |
| Numeric -> Alphanumeric | Convert to display string, right-pad with spaces |
| Alphanumeric -> Alphanumeric | Left-justify, right-pad or truncate |
| Alphanumeric -> Numeric | Parse as numeric (or zero if non-numeric) |
| Group -> Group | Flat byte move (alphanumeric semantics) |
| Group -> Elementary | Flat byte move (alphanumeric semantics) |
| Elementary -> Group | Flat byte move (alphanumeric semantics) |

### 11.3 EBCDIC Collating Sequence

String comparisons on mainframes use EBCDIC ordering, which differs from ASCII:
- EBCDIC: space < lowercase < uppercase < digits
- ASCII: space < digits < uppercase < lowercase

The transpiler must provide an EBCDIC-aware comparison mode for programs that
depend on sort order.

### 11.4 1-Based Indexing

All COBOL arrays are 1-indexed. The `CobolArray` wrapper must consistently
convert between COBOL's 1-based and Rust's 0-based indexing. All generated
code must use the wrapper, never raw array indexing.

### 11.5 OCCURS DEPENDING ON (ODO)

Variable-length arrays where field offsets after the ODO item change at
runtime. This requires:
- Dynamic offset calculation for subsequent fields
- Record length recalculation when the DEPENDING ON variable changes
- Special handling during record I/O (only "active" elements are read/written)

### 11.6 Numeric Truncation Direction

COBOL truncates from the LEFT (high-order digits are discarded). This is the
opposite of most programmer intuition. Example:
- MOVE 12345 TO PIC 9(3) -> result is 345 (not 123)

This must be implemented explicitly in all numeric assignment operations.

---

## 12. Cross-Dialect Mapping Matrix

This section maps every COBOL data type across five dialect columns and the
corresponding Rust target type. Where a cell shows "--" the feature is not
available in that dialect. Where a cell shows "= IBM" or similar, the behavior
is identical to the referenced dialect.

**Dialect abbreviations used in tables:**

| Abbrev | Dialect | Version/Notes |
|---|---|---|
| **IBM** | IBM Enterprise COBOL for z/OS | V6.4+ (primary transpiler target) |
| **GNU** | GnuCOBOL | 3.x (open-source, compiles to C) |
| **MF** | Micro Focus Visual COBOL | Latest (commercial, widespread) |
| **C85** | COBOL-85 Standard | ANSI X3.23-1985 (baseline compatibility) |
| **C14** | COBOL 2014 Standard | ISO/IEC 1989:2014 (modern standard) |

---

### 12.1 Numeric Types -- USAGE Variants

| Concept | IBM | GNU | MF | C85 | C14 | Rust Type |
|---|---|---|---|---|---|---|
| **Zoned decimal** | `DISPLAY` (default) | `DISPLAY` (default) | `DISPLAY` (default) | `DISPLAY` (default) | `DISPLAY` (default) | `ZonedDecimal<I,D,S>` |
| Max digits (zoned) | 18 | 38 | 18 (38 opt) | 18 | 18 | `rust_decimal` (28) / `i128` (38) |
| **Packed decimal** | `COMP-3` | `COMP-3` | `COMP-3` | `COMP-3` | `PACKED-DECIMAL` | `PackedDecimal<D,S,B>` |
| Packed synonym | `PACKED-DECIMAL` | `PACKED-DECIMAL` | `PACKED-DECIMAL` | -- | `COMP-3` | (same type) |
| Max digits (packed) | 18 | 38 | 18 (38 opt) | 18 | 18 | `rust_decimal` / `i128` |
| **PIC-limited binary** | `COMP` / `COMP-4` | `COMP` (configurable) | `COMP` / `COMP-4` | `COMP` | `BINARY` (PIC-limited) | `CompBinary<N,S>` |
| COMP behavior | PIC-range-limited | Configurable (see note) | PIC-range-limited | PIC-range-limited | PIC-range-limited | Range-check wrapper |
| **Native binary** | `COMP-5` | `COMP-5` | `COMP-5` | -- | -- | `i16`/`i32`/`i64` direct |
| BINARY keyword meaning | = COMP (PIC-limited) | Configurable | = COMP-5 (full range) | = COMP | = COMP | Depends on dialect config |
| **Standard binary names** | Supported | Supported | Supported | -- | Defined here | (see below) |
| BINARY-CHAR (1 byte) | `i8`/`u8` | `i8`/`u8` | `i8`/`u8` | -- | `i8`/`u8` | `i8` / `u8` |
| BINARY-SHORT (2 bytes) | `i16`/`u16` | `i16`/`u16` | `i16`/`u16` | -- | `i16`/`u16` | `i16` / `u16` |
| BINARY-LONG (4 bytes) | `i32`/`u32` | `i32`/`u32` | `i32`/`u32` | -- | `i32`/`u32` | `i32` / `u32` |
| BINARY-DOUBLE (8 bytes) | `i64`/`u64` | `i64`/`u64` | `i64`/`u64` | -- | `i64`/`u64` | `i64` / `u64` |
| **Single float** | `COMP-1` | `COMP-1` | `COMP-1` | `COMP-1` | `FLOAT-SHORT` | `f32` |
| Float format (single) | HFP or IEEE (option) | IEEE 754 | IEEE 754 | Impl-defined | IEEE 754 | `f32` (IEEE only) |
| **Double float** | `COMP-2` | `COMP-2` | `COMP-2` | `COMP-2` | `FLOAT-LONG` | `f64` |
| Float format (double) | HFP or IEEE (option) | IEEE 754 | IEEE 754 | Impl-defined | IEEE 754 | `f64` (IEEE only) |
| **Extended float** | -- | `FLOAT-EXTENDED` | -- | -- | `FLOAT-EXTENDED` | -- (not supported) |
| **Unsigned packed** | -- | -- | `COMP-6` | -- | -- | `PackedDecimal` (unsigned) |
| **Byte-sized binary** | -- | -- | `COMP-X` | -- | -- | `u8`/`u16`/`u32`/`u64` |

**GnuCOBOL COMP behavior note:** GnuCOBOL's `binary-size` directive controls
whether plain `COMP` behaves like IBM's PIC-limited COMP or like COMP-5
(full binary range). Settings: `1-2-4-8` (COMP-5 behavior), `2-4-8` (IBM
default), or `1--8` (Micro Focus). The transpiler must respect this setting.

---

### 12.2 Numeric Storage Sizes by Dialect

| PIC Digits | IBM COMP | IBM COMP-5 | GNU COMP (default) | MF COMP | MF COMP-5 | Rust Primitive |
|---|---|---|---|---|---|---|
| 1-2 | 2 bytes | 2 bytes | 1 byte (config) | 2 bytes | 1 byte | `i8`/`i16` |
| 3-4 | 2 bytes | 2 bytes | 2 bytes | 2 bytes | 2 bytes | `i16` |
| 5-9 | 4 bytes | 4 bytes | 4 bytes | 4 bytes | 4 bytes | `i32` |
| 10-18 | 8 bytes | 8 bytes | 8 bytes | 8 bytes | 8 bytes | `i64` |
| 19-38 | -- | -- | 16 bytes (ext) | -- | -- | `i128` |

---

### 12.3 Alphanumeric and String Types

| Concept | IBM | GNU | MF | C85 | C14 | Rust Type |
|---|---|---|---|---|---|---|
| **Alphanumeric** | `PIC X(n)` | `PIC X(n)` | `PIC X(n)` | `PIC X(n)` | `PIC X(n)` | `PicX<N>` |
| Max PIC X length | ~128 MB | Memory-limited | ~2 GB | Impl-defined | Impl-defined | `PicX<N>` (stack) |
| Character encoding | EBCDIC (z/OS) | ASCII (native) | ASCII (native) | Impl-defined | Impl-defined | ASCII internal |
| **Alphabetic** | `PIC A(n)` | `PIC A(n)` | `PIC A(n)` | `PIC A(n)` | `PIC A(n)` | `PicX<N>` + validation |
| **National (UTF-16)** | `PIC N(n) NATIONAL` | Limited support | Supported (newer) | -- | `PIC N(n) NATIONAL` | `PicNational<N>` |
| **DBCS** | `PIC G(n) DISPLAY-1` | -- | -- | -- | -- | `PicNational<N>` |
| **Boolean** | -- | `PIC 1` (partial) | -- | -- | `PIC 1 USAGE BIT` | `bool` |
| JUSTIFIED RIGHT | Supported | Supported | Supported | Supported | Supported | `PicX` flag/method |

---

### 12.4 Edited Types

| Concept | IBM | GNU | MF | C85 | C14 | Rust Type |
|---|---|---|---|---|---|---|
| **Numeric edited** | Full support | Full support | Full support | Full support | Full support | `NumericEdited<N>` |
| Edit chars: Z * $ + - | Yes | Yes | Yes | Yes | Yes | `EditPattern` formatter |
| Edit chars: . , B 0 / | Yes | Yes | Yes | Yes | Yes | (same formatter) |
| CR / DB | Yes | Yes | Yes | Yes | Yes | (same formatter) |
| **Alpha edited** | `X` with B 0 / | = IBM | = IBM | = IBM | = IBM | `AlphanumericEdited<N>` |
| BLANK WHEN ZERO | Supported | Supported | Supported | Supported | Supported | Flag on formatter |
| DECIMAL-POINT IS COMMA | Supported | Supported | Supported | Supported | Supported | Runtime config flag |
| Custom CURRENCY SIGN | Supported | Supported | Supported | -- | Supported | Runtime config |

---

### 12.5 Sign Handling

| Concept | IBM | GNU | MF | C85 | C14 | Rust Serialization |
|---|---|---|---|---|---|---|
| Default sign position | Trailing embedded | Trailing embedded | Trailing embedded | Trailing embedded | Trailing embedded | Sign in zone nibble |
| SIGN LEADING | Supported | Supported | Supported | Supported | Supported | Leading zone encode |
| SIGN TRAILING | Supported | Supported | Supported | Supported | Supported | Trailing zone encode |
| SIGN LEADING SEPARATE | Supported | Supported | Supported | Supported | Supported | +1 byte prefix |
| SIGN TRAILING SEPARATE | Supported | Supported | Supported | Supported | Supported | +1 byte suffix |
| EBCDIC sign zone (C/D/F) | EBCDIC zones | ASCII overpunch | ASCII overpunch | Impl-defined | Impl-defined | Dialect-aware codec |
| ASCII overpunch chars | N/A (EBCDIC) | `{` +0, `}` -0, A-I +1..+9, J-R -1..-9 | Similar to GNU | N/A | N/A | Lookup table |

---

### 12.6 Structural Constructs

| Concept | IBM | GNU | MF | C85 | C14 | Rust Type |
|---|---|---|---|---|---|---|
| **GROUP items** | Level 01-49 | = IBM | = IBM | = IBM | = IBM | `#[repr(C)] struct` |
| Group-as-alpha duality | Yes | Yes | Yes | Yes | Yes | `to_bytes()` method |
| Max level depth | 49 | 49 | 49 | 49 | 49 | Nested structs |
| **REDEFINES** | Supported | Supported | Supported | Supported | Supported | `enum` / `union` |
| Redefining > redefined | Error | Allowed (warning) | Allowed | Error | Error | Match strictest |
| **Fixed OCCURS** | `OCCURS n` | = IBM | = IBM | = IBM | = IBM | `CobolArray<T,N>` |
| Max OCCURS count | 134,217,727 | Memory-limited | Memory-limited | Impl-defined | Impl-defined | Const generic N |
| Max OCCURS dimensions | 7 | 7+ | 7 | 7 | 7 | Nested arrays |
| **Variable OCCURS** | `OCCURS m TO n DEPENDING ON` | = IBM | = IBM | = IBM | = IBM | `CobolVarArray<T,N>` |
| INDEXED BY | Supported | Supported | Supported | Supported | Supported | `usize` variable |
| SEARCH / SEARCH ALL | Supported | Supported | Supported | Supported | Supported | `.search()` / `.search_all()` |
| **ASCENDING/DESCENDING KEY** | Supported | Supported | Supported | Supported | Supported | Sort metadata |
| **Level 88** (condition) | Supported | Supported | Supported | Supported | Supported | `enum` / predicate |
| 88 VALUE THRU | Supported | Supported | Supported | Supported | Supported | Range predicate |
| 88 FALSE clause | -- | Supported | -- | -- | Supported | Separate false value |
| **Level 66 RENAMES** | Supported | Supported | Supported | Supported | Supported | Slice method |
| **Level 77** (independent) | Supported | Supported | Supported | Supported | Supported | Standalone variable |
| **FILLER** | Named/unnamed | = IBM | = IBM | = IBM | = IBM | `[u8; N]` padding |
| Implicit FILLER (COBOL 2002+) | Unnamed fields OK | = IBM | = IBM | Named only | Unnamed OK | (same) |

---

### 12.7 Alignment and Storage Modifiers

| Concept | IBM | GNU | MF | C85 | C14 | Rust Equivalent |
|---|---|---|---|---|---|---|
| **SYNCHRONIZED** | Enforced (slack bytes) | Accepted, often ignored | Directive-dependent | Supported | Supported | `#[repr(align(N))]` |
| SYNC LEFT / RIGHT | Supported | Partial | Supported | Supported | Supported | Alignment attribute |
| Slack bytes inserted | Yes (automatic) | Rarely | Configurable | Impl-defined | Impl-defined | Transpiler computes |
| **USAGE INDEX** | 4 bytes (31-bit) | 4 or 8 bytes | 4 or 8 bytes | Impl-defined | Impl-defined | `usize` |
| **USAGE POINTER** | 4 bytes (31-bit) / 8 (64-bit) | Platform ptr size | Platform ptr size | -- | Supported | `usize` |
| **FUNCTION-POINTER** | Supported | -- | -- | -- | Supported | `fn()` pointer |
| **PROCEDURE-POINTER** | Supported (legacy) | Partial | Partial | -- | Deprecated | `fn()` pointer |

---

### 12.8 Arithmetic and Overflow Behavior

| Concept | IBM | GNU | MF | C85 | C14 | Rust Implementation |
|---|---|---|---|---|---|---|
| Overflow behavior | Silent left-truncation | = IBM | = IBM | = IBM | = IBM | Custom truncation logic |
| ON SIZE ERROR | Supported | Supported | Supported | Supported | Supported | `Result<T, SizeError>` |
| ROUNDED phrase | Supported | Supported | Supported | Supported | Supported | `rust_decimal` rounding |
| Rounding modes | NEAREST-AWAY-FROM-ZERO | = IBM | = IBM | NEAREST-AWAY-FROM-ZERO | 8 modes defined | Configurable |
| COBOL 2014 rounding modes | -- | Partial | Partial | -- | AWAY/NEAREST-EVEN/NEAREST-TOWARD-ZERO/PROHIBITED/TOWARD-GREATER/TOWARD-LESSER/TRUNCATION | `rust_decimal::RoundingStrategy` |
| Intermediate precision | 18+ digits (compiler) | GMP (arbitrary) | 18+ digits | Impl-defined | Impl-defined | `rust_decimal` (28 digits) |
| ARITH(EXTEND) option | 31-digit intermediates | N/A (always arbitrary) | N/A | -- | -- | `i128` fallback |

---

### 12.9 Encoding and Character Set

| Concept | IBM | GNU | MF | C85 | C14 | Rust Implementation |
|---|---|---|---|---|---|---|
| Native encoding | EBCDIC (z/OS) | ASCII | ASCII | Impl-defined | Impl-defined | ASCII internal |
| Primary code page | CP037 / CP1140 | System locale | System locale | -- | -- | Lookup table per CP |
| EBCDIC support needed | Source platform | Only for data interchange | Only for data interchange | -- | -- | I/O boundary conversion |
| Collating sequence | EBCDIC order | ASCII order | ASCII order | Impl-defined | Impl-defined | Dialect-aware comparator |
| PROGRAM COLLATING SEQUENCE | NATIVE / EBCDIC / custom | NATIVE / custom | = GNU | Supported | Supported | Runtime collation config |
| ALPHABET clause | Supported | Supported | Supported | Supported | Supported | Custom sort order table |
| National literal (N"...") | Supported | Limited | Supported | -- | Supported | UTF-8 `String` |
| Hex literal (X"...") | Supported | Supported | Supported | Supported | Supported | `&[u8]` literal |
| Null-terminated strings | Z"..." (V6.1+) | Z"..." | Z"..." | -- | Z"..." | `CString` / `&CStr` |

---

### 12.10 Dialect-Specific Extensions (No Cross-Dialect Equivalent)

| Feature | Dialect | COBOL Syntax | Rust Type | Notes |
|---|---|---|---|---|
| COMP-6 (unsigned packed) | MF only | `PIC 9(n) COMP-6` | `PackedDecimal` (unsigned, no sign nibble) | Storage = ceil(n/2) bytes |
| COMP-X (byte-binary) | MF only | `PIC X(n) COMP-X` | `u8`/`u16`/`u32`/`u64` by byte count | n bytes = n-byte unsigned int |
| FLOAT-DECIMAL-16 | IBM (V6.1+) | `USAGE FLOAT-DECIMAL-16` | -- (no Rust IEEE decimal) | IEEE 754 decimal64 |
| FLOAT-DECIMAL-34 | IBM (V6.1+) | `USAGE FLOAT-DECIMAL-34` | -- (no Rust IEEE decimal) | IEEE 754 decimal128 |
| FLOAT-BINARY-32 | IBM (V6.1+) | `USAGE FLOAT-BINARY-32` | `f32` | IEEE 754 binary32 |
| FLOAT-BINARY-64 | IBM (V6.1+) | `USAGE FLOAT-BINARY-64` | `f64` | IEEE 754 binary64 |
| FLOAT-BINARY-128 | IBM (V6.1+) | `USAGE FLOAT-BINARY-128` | -- (no Rust f128) | IEEE 754 binary128 |
| IBM HFP (hex float) | IBM (legacy) | `COMP-1`/`COMP-2` with `FLOAT(HEX)` | Custom `HexFloat32`/`HexFloat64` | Base-16 exponent, different precision |
| DISPLAY-1 (DBCS) | IBM only | `PIC G(n) DISPLAY-1` | `PicNational<N>` | CJK double-byte chars |
| UTF-8 data items | IBM (V6.4+) | `PIC X(n) USAGE UTF-8` | `String` / `Vec<u8>` | Variable-width encoding |
| JSON GENERATE / PARSE | IBM (V6.2+) | Verb, not type | `serde_json` | Runtime serialization |
| XML GENERATE / PARSE | IBM (V5.1+) | Verb, not type | `quick-xml` / `serde_xml` | Runtime serialization |
| VALIDATE statement | C14 only | `VALIDATE identifier` | Custom validation | Rarely implemented |

---

### 12.11 Feature Support Summary

A quick-reference showing which major features each dialect supports.
Y = full support, P = partial, -- = not available.

| Feature | IBM | GNU | MF | C85 | C14 | Transpiler Priority |
|---|---|---|---|---|---|---|
| PIC 9 DISPLAY (zoned) | Y | Y | Y | Y | Y | Phase 1 |
| COMP / COMP-4 (binary) | Y | Y | Y | Y | Y | Phase 1 |
| COMP-3 (packed decimal) | Y | Y | Y | Y | Y | Phase 1 |
| COMP-5 (native binary) | Y | Y | Y | -- | -- | Phase 1 |
| COMP-1 / COMP-2 (float) | Y | Y | Y | Y | Y | Phase 3 |
| BINARY-CHAR/SHORT/LONG/DOUBLE | Y | Y | Y | -- | Y | Phase 1 |
| PIC X (alphanumeric) | Y | Y | Y | Y | Y | Phase 1 |
| PIC A (alphabetic) | Y | Y | Y | Y | Y | Phase 1 |
| PIC N NATIONAL | Y | P | P | -- | Y | Phase 3 |
| Numeric edited | Y | Y | Y | Y | Y | Phase 2 |
| Alpha edited | Y | Y | Y | Y | Y | Phase 3 |
| GROUP items | Y | Y | Y | Y | Y | Phase 1 |
| REDEFINES | Y | Y | Y | Y | Y | Phase 2 |
| OCCURS (fixed) | Y | Y | Y | Y | Y | Phase 1 |
| OCCURS DEPENDING ON | Y | Y | Y | Y | Y | Phase 2 |
| Level 88 conditions | Y | Y | Y | Y | Y | Phase 1 |
| Level 66 RENAMES | Y | Y | Y | Y | Y | Phase 3 |
| SYNCHRONIZED | Y | P | P | Y | Y | Phase 3 |
| SIGN clauses (all) | Y | Y | Y | Y | Y | Phase 2 |
| EBCDIC encoding | Y (native) | P (data only) | P (data only) | -- | -- | Phase 2 |
| DECIMAL-POINT IS COMMA | Y | Y | Y | Y | Y | Phase 2 |
| CURRENCY SIGN | Y | Y | Y | -- | Y | Phase 2 |
| COMP-6 (unsigned packed) | -- | -- | Y | -- | -- | Phase 3 |
| COMP-X (byte-binary) | -- | -- | Y | -- | -- | Phase 3 |
| IEEE decimal float | Y (V6.1+) | P | -- | -- | Y | Phase 3 |
| IBM HFP float | Y (legacy) | -- | -- | -- | -- | Phase 3 |
| UTF-8 data items | Y (V6.4+) | -- | -- | -- | -- | Phase 3 |
| USAGE BIT (boolean) | -- | P | -- | -- | Y | Phase 3 |
| VALIDATE statement | -- | -- | -- | -- | Y | Not planned |

---

### 12.12 Transpiler Dialect Configuration

The transpiler must accept a dialect parameter that controls type mapping
behavior. The recommended configuration structure:

```
[dialect]
name = "ibm-enterprise"   # ibm-enterprise | gnucobol | microfocus | standard-85 | standard-2014
version = "6.4"

[dialect.binary]
# Controls how BINARY/COMP keywords map to storage
comp_is_pic_limited = true      # false for GnuCOBOL with binary-size=1-2-4-8
binary_keyword = "pic-limited"  # "pic-limited" (IBM) | "full-range" (MF)

[dialect.precision]
max_pic_digits = 18             # 38 for GnuCOBOL
max_comp3_digits = 18           # 38 for GnuCOBOL

[dialect.encoding]
source_encoding = "ebcdic"      # "ebcdic" | "ascii"
ebcdic_codepage = "cp037"       # cp037 | cp1140 | cp500

[dialect.float]
format = "ieee"                 # "ieee" | "hfp" (IBM legacy)

[dialect.sign]
default_position = "trailing"   # trailing | leading
default_separate = false
overpunch_table = "ebcdic"      # "ebcdic" | "ascii-gnucobol" | "ascii-microfocus"

[dialect.collation]
default_sequence = "ebcdic"     # "ebcdic" | "ascii" | "native"
```

This configuration determines which Rust type and serialization codec the
transpiler selects for each COBOL data item.

---

## Appendix A: GnuCOBOL Type Mapping (Prior Art)

GnuCOBOL compiles COBOL to C, providing a reference for how COBOL types
map to a systems language:

| COBOL Type | GnuCOBOL C Representation |
|---|---|
| PIC 9 DISPLAY | `unsigned char[]` (zoned, 1 byte/digit) |
| PIC S9 DISPLAY | `unsigned char[]` (sign in zone nibble) |
| COMP (small) | `short` / `int` / `long long` |
| COMP-3 | `unsigned char[]` (BCD packed, libcob handles arithmetic) |
| COMP-1 | `float` |
| COMP-2 | `double` |
| PIC X | `unsigned char[]` |
| GROUP | `struct` (with byte-level layout) |
| OCCURS | C array |
| REDEFINES | C union |

GnuCOBOL uses a runtime library (`libcob`) for decimal arithmetic, MOVE
semantics, and file I/O. Our Rust runtime library (`cobol-runtime`) serves
the same role.

**Key lesson from GnuCOBOL:** The C representation is deliberately low-level
(byte arrays + runtime library functions). Our Rust approach improves on this
by using the type system (const generics, traits, enums) to encode more
information at compile time, catching errors earlier while maintaining the
same runtime semantics.

---

## Appendix B: Implementation Priority

**Phase 1 -- Core (must have for any transpiled program):**
1. `PackedDecimal` (COMP-3) -- the workhorse of financial COBOL
2. `PicX` -- every program uses alphanumeric fields
3. `CompBinary` (COMP/COMP-5) -- common for counters and flags
4. GROUP item struct generation
5. COBOL MOVE engine (numeric-to-numeric, alpha-to-alpha)
6. CobolArray (fixed OCCURS)
7. Level 88 enum generation

**Phase 2 -- Extended:**
8. `ZonedDecimal` (DISPLAY numeric) -- common but COMP-3 is more critical
9. NumericEdited formatting
10. REDEFINES -> enum conversion
11. Variable OCCURS (DEPENDING ON)
12. EBCDIC translation tables
13. Record I/O (to_bytes / from_bytes)

**Phase 3 -- Complete:**
14. COMP-1 / COMP-2 (float)
15. PicNational (PIC N)
16. Level 66 RENAMES
17. Alphanumeric edited
18. P-scaling positions
19. SYNC / alignment slack bytes
20. IBM HFP floating-point (if needed)

---

## References

- IBM Enterprise COBOL for z/OS Language Reference (SC27-1408)
- COBOL 2014 Standard (ISO/IEC 1989:2014)
- GnuCOBOL Programmer's Guide (gnucobol.sourceforge.io)
- Micro Focus COBOL Language Reference
- rust_decimal crate documentation (docs.rs/rust_decimal)
- c2rust project (github.com/immunant/c2rust) -- architecture reference
- DARPA TRACTOR program -- C-to-Rust translation approach
