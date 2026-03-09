# COBOL Migration Workbench -- UI/UX Brainstorm

Status: BRAINSTORM (under consideration)
Date: 2026-03-08

---

## 1. Vision

A **purpose-built COBOL migration workbench** -- not an IDE, not competing with
VS Code/IntelliJ/RustRover. A high-performing UI that provides good UX to use the
solid backends already built across three projects:

- **cobol2rust** (Rust): COBOL-to-Rust transpiler + runtime library
- **coqu** (Python): COBOL parser + interactive query engine
- **coqu-di** (Rust): Binary data file inspector + REDEFINES resolver

The code editor is a **supporting component** (read-mostly viewer with syntax
highlighting and folding), not the centerpiece. The real value is in the analysis
viewers, data inspection screens, and workflow orchestration.

---

## 2. Requirements Summary

### 2.1 Core Requirements

| Requirement | Detail |
|-------------|--------|
| Code viewing | COBOL + Rust syntax highlighting, folding, read-mostly |
| Analysis viewers | Trees, tables, graphs, coverage reports, query results |
| Data inspection | Hex dump, decoded record tables, field-level inspection |
| Batch operations | Transpile, discover, decode, export -- async with progress |
| Cross-platform | Windows, macOS, Linux |
| Performance | TB-scale data, 32K+ character records, proper windowing |
| Parallel execution | Multiple jobs running concurrently |

### 2.2 Non-Goals

- Full IDE editing experience (autocomplete, refactoring, debugging)
- Competing with VS Code / IntelliJ feature sets
- Server/team deployment (local-only for now)

### 2.3 Data Scale Constraints

- Data files in multiple terabytes
- Record lengths exceeding 32,000 characters
- Handled through windowed/batched access (pattern proven in coqu-di)
- Frontend never holds more than one window of records in memory

---

## 3. Platform Decision

### 3.1 Options Evaluated

#### Zed Editor (REJECTED)

- Extension API too immature -- no custom editors, no webview panels, no tree views
- GPUI framework not exposed to extension authors
- Would require forking Zed itself for custom panels
- COBOL language support would need to be built from scratch

#### VS Code Extension Suite (CONSIDERED)

- Pros: Existing coqu-di extension, mature extension API, webview panels
- Cons: Constrained by VS Code's extension model, TypeScript-heavy

#### Tauri + Svelte (RECOMMENDED)

- Rust backend with direct crate integration (no subprocess/stdio bridge)
- Svelte frontend as thin rendering shell -- zero business logic in TS
- CodeMirror 6 for code viewing
- Cross-platform, small binary (~10MB)
- Full layout control for all viewer types

#### egui (CONSIDERED)

- Pure Rust, GPU-accelerated
- Table/grid widgets less mature than web equivalents
- Text editor widgets exist but aren't CodeMirror-quality

#### Slint (CONSIDERED)

- Declarative markup + Rust logic
- Smaller ecosystem for rich data viewers
- Commercial license required for closed-source

#### Leptos/Dioxus (CONSIDERED)

- Rust-authored web UI compiling to WASM
- JS interop still needed for editor components
- Smaller community

### 3.2 Decision Rationale

**Tauri 2.0 + Svelte 5** selected because:

1. Direct Rust FFI to existing crates (no subprocess overhead)
2. Web ecosystem excels at the data-heavy viewers needed (tables, trees, grids)
3. CodeMirror 6 provides adequate code viewing without full IDE weight
4. Cross-platform out of the box
5. Clear separation: all logic in Rust, all rendering in Svelte

---

## 4. Architecture

### 4.1 Layer Diagram

```
+------------------------------------------+
|  Svelte 5 / TypeScript (thin shell)      |
|  - Component rendering                   |
|  - Event binding                         |
|  - Layout / navigation / routing         |
|  - Display formatting ONLY               |
|  - ZERO business logic                   |
+------------------------------------------+
          |  Tauri invoke (commands down)
          |  Tauri events  (events up)
          v
+------------------------------------------+
|  Tauri Rust layer (command handlers)      |
|  - Command routing                       |
|  - Session / state management            |
|  - Job orchestration (JobManager)        |
|  - Event emission to frontend            |
+------------------------------------------+
          |  Direct crate calls
          v
+------------------------------------------+
|  Consolidated Rust backend               |
|  cobol2rust | coqu-core | coqu-di-core   |
+------------------------------------------+
```

### 4.2 Frontend Principle

The frontend is a **rendering-only shell**. It:
- Sends commands to Rust via `invoke()`
- Listens for events via `listen()`
- Renders received data into components
- Manages only UI state (selected tab, scroll position, panel sizes)
- Contains NO parsing, no data transformation, no business rules

### 4.3 Technology Stack

| Layer | Technology | Role |
|-------|-----------|------|
| Shell | Tauri 2.0 | Window management, IPC, cross-platform |
| Rendering | Svelte 5 | Reactive UI components |
| Code viewing | CodeMirror 6 | Syntax highlighting, folding, read-only |
| Data grids | TanStack Table + virtual scroll | Large record display |
| Graphs | elkjs or d3-dag | Call graphs, dependency trees |
| Styling | Tailwind CSS | Consistent, utility-first |
| State (frontend) | Svelte stores | UI state only |
| IPC | Tauri invoke + events | Commands down, events up |
| Backend | Consolidated Rust crates | All logic |
| Async runtime | tokio | Job execution, parallelism |
| Cancellation | tokio-util CancellationToken | Job abort |

---

## 5. Async Job System

### 5.1 Pattern

All backend operations are async jobs. The frontend never blocks waiting for results.

```
Frontend                        Rust Backend
--------                        ------------
invoke("submit_job", {          --> JobManager.submit(spec)
  type: "transpile",                --> spawns tokio task
  params: {...}                     --> returns job_id immediately
}) --> job_id

listen("job:progress",          <-- JobManager emits events
  {job_id, percent,                 via Tauri app_handle
   stage, message})

listen("job:complete",          <-- Final result or error
  {job_id, result})

invoke("cancel_job",            --> JobManager.cancel(job_id)
  {job_id})                         --> signals CancellationToken
```

### 5.2 Job Types

| Job Type | Backend Source | Output | Parallelizable? |
|----------|--------------|--------|-----------------|
| transpile | cobol2rust | Rust source + diagnostics | Yes (per file) |
| check_coverage | cobol2rust | Coverage report | Yes (per file) |
| parse_ast | cobol2rust / coqu | AST (JSON) | Yes (per file) |
| discover_schemas | coqu-di | Schema mappings | Yes (per directory) |
| decode_records | coqu-di | Record batches (windowed) | Sequential (windowed) |
| analyze_layout | coqu-di | Record layout tree | Single |
| query | coqu | Query results | Yes (multiple queries) |
| batch_transpile | cobol2rust | Workspace build | Dependency-ordered |

### 5.3 Job Manager Design

```
JobManager
  |
  +-- JobQueue (bounded channel)
  |
  +-- Worker pool (tokio::spawn, configurable concurrency)
  |     +-- Worker 1: transpile program_a.cbl
  |     +-- Worker 2: transpile program_b.cbl
  |     +-- Worker 3: decode dataset_x.dat
  |     +-- Worker 4: discover schemas in /data/
  |
  +-- Progress aggregator
  |     +-- Per-job progress events
  |     +-- Overall batch progress
  |
  +-- Result store (completed job results, queryable)
```

Design decisions:
- Every job gets a CancellationToken (tokio-util) for UI-initiated abort
- Bounded concurrency: configurable worker count, default to num_cpus
- Memory-safe windowing: backend never holds more than one window in memory
- Job dependencies: batch transpile respects CALL graph ordering

---

## 6. Data Flow for Large Datasets

### 6.1 Windowed Access (proven in coqu-di)

```
Frontend: "show records 1000-1099 of dataset.dat"
    |
    invoke("decode_window", {file, schema, offset: 1000, limit: 100})
    |
    Rust: seek to record 1000, decode 100 records, return JSON
    |
    Frontend: render table (100 rows only, virtual scroll indicators)
```

### 6.2 Wide Records (32K+ characters)

- Backend returns field-level decoded data (not raw strings)
- Frontend renders a scrollable field table, not one giant row
- Horizontal virtualization -- only render visible columns
- Field details on click/hover (PIC, USAGE, offset, raw bytes)

---

## 7. Screen Inventory

### 7.1 Navigation Layout

```
Sidebar (project tree)          Main Area (tabbed panels)
+-- Workspace                   +-- [Tab: program_a.cbl]
|   +-- Programs                |   +-- Code View (COBOL)
|   |   +-- program_a.cbl      |   +-- Structure Panel
|   |   +-- program_b.cbl      |   +-- Coverage Overlay
|   +-- Copybooks               +-- [Tab: Transpile Results]
|   |   +-- copy1.cpy          |   +-- Side-by-side COBOL/Rust
|   +-- Data Files              |   +-- Diagnostics list
|   |   +-- dataset.dat         +-- [Tab: dataset.dat]
|   +-- Schemas                 |   +-- Record Table (decoded)
|       +-- layout.cpy          |   +-- Hex View toggle
                                +-- [Tab: Batch Jobs]
Bottom Panel                    |   +-- Job list + progress
+-- Job Monitor                 +-- [Tab: Analysis]
+-- Console / Log                   +-- Call graph / coverage
```

### 7.2 Screen Catalog

| Screen | Primary Widget | Data Source | Category |
|--------|---------------|-------------|----------|
| COBOL Viewer | CodeMirror 6 (read-only, COBOL highlighting) | File read | Code |
| Rust Viewer | CodeMirror 6 (Rust mode) | Transpile output | Code |
| Side-by-side | Dual CodeMirror, synced scroll | Transpile job | Code |
| Structure Browser | Tree view (divisions/sections/paragraphs) | parse_ast job | Analysis |
| Data Item Explorer | Hierarchical table (level/PIC/offset) | analyze_layout job | Analysis |
| Record Table | Virtual-scroll data grid | decode_window (paginated) | Data |
| Hex Viewer | Hex + ASCII grid with field highlighting | read_raw_window | Data |
| Coverage Report | Code view + line annotations | check_coverage job | Analysis |
| Call Graph | DAG visualization (d3-dag or elkjs) | parse_ast job | Analysis |
| Schema Discovery | Card list with confidence badges | discover_schemas job | Workflow |
| Copybook Dependencies | Tree / DAG | query (copybook-deps) | Analysis |
| Job Monitor | Table with progress bars | JobManager state | System |
| Query Console | Input + results table (coqu REPL equivalent) | query job | Interactive |
| Workspace Settings | Forms (copybook paths, encoding, schemas) | Config read/write | System |
| Transpile Dashboard | File list + status + coverage % | Batch transpile | Workflow |
| Diagnostics Panel | Filterable list (severity, category, line) | Transpile diagnostics | Analysis |

### 7.3 Screens by Source Project

**From cobol2rust:**
- COBOL Viewer (source input)
- Rust Viewer (transpile output)
- Side-by-side Viewer (COBOL vs generated Rust)
- Coverage Report (transpilation coverage with line numbers)
- Transpile Dashboard (batch transpile status)
- Diagnostics Panel (errors, warnings, unsupported features)

**From coqu:**
- Structure Browser (divisions, sections, paragraphs)
- Query Console (21 built-in commands as interactive UI)
- Call Graph (CALL + PERFORM targets)
- Copybook Dependencies (resolution chains)
- Cross-reference Search (find, references, where-used)

**From coqu-di:**
- Record Table (decoded binary data, paginated)
- Hex Viewer (raw hex + ASCII with field overlay)
- Data Item Explorer (record layout hierarchy)
- Schema Discovery (auto-match data files to copybooks)
- Workspace Config (schema mappings management)

---

## 8. Backend Consolidation

### 8.1 Current Duplication

The three projects duplicate significant infrastructure:

| Component | cobol2rust | coqu | coqu-di |
|-----------|-----------|------|---------|
| ANTLR4 grammar (Cobol85.g4) | Copy | Copy | Copy |
| Data division listener | data_listener.rs | cobol_parser.py | data_listener.rs |
| Procedure division listener | proc_listener.rs | cobol_parser.py | proc_listener.rs |
| Hierarchy builder | hierarchy.rs | (in cobol_parser.py) | hierarchy.rs |
| PIC parser | pic_parser.rs | (in cobol_parser.py) | pic.rs |
| Byte layout computation | data_gen.rs | No | layout.rs |
| EBCDIC CP037 tables | ebcdic.rs | No | decode.rs |

### 8.2 Consolidation Strategy (to be decided)

**Option A: Consolidate before GUI**
- Create shared `cobol-parser-core` crate
- Single parsing interface for the GUI
- Cleaner Tauri command surface
- More upfront work before any visible progress

**Option B: GUI first, consolidate later**
- Build GUI against existing three CLIs (subprocess/JSON)
- Migrate to direct crate calls incrementally
- Visible progress sooner
- Temporary subprocess bridge adds complexity

**Option C: Hybrid**
- Build Tauri shell with job manager first
- Start with coqu-di (already has Rust crate API: coqu-di-viewer)
- Add cobol2rust next (Rust, direct integration)
- Add coqu last (Python -- needs subprocess bridge or Rust port)
- Consolidate shared parsing as you integrate

---

## 9. Open Questions

1. **Project name?** Needs an identity.
   Candidates: cobol-workbench, cobol-forge, coqu-studio, cobol-lens, ...

2. **Consolidation timing?** Option A, B, or C above.

3. **Priority screens?** Which 3-4 screens to build first to validate the approach?
   Suggested candidates for first milestone:
   - Job Monitor (validates async architecture)
   - Record Table / Hex Viewer (validates data windowing)
   - COBOL Viewer (validates CodeMirror integration)
   - Side-by-side Viewer (validates transpile workflow)

4. **coqu integration path?** coqu is Python-based. Options:
   - Subprocess bridge (coqu CLI with JSON output) -- simplest
   - Port coqu query engine to Rust -- cleanest but significant effort
   - Python embedding (pyo3) -- possible but adds complexity

5. **Offline-only or future server mode?** All local for now, but if team/server
   deployment is ever planned, the job manager could become a service.

6. **Theming?** Dark/light mode? Custom branding?

7. **Keyboard-driven workflow?** Command palette (Ctrl+P style)?
   Keyboard shortcuts for navigation?

---

## 10. Reference: Feature Inventory

Full feature catalog in: `cobol_tools_feature_inventory.md`

Key numbers:
- cobol2rust: 38+ COBOL verbs, 10 runtime type modules, 5 CLI subcommands
- coqu: 21 query commands, REPL with meta-commands, AST caching
- coqu-di: 7 CLI subcommands, 11 data decoders, 4-layer architecture, VS Code extension
- 799 tests passing across cobol2rust
