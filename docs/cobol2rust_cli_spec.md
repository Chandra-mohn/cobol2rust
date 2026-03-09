# cobol2rust CLI Specification

## Overview

`cobol2rust` is a command-line tool that transpiles COBOL source code to Rust,
generates Cargo workspaces for enterprise codebases, and integrates with the
build toolchain. It complements `coqu-di` which handles data inspection and
analysis.

**Tool boundary:**
- `cobol2rust` -- code transpilation, workspace generation, build integration
- `coqu-di` -- data inspection, record analysis, binary decoding, discovery

## Binary

- **Name:** `cobol2rust`
- **Crate:** `cobol-cli` (workspace member, binary crate)
- **Install:** `cargo install --path crates/cobol-cli`

## Global Flags

All subcommands accept these flags:

| Flag | Short | Description |
|---|---|---|
| `--copybook-path <DIR>` | `-C` | Copybook search directory (repeatable) |
| `--source-format <FMT>` | `-f` | Override format: `fixed`, `free`, `auto` (default: `auto`) |
| `--verbose` | `-v` | Increase output verbosity (repeatable: -vv, -vvv) |
| `--quiet` | `-q` | Suppress non-error output |
| `--color <WHEN>` | | Color output: `auto`, `always`, `never` (default: `auto`) |
| `--config <FILE>` | | Config file path (TOML) |

## Subcommands

### 1. `cobol2rust transpile`

Transpile COBOL source to Rust. Supports single-file and directory (workspace) modes.

#### Single-file mode

```bash
cobol2rust transpile program.cbl -o program.rs
cobol2rust transpile program.cbl --lib           # generate lib.rs (no main)
cobol2rust transpile program.cbl --main           # force main.rs
cobol2rust transpile program.cbl                  # auto-detect main vs lib
```

**Flags:**

| Flag | Short | Description |
|---|---|---|
| `--output <PATH>` | `-o` | Output file or directory (default: stdout) |
| `--main` | | Force main program output (generates `fn main()`) |
| `--lib` | | Force library output (no `fn main()`) |
| `--library-map <NAME=DIR>` | `-L` | COPY library mapping (repeatable) |

**Main vs lib auto-detection:**

1. Has `LINKAGE SECTION` + `PROCEDURE DIVISION USING` -> lib (subprogram)
2. Has `STOP RUN` or `GOBACK` at top-level paragraph -> main
3. Default -> main (conservative)

**Output:** A complete `.rs` file with `use cobol_runtime::prelude::*`.

#### Directory mode (workspace generation)

```bash
cobol2rust transpile ./cobol-src/ -o ./rust-output/ --workspace
```

**Additional flags for directory mode:**

| Flag | Description |
|---|---|
| `--workspace` | Generate a Cargo workspace (required for directory input) |
| `--parallel` | Transpile files in parallel (uses available cores) |
| `--continue-on-error` | Skip files that fail, report at end |

**Output structure:**

```
rust-output/
  Cargo.toml                    # workspace manifest
  copybooks/                    # shared types from .cpy files
    Cargo.toml
    src/lib.rs
  programs/
    payroll_main/               # detected as main program
      Cargo.toml
      src/main.rs
    calc_tax/                   # detected as subprogram
      Cargo.toml
      src/lib.rs
```

**Main detection in workspace mode** uses cross-file CALL graph analysis:

1. Parse all `.cbl` files for CALL targets
2. Programs that are CALL targets = subprograms (lib)
3. Programs with LINKAGE SECTION + USING = subprograms (lib)
4. Remaining programs with STOP RUN/GOBACK = main programs
5. Override per-file via config file

**Workspace Cargo.toml generation:**

- Each program becomes a crate in `programs/`
- Copybooks become a shared `copybooks` crate
- Inter-crate dependencies derived from CALL graph
- All crates depend on `cobol-runtime`

#### Error output

Errors use `miette` for rich diagnostics:

```
Error: cobol2rust::parse

  x Unexpected token in PROCEDURE DIVISION
   ,-[payroll.cbl:1247:12]
1247 |     MOVE WS-AMT TOO WS-TOTAL
     :                 ^^^ expected TO, found identifier
   `----
  help: Did you mean 'TO' instead of 'TOO'?
```

---

### 2. `cobol2rust check`

Validate COBOL source without transpiling. Reports syntax errors, unsupported
features, and source format detection.

```bash
cobol2rust check program.cbl
cobol2rust check ./cobol-src/                    # batch validation
cobol2rust check program.cbl --format json       # machine-readable output
```

**Flags:**

| Flag | Short | Description |
|---|---|---|
| `--format <FMT>` | | Output format: `text` (default), `json` |
| `--strict` | | Treat warnings as errors |

**Exit codes:**

| Code | Meaning |
|---|---|
| 0 | Valid, no issues |
| 1 | Syntax errors found |
| 2 | Warnings only (valid but has issues) |

**Text output:**

```
Checking payroll.cbl...
  Format: Fixed (detected)
  Program-ID: PAYROLL-MAIN
  [OK] Syntax valid
  [WARN] Line 834: EXEC SQL not supported (will be skipped)
  [WARN] Line 1102: ALTER verb detected (consider refactoring)
  [INFO] 87 paragraphs, 12 CALL statements, 3 file operations

Checking calc_tax.cbl...
  Format: Fixed (detected)
  Program-ID: CALC-TAX
  [OK] Syntax valid
  [INFO] Subprogram (has LINKAGE SECTION + USING)

Summary: 2 files checked, 0 errors, 2 warnings
```

**JSON output:**

```json
{
  "files": [{
    "path": "payroll.cbl",
    "program_id": "PAYROLL-MAIN",
    "format": "fixed",
    "valid": true,
    "errors": [],
    "warnings": [
      {"line": 834, "message": "EXEC SQL not supported", "code": "W001"}
    ],
    "info": {
      "paragraphs": 87,
      "calls": 12,
      "file_ops": 3,
      "is_subprogram": false
    }
  }],
  "summary": {"files": 2, "errors": 0, "warnings": 2}
}
```

---

### 3. `cobol2rust preprocess`

Expand COPY statements and clean source format. Outputs preprocessed COBOL
suitable for other tools or manual review.

```bash
cobol2rust preprocess program.cbl                          # stdout
cobol2rust preprocess program.cbl -o program.expanded.cbl  # to file
cobol2rust preprocess program.cbl --no-copy                # strip format only
```

**Flags:**

| Flag | Short | Description |
|---|---|---|
| `--output <PATH>` | `-o` | Output file (default: stdout) |
| `--no-copy` | | Skip COPY expansion, only strip fixed-format columns |
| `--library-map <NAME=DIR>` | `-L` | COPY library mapping (repeatable) |
| `--show-origins` | | Add comments showing where COPY content came from |

**What it does:**

1. Detects and strips fixed-format columns (1-6 seq numbers, col 7 indicators, 73+ ID)
2. Handles continuation lines (col 7 hyphen)
3. Strips comment lines and debug lines
4. Expands COPY statements (recursively, with REPLACING support)
5. Handles COPY SUPPRESS

**Output with --show-origins:**

```cobol
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PAYROLL.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      *> BEGIN COPY EMPLOYEE-COPY
       01 WS-EMPLOYEE-RECORD.
          05 WS-EMP-ID     PIC 9(6).
          05 WS-EMP-NAME   PIC X(30).
      *> END COPY EMPLOYEE-COPY
```

---

### 4. `cobol2rust parse`

Parse COBOL source to a typed AST. Outputs JSON for tooling integration or
a human-readable tree for exploration.

```bash
cobol2rust parse program.cbl                     # tree view (default)
cobol2rust parse program.cbl --format json       # JSON AST
cobol2rust parse program.cbl --section procedure # procedure division only
cobol2rust parse program.cbl --section data      # data division only
```

**Flags:**

| Flag | Short | Description |
|---|---|---|
| `--format <FMT>` | | Output format: `tree` (default), `json` |
| `--section <SEC>` | | Filter: `all` (default), `data`, `procedure`, `identification` |
| `--output <PATH>` | `-o` | Output file (default: stdout) |

**Tree output:**

```
PAYROLL-MAIN
+-- IDENTIFICATION DIVISION
|   Program-ID: PAYROLL-MAIN
+-- DATA DIVISION
|   +-- WORKING-STORAGE SECTION
|   |   +-- 01 WS-EMPLOYEE-RECORD (Group, 89 bytes)
|   |   |   +-- 05 WS-EMP-ID (PIC 9(6), Numeric, 6 bytes)
|   |   |   +-- 05 WS-EMP-NAME (PIC X(30), Alphanumeric, 30 bytes)
|   |   |   +-- 05 WS-EMP-SALARY (PIC S9(7)V99 COMP-3, Packed, 5 bytes)
|   |   +-- 01 WS-FLAGS (Group, 2 bytes)
|   |       +-- 05 WS-EOF-FLAG (PIC X, Alphanumeric, 1 byte)
|   |           +-- 88 WS-EOF = 'Y'
|   +-- LINKAGE SECTION
|       (empty)
+-- PROCEDURE DIVISION
    +-- MAIN-PARA
    |   PERFORM INIT-PARA
    |   PERFORM PROCESS-PARA UNTIL WS-EOF
    |   PERFORM CLEANUP-PARA
    |   STOP RUN
    +-- INIT-PARA
    |   OPEN INPUT EMPLOYEE-FILE
    +-- PROCESS-PARA
    |   READ EMPLOYEE-FILE AT END SET WS-EOF TO TRUE
    |   ADD WS-EMP-SALARY TO WS-TOTAL
    +-- CLEANUP-PARA
        CLOSE EMPLOYEE-FILE
        DISPLAY WS-TOTAL
```

**JSON output:** Full serialized `CobolProgram` AST (requires serde::Serialize
on AST types).

---

### 5. `cobol2rust init`

Scaffold a Cargo workspace from a COBOL codebase without transpiling.
Creates the directory structure, Cargo.toml files, and a manifest mapping
COBOL programs to Rust crates.

```bash
cobol2rust init ./cobol-src/ -o ./rust-project/
cobol2rust init ./cobol-src/ -o ./rust-project/ --dry-run   # preview only
```

**Flags:**

| Flag | Short | Description |
|---|---|---|
| `--output <DIR>` | `-o` | Output directory (required) |
| `--dry-run` | | Show what would be created without writing |
| `--manifest <FILE>` | | Write/read a TOML manifest for main/lib overrides |

**What it does:**

1. Scans all `.cbl` files in input directory
2. Detects main vs subprogram for each
3. Identifies copybook dependencies
4. Creates workspace skeleton:

```
rust-project/
  Cargo.toml              # workspace
  cobol2rust-manifest.toml # mapping file (editable)
  copybooks/
    Cargo.toml
    src/lib.rs             # placeholder
  programs/
    payroll_main/
      Cargo.toml           # depends on cobol-runtime + copybooks
      src/main.rs           # placeholder: "// Transpile with: cobol2rust transpile ..."
    calc_tax/
      Cargo.toml
      src/lib.rs            # placeholder
```

**Manifest file** (`cobol2rust-manifest.toml`):

```toml
# Auto-generated by cobol2rust init. Edit to override detection.
# Re-run `cobol2rust transpile --workspace` to apply changes.

[programs.payroll_main]
source = "cobol-src/PAYROLL-MAIN.CBL"
type = "main"         # auto-detected, override to "lib" if needed
calls = ["CALC-TAX", "DB-ACCESS"]

[programs.calc_tax]
source = "cobol-src/CALC-TAX.CBL"
type = "lib"           # has LINKAGE SECTION + USING
called_by = ["PAYROLL-MAIN"]

[copybooks]
sources = ["cobol-src/copybooks/"]
files = ["EMPLOYEE-COPY.cpy", "DATE-FIELDS.cpy"]
```

This manifest lets users review and override auto-detection before running
`transpile --workspace`.

---

### 6. `cobol2rust compile`

Transpile and build in one step. Runs `transpile` then invokes `cargo build`.

```bash
cobol2rust compile program.cbl -o ./build/
cobol2rust compile program.cbl -o ./build/ --release
cobol2rust compile ./cobol-src/ -o ./build/ --workspace --release
```

**Flags:**

| Flag | Short | Description |
|---|---|---|
| `--output <DIR>` | `-o` | Output directory (required) |
| `--release` | | Build in release mode |
| `--target <TRIPLE>` | | Cross-compilation target |
| `--workspace` | | Directory mode (generate + build workspace) |
| All `transpile` flags | | Passed through to transpile step |

**Process:**

1. Run `transpile` (single file or workspace mode)
2. Ensure `cobol-runtime` dependency is resolvable
3. Invoke `cargo build` in the output directory
4. Report build results

**Output:**

```
[1/2] Transpiling payroll.cbl -> ./build/src/main.rs
[2/2] Building with cargo...
    Compiling cobol-runtime v0.1.0
    Compiling payroll v0.1.0 (./build)
    Finished dev [unoptimized + debuginfo] target(s)

Binary: ./build/target/debug/payroll
```

---

### 7. `cobol2rust diff`

Compare transpilation outputs between two COBOL source versions or two
different COBOL files.

```bash
cobol2rust diff old.cbl new.cbl                  # diff generated Rust
cobol2rust diff old.cbl new.cbl --ast            # diff ASTs
cobol2rust diff old.cbl new.cbl --context 5      # more context lines
```

**Flags:**

| Flag | Short | Description |
|---|---|---|
| `--ast` | | Compare AST structures instead of generated Rust |
| `--context <N>` | | Context lines around differences (default: 3) |
| `--format <FMT>` | | Output: `unified` (default), `json` |
| `--no-color` | | Disable colored diff output |

**Process:**

1. Transpile both inputs independently
2. Diff the outputs (Rust source or serialized AST)
3. Display with unified diff format

**Use case:** Verify that a COBOL change produces the expected Rust change.
Useful during migration validation -- teams can review exactly what the
transpiler produces for a given COBOL modification.

**Output:**

```
--- old.cbl -> old.rs
+++ new.cbl -> new.rs
@@ -45,7 +45,8 @@
     pub ws_emp_salary: PackedDecimal,
     pub ws_emp_dept: PicX,
+    pub ws_emp_bonus: PackedDecimal,
 }

@@ -112,6 +113,9 @@
     cobol_add(&ws.ws_emp_salary, &mut ws.ws_total, None, &ctx.config);
+    // New bonus calculation
+    cobol_multiply(&ws.ws_emp_salary, &mut ws.ws_emp_bonus, None, &ctx.config);
+    cobol_add(&ws.ws_emp_bonus, &mut ws.ws_total, None, &ctx.config);
```

---

## Configuration File

Optional TOML config file (`cobol2rust.toml` or `--config`):

```toml
[source]
format = "auto"                           # fixed | free | auto
copybook_paths = ["./copybooks", "./shared/copy"]
max_copy_depth = 10

[output]
color = "auto"

[workspace]
# Per-program overrides for main/lib detection
[workspace.overrides]
"PAYROLL-MAIN" = "main"
"CALC-TAX" = "lib"
"OLDUTIL" = "skip"                       # exclude from workspace
```

---

## Implementation Phases

| Phase | Session | Deliverables |
|---|---|---|
| **1 - Foundation** | 38 | `cobol-cli` crate, clap setup, `transpile` (single-file), `check`, `preprocess` |
| **2 - Tooling** | 39 | `parse` (JSON + tree), serde::Serialize on AST types |
| **3 - Workspace** | 40 | `init`, `transpile --workspace`, batch processing, manifest |
| **4 - Build** | 41 | `compile`, `diff` |
| **5 - Scale** | 42+ | Progress bars, parallel transpilation, incremental mode |

## Dependencies (new)

| Crate | Purpose |
|---|---|
| `clap` (derive) | CLI framework |
| `serde_json` | JSON output |
| `similar` | Diff algorithm (for `diff` subcommand) |
| `indicatif` | Progress bars (Phase 5) |
| `rayon` | Parallel batch processing (Phase 5) |

## Relationship to coqu-di

`coqu-di` provides data-level analysis:
- Record layout inspection (field offsets, PIC analysis, byte maps)
- Binary data decoding (EBCDIC, packed decimal, COMP fields)
- Copybook-to-data-file discovery and matching
- REDEFINES discrimination analysis
- VS Code extension for visual data exploration

`cobol2rust` provides code-level transpilation:
- COBOL source -> Rust source conversion
- Workspace scaffolding for enterprise codebases
- Build toolchain integration (transpile + cargo build)
- Migration diff analysis (COBOL change -> Rust change)

The two tools complement each other: use `coqu-di` to understand data, use
`cobol2rust` to convert code.
