# COBOL Tools -- Feature Inventory

Comprehensive feature catalog across all three projects.
Last updated: 2026-03-08

---

## 1. cobol2rust (Rust)

**Purpose**: Transpile COBOL source code to idiomatic Rust.

### 1.1 Parsing

| Feature | Detail |
|---------|--------|
| Grammar | ANTLR4 Cobol85.g4 (Rust bindings via antlr-rust) |
| Source formats | Fixed-format (columns 1-6/7/8-72/73-80), free-format |
| Preprocessing | Column stripping, comment removal, continuation lines |
| COPY expansion | Recursive COPY resolution with REPLACING clause |
| Copybook search | Configurable search paths, library-map (COPY x OF lib) |
| Format detection | Auto-detect fixed vs free format |
| AST output | Typed Rust AST (CobolProgram, DataDivision, ProcedureDivision) |

### 1.2 Data Division Support

| Feature | Detail |
|---------|--------|
| Level numbers | 01-49, 66 (RENAMES), 77, 88 (condition names) |
| PIC clause | Full parser: 9, X, A, S, V, P, Z, *, +, -, $, CR, DB, B, 0, / |
| USAGE types | DISPLAY, COMP/COMP-4, COMP-3/PACKED-DECIMAL, COMP-5, COMP-1 (float), COMP-2 (double) |
| VALUE clause | Numeric literals, alphanumeric literals, figurative constants |
| OCCURS | Fixed arrays, OCCURS DEPENDING ON (variable length) |
| OCCURS INDEXED BY | Index name extraction |
| REDEFINES | Overlapping field definitions |
| RENAMES (66) | Level-66 range renames |
| Group items | Hierarchical field nesting with byte layout computation |
| FILE SECTION | FD/SD declarations with record definitions |
| Byte layout | Automatic offset/length computation for all fields |

### 1.3 Procedure Division Support

| COBOL Verb | Status |
|------------|--------|
| MOVE | Full (simple, CORRESPONDING, figurative constants) |
| DISPLAY | Full (multiple operands, NO ADVANCING, UPON) |
| ACCEPT | Full (FROM DATE/DAY/TIME) |
| ADD | Full (TO, GIVING, CORRESPONDING, SIZE ERROR) |
| SUBTRACT | Full (FROM, GIVING, CORRESPONDING, SIZE ERROR) |
| MULTIPLY | Full (BY, GIVING, SIZE ERROR) |
| DIVIDE | Full (INTO, GIVING, REMAINDER, SIZE ERROR) |
| COMPUTE | Full (arbitrary expressions with +, -, *, /, **) |
| IF/ELSE/END-IF | Full (nested, compound conditions AND/OR/NOT) |
| EVALUATE/WHEN | Full (EVALUATE TRUE, WHEN OTHER, multiple subjects) |
| PERFORM | Full (procedure call, THRU, TIMES, UNTIL, VARYING, inline) |
| GO TO | Full (simple, DEPENDING ON) |
| STOP RUN | Full |
| GOBACK | Full |
| EXIT PROGRAM/SECTION/PARAGRAPH | Full |
| CONTINUE | Full |
| NEXT SENTENCE | Full |
| INITIALIZE | Full (default + REPLACING NUMERIC/ALPHANUMERIC/ALPHABETIC) |
| SET | Full (index, condition name, UP/DOWN BY) |
| CALL | Full (BY REFERENCE/CONTENT/VALUE, ON EXCEPTION) |
| CANCEL | Full |
| OPEN | Full (INPUT, OUTPUT, I-O, EXTEND) |
| CLOSE | Full |
| READ | Full (NEXT, INTO, AT END, KEY IS) |
| WRITE | Full (FROM, AFTER/BEFORE ADVANCING, AT EOP, INVALID KEY) |
| REWRITE | Full (FROM, INVALID KEY) |
| DELETE | Full (INVALID KEY) |
| START | Full (KEY IS, INVALID KEY) |
| SORT | Full (ASCENDING/DESCENDING KEY, INPUT/OUTPUT PROCEDURE, USING/GIVING) |
| MERGE | Full (similar to SORT) |
| RELEASE | Full (FROM) |
| RETURN | Full (INTO, AT END) |
| INSPECT | Full (TALLYING, REPLACING, CONVERTING) |
| STRING | Full (DELIMITED BY, INTO, POINTER, OVERFLOW) |
| UNSTRING | Full (DELIMITED BY, INTO, COUNT, POINTER, OVERFLOW) |
| EXEC SQL | Not supported (skipped) |
| EXEC CICS | Not supported (skipped) |
| ALTER | Not supported |

### 1.4 Runtime Library (cobol-runtime)

| Module | Features |
|--------|----------|
| cobol-types | PicX, PicA, PackedDecimal, ZonedDecimal, CompBinary, NumericEdited, AlphaEdited, CobolFloat, CobolArray, CobolVarArray |
| cobol-move | MOVE engine (numeric-to-numeric, alpha-to-alpha, cross-type, figurative constants, CORRESPONDING) |
| cobol-arithmetic | ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE with ON SIZE ERROR, ROUNDED |
| cobol-io | Sequential, Relative, Indexed (SQLite-backed) file I/O |
| cobol-sort | In-memory + external merge sort, RELEASE/RETURN |
| EBCDIC | CP037 encode/decode tables |
| Reference modification | field(start:length) substring access |
| Intrinsic functions | 27 functions (FUNCTION LENGTH, UPPER-CASE, LOWER-CASE, REVERSE, TRIM, NUMVAL, CURRENT-DATE, etc.) |
| CALL dispatcher | Dynamic program loading, BY REFERENCE/CONTENT/VALUE |
| Condition names | 88-level condition evaluation (single value, range, list) |

### 1.5 Code Generation

| Feature | Detail |
|---------|--------|
| Output | Complete Rust crate (main.rs) depending on cobol-runtime |
| WorkingStorage struct | All COBOL fields as typed Rust fields |
| ProgramContext | Runtime control (stopped, exit_program, goto_target, return_code) |
| Paragraph dispatch | Paragraph fall-through with goto_target mechanism |
| Section dispatch | Section-level dispatch tables |
| Group child map | Compile-time expansion for INITIALIZE, MOVE CORRESPONDING |
| Condition map | 88-level conditions resolved to parent field comparisons |
| Record-file map | Links record names to file descriptors for I/O |
| Sort field map | Byte offset/length for SD record fields |

### 1.6 CLI (cobol2rust)

| Subcommand | Features |
|------------|----------|
| transpile | Single file (stdout or -o file), --workspace (directory -> Cargo workspace) |
| check | Syntax validation, program stats, --strict, --format text/json, **--coverage** (transpilation coverage analysis with line numbers) |
| preprocess | Show preprocessed output (columns stripped, continuations resolved) |
| parse | AST dump (--format tree or json) |
| init | Scaffold Cargo workspace skeleton from COBOL directory |

### 1.7 Diagnostics (NEW)

| Feature | Detail |
|---------|--------|
| TranspileDiagnostic | Line number, severity, category, message, COBOL source text |
| Severity levels | Error, Warning, Info |
| Categories | ParseError, UnhandledStatement, UnhandledClause, UnsupportedFeature, DataDivisionGap |
| TranspileStats | total_statements, mapped_statements, total_data_entries, verbs_used, verbs_unsupported |
| TranspileResult | Rust code + diagnostics + stats |
| Coverage % | mapped_statements / total_statements |
| CLI integration | `check --coverage` reports unhandled constructs with line numbers (text + JSON) |

---

## 2. coqu (Python)

**Purpose**: Parse COBOL source code and provide an interactive querying mechanism for understanding COBOL programs.

### 2.1 Parsing

| Feature | Detail |
|---------|--------|
| Grammar | ANTLR4 Cobol85.g4 (Python bindings) |
| Hybrid strategy | Pass 1: fast regex indexer (always), Pass 2: full ANTLR parse (on demand) |
| Source formats | Free-format, fixed with sequence numbers, Panvalet/Librarian version markers |
| Preprocessing | COPY/REPLACE resolution, format normalization |
| EXEC blocks | EXEC SQL and EXEC CICS block extraction |
| Dialects | COBOL-85 (ANSI), IBM Enterprise COBOL |
| Performance | 2M+ line files via indexer (~5-10s), full parse cached via MessagePack |

### 2.2 AST Model

| Node Type | Fields |
|-----------|--------|
| CobolProgram | program_id, source_path, hash, line_count, divisions, copybook_refs, comments |
| Division | name (IDENTIFICATION, ENVIRONMENT, DATA, PROCEDURE), sections, location |
| Section | name (FILE, WORKING-STORAGE, LINKAGE, PROCEDURE sections), paragraphs, data_items, location |
| Paragraph | name, statements, location |
| DataItem | level, name, pic, usage, value, occurs, redefines, children (hierarchical), location |
| Statement | type (MOVE, CALL, PERFORM, IF, EVALUATE, ...), text, targets, location |
| CopybookRef | name, replacing, resolved, location |
| Comment | text, location, inline flag |
| SourceLocation | start_line, start_col, end_line, end_col |

### 2.3 Structural Indexer (Fast Pass)

| Feature | Detail |
|---------|--------|
| Division detection | Regex scan for all four divisions with line positions |
| Section detection | WORKING-STORAGE, FILE, LINKAGE, PROCEDURE sections |
| Paragraph detection | Named paragraphs in PROCEDURE DIVISION |
| Copybook detection | COPY statements with member names |
| Format handling | Handles sequence numbers, Panvalet markers, continuations |
| Speed | ~5-10 seconds for 2M+ lines (vs. minutes for full ANTLR) |

### 2.4 Query Commands (21 built-in)

**Structural Navigation:**

| Command | Description |
|---------|-------------|
| divisions | List all divisions with line ranges |
| division \<name\> | Show specific division details |
| sections | List all sections |
| section \<name\> | Show specific section |
| procedure-sections | List PROCEDURE DIVISION sections only |
| paragraphs | List all paragraphs with line ranges |
| paragraph \<name\> [--body] | Show paragraph details, optionally with source code |

**Data Definition:**

| Command | Description |
|---------|-------------|
| working-storage | List all WORKING-STORAGE items with PIC, USAGE, VALUE |
| variable \<name\> | Show specific variable definition and hierarchy |
| file-section | List FILE SECTION definitions |
| linkage | List LINKAGE SECTION items |

**Statement Analysis:**

| Command | Description |
|---------|-------------|
| calls | Extract all CALL statements (target program names) |
| performs | Extract all PERFORM targets (paragraph/section names) |
| moves | Extract MOVE statements (source -> destinations) |
| sql | Extract EXEC SQL blocks |
| cics | Extract EXEC CICS blocks |

**Copybook Intelligence:**

| Command | Description |
|---------|-------------|
| copybooks | List all COPY statements with resolution status |
| copybook \<name\> | Show copybook details (resolved path, REPLACING) |
| copybook-deps | Dependency graph of copybook chains |

**Code Search:**

| Command | Description |
|---------|-------------|
| find \<pattern\> [--context=N] | Regex search across loaded source |
| references \<name\> | Find all references to a name (word boundary match) |
| where-used \<name\> | Find which paragraphs call a specific paragraph |

### 2.5 CLI Interface

| Mode | Usage |
|------|-------|
| Interactive REPL | `coqu program.cbl [copybooks/]` -- tab completion, history, /meta commands |
| Single command | `coqu -c "paragraphs" program.cbl` |
| Script execution | `coqu -s analysis.coqu program.cbl` |
| Coverage analysis | `coqu coverage file.cbl --mode both --missed-lines --missed-code` |

**REPL Meta Commands:**

| Command | Description |
|---------|-------------|
| /load \<file\> | Load a COBOL source file |
| /loaddir \<dir\> [pattern] | Batch load from directory |
| /unload \<name\> | Remove program from workspace |
| /reload [name] | Force re-parse |
| /list, /programs | Show loaded programs |
| /copypath \<path\> | Add copybook search path |
| /info [name] | Program statistics |
| /cache | Cache hit/miss stats |
| /clear-cache | Flush AST cache |
| /help [cmd] | Contextual help |

### 2.6 Workspace Management

| Feature | Detail |
|---------|--------|
| Multi-program | Load and query across multiple programs simultaneously |
| Copybook resolution | Configurable search paths, extension-agnostic (.cpy, .copy, .cbl, .cob) |
| AST caching | MessagePack serialization to ~/.cache/coqu/, SHA256-based invalidation |
| Configuration | .coqu.toml or ~/.config/coqu/config.toml (copybook paths, cache settings) |

### 2.7 Output Formats

| Format | Detail |
|--------|--------|
| Text | Human-readable with indentation, location info, optional source body |
| JSON | Structured output via .to_dict() serialization, top-level count + items |

### 2.8 Parser Coverage Analysis

| Feature | Detail |
|---------|--------|
| CoverageAnalyzer | Identifies lines parsed vs. missed by ANTLR |
| Modes | parser-only, indexer-only, both |
| Output | Missed lines, missed source code, coverage percentage |
| Purpose | Debugging grammar gaps, assessing parser quality |

---

## 3. coqu-di (Rust)

**Purpose**: Read and interpret COBOL binary data files by applying copybook schemas, with automatic REDEFINES resolution.

### 3.1 Architecture (4 layers)

| Layer | Crate | Role |
|-------|-------|------|
| 0 | coqu-di-core | Pure data logic, no I/O. Operates on &str and &[u8] |
| 1 | coqu-di-viewer | Session state, windowed file access, export |
| 2a | coqu-di (CLI) | Command-line interface |
| 2b | vscode extension | VS Code custom editor (TypeScript) |

### 3.2 Parsing (Core)

| Feature | Detail |
|---------|--------|
| Grammar | ANTLR4 Cobol85.g4 (Rust bindings via antlr-rust) |
| Target | DATA DIVISION only (working-storage, file section, linkage) |
| PROCEDURE DIVISION | Parsed only for discriminator detection (IF/EVALUATE patterns) |
| Copybook wrapping | Standalone copybooks auto-wrapped in minimal program skeleton |
| COPY expansion | Inline expansion with search path resolution |
| AST | DataItem tree (level, PIC, USAGE, REDEFINES, OCCURS, 88-conditions) |
| Output | Internal only (no ANTLR types in public API) |

### 3.3 PIC Clause Parsing

| Feature | Detail |
|---------|--------|
| Numeric PIC | 9, S, V, P with repeat counts |
| Alpha PIC | X, A with repeat counts |
| Edited PIC | Z, *, +, -, $, CR, DB, B, 0, / |
| Byte length | Computed per USAGE type |
| COMP | 2, 4, or 8 bytes based on digit count |
| COMP-3 | (digits + 2) / 2 bytes |
| COMP-1 | 4 bytes (single float) |
| COMP-2 | 8 bytes (double float) |

### 3.4 Record Layout

| Feature | Detail |
|---------|--------|
| Byte offsets | Automatic computation for all fields in hierarchy |
| Group items | Sum of children byte lengths |
| REDEFINES | Shares base field byte offset |
| OCCURS | Field length * occurrence count |
| OCCURS DEPENDING ON | Variable record length support |
| FD RECORD CONTAINS | Fixed vs variable record length from file declarations |

### 3.5 Data Decoders

| Data Type | Decoder |
|-----------|---------|
| PIC X (DISPLAY) | EBCDIC CP037 -> UTF-8, ASCII passthrough |
| PIC A (DISPLAY) | Same as PIC X |
| PIC 9 (DISPLAY) | Zoned decimal (EBCDIC: high nibble F/C/D, ASCII: 0x30-0x39) |
| PIC S9 (DISPLAY) | Signed zoned decimal (trailing sign in zone nibble) |
| PIC 9V9 (DISPLAY) | Fixed-point decimal with implied decimal |
| COMP/COMP-4 | Big-endian binary integer (2/4/8 bytes) |
| COMP-5 | Native binary integer (full range) |
| COMP-3/PACKED | BCD packed decimal (nibbles, trailing sign nibble) |
| COMP-1 | IBM hexadecimal float (4 bytes, NOT IEEE 754) |
| COMP-2 | IBM hexadecimal float (8 bytes, NOT IEEE 754) |
| Edited numeric | Z, *, +, -, $, CR, DB formatting |

### 3.6 Encoding Support

| Feature | Detail |
|---------|--------|
| EBCDIC CP037 | Full 256-entry translation table (US/Canada mainframe) |
| ASCII/Latin-1 | Distributed systems |
| Auto-detection | Scans first record bytes to determine encoding |

### 3.7 REDEFINES Resolution

| Feature | Detail |
|---------|--------|
| Group extraction | Groups all REDEFINES variants sharing same byte region |
| Byte range tracking | Offset + length from base field |
| Discriminator detection | Scans PROCEDURE DIVISION for IF/EVALUATE patterns referencing REDEFINES fields |
| 88-level conditions | Uses condition names as discriminator hints |
| Confidence levels | High (direct IF on discriminator), Medium (88-level), Low (nested/indirect), Unresolved |
| Runtime selection | Applies discriminator logic to choose correct variant per record |

### 3.8 File-to-Schema Discovery

| Feature | Detail |
|---------|--------|
| SELECT/ASSIGN chains | Traces logical -> physical file name mappings |
| FD/COPY linkage | Maps FD record definitions to copybooks |
| Record length matching | Matches data file record length to copybook-defined length |
| Trial decode scoring | Decodes sample records to assess schema quality |
| Filename similarity | Fuzzy matching of data file names to program/copybook names |
| Confidence output | High/Medium/Low confidence per mapping |
| Workspace registration | Auto-save high-confidence matches to config |

### 3.9 CLI (coqu-di)

| Subcommand | Features |
|------------|----------|
| analyze | Show record layout (offset, length, field, PIC, USAGE), REDEFINES groups, discriminators. --program for discriminator source. --format text/json |
| decode | Decode binary data file to JSON. --copybook, --program, --encoding auto/ebcdic/ascii, --limit N, --pretty |
| discover | Auto-scan directories for .DAT/.cpy/.cbl files and match data files to schemas. --min-confidence, --register |
| view | View binary file: raw hex/ASCII (--raw) or decoded with schema. --format json/csv, --offset, --limit |
| schema add | Register data-file-to-copybook mapping with optional --program and --label |
| schema list | Show all registered mappings |
| schema remove | Unregister mapping |
| stdio | JSON protocol server for VS Code (stdin/stdout, newline-delimited JSON) |

### 3.10 Viewer Session (Library API)

| Feature | Detail |
|---------|--------|
| open_dataset | Open binary data file for reading |
| attach_schema | Apply copybook schema to dataset |
| detach_schema | Remove schema (return to raw view) |
| decode_window | Decode N records starting at offset (windowed for TB-scale files) |
| read_raw_window | Read raw bytes at offset |
| export_range | Export record range to JSON or CSV |
| record_count | Total records in file |
| record_length | Record size (from schema or auto-detect) |
| has_schema | Whether a schema is attached |
| encoding | Current encoding (auto/ebcdic/ascii) |
| groups | REDEFINES group information |

### 3.11 File Access

| Feature | Detail |
|---------|--------|
| NativeFileAccess | Standard file I/O for small files |
| Memory-mapped I/O | memmap2 for files > 64MB |
| Windowed decoding | Process TB-scale files without loading entirely |
| Record types | Fixed-length, variable-length (RDW prefix), undefined |

### 3.12 Export Formats

| Format | Detail |
|--------|--------|
| JSON | Array of decoded records. Decimals as strings (preserves precision). Pretty-print optional |
| CSV | RFC 4180. Header row. Escaped values |
| Text table | Offset, Length, Occ, Field, PIC, Usage, Type (analyze command) |
| Raw hex | Hex dump with ASCII sidebar (view --raw) |

### 3.13 VS Code Extension

| Feature | Detail |
|---------|--------|
| Custom editor | Opens .DAT files with custom readonly editor |
| Stage 0 | Raw hex/ASCII view of binary data |
| Stage 1 | Decoded record table (apply copybook via dialog) |
| Copybook dialog | Browse and select .cpy file to apply |
| Program dialog | Browse and select .cbl file for discriminator detection |
| Pagination | 100 records per page with navigation |
| View toggle | Switch between table and hex views |
| Encoding detection | Auto-detect or manual EBCDIC/ASCII selection |
| Process bridge | Spawns `coqu-di stdio` subprocess, JSON I/O over stdin/stdout |
| Extension size | ~1,500 LOC TypeScript (rendering only, all logic in Rust) |

### 3.14 Workspace Config

| Feature | Detail |
|---------|--------|
| Persistence | .coqu-di/workspace.yml |
| Content | File-to-schema mappings (data file path, copybook path, program path, label) |
| CLI management | schema add/list/remove subcommands |
| VS Code integration | loadConfig/saveConfig via stdio protocol |

---

## Feature Matrix: Cross-Project Comparison

### Parsing Capabilities

| Capability | cobol2rust | coqu | coqu-di |
|------------|------------|------|---------|
| ANTLR4 grammar | Yes (Rust) | Yes (Python) | Yes (Rust) |
| DATA DIVISION | Full | Full | Full |
| PROCEDURE DIVISION | Full (38+ verbs) | Structural (types + targets) | Discriminator patterns only |
| ENVIRONMENT DIVISION | Partial (FILE-CONTROL) | Full | FILE-CONTROL only |
| IDENTIFICATION DIVISION | PROGRAM-ID only | Full | PROGRAM-ID only |
| PIC parser | Full | Full | Full |
| COPY expansion | Yes (recursive, REPLACING) | Yes (recursive, REPLACING) | Yes (basic) |
| Fast indexer | No | Yes (regex, 2M+ lines) | No |
| AST caching | No | Yes (MessagePack) | No |
| Source format detection | Yes (fixed/free) | Yes (fixed/free/Panvalet) | No (assumes pre-processed) |

### Data Type Handling

| Type | cobol2rust | coqu | coqu-di |
|------|------------|------|---------|
| PIC X (alphanumeric) | Runtime type (PicX) | AST metadata | Binary decoder |
| PIC A (alphabetic) | Runtime type (PicA) | AST metadata | Binary decoder |
| PIC 9 (zoned decimal) | Runtime type (ZonedDecimal) | AST metadata | Binary decoder |
| COMP/COMP-4 (binary) | Runtime type (CompBinary) | AST metadata | Binary decoder |
| COMP-3 (packed decimal) | Runtime type (PackedDecimal) | AST metadata | Binary decoder |
| COMP-1 (float) | Runtime type (CobolFloat) | AST metadata | Binary decoder (IBM hex float) |
| COMP-2 (double) | Runtime type (CobolFloat) | AST metadata | Binary decoder (IBM hex float) |
| COMP-5 (native binary) | Runtime type (CompBinary) | AST metadata | Binary decoder |
| NumericEdited | Runtime type | AST metadata | Binary decoder |
| AlphaEdited | Runtime type | AST metadata | No |
| REDEFINES | Struct overlay | AST metadata | Full resolution + discriminator detection |
| OCCURS | Fixed + DEPENDING ON | AST metadata | Fixed + DEPENDING ON |
| 88-level conditions | Runtime evaluation | AST metadata | Discriminator hints |
| Group items | Struct nesting | AST hierarchy | Byte layout hierarchy |
| EBCDIC | CP037 tables | No | CP037 tables |

### Output/Interface

| Capability | cobol2rust | coqu | coqu-di |
|------------|------------|------|---------|
| CLI | Yes (clap) | Yes (argparse) | Yes (clap) |
| Interactive REPL | No | Yes (tab completion, history) | No |
| VS Code extension | No | No | Yes (custom editor) |
| TUI | No | No | Planned (ratatui) |
| JSON output | Yes (check, parse) | Yes (-o json) | Yes (decode, analyze, discover) |
| CSV output | No | No | Yes (view, export) |
| Text output | Yes (check, parse --tree) | Yes (default) | Yes (analyze) |
| Hex dump | No | No | Yes (view --raw) |
| Rust source output | Yes (transpile) | No | No |
| Script execution | No | Yes (.coqu scripts) | No |

### Analysis Capabilities

| Capability | cobol2rust | coqu | coqu-di |
|------------|------------|------|---------|
| Transpilation coverage | Yes (statement %, line numbers) | No | No |
| Parser coverage | No | Yes (parsed vs missed lines) | No |
| CALL graph extraction | Yes (AST-based) | Yes (calls command) | No |
| PERFORM graph | No | Yes (performs, where-used) | No |
| Cross-reference search | No | Yes (references, find) | No |
| Copybook dependency graph | No | Yes (copybook-deps) | Partial (for schema discovery) |
| REDEFINES analysis | No | No | Yes (groups, discriminators, confidence) |
| File-to-schema discovery | No | No | Yes (multi-signal matching) |
| Data file decoding | No | No | Yes (all COBOL types) |
| Migration readiness | Partial (coverage %) | Partial (inventory) | No |
| Program statistics | Yes (paragraphs, calls, file ops) | Yes (similar) | No |

---

## Shared Infrastructure (Duplicated Across Projects)

| Component | cobol2rust | coqu | coqu-di |
|-----------|------------|------|---------|
| ANTLR4 grammar (Cobol85.g4) | Copy | Copy | Copy |
| ANTLR4 generated code | Rust | Python | Rust |
| Data division listener | data_listener.rs | cobol_parser.py | data_listener.rs |
| Procedure division listener | proc_listener.rs | cobol_parser.py | proc_listener.rs |
| File section listener | file_listener.rs | (in cobol_parser.py) | file_listener.rs |
| Hierarchy builder | hierarchy.rs | (in cobol_parser.py) | hierarchy.rs |
| PIC parser | pic_parser.rs | (in cobol_parser.py) | pic.rs |
| Byte layout computation | data_gen.rs | No | layout.rs |
| Copybook resolution | copy_expand.rs, copybook.rs | preprocessor.py, copybook.py | (in parser) |
| EBCDIC CP037 tables | cobol-types/ebcdic.rs | No | decode.rs |
| Source preprocessor | preprocess.rs | preprocessor.py | No |
