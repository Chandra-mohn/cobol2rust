# cobol2rust Project & Corpus Spec

Unified pipeline for COBOL-to-Rust migration with checkpoint resumption.

## Overview

Two subcommands:
- `project` -- single project, full lifecycle (real client use case)
- `corpus` -- multi-repo orchestrator (assessment/bulk use case)

Both share the same phase engine. The pipeline is resumable -- each phase
checks if prior phases have completed and skips or continues accordingly.

## Configuration

Most options live in `.cobol2rust.toml` rather than on the command line.
CLI flags override config values when provided.

### `.cobol2rust.toml` format

```toml
[workspace]
copy_paths = ["copy", "src/copylib", "includes"]
# extensions = ["cbl", "cob", "cobol"]   # optional, default shown
# exclude = ["test/**"]                   # optional

[pipeline]
output = "./rust-out"              # Rust output directory
jobs = 8                           # parallel workers (default: num CPUs)
continue_on_error = true           # skip files that fail (default: true)
incremental = true                 # skip unchanged files (default: true)
# runtime_path = "../cobol-runtime"  # path to cobol-runtime crate
```

### Resolution Order

1. **Built-in defaults** -- sensible values for all options
2. **`.cobol2rust.toml`** -- project-level config (read from project root)
3. **CLI flags** -- override anything per invocation

### Defaults (when no config file and no CLI flags)

| Option             | Default          |
|--------------------|------------------|
| output             | `./rust-out`     |
| jobs               | num CPUs         |
| continue_on_error  | true             |
| incremental        | true             |
| phases             | all (0-5)        |
| extensions         | cbl, cob, cobol  |

## Pipeline Phases

```
Phase 0: Configure    -- discover copybooks, write/read .cobol2rust.toml
Phase 1: Inventory    -- parse all COBOL files, extract structure, CALL graph
Phase 2: Coverage     -- statement coverage analysis, verb inventory
Phase 3: Report       -- pre-transpile assessment report
Phase 4: Transpile    -- generate Rust workspace
Phase 5: Report       -- combined scan + transpile report
```

### Phase 0: Configure

Inputs: project directory
Outputs: `.cobol2rust.toml` in project root

Logic:
1. Check if `.cobol2rust.toml` exists
2. If yes: read it, use configured `copy_paths` -- done
3. If no: auto-discover copybook directories using heuristics:
   - Directories named `copy/`, `copylib/`, `cpy/`, `copybooks/`, `includes/` (case-insensitive)
   - Directories containing `.cpy` files
   - `.cbl`/`.cob` files without PROCEDURE DIVISION (data-only = copybook)
   - Directories referenced by `COPY member OF library` statements
4. Write discovered paths to `.cobol2rust.toml`
5. Log what was found

### Phase 1: Inventory

Inputs: project directory, copy_paths from Phase 0
Outputs: `parse_results.ndjson`, `files.ndjson`, `copybooks.ndjson`

Logic:
1. Discover all COBOL files (`.cbl`, `.cob`, `.cobol`)
2. Parse each file (parallel, `-j` workers)
3. Extract: program ID, CALL targets, LINKAGE, USING, copybook refs
4. Write per-file records to NDJSON

Checkpoint: if `parse_results.ndjson` exists with matching file count, skip.

### Phase 2: Coverage

Inputs: parsed files from Phase 1
Outputs: `coverage.ndjson`, `diagnostics.ndjson`

Logic:
1. For each parsed file, analyze statement coverage
2. Identify handled vs unhandled verbs
3. Write per-file coverage and diagnostic records

Checkpoint: if `coverage.ndjson` exists with matching file count, skip.

### Phase 3: Pre-Transpile Report

Inputs: Phase 1 + Phase 2 NDJSON data
Outputs: report to stdout/file

Logic:
1. Load NDJSON into DuckDB (in-memory)
2. Generate summary: file inventory, coverage distribution, complexity,
   verb frequency, copybook dependency graph, estimated transpile success rate

This is informational only -- no NDJSON output. Can be re-run at any time.

### Phase 4: Transpile

Inputs: project directory, copy_paths, Phase 1 workspace analysis
Outputs: Rust workspace in output dir, `transpile_results.ndjson`

Logic:
1. Run workspace analysis (reuse Phase 1 data if available)
2. Detect main vs lib programs (CALL graph)
3. Generate Cargo workspace structure
4. Transpile each file (parallel, `-j` workers)
5. Write per-file transpile records: success/failure, timing, coverage,
   verbs used, verbs unsupported, Rust line count

Checkpoint: if `transpile_results.ndjson` exists, use incremental mode to
skip files where output `.rs` is newer than source `.cbl`.

### Phase 5: Combined Report

Inputs: all NDJSON from Phase 1-4
Outputs: report to stdout/file

Logic:
1. Load all available NDJSON into DuckDB
2. Generate combined report: inventory + coverage + transpile results
3. Gracefully handle missing phases (report what's available)

## CLI Design

### `project` -- Single Project

```
cobol2rust project <project_dir> [options]

Arguments:
  <project_dir>          COBOL project root directory

Options:
  -o, --output <dir>     Override output directory
  -j, --jobs <n>         Override parallel workers
  --phase <phase>        Run specific phase: 0,1,2,3,4,5,all
  --from <phase>         Start from phase
  --to <phase>           Stop after phase
```

All other options come from `.cobol2rust.toml` or built-in defaults.

Examples:
```bash
# Full pipeline -- config file has output dir and jobs
cobol2rust project myproject/

# Override output dir for this run
cobol2rust project myproject/ -o ./different-output

# Scan only (Phase 0-3) -- assess before transpiling
cobol2rust project myproject/ --to 3

# Transpile only (assumes Phase 0-2 already done)
cobol2rust project myproject/ --from 4

# Just generate report from existing data
cobol2rust project myproject/ --phase 5
```

### `corpus` -- Multi-Repo Orchestrator

```
cobol2rust corpus <repos_dir> [options]

Arguments:
  <repos_dir>            Root containing repo subdirectories

Options:
  -o, --output <dir>     Override output root
  -j, --jobs <n>         Override total parallel workers
  --phase <phase>        Phase to run per repo: 0,1,2,3,4,5,all
  --from <phase>         Start from phase per repo
  --to <phase>           Stop after phase per repo
```

Per-repo options come from each repo's `.cobol2rust.toml`. The corpus
command only exposes orchestration-level overrides.

Examples:
```bash
# Full corpus pipeline
cobol2rust corpus repos/ -o ./data/output

# Scan-only assessment of entire corpus
cobol2rust corpus repos/ -o ./data/output --to 3

# Resume a partially completed run (incremental is default)
cobol2rust corpus repos/ -o ./data/output
```

## Repo Discovery (`corpus` mode)

The `corpus` subcommand discovers repos by walking `<repos_dir>`:

```
repos_dir/
  source_a/
    repo1/        <- independent project (has .cbl files)
    repo2/
  source_b/
    repo3/
```

Discovery logic:
1. Walk `<repos_dir>/<source>/<repo>/` (2 levels deep)
2. A directory is a repo if it contains `.cbl`/`.cob`/`.cobol` files
   (searched up to 5 levels deep within the repo)
3. Each repo is processed independently through the pipeline

## Checkpoint / Resume Logic

Each phase writes NDJSON to `<output>/reports/`. The pipeline detects
completed phases by checking for the existence and completeness of
output files:

| Phase | Checkpoint File          | Complete When                |
|-------|-------------------------|------------------------------|
| 0     | `.cobol2rust.toml`       | File exists in project root  |
| 1     | `parse_results.ndjson`   | Line count >= discovered files |
| 2     | `coverage.ndjson`        | Line count >= Phase 1 files  |
| 3     | (no output)              | Always re-runnable           |
| 4     | `transpile_results.ndjson` | Line count >= Phase 1 files |
| 5     | (no output)              | Always re-runnable           |

With incremental mode (default: on):
- Phase 1: skip files whose parse_results entry has mtime >= source mtime
- Phase 4: skip files whose output .rs has mtime >= source .cbl mtime

When a phase is skipped, the pipeline logs:
```
Phase 1: Inventory -- skipped (parse_results.ndjson exists, 45 files)
Phase 2: Coverage -- skipped (coverage.ndjson exists, 45 files)
Phase 4: Transpile -- running (23 files need update)
```

## NDJSON Output Structure

All phases write to `<output>/reports/`:

```
<output>/reports/
  scan_meta.json              -- run metadata (updated per phase)
  files.ndjson                -- file registry
  parse_results.ndjson        -- Phase 1: parse inventory
  copybooks.ndjson            -- Phase 1: copybook dependencies
  coverage.ndjson             -- Phase 2: statement coverage
  diagnostics.ndjson          -- Phase 2: parse warnings/errors
  transpile_results.ndjson    -- Phase 4: transpile outcomes
```

For `corpus` mode, each repo gets its own `reports/` directory under
`<output>/<source>/<repo>/reports/`. The orchestrator merges all per-repo
NDJSON into `<output>/reports/` (the merged directory).

The merge happens for ALL repos regardless of success/failure status,
so partial-success repos contribute their successful file records.

## Report Types

`cobol2rust report --scan-dir <dir> --type <type>` reads from NDJSON:

| Type       | Requires              | Description                    |
|------------|----------------------|--------------------------------|
| summary    | Phase 1              | File inventory, structure      |
| coverage   | Phase 2              | Statement coverage analysis    |
| errors     | Phase 2              | Parse errors and warnings      |
| complexity | Phase 1              | Program complexity metrics     |
| transpile  | Phase 4              | Transpile results, timing, verbs |
| full       | Any available        | All sections, skips missing data |

## Relationship to Existing Subcommands

The new subcommands unify existing functionality:

| Existing           | Replaced By              |
|--------------------|--------------------------|
| `scan --phase 1`   | `project --phase 1`     |
| `scan --phase 2`   | `project --phase 2`     |
| `scan --phase 3`   | `project --phase 3`     |
| `transpile --workspace` | `project --phase 4` |
| `status`           | `project --phase 5` (or `report`) |
| `report`           | remains (reads from any scan-dir) |
| `transpile_corpus.sh` | `corpus`              |
| `discover_copybooks.sh` | `project --phase 0` / `corpus` Phase 0 |

Existing subcommands remain for backwards compatibility. The `project`
and `corpus` subcommands are the recommended entry points.

## Implementation Plan

### Stage 1: Project subcommand (per-repo)
1. Add `project` subcommand to clap
2. Implement config file loading with CLI override merging
3. Implement phase dispatcher with checkpoint detection
4. Port Phase 0 (copybook discovery) from shell to Rust
5. Wire existing scan Phase 1, 2 to pipeline phases
6. Wire existing transpile to pipeline Phase 4
7. Add combined Phase 5 report

### Stage 2: Corpus subcommand (multi-repo)
1. Add `corpus` subcommand to clap
2. Implement repo discovery (walk repos_dir 2 levels deep)
3. Per-repo pipeline dispatch (parallel across repos)
4. NDJSON merge from all repos (always, not just successful)
5. Aggregate reporting

### Stage 3: Polish
1. Progress bars (per-phase, per-repo in corpus mode)
2. Resume/interrupt handling (Ctrl-C saves state cleanly)
3. Timing summary per phase
4. Log file output
