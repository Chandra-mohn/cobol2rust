# COBOL-to-Rust Runtime Library: Architecture Specification

## Purpose

This document defines the complete architecture of the `cobol-runtime`
workspace -- the Rust crate ecosystem that all transpiled COBOL programs
depend on. It is the capstone document that ties together the seven
foundational specifications into a concrete, implementable Rust project
structure.

This spec covers: workspace layout, crate dependency graph, canonicalized
trait hierarchy, concrete types catalog, error handling strategy, program
lifecycle model, configuration system, transpiler code generation patterns,
testing strategy, feature flags, performance considerations, and the phased
implementation roadmap.

**Prerequisite documents** (read these first):
1. `cobol_migration_target_analysis.md` -- Why Rust
2. `cobol_to_rust_datatype_mapping.md` -- Data types
3. `cobol_arithmetic_operations.md` -- Arithmetic engine
4. `cobol_control_flow_constructs.md` -- Control flow
5. `cobol_move_engine_spec.md` -- MOVE engine
6. `cobol_file_io_record_processing.md` -- File I/O
7. `cobol_sort_merge_spec.md` -- SORT/MERGE

---

## Table of Contents

1. [Design Philosophy](#1-design-philosophy)
2. [Workspace Structure](#2-workspace-structure)
3. [Crate Dependency Graph](#3-crate-dependency-graph)
4. [cobol-core: Foundation Crate](#4-cobol-core-foundation-crate)
5. [cobol-types: Concrete Types](#5-cobol-types-concrete-types)
6. [cobol-move: MOVE Engine](#6-cobol-move-move-engine)
7. [cobol-arithmetic: Arithmetic Engine](#7-cobol-arithmetic-arithmetic-engine)
8. [cobol-io: File I/O](#8-cobol-io-file-io)
9. [cobol-sort: SORT/MERGE](#9-cobol-sort-sortmerge)
10. [cobol-runtime: Program Lifecycle](#10-cobol-runtime-program-lifecycle)
11. [Configuration System](#11-configuration-system)
12. [Error Handling Strategy](#12-error-handling-strategy)
13. [Transpiler Code Generation Patterns](#13-transpiler-code-generation-patterns)
14. [Testing Strategy](#14-testing-strategy)
15. [Performance Considerations](#15-performance-considerations)
16. [Feature Flags and Optional Dependencies](#16-feature-flags-and-optional-dependencies)
17. [Implementation Roadmap](#17-implementation-roadmap)

---

## 1. Design Philosophy

### 1.1 Core Principles (Priority Order)

1. **Semantic fidelity** -- Transpiled Rust must produce identical results to
   the original COBOL under all conditions: arithmetic, truncation, padding,
   overflow, sign handling, collating sequence, file I/O behavior.

2. **Type safety** -- Use Rust's type system to catch at compile time what
   COBOL catches never. Make illegal states unrepresentable where possible
   without sacrificing fidelity.

3. **Zero-cost abstraction** -- Newtypes, const generics, and `#[repr]`
   attributes should produce code with no runtime overhead beyond what the
   equivalent COBOL machine code would incur.

4. **Modularity** -- A transpiled program that uses no file I/O should not
   link against SQLite. A program that uses no SORT should not link against
   tempfile. Feature flags and workspace crates enable this.

5. **Debuggability** -- When a transpiled program produces wrong results,
   it should be possible to trace the issue through meaningful Rust types
   and diagnostics, not opaque byte arrays.

### 1.2 What This Runtime Is NOT

- **Not a COBOL interpreter** -- No parsing, no runtime compilation. The
  transpiler generates static Rust code at build time.
- **Not an emulator** -- We do not emulate z/OS, VSAM, DFSORT, or JES. We
  provide equivalent semantics using native Rust/OS facilities.
- **Not a 1:1 byte layout** -- Internal representation uses Rust-native
  types (rust_decimal, etc.). Byte-exact layout is only maintained at I/O
  boundaries for record compatibility.

---

## 2. Workspace Structure

```
cobol2rust/
|
+-- Cargo.toml                     # Workspace root
|
+-- crates/
|   +-- cobol-core/                # Foundation: traits, enums, errors
|   |   +-- Cargo.toml
|   |   +-- src/
|   |       +-- lib.rs
|   |       +-- traits.rs          # Canonical trait hierarchy
|   |       +-- category.rs        # DataCategory enum
|   |       +-- error.rs           # CobolError, FileStatus, SortReturn
|   |       +-- config.rs          # CobolDialect, RuntimeConfig
|   |       +-- decimal_ext.rs     # Decimal utility extensions
|   |
|   +-- cobol-types/               # Concrete COBOL data types
|   |   +-- Cargo.toml
|   |   +-- src/
|   |       +-- lib.rs
|   |       +-- packed_decimal.rs  # COMP-3
|   |       +-- zoned_decimal.rs   # DISPLAY numeric
|   |       +-- comp_binary.rs     # COMP / COMP-4 / COMP-5
|   |       +-- pic_x.rs           # PIC X (alphanumeric)
|   |       +-- pic_a.rs           # PIC A (alphabetic)
|   |       +-- numeric_edited.rs  # Numeric-Edited fields
|   |       +-- alpha_edited.rs    # Alphanumeric-Edited fields
|   |       +-- group.rs           # GROUP item (struct + raw bytes)
|   |       +-- cobol_array.rs     # OCCURS (1-based array)
|   |       +-- level88.rs         # Level 88 condition names
|   |       +-- figurative.rs      # Figurative constants
|   |       +-- national.rs        # PIC N / PIC G (Phase 3)
|   |       +-- ebcdic.rs          # EBCDIC translation tables
|   |
|   +-- cobol-move/                # MOVE engine
|   |   +-- Cargo.toml
|   |   +-- src/
|   |       +-- lib.rs
|   |       +-- engine.rs          # Central cobol_move() dispatch
|   |       +-- to_alphabetic.rs   # Receiving alphabetic
|   |       +-- to_alphanumeric.rs # Receiving alphanumeric
|   |       +-- to_alpha_edited.rs # Receiving alpha-edited
|   |       +-- to_numeric.rs      # Receiving numeric
|   |       +-- to_numeric_edited.rs # Receiving numeric-edited
|   |       +-- to_group.rs        # Receiving group
|   |       +-- corresponding.rs   # MOVE CORRESPONDING
|   |       +-- de_edit.rs         # De-editing (IBM extension)
|   |       +-- initialize.rs      # INITIALIZE verb
|   |       +-- diagnostics.rs     # MoveDiagnostic, MoveWarning
|   |
|   +-- cobol-arithmetic/          # Arithmetic engine
|   |   +-- Cargo.toml
|   |   +-- src/
|   |       +-- lib.rs
|   |       +-- add.rs             # ADD verb
|   |       +-- subtract.rs        # SUBTRACT verb
|   |       +-- multiply.rs        # MULTIPLY verb
|   |       +-- divide.rs          # DIVIDE verb + REMAINDER
|   |       +-- compute.rs         # COMPUTE expression evaluator
|   |       +-- rounded.rs         # ROUNDED phrase, rounding modes
|   |       +-- size_error.rs      # ON SIZE ERROR handling
|   |       +-- intrinsics.rs      # FUNCTION intrinsics
|   |       +-- intermediate.rs    # Intermediate result precision
|   |
|   +-- cobol-io/                  # File I/O and record processing
|   |   +-- Cargo.toml
|   |   +-- src/
|   |       +-- lib.rs
|   |       +-- traits.rs          # CobolFile, SeqFileOps, IndexedFileOps, etc.
|   |       +-- sequential.rs      # Sequential file implementation
|   |       +-- line_sequential.rs # Line sequential
|   |       +-- indexed.rs         # Indexed file (SQLite backend)
|   |       +-- relative.rs        # Relative file
|   |       +-- print_file.rs      # LINAGE / ADVANCING
|   |       +-- file_status.rs     # FileStatus codes
|   |       +-- record_buffer.rs   # Record area management
|   |       +-- variable_length.rs # RDW/BDW handling
|   |       +-- resolver.rs        # File name resolution
|   |       +-- encoding.rs        # EBCDIC/ASCII conversion at I/O boundary
|   |       +-- declaratives.rs    # USE AFTER ERROR handlers
|   |
|   +-- cobol-sort/                # SORT and MERGE
|   |   +-- Cargo.toml
|   |   +-- src/
|   |       +-- lib.rs
|   |       +-- sort.rs            # CobolSort entry point
|   |       +-- merge.rs           # CobolMerge entry point
|   |       +-- keys.rs            # SortKeySpec, comparator builder
|   |       +-- in_memory.rs       # In-memory sort engine
|   |       +-- external.rs        # External merge sort
|   |       +-- adaptive.rs        # Adaptive engine (auto-switch)
|   |       +-- collating.rs       # Collating sequence support
|   |       +-- procedures.rs      # Releaser / Returner
|   |
|   +-- cobol-runtime/             # Program lifecycle and orchestration
|       +-- Cargo.toml
|       +-- src/
|           +-- lib.rs
|           +-- prelude.rs         # Re-exports for transpiler convenience
|           +-- program.rs         # CobolProgram struct, lifecycle
|           +-- perform_stack.rs   # PERFORM nesting tracker
|           +-- special_regs.rs    # Special registers
|           +-- call.rs            # CALL / GOBACK / STOP RUN
|           +-- display.rs         # DISPLAY / ACCEPT
|           +-- inspect.rs         # INSPECT verb
|           +-- string_verbs.rs    # STRING / UNSTRING
|           +-- reference_mod.rs   # Reference modification
|
+-- tests/                         # Integration tests
|   +-- move_matrix_test.rs        # All NxN MOVE combinations
|   +-- arithmetic_test.rs         # Arithmetic verb tests
|   +-- file_io_test.rs            # File I/O round-trip tests
|   +-- sort_test.rs               # SORT/MERGE tests
|   +-- end_to_end/                # Full COBOL program translations
|       +-- hello_world.rs
|       +-- batch_sequential.rs
|       +-- indexed_crud.rs
|       +-- report_writer.rs
|
+-- benches/                       # Performance benchmarks
|   +-- packed_decimal_bench.rs
|   +-- move_engine_bench.rs
|   +-- sort_bench.rs
|
+-- docs/                          # Specification documents
|   +-- (all 8 .md specification files)
|
+-- examples/                      # Example transpiled programs
    +-- simple_compute.rs
    +-- sequential_file_read.rs
    +-- indexed_file_crud.rs
```

---

## 3. Crate Dependency Graph

```
                    cobol-core
                   /    |    \
                  /     |     \
                 v      v      v
          cobol-types   |   (used by all)
           /   |   \    |
          /    |    \   |
         v     v     v  v
  cobol-move  cobol-arithmetic
         \       /
          \     /
           v   v
          cobol-io
             |
             v
         cobol-sort
             |
             v
        cobol-runtime  (re-exports all)
```

### 3.1 Explicit Dependencies

| Crate | Depends On | External Deps |
|---|---|---|
| `cobol-core` | (none) | `rust_decimal`, `thiserror` |
| `cobol-types` | `cobol-core` | `rust_decimal`, `byteorder`, `derive_more` |
| `cobol-move` | `cobol-core`, `cobol-types` | `rust_decimal` |
| `cobol-arithmetic` | `cobol-core`, `cobol-types` | `rust_decimal` |
| `cobol-io` | `cobol-core`, `cobol-types`, `cobol-move` | `rusqlite` (optional) |
| `cobol-sort` | `cobol-core`, `cobol-types`, `cobol-io` | `tempfile` |
| `cobol-runtime` | ALL above | `serde`, `toml` |

### 3.2 No Circular Dependencies

The dependency graph is a strict DAG. Each crate depends only on crates
above it in the layering. This is enforced by Cargo's build system.

---

## 4. cobol-core: Foundation Crate

This crate defines the canonical trait hierarchy, data category system,
error types, and configuration enums used by all other crates.

### 4.1 Canonical Trait Hierarchy

The previous specs used slightly different trait names (`CobolType` in the
datatype spec, `CobolField` in the MOVE spec). The canonical hierarchy
reconciles them:

```rust
// =============================================================
// cobol-core/src/traits.rs -- CANONICAL TRAIT HIERARCHY
// =============================================================

use rust_decimal::Decimal;
use std::cmp::Ordering;

/// Root trait: every COBOL data item implements this.
/// This is the universal interface for the MOVE engine, I/O, and display.
pub trait CobolField: std::fmt::Debug {
    /// The item's data category (determines MOVE/comparison rules)
    fn category(&self) -> DataCategory;

    /// Storage size in bytes (for record I/O, GROUP layout)
    fn byte_length(&self) -> usize;

    /// Raw storage bytes (for GROUP moves, I/O)
    fn as_bytes(&self) -> &[u8];

    /// Mutable raw storage bytes
    fn as_bytes_mut(&mut self) -> &mut [u8];

    /// Display representation as bytes (for alphanumeric moves)
    /// For numeric items: the DISPLAY-format string (e.g., "00123")
    /// For alphanumeric items: same as as_bytes()
    fn display_bytes(&self) -> Vec<u8>;

    /// Fill all storage bytes with a single value
    fn fill_bytes(&mut self, byte: u8);

    /// JUSTIFIED RIGHT flag (only for alphabetic/alphanumeric)
    fn is_justified_right(&self) -> bool { false }

    /// BLANK WHEN ZERO flag (only for numeric/numeric-edited)
    fn has_blank_when_zero(&self) -> bool { false }

    /// COBOL INITIALIZE default: called by INITIALIZE verb
    fn initialize_default(&mut self);
}

/// Numeric items: any field with a numeric value
/// (PIC 9 DISPLAY, COMP, COMP-3, COMP-5, COMP-1, COMP-2)
pub trait CobolNumeric: CobolField {
    /// Get the value as a canonical Decimal
    fn to_decimal(&self) -> Decimal;

    /// Set the value from a canonical Decimal (with truncation)
    fn set_decimal(&mut self, value: Decimal);

    /// Number of decimal places (V position)
    fn scale(&self) -> u32;

    /// Total digit count (integer + decimal)
    fn precision(&self) -> u32;

    /// Is this a signed type? (PIC S...)
    fn is_signed(&self) -> bool;

    /// Numeric comparison (decimal-point-aligned)
    fn compare_numeric(&self, other: &dyn CobolNumeric) -> Ordering {
        self.to_decimal().cmp(&other.to_decimal())
    }
}

/// Numeric-Edited items: numeric with editing mask
pub trait CobolNumericEdited: CobolField {
    /// The editing mask (parsed PIC string)
    fn edit_mask(&self) -> &[EditSymbol];

    /// Number of integer digit positions
    fn integer_positions(&self) -> u32;

    /// Number of decimal digit positions
    fn decimal_positions(&self) -> u32;

    /// Set the display from a numeric value (apply editing)
    fn set_from_numeric(&mut self, value: Decimal, is_negative: bool);

    /// Extract the numeric value (de-edit) -- IBM extension
    fn de_edit(&self) -> Option<Decimal>;
}

/// Group items: structured records with subordinate fields
pub trait CobolGroup: CobolField {
    /// Access elementary fields by traversal order
    fn elementary_fields(&self) -> Vec<&dyn CobolField>;

    /// Access elementary fields (mutable)
    fn elementary_fields_mut(&mut self) -> Vec<&mut dyn CobolField>;

    /// Lookup elementary fields by COBOL data-name
    fn field_by_name(&self, name: &str) -> Option<&dyn CobolField>;

    /// Lookup elementary fields by name (mutable)
    fn field_by_name_mut(&mut self, name: &str) -> Option<&mut dyn CobolField>;
}
```

### 4.2 DataCategory Enum

```rust
// cobol-core/src/category.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataCategory {
    Alphabetic,
    Alphanumeric,
    AlphanumericEdited,
    Numeric,
    NumericEdited,
    National,        // Phase 3
    Group,
}
```

### 4.3 Editing Symbol Enum

```rust
// cobol-core/src/editing.rs

/// Parsed PIC editing symbols (for numeric-edited and alpha-edited types)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditSymbol {
    Digit,                  // 9 -- always display digit
    ZeroSuppress(u8),       // Z (space fill) or * (asterisk fill)
    FloatCurrency(u8),      // $ (or other currency symbol)
    FloatPlus,              // + (floating plus/minus)
    FloatMinus,             // - (floating minus/space)
    FixedPlus,              // + at fixed position
    FixedMinus,             // - at fixed position
    FixedCurrency(u8),      // $ at fixed position
    Period,                 // . decimal point
    Comma,                  // , thousands separator
    InsertSpace,            // B insertion
    InsertZero,             // 0 insertion
    InsertSlash,            // / insertion
    CreditRight,            // CR (2-char suffix)
    DebitRight,             // DB (2-char suffix)
    SignPosition,           // S (sign position indicator)
    AlphaChar,              // X or A in alpha-edited
}
```

### 4.4 Dialect and Configuration Enums

```rust
// cobol-core/src/config.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CobolDialect {
    IbmEnterprise,
    GnuCobol,
    MicroFocus,
    AnsiStandard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumProc {
    Nopfd,    // Correct signs on MOVE
    Pfd,      // Assume valid signs (performance)
    Mig,      // Migration mode
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithMode {
    Compat,   // 18-digit intermediates (IBM ARITH(COMPAT))
    Extend,   // 31-digit intermediates (IBM ARITH(EXTEND))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Silent,   // Match COBOL behavior (no warnings)
    Warn,     // Log warnings for data loss (dev/test)
    Strict,   // Panic on data loss (strict testing)
}

/// Master runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub dialect: CobolDialect,
    pub numproc: NumProc,
    pub arith_mode: ArithMode,
    pub diagnostic_level: DiagnosticLevel,
    pub allow_de_editing: bool,
    pub default_collating: CollatingSequence,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            dialect: CobolDialect::IbmEnterprise,
            numproc: NumProc::Nopfd,
            arith_mode: ArithMode::Compat,
            diagnostic_level: DiagnosticLevel::Silent,
            allow_de_editing: true,
            default_collating: CollatingSequence::Native,
        }
    }
}
```

---

## 5. cobol-types: Concrete Types

This crate implements all COBOL data types as Rust structs that implement
the traits from `cobol-core`.

### 5.1 Type Catalog

| Rust Type | COBOL Source | Size | Phase |
|---|---|---|---|
| `PackedDecimal` | COMP-3 / PACKED-DECIMAL | `(precision/2)+1` bytes | 1 |
| `CompBinary<T>` | COMP / COMP-4 / COMP-5 | 2, 4, or 8 bytes | 1 |
| `PicX<const N: usize>` | PIC X(N) | N bytes | 1 |
| `PicA<const N: usize>` | PIC A(N) | N bytes | 1 |
| `ZonedDecimal` | PIC 9 DISPLAY | precision bytes | 2 |
| `NumericEdited` | PIC with Z, *, $, etc. | display length bytes | 2 |
| `AlphaEdited` | PIC X with B, 0, / | display length bytes | 3 |
| `CobolFloat32` | COMP-1 | 4 bytes | 3 |
| `CobolFloat64` | COMP-2 | 8 bytes | 3 |
| `NationalString` | PIC N | 2*N bytes | 3 |

### 5.2 PackedDecimal (COMP-3)

```rust
// cobol-types/src/packed_decimal.rs

/// COMP-3 / PACKED-DECIMAL storage
/// Stores decimal value as BCD nibbles with trailing sign nibble.
/// Byte layout: [d1d2][d3d4]...[dN sign]
/// Sign nibble: 0xC = positive, 0xD = negative, 0xF = unsigned
pub struct PackedDecimal {
    /// Raw BCD bytes
    data: Vec<u8>,
    /// Total digits (precision)
    precision: u32,
    /// Decimal places (scale)
    scale: u32,
    /// Whether the type is signed (PIC S...)
    signed: bool,
}

impl PackedDecimal {
    pub fn new(precision: u32, scale: u32, signed: bool) -> Self {
        let byte_len = (precision / 2) + 1;
        let mut data = vec![0u8; byte_len as usize];
        // Initialize sign nibble
        let last = data.len() - 1;
        data[last] = if signed { 0x0C } else { 0x0F };
        Self { data, precision, scale, signed }
    }

    /// Pack a Decimal value into BCD bytes
    pub fn pack(&mut self, value: Decimal) { /* ... */ }

    /// Unpack BCD bytes into a Decimal value
    pub fn unpack(&self) -> Decimal { /* ... */ }
}

impl CobolField for PackedDecimal {
    fn category(&self) -> DataCategory { DataCategory::Numeric }
    fn byte_length(&self) -> usize { self.data.len() }
    fn as_bytes(&self) -> &[u8] { &self.data }
    fn as_bytes_mut(&mut self) -> &mut [u8] { &mut self.data }
    fn display_bytes(&self) -> Vec<u8> {
        // Convert to display representation: "00123" etc.
        format_zoned_display(self.unpack(), self.precision, self.scale, self.signed)
    }
    fn fill_bytes(&mut self, byte: u8) {
        for b in self.data.iter_mut() { *b = byte; }
    }
    fn initialize_default(&mut self) {
        self.set_decimal(Decimal::ZERO);
    }
}

impl CobolNumeric for PackedDecimal {
    fn to_decimal(&self) -> Decimal { self.unpack() }
    fn set_decimal(&mut self, value: Decimal) { self.pack(value); }
    fn scale(&self) -> u32 { self.scale }
    fn precision(&self) -> u32 { self.precision }
    fn is_signed(&self) -> bool { self.signed }
}
```

### 5.3 CompBinary (COMP / COMP-4 / COMP-5)

```rust
// cobol-types/src/comp_binary.rs

/// Binary integer storage (COMP, COMP-4, COMP-5)
/// Generic over the Rust integer type (i16, i32, i64, u16, u32, u64)
pub struct CompBinary<T: BinaryInt> {
    value: T,
    precision: u32,
    scale: u32,
    /// COMP (PIC-limited) vs COMP-5 (full binary range)
    pic_limited: bool,
}

pub trait BinaryInt: Copy + Ord + std::fmt::Debug {
    fn to_decimal(self, scale: u32) -> Decimal;
    fn from_decimal(value: Decimal, scale: u32) -> Self;
    fn byte_size() -> usize;
    fn to_be_bytes(self) -> Vec<u8>;
    fn from_be_bytes(bytes: &[u8]) -> Self;
}

// Implement BinaryInt for i16, i32, i64, u16, u32, u64
```

### 5.4 PicX (Alphanumeric)

```rust
// cobol-types/src/pic_x.rs

/// PIC X(N): Fixed-length alphanumeric field
/// Wraps a byte array with COBOL string semantics:
/// - Left-justified
/// - Space-padded on right
/// - Right-truncated on assignment
pub struct PicX<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> PicX<N> {
    pub fn new(initial: &[u8]) -> Self {
        let mut data = [b' '; N]; // space-filled
        let copy_len = initial.len().min(N);
        data[..copy_len].copy_from_slice(&initial[..copy_len]);
        Self { data }
    }

    pub fn as_str(&self) -> &str {
        // COBOL strings are not necessarily valid UTF-8 but often are
        std::str::from_utf8(&self.data).unwrap_or("<non-utf8>")
    }

    pub fn trimmed(&self) -> &[u8] {
        let end = self.data.iter()
            .rposition(|&b| b != b' ')
            .map(|p| p + 1)
            .unwrap_or(0);
        &self.data[..end]
    }
}

impl<const N: usize> CobolField for PicX<N> {
    fn category(&self) -> DataCategory { DataCategory::Alphanumeric }
    fn byte_length(&self) -> usize { N }
    fn as_bytes(&self) -> &[u8] { &self.data }
    fn as_bytes_mut(&mut self) -> &mut [u8] { &mut self.data }
    fn display_bytes(&self) -> Vec<u8> { self.data.to_vec() }
    fn fill_bytes(&mut self, byte: u8) { self.data.fill(byte); }
    fn initialize_default(&mut self) { self.data.fill(b' '); }
}
```

### 5.5 CobolArray (OCCURS)

```rust
// cobol-types/src/cobol_array.rs

/// COBOL array with 1-based indexing
/// Wraps a Vec<T> and translates COBOL 1-based subscripts to 0-based
pub struct CobolArray<T: CobolField> {
    elements: Vec<T>,
}

impl<T: CobolField> CobolArray<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Access by 1-based COBOL subscript
    pub fn get(&self, subscript: usize) -> &T {
        &self.elements[subscript - 1]
    }

    /// Access by 1-based COBOL subscript (mutable)
    pub fn get_mut(&mut self, subscript: usize) -> &mut T {
        &mut self.elements[subscript - 1]
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
```

### 5.6 Figurative Constants

```rust
// cobol-types/src/figurative.rs

/// Figurative constants that adapt to destination size
#[derive(Debug, Clone)]
pub enum FigurativeConstant {
    Spaces,
    Zeros,
    HighValues,
    LowValues,
    Quotes,
    AllLiteral(Vec<u8>),  // repeating pattern
    Nulls,
}
```

---

## 6. cobol-move: MOVE Engine

Implements the complete MOVE engine as specified in `cobol_move_engine_spec.md`.

### 6.1 Public API

```rust
// cobol-move/src/lib.rs

pub use engine::cobol_move;
pub use engine::cobol_move_figurative;
pub use corresponding::move_corresponding;
pub use initialize::cobol_initialize;

// Re-export diagnostics
pub use diagnostics::{MoveDiagnostic, MoveWarning, MoveWarningKind};
```

### 6.2 Central Dispatch

```rust
// cobol-move/src/engine.rs

/// Central MOVE dispatch: MOVE src TO dest
pub fn cobol_move(
    src: &dyn CobolField,
    dest: &mut dyn CobolField,
    config: &RuntimeConfig,
) {
    let src_cat = src.category();
    let dest_cat = dest.category();

    // Legality check (should be caught at transpile time, but guard here)
    debug_assert!(
        is_legal_move(src_cat, dest_cat, config),
        "Illegal MOVE from {:?} to {:?}",
        src_cat, dest_cat
    );

    // Dispatch by destination category
    match dest_cat {
        DataCategory::Alphabetic => to_alphabetic::execute(src, dest),
        DataCategory::Alphanumeric => to_alphanumeric::execute(src, dest),
        DataCategory::AlphanumericEdited => to_alpha_edited::execute(src, dest),
        DataCategory::Numeric => to_numeric::execute(src, dest, config),
        DataCategory::NumericEdited => to_numeric_edited::execute(src, dest, config),
        DataCategory::Group => to_group::execute(src, dest),
        DataCategory::National => unimplemented!("National MOVE: Phase 3"),
    }

    // Post-processing: BLANK WHEN ZERO
    if dest.has_blank_when_zero() {
        apply_blank_when_zero(dest);
    }
}
```

---

## 7. cobol-arithmetic: Arithmetic Engine

Implements the arithmetic operations from `cobol_arithmetic_operations.md`.

### 7.1 Public API

```rust
// cobol-arithmetic/src/lib.rs

/// ADD verb: ADD src TO dest [ROUNDED] [ON SIZE ERROR ...]
pub fn cobol_add(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult;

/// SUBTRACT verb
pub fn cobol_subtract(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult;

/// MULTIPLY verb
pub fn cobol_multiply(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult;

/// DIVIDE verb (with optional REMAINDER)
pub fn cobol_divide(
    src: &dyn CobolNumeric,
    dest: &mut dyn CobolNumeric,
    remainder: Option<&mut dyn CobolNumeric>,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult;

/// COMPUTE verb: evaluate expression and store
pub fn cobol_compute(
    expression_value: Decimal,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult;

/// Result of an arithmetic operation
pub struct ArithResult {
    pub size_error: bool,  // ON SIZE ERROR condition
}
```

### 7.2 Store-to-Destination (Shared Logic)

All arithmetic verbs share the same result storage logic -- this is
essentially a numeric MOVE with optional rounding:

```rust
fn store_arithmetic_result(
    value: Decimal,
    dest: &mut dyn CobolNumeric,
    rounded: Option<RoundingMode>,
    config: &RuntimeConfig,
) -> ArithResult {
    let dest_scale = dest.scale();
    let dest_precision = dest.precision();
    let dest_int_digits = dest_precision - dest_scale;

    // Step 1: Round or truncate to destination scale
    let scaled = match rounded {
        Some(mode) => round_decimal(value, dest_scale, mode),
        None => truncate_decimal(value, dest_scale),
    };

    // Step 2: Check for overflow (size error)
    let max_value = max_for_precision(dest_int_digits, dest_scale);
    let size_error = scaled.abs() > max_value;

    if size_error {
        // ON SIZE ERROR: destination is unchanged
        return ArithResult { size_error: true };
    }

    // Step 3: Handle sign
    let final_value = if !dest.is_signed() {
        scaled.abs()
    } else {
        scaled
    };

    // Step 4: Left-truncate if needed (no SIZE ERROR = silent truncation)
    let truncated = left_truncate_to_precision(final_value, dest_int_digits, dest_scale);

    dest.set_decimal(truncated);
    ArithResult { size_error: false }
}
```

---

## 8. cobol-io: File I/O

Implements file I/O from `cobol_file_io_record_processing.md`.

### 8.1 Public API

```rust
// cobol-io/src/lib.rs

// File implementations
pub use sequential::SequentialFile;
pub use line_sequential::LineSequentialFile;
pub use print_file::PrintFile;

#[cfg(feature = "indexed-file")]
pub use indexed::IndexedFile;

pub use relative::RelativeFile;

// Traits
pub use traits::{CobolFile, SeqFileOps, IndexedFileOps, RelativeFileOps, PrintFileOps};

// Support types
pub use file_status::FileStatus;
pub use resolver::FileResolver;
pub use encoding::EbcdicConverter;
```

### 8.2 Feature Gating

The `indexed-file` feature gates the SQLite dependency:

```toml
# cobol-io/Cargo.toml
[features]
default = ["indexed-file"]
indexed-file = ["rusqlite"]

[dependencies]
cobol-core = { path = "../cobol-core" }
cobol-types = { path = "../cobol-types" }
cobol-move = { path = "../cobol-move" }
rusqlite = { version = "0.31", optional = true }
```

---

## 9. cobol-sort: SORT/MERGE

Implements SORT/MERGE from `cobol_sort_merge_spec.md`.

### 9.1 Public API

```rust
// cobol-sort/src/lib.rs

pub use sort::CobolSort;
pub use merge::CobolMerge;
pub use keys::{SortKeySpec, SortKey, SortKeyType, CollatingSequence};
pub use procedures::{Releaser, Returner};
```

---

## 10. cobol-runtime: Program Lifecycle

The top-level crate that provides the program execution model and re-exports
everything a transpiled program needs.

### 10.1 CobolProgram

```rust
// cobol-runtime/src/program.rs

/// Represents a running COBOL program's execution context
pub struct CobolProgram {
    /// Runtime configuration
    pub config: RuntimeConfig,

    /// File resolver for ASSIGN TO names
    pub file_resolver: FileResolver,

    /// Special registers
    pub return_code: i32,
    pub sort_return: i32,
    pub sort_file_size: i64,
    pub sort_core_size: i64,

    /// PERFORM stack (tracks nested PERFORMs)
    perform_stack: PerformStack,

    /// Active DECLARATIVES error handlers
    error_handlers: HashMap<String, Box<dyn Fn(FileStatus)>>,
}

impl CobolProgram {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            file_resolver: FileResolver::from_env_or_default(),
            return_code: 0,
            sort_return: 0,
            sort_file_size: 0,
            sort_core_size: 0,
            perform_stack: PerformStack::new(),
            error_handlers: HashMap::new(),
        }
    }

    /// STOP RUN: terminate program with return code
    pub fn stop_run(&self) -> ! {
        std::process::exit(self.return_code);
    }

    /// GOBACK: return from called program
    pub fn goback(&self) -> i32 {
        self.return_code
    }

    /// DISPLAY: output to SYSOUT
    pub fn display(&self, values: &[&dyn CobolField]) {
        let mut output = String::new();
        for val in values {
            let bytes = val.display_bytes();
            output.push_str(&String::from_utf8_lossy(&bytes));
        }
        println!("{}", output);
    }

    /// ACCEPT: input from SYSIN
    pub fn accept(&self, dest: &mut dyn CobolField) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap_or(0);
        let trimmed = input.trim_end_matches('\n').as_bytes();
        cobol_move_bytes_to_field(trimmed, dest, &self.config);
    }

    /// Register a DECLARATIVES error handler
    pub fn register_error_handler(
        &mut self,
        file_name: &str,
        handler: Box<dyn Fn(FileStatus)>,
    ) {
        self.error_handlers.insert(file_name.to_string(), handler);
    }
}
```

### 10.2 PERFORM Stack

```rust
// cobol-runtime/src/perform_stack.rs

/// Tracks nested PERFORM invocations for paragraph/section execution
pub struct PerformStack {
    frames: Vec<PerformFrame>,
}

pub struct PerformFrame {
    /// The paragraph/section being performed
    pub target: String,
    /// Return address (for PERFORM THRU)
    pub return_after: String,
    /// Loop state for PERFORM VARYING
    pub loop_state: Option<PerformLoopState>,
}

pub struct PerformLoopState {
    /// Current iteration count
    pub iteration: usize,
    /// Loop variable reference (for post-loop value observation)
    pub varying_final_value: Option<Decimal>,
}

impl PerformStack {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn push(&mut self, frame: PerformFrame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Option<PerformFrame> {
        self.frames.pop()
    }

    pub fn is_within(&self, paragraph: &str) -> bool {
        self.frames.iter().any(|f| f.target == paragraph)
    }
}
```

### 10.3 Prelude Module

```rust
// cobol-runtime/src/prelude.rs

// Re-export everything a transpiled program needs

// Core
pub use cobol_core::traits::*;
pub use cobol_core::category::DataCategory;
pub use cobol_core::config::*;

// Types
pub use cobol_types::packed_decimal::PackedDecimal;
pub use cobol_types::comp_binary::CompBinary;
pub use cobol_types::pic_x::PicX;
pub use cobol_types::pic_a::PicA;
pub use cobol_types::zoned_decimal::ZonedDecimal;
pub use cobol_types::numeric_edited::NumericEdited;
pub use cobol_types::cobol_array::CobolArray;
pub use cobol_types::level88::Level88;
pub use cobol_types::figurative::FigurativeConstant;

// MOVE
pub use cobol_move::{cobol_move, cobol_move_figurative, move_corresponding};
pub use cobol_move::cobol_initialize;

// Arithmetic
pub use cobol_arithmetic::{
    cobol_add, cobol_subtract, cobol_multiply, cobol_divide, cobol_compute,
};
pub use cobol_arithmetic::{ArithResult, RoundingMode};

// I/O
pub use cobol_io::*;

// SORT/MERGE
pub use cobol_sort::{CobolSort, CobolMerge, SortKeySpec, SortKey};

// Runtime
pub use crate::program::CobolProgram;

// Decimal re-export
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
```

---

## 11. Configuration System

### 11.1 Configuration Sources

Configuration is resolved in priority order:

1. **Programmatic** -- `RuntimeConfig` passed to `CobolProgram::new()`
2. **Environment variables** -- `COBOL_DIALECT`, `COBOL_FILE_CONFIG`, etc.
3. **Config file** -- `cobol-runtime.toml` in the working directory
4. **Defaults** -- IBM Enterprise COBOL dialect

### 11.2 File Resolver Configuration

```toml
# cobol-runtime.toml

[runtime]
dialect = "ibm-enterprise"
diagnostic_level = "silent"
arith_mode = "compat"
numproc = "nopfd"

[files]
resolver_mode = "config"  # "config" | "direct" | "env"

[files.assignments]
# COBOL ASSIGN TO name = physical path/config

[files.assignments.CUSTFILE]
path = "/data/customer.dat"
organization = "indexed"
record_size = 200
index_db = "/data/customer.idx.sqlite"

[files.assignments.TRANSIN]
path = "/data/transactions.dat"
organization = "sequential"
record_size = 80
recording_mode = "F"

[files.assignments.REPORT]
path = "/output/report.txt"
organization = "line-sequential"

[sort]
memory_limit = 268435456  # 256 MB
temp_dir = "/tmp/cobol-sort"
max_work_files = 16
```

### 11.3 Environment Variables

| Variable | Purpose | Example |
|---|---|---|
| `COBOL_DIALECT` | Override dialect | `ibm-enterprise` |
| `COBOL_FILE_CONFIG` | Path to file config | `/etc/cobol/files.toml` |
| `COBOL_DIAGNOSTIC` | Diagnostic level | `warn` |
| `COBOL_SORT_MEMORY` | Sort memory limit (bytes) | `536870912` |
| `DD_<name>` | Direct file assignment | `DD_CUSTFILE=/data/cust.dat` |

The `DD_<name>` convention mirrors IBM's JCL DD statement naming, allowing
simple file assignment via environment variables.

---

## 12. Error Handling Strategy

### 12.1 Design Principle

COBOL does not have exceptions. Errors are communicated through:
- FILE STATUS fields (I/O operations)
- ON SIZE ERROR phrases (arithmetic)
- SORT-RETURN register (sort operations)
- DECLARATIVES / USE procedures (I/O error handlers)

The Rust runtime uses `Result<T, E>` internally but translates results
into COBOL-compatible error reporting at the API boundary.

### 12.2 Error Type Hierarchy

```rust
// cobol-core/src/error.rs

use thiserror::Error;

/// Root error type for the runtime
#[derive(Error, Debug)]
pub enum CobolError {
    #[error("File I/O error: {0}")]
    FileError(#[from] FileError),

    #[error("Sort error: {0}")]
    SortError(#[from] SortError),

    #[error("Arithmetic error: {0}")]
    ArithError(#[from] ArithError),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found: {0}")]
    NotFound(String),

    #[error("File already open: {0}")]
    AlreadyOpen(String),

    #[error("File not open: {0}")]
    NotOpen(String),

    #[error("Duplicate key")]
    DuplicateKey,

    #[error("Record not found")]
    RecordNotFound,

    #[error("End of file")]
    EndOfFile,

    #[error("Sequence error")]
    SequenceError,

    #[cfg(feature = "indexed-file")]
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

#[derive(Error, Debug)]
pub enum SortError {
    #[error("I/O error during sort: {0}")]
    Io(#[from] std::io::Error),

    #[error("Insufficient work space for sort")]
    InsufficientWorkSpace,

    #[error("Nested SORT not allowed")]
    NestedSort,
}

#[derive(Error, Debug)]
pub enum ArithError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Exponentiation overflow")]
    ExponentiationOverflow,
}
```

### 12.3 Error-to-Status Mapping

```rust
impl FileStatus {
    /// Map a FileError to a 2-byte FILE STATUS code
    pub fn from_error(err: &FileError) -> Self {
        match err {
            FileError::Io(_) => FileStatus::PERMANENT_ERROR,
            FileError::NotFound(_) => FileStatus::FILE_NOT_FOUND,
            FileError::AlreadyOpen(_) => FileStatus::ALREADY_OPEN,
            FileError::NotOpen(_) => FileStatus::NOT_OPEN,
            FileError::DuplicateKey => FileStatus::DUPLICATE_PRIMARY,
            FileError::RecordNotFound => FileStatus::NOT_FOUND,
            FileError::EndOfFile => FileStatus::EOF,
            FileError::SequenceError => FileStatus::SEQUENCE_ERROR,
            #[cfg(feature = "indexed-file")]
            FileError::Sqlite(_) => FileStatus::PERMANENT_ERROR,
        }
    }
}
```

---

## 13. Transpiler Code Generation Patterns

The transpiler converts COBOL source to Rust source that depends on
`cobol-runtime`. These patterns show what the generated code looks like.

### 13.1 WORKING-STORAGE Section

```cobol
WORKING-STORAGE SECTION.
01  WS-COUNTER     PIC S9(5) COMP-3 VALUE 0.
01  WS-NAME        PIC X(30) VALUE "JOHN DOE".
01  WS-AMOUNT      PIC S9(7)V99 COMP-3.
01  WS-TABLE.
    05  WS-ITEM    PIC X(10) OCCURS 5 TIMES.
01  WS-STATUS      PIC XX.
```

**Generated Rust**:

```rust
use cobol_runtime::prelude::*;

struct WorkingStorage {
    ws_counter: PackedDecimal,       // PIC S9(5) COMP-3
    ws_name: PicX<30>,              // PIC X(30)
    ws_amount: PackedDecimal,       // PIC S9(7)V99 COMP-3
    ws_item: CobolArray<PicX<10>>,  // PIC X(10) OCCURS 5
    ws_status: PicX<2>,            // PIC XX
}

impl WorkingStorage {
    fn new() -> Self {
        Self {
            ws_counter: PackedDecimal::from_value(5, 0, true, dec!(0)),
            ws_name: PicX::new(b"JOHN DOE"),
            ws_amount: PackedDecimal::new(9, 2, true),
            ws_item: CobolArray::new(vec![PicX::new(b""); 5]),
            ws_status: PicX::new(b""),
        }
    }
}
```

### 13.2 Arithmetic Statements

```cobol
ADD WS-A TO WS-B ROUNDED
    ON SIZE ERROR PERFORM ERROR-HANDLER
END-ADD.

COMPUTE WS-RESULT = (WS-A + WS-B) * WS-C / WS-D.
```

**Generated Rust**:

```rust
// ADD WS-A TO WS-B ROUNDED ON SIZE ERROR ...
{
    let result = cobol_add(
        &ws.ws_a, &mut ws.ws_b,
        Some(RoundingMode::AwayFromZero),
        &ctx.config,
    );
    if result.size_error {
        error_handler(&mut ws, &mut ctx);
    }
}

// COMPUTE WS-RESULT = (WS-A + WS-B) * WS-C / WS-D
{
    let expr_value = (ws.ws_a.to_decimal() + ws.ws_b.to_decimal())
        * ws.ws_c.to_decimal()
        / ws.ws_d.to_decimal();
    let result = cobol_compute(
        expr_value, &mut ws.ws_result,
        None,  // no ROUNDED
        &ctx.config,
    );
}
```

### 13.3 MOVE Statements

```cobol
MOVE WS-ALPHA TO WS-NUMERIC.
MOVE CORRESPONDING SOURCE-REC TO DEST-REC.
INITIALIZE CUSTOMER-REC.
```

**Generated Rust**:

```rust
cobol_move(&ws.ws_alpha, &mut ws.ws_numeric, &ctx.config);

move_corresponding(&ws.source_rec, &mut ws.dest_rec, &ctx.config);

cobol_initialize(&mut ws.customer_rec, &[], false, false, &ctx.config);
```

### 13.4 File I/O Statements

```cobol
OPEN INPUT CUSTOMER-FILE.
READ CUSTOMER-FILE INTO WS-CUSTOMER
    AT END SET WS-EOF TO TRUE
END-READ.
CLOSE CUSTOMER-FILE.
```

**Generated Rust**:

```rust
// OPEN INPUT
{
    let status = customer_file.open(OpenMode::Input);
    status.store_to(&mut ws.ws_cust_status);
}

// READ ... INTO ... AT END
{
    let status = customer_file.read_next();
    status.store_to(&mut ws.ws_cust_status);
    if status.is_end_of_file() {
        ws.ws_eof = true;
    } else if status.is_success() {
        cobol_move(
            &ByteSliceField::new(customer_file.record_area()),
            &mut ws.ws_customer,
            &ctx.config,
        );
    }
}

// CLOSE
{
    let status = customer_file.close();
    status.store_to(&mut ws.ws_cust_status);
}
```

### 13.5 PERFORM Statements

```cobol
PERFORM 1000-PROCESS.
PERFORM 2000-LOOP VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 10.
```

**Generated Rust**:

```rust
// PERFORM 1000-PROCESS
para_1000_process(&mut ws, &mut ctx);

// PERFORM 2000-LOOP VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 10
{
    ws.ws_i.set_decimal(dec!(1));
    while !(ws.ws_i.to_decimal() > dec!(10)) {
        para_2000_loop(&mut ws, &mut ctx);
        let next = ws.ws_i.to_decimal() + dec!(1);
        ws.ws_i.set_decimal(next);
    }
    // Note: ws_i is now 11 (observable post-loop value)
}
```

### 13.6 EVALUATE (Switch)

```cobol
EVALUATE WS-CODE
    WHEN "A" PERFORM ACTION-A
    WHEN "B" THRU "D" PERFORM ACTION-BD
    WHEN OTHER PERFORM ACTION-DEFAULT
END-EVALUATE.
```

**Generated Rust**:

```rust
{
    let eval_val = ws.ws_code.as_bytes();
    if eval_val == b"A" {
        action_a(&mut ws, &mut ctx);
    } else if eval_val >= b"B" && eval_val <= b"D" {
        action_bd(&mut ws, &mut ctx);
    } else {
        action_default(&mut ws, &mut ctx);
    }
}
```

### 13.7 Level 88 Conditions

```cobol
01  WS-STATUS PIC X.
    88  STATUS-ACTIVE   VALUE "A".
    88  STATUS-INACTIVE VALUE "I" "D".
    88  STATUS-VALID    VALUE "A" "I" "D" "S".

IF STATUS-ACTIVE
    PERFORM PROCESS-ACTIVE.
SET STATUS-INACTIVE TO TRUE.
```

**Generated Rust**:

```rust
// 88-level predicates (generated as methods or free functions)
fn status_active(ws_status: &PicX<1>) -> bool {
    ws_status.as_bytes() == b"A"
}
fn status_inactive(ws_status: &PicX<1>) -> bool {
    matches!(ws_status.as_bytes(), b"I" | b"D")
}
fn status_valid(ws_status: &PicX<1>) -> bool {
    matches!(ws_status.as_bytes(), b"A" | b"I" | b"D" | b"S")
}

// IF STATUS-ACTIVE
if status_active(&ws.ws_status) {
    process_active(&mut ws, &mut ctx);
}

// SET STATUS-INACTIVE TO TRUE (sets to first VALUE)
cobol_move(&PicX::<1>::new(b"I"), &mut ws.ws_status, &ctx.config);
```

---

## 14. Testing Strategy

### 14.1 Test Layers

```
Layer 4: End-to-End       Hand-translated COBOL programs -> validate output
Layer 3: Integration      Cross-crate operations (MOVE + I/O, Arithmetic + MOVE)
Layer 2: Module           Per-module tests (each MOVE destination, each arith verb)
Layer 1: Unit             Individual types (PackedDecimal pack/unpack, PicX operations)
```

### 14.2 Unit Tests (Layer 1)

Each type in `cobol-types` has exhaustive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_decimal_pack_unpack() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.set_decimal(dec!(123.45));
        assert_eq!(pd.to_decimal(), dec!(123.45));
        assert_eq!(pd.as_bytes(), &[0x01, 0x23, 0x4C]); // BCD encoding
    }

    #[test]
    fn packed_decimal_negative() {
        let mut pd = PackedDecimal::new(5, 2, true);
        pd.set_decimal(dec!(-123.45));
        assert_eq!(pd.to_decimal(), dec!(-123.45));
        assert_eq!(pd.as_bytes(), &[0x01, 0x23, 0x4D]); // D = negative
    }

    #[test]
    fn pic_x_padding_and_truncation() {
        let field = PicX::<5>::new(b"AB");
        assert_eq!(field.as_bytes(), b"AB   "); // space-padded

        let field = PicX::<3>::new(b"ABCDE");
        assert_eq!(field.as_bytes(), b"ABC"); // right-truncated
    }
}
```

### 14.3 MOVE Matrix Tests (Layer 2)

Test every legal source-destination category combination from the MOVE spec:

```rust
#[test]
fn move_numeric_to_numeric_left_truncation() {
    let src = PackedDecimal::from_value(7, 2, true, dec!(1234567.89));
    let mut dest = PackedDecimal::new(4, 1, true); // PIC S9(3)V9
    cobol_move(&src, &mut dest, &RuntimeConfig::default());
    assert_eq!(dest.to_decimal(), dec!(567.8)); // left-truncated, right-truncated
}

#[test]
fn move_alphanumeric_to_numeric() {
    let src = PicX::<8>::new(b"01234.56");
    let mut dest = PackedDecimal::new(7, 2, false);
    cobol_move(&src, &mut dest, &RuntimeConfig::default());
    assert_eq!(dest.to_decimal(), dec!(1234.56));
}
```

### 14.4 End-to-End Tests (Layer 4)

Hand-translate small COBOL programs to Rust using the runtime and verify
output matches the COBOL program's expected output:

```rust
#[test]
fn e2e_batch_sequential_processing() {
    // Create test input file
    let input = create_test_seq_file(&[
        b"A000000001000050000",  // type=A, acct=1, amount=500.00
        b"A000000002000075000",  // type=A, acct=2, amount=750.00
    ]);

    // Run the transpiled program equivalent
    let output = run_batch_program(&input);

    // Verify
    assert_eq!(output.total, dec!(1250.00));
    assert_eq!(output.record_count, 2);
}
```

### 14.5 Test Data Management

Test data files live in `tests/fixtures/`:
```
tests/
  fixtures/
    sequential/
      fixed_80.dat          # 80-byte fixed-length records
      variable_mode_v.dat   # Variable-length with RDW
    indexed/
      customer.dat          # Sample customer records
      customer.idx.sqlite   # Pre-built SQLite index
    sort/
      unsorted_100.dat      # 100 unsorted records
      sorted_expected.dat   # Expected sorted output
    ebcdic/
      cp037_sample.dat      # EBCDIC-encoded test data
```

---

## 15. Performance Considerations

### 15.1 Hot Paths

The most performance-critical operations in a typical COBOL batch program:

1. **MOVE (numeric to numeric)** -- executed millions of times per batch run
2. **PackedDecimal pack/unpack** -- every numeric operation involves this
3. **Sequential READ/WRITE** -- I/O bound but buffer management matters
4. **Sort comparator** -- called O(n log n) times per sort

### 15.2 Optimization Strategies

| Area | Strategy |
|---|---|
| PackedDecimal | Avoid rust_decimal for simple cases; use i64 fast path for <= 18 digits |
| MOVE (numeric) | Specialize for common cases (same-scale, same-precision) |
| MOVE (alphanumeric) | Use `copy_from_slice` for fixed-length same-size moves |
| Sequential I/O | Large BufReader/BufWriter buffers (match COBOL BLOCK CONTAINS) |
| Sort comparator | Pre-extract key bytes to avoid re-slicing per comparison |
| PicX comparison | Use byte-slice comparison (no UTF-8 overhead) |

### 15.3 Benchmarking Targets

```rust
// benches/packed_decimal_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_packed_decimal_pack(c: &mut Criterion) {
    c.bench_function("pack_9_digit", |b| {
        let mut pd = PackedDecimal::new(9, 2, true);
        b.iter(|| pd.set_decimal(dec!(1234567.89)));
    });
}

fn bench_move_numeric(c: &mut Criterion) {
    c.bench_function("move_comp3_to_comp3", |b| {
        let src = PackedDecimal::from_value(9, 2, true, dec!(1234567.89));
        let mut dest = PackedDecimal::new(7, 2, true);
        let config = RuntimeConfig::default();
        b.iter(|| cobol_move(&src, &mut dest, &config));
    });
}
```

### 15.4 Inline Hints

For hot-path functions, use `#[inline]` judiciously:

```rust
#[inline]
pub fn cobol_move(src: &dyn CobolField, dest: &mut dyn CobolField, config: &RuntimeConfig) {
    // ...dispatch...
}

#[inline(always)]
fn copy_left_justified(src: &[u8], dest: &mut [u8]) {
    let copy_len = src.len().min(dest.len());
    dest[..copy_len].copy_from_slice(&src[..copy_len]);
    dest[copy_len..].fill(b' ');
}
```

---

## 16. Feature Flags and Optional Dependencies

### 16.1 Feature Definitions

```toml
# cobol-runtime/Cargo.toml

[features]
default = ["full"]
full = ["indexed-file", "sort", "ebcdic", "print"]
minimal = []                          # Just types, MOVE, arithmetic
indexed-file = ["cobol-io/indexed-file"]  # Enables SQLite
sort = ["cobol-sort"]                 # Enables SORT/MERGE + tempfile
ebcdic = ["cobol-types/ebcdic"]       # Enables EBCDIC tables
print = ["cobol-io/print"]            # Enables LINAGE print support
```

### 16.2 Dependency Matrix

| Feature | Adds Crate | Adds External Dep |
|---|---|---|
| `minimal` | cobol-core, cobol-types, cobol-move, cobol-arithmetic | rust_decimal, byteorder |
| `+indexed-file` | + cobol-io | + rusqlite (+ libsqlite3-sys) |
| `+sort` | + cobol-sort | + tempfile |
| `+ebcdic` | (tables in cobol-types) | (none -- built-in) |
| `+print` | (in cobol-io) | (none) |
| `full` | all crates | all external deps |

### 16.3 Size Impact

| Configuration | Est. Binary Size (release) | External Deps |
|---|---|---|
| `minimal` | ~500 KB | 2 crates |
| `minimal` + `sort` | ~600 KB | 3 crates |
| `full` (no SQLite static) | ~2 MB | 6 crates |
| `full` (SQLite static) | ~4 MB | 6 crates (SQLite bundled) |

---

## 17. Implementation Roadmap

### Phase 1: Foundation (Weeks 1-4)

**Goal**: Core types, MOVE engine, basic arithmetic -- enough to transpile
programs that do computation and display but no file I/O.

| Task | Crate | Priority |
|---|---|---|
| RuntimeConfig, DataCategory, traits | cobol-core | P0 |
| PackedDecimal (COMP-3) | cobol-types | P0 |
| CompBinary (COMP/COMP-5) | cobol-types | P0 |
| PicX (alphanumeric) | cobol-types | P0 |
| PicA (alphabetic) | cobol-types | P1 |
| CobolArray (OCCURS) | cobol-types | P0 |
| Level 88 predicates | cobol-types | P1 |
| FigurativeConstant | cobol-types | P0 |
| MOVE engine (all 6 dest categories) | cobol-move | P0 |
| MOVE CORRESPONDING | cobol-move | P1 |
| INITIALIZE | cobol-move | P1 |
| ADD/SUBTRACT/MULTIPLY/DIVIDE | cobol-arithmetic | P0 |
| COMPUTE | cobol-arithmetic | P0 |
| ROUNDED phrase | cobol-arithmetic | P0 |
| ON SIZE ERROR | cobol-arithmetic | P0 |
| CobolProgram struct | cobol-runtime | P0 |
| DISPLAY/ACCEPT | cobol-runtime | P0 |
| STOP RUN | cobol-runtime | P0 |
| Prelude module | cobol-runtime | P0 |
| Unit tests for all above | tests | P0 |
| MOVE matrix tests | tests | P0 |

**Deliverable**: Can transpile and run a COBOL program that reads no files,
does computation, and displays results.

### Phase 2: File I/O and Strings (Weeks 5-8)

**Goal**: Sequential and indexed file I/O, record processing, EBCDIC.

| Task | Crate | Priority |
|---|---|---|
| ZonedDecimal (DISPLAY numeric) | cobol-types | P0 |
| NumericEdited | cobol-types | P0 |
| EBCDIC tables (CP037, CP1140) | cobol-types | P1 |
| Sequential file implementation | cobol-io | P0 |
| Line sequential implementation | cobol-io | P0 |
| Indexed file (SQLite backend) | cobol-io | P0 |
| Relative file implementation | cobol-io | P1 |
| FileStatus mapping | cobol-io | P0 |
| File resolver (config-based) | cobol-io | P0 |
| Variable-length records (RDW) | cobol-io | P1 |
| EBCDIC I/O conversion | cobol-io | P1 |
| DECLARATIVES error handlers | cobol-io | P2 |
| De-editing (IBM extension) | cobol-move | P2 |
| INSPECT verb | cobol-runtime | P1 |
| STRING/UNSTRING | cobol-runtime | P1 |
| Reference modification | cobol-runtime | P1 |
| File I/O integration tests | tests | P0 |

**Deliverable**: Can transpile and run batch COBOL programs with sequential
file processing and indexed file CRUD.

### Phase 3: SORT/MERGE and Advanced (Weeks 9-12)

**Goal**: SORT/MERGE, print files, remaining types, performance tuning.

| Task | Crate | Priority |
|---|---|---|
| In-memory sort | cobol-sort | P0 |
| External merge sort | cobol-sort | P0 |
| MERGE implementation | cobol-sort | P1 |
| Adaptive sort engine | cobol-sort | P0 |
| Sort key comparator (all types) | cobol-sort | P0 |
| Collating sequence (EBCDIC) | cobol-sort | P1 |
| INPUT/OUTPUT PROCEDURE closures | cobol-sort | P0 |
| Print file (LINAGE/ADVANCING) | cobol-io | P1 |
| AlphaEdited | cobol-types | P2 |
| CobolFloat32/64 (COMP-1/COMP-2) | cobol-types | P2 |
| REDEFINES support | cobol-types | P2 |
| Variable OCCURS DEPENDING ON | cobol-types | P2 |
| CALL/GOBACK support | cobol-runtime | P1 |
| Intrinsic functions | cobol-arithmetic | P1 |
| Performance benchmarks | benches | P1 |
| Performance optimization | all crates | P1 |
| Sort integration tests | tests | P0 |
| End-to-end test suite | tests | P0 |

**Deliverable**: Feature-complete runtime supporting the full range of
common COBOL batch programs. Ready for transpiler integration.

### Phase 4: Polish and Production (Weeks 13+)

| Task | Priority |
|---|---|
| NationalString (PIC N) | P3 |
| RENAMES (level 66) | P3 |
| P-scaling (implied scaling positions) | P3 |
| SYNC alignment | P3 |
| IBM HFP float format | P3 |
| Documentation (rustdoc for all public APIs) | P1 |
| CI/CD pipeline | P1 |
| Crates.io publication | P2 |
| Fuzzing (proptest for MOVE engine, arithmetic) | P2 |
| Memory-mapped I/O (LDS emulation) | P3 |

---

## Appendix A: Workspace Cargo.toml

```toml
[workspace]
members = [
    "crates/cobol-core",
    "crates/cobol-types",
    "crates/cobol-move",
    "crates/cobol-arithmetic",
    "crates/cobol-io",
    "crates/cobol-sort",
    "crates/cobol-runtime",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
license = "MIT OR Apache-2.0"
repository = "https://github.com/example/cobol2rust"

[workspace.dependencies]
rust_decimal = "1.34"
rust_decimal_macros = "1.34"
byteorder = "1.5"
thiserror = "2.0"
derive_more = "1.0"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
rusqlite = { version = "0.31", features = ["bundled"] }
tempfile = "3.10"
criterion = { version = "0.5", features = ["html_reports"] }
```

---

## Appendix B: Cross-Reference to All Specifications

| Spec Document | Primary Crate(s) | Key Section in This Doc |
|---|---|---|
| `cobol_to_rust_datatype_mapping.md` | cobol-core, cobol-types | Sections 4, 5 |
| `cobol_arithmetic_operations.md` | cobol-arithmetic | Section 7 |
| `cobol_control_flow_constructs.md` | cobol-runtime | Sections 10, 13 |
| `cobol_move_engine_spec.md` | cobol-move | Section 6 |
| `cobol_file_io_record_processing.md` | cobol-io | Section 8 |
| `cobol_sort_merge_spec.md` | cobol-sort | Section 9 |
| `cobol_migration_target_analysis.md` | (strategic, all) | Section 1 |
