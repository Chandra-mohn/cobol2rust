# cobol2rust scan -- Enterprise Codebase Scanner Specification

## Overview

`cobol2rust scan` is an eighth CLI subcommand that performs large-scale inventory,
coverage analysis, and reporting across enterprise COBOL codebases. It is designed
for a target of 269,477 COBOL files (149M lines) and 10,874 JCL files on a 20+
core machine with local SSD storage.

Unlike `check` (which operates on explicit file lists), `scan` recursively discovers
files, persists all results to a DuckDB database, and produces aggregated reports.
Every file result is committed immediately, making the scan resumable after
interruption.

**Design principles:**

- Never lose work: DuckDB persistence after every batch
- Never re-do work: incremental mode skips unchanged files
- Never block the user: `--status` queries progress without waiting
- Scale linearly: rayon parallelism across 20+ cores

---

## CLI Interface

```
cobol2rust scan <root-dir> [options]
```

### Positional Arguments

| Argument | Description |
|---|---|
| `<root-dir>` | Root directory to scan recursively |

### Phase Control

| Flag | Description |
|---|---|
| `--phase <1\|2\|3\|all>` | Which phase to run (default: `all`) |

Phase descriptions:

- **Phase 1 -- Inventory**: Parse each file, collect stats (program-id, format,
  paragraphs, calls, file-ops, SQL statements, subprogram detection, copybook
  references). Fast: parse-only, no transpilation.
- **Phase 2 -- Coverage**: Run transpilation coverage analysis on files that passed
  Phase 1. Produces statement counts, mapped counts, coverage percentage, and
  unhandled construct diagnostics.
- **Phase 3 -- Reporting**: Query the DuckDB database and produce aggregated reports.
  No file I/O except database reads.

When `--phase all` (default), phases run sequentially: 1 -> 2 -> 3.

### Database

| Flag | Description |
|---|---|
| `--db <path>` | DuckDB database file path (default: `./cobol_scan.duckdb`) |

### Copybook Resolution

| Flag | Description |
|---|---|
| `--copy-path <dir>` | Copybook search directory (repeatable, in addition to auto-discovery) |

This flag supplements the global `-C / --copybook-path` flag. Both are merged.

### Parallelism

| Flag | Short | Description |
|---|---|---|
| `--jobs <n>` | `-j` | Number of parallel workers (default: `num_cpus::get()`) |

### Incremental and Resume

| Flag | Description |
|---|---|
| `--incremental` | Skip files whose mtime matches the stored mtime from last scan |
| `--resume` | Resume an interrupted scan (implies `--incremental`) |

Difference between `--incremental` and `--resume`:

- `--incremental` creates a new `scan_runs` row but skips unchanged files.
- `--resume` reuses the most recent incomplete `scan_runs` row and only processes
  files that have no result for that run.

### Status Query

| Flag | Description |
|---|---|
| `--status` | Print current scan progress from the database and exit immediately |

### Reporting

| Flag | Description |
|---|---|
| `--report <type>` | Report type: `summary`, `coverage`, `errors`, `complexity`, `full` |
| `--format <fmt>` | Report output format: `text` (default), `json`, `csv` |

When `--report` is specified without `--phase`, it implies `--phase 3` (report-only).

### Batch Control

| Flag | Description |
|---|---|
| `--batch-size <n>` | Files per DuckDB transaction commit (default: 100) |

Larger values reduce transaction overhead but increase data-at-risk on crash.

### File Filtering

| Flag | Description |
|---|---|
| `--exclude <pattern>` | Glob pattern to exclude (repeatable). Applied to relative path from root-dir. |
| `--extensions <ext,...>` | Override file extensions (default: `cbl,cob,cobol,CBL,COB,COBOL`) |

### Full Example

```bash
# Full scan with 24 workers, custom DB location
cobol2rust scan /data/cobol-mainframe \
  --db /data/scan_results.duckdb \
  --copy-path /data/copybooks \
  --copy-path /data/shared/copy \
  -j 24 \
  --batch-size 200

# Check progress while scan is running (separate terminal)
cobol2rust scan /data/cobol-mainframe \
  --db /data/scan_results.duckdb \
  --status

# Resume after interruption
cobol2rust scan /data/cobol-mainframe \
  --db /data/scan_results.duckdb \
  --resume

# Run coverage only (Phase 1 already done)
cobol2rust scan /data/cobol-mainframe \
  --db /data/scan_results.duckdb \
  --phase 2

# Generate report only
cobol2rust scan /data/cobol-mainframe \
  --db /data/scan_results.duckdb \
  --report full --format json
```

---

## DuckDB Schema

All tables live in the database file specified by `--db`. The schema is created
on first use via `CREATE TABLE IF NOT EXISTS`.

### scan_runs

Tracks each invocation of the scanner.

```sql
CREATE TABLE IF NOT EXISTS scan_runs (
    run_id          INTEGER PRIMARY KEY,
    started_at      TIMESTAMP NOT NULL DEFAULT current_timestamp,
    finished_at     TIMESTAMP,
    root_dir        VARCHAR NOT NULL,
    phase           VARCHAR NOT NULL,           -- '1', '2', '3', 'all'
    status          VARCHAR NOT NULL DEFAULT 'running',  -- 'running', 'completed', 'interrupted'
    total_files     INTEGER DEFAULT 0,
    processed_files INTEGER DEFAULT 0,
    skipped_files   INTEGER DEFAULT 0,
    failed_files    INTEGER DEFAULT 0,
    config_json     VARCHAR,                    -- serialized scan config for reproducibility
    worker_count    INTEGER NOT NULL,
    batch_size      INTEGER NOT NULL,
    incremental     BOOLEAN NOT NULL DEFAULT false
);

CREATE SEQUENCE IF NOT EXISTS scan_runs_seq START 1;
```

### files

Master file inventory. One row per unique file path. Updated each scan with
current metadata.

```sql
CREATE TABLE IF NOT EXISTS files (
    file_id         INTEGER PRIMARY KEY,
    path            VARCHAR NOT NULL UNIQUE,     -- relative to root_dir
    absolute_path   VARCHAR NOT NULL,
    extension       VARCHAR NOT NULL,            -- 'cbl', 'cob', 'cobol', 'cpy', 'jcl'
    file_size       BIGINT NOT NULL,
    mtime           TIMESTAMP NOT NULL,
    first_seen_run  INTEGER NOT NULL REFERENCES scan_runs(run_id),
    last_scan_run   INTEGER NOT NULL REFERENCES scan_runs(run_id),
    file_type       VARCHAR NOT NULL DEFAULT 'source',  -- 'source', 'copybook', 'jcl', 'unknown'
    status          VARCHAR NOT NULL DEFAULT 'pending'   -- 'pending', 'parsed', 'covered', 'failed'
);

CREATE SEQUENCE IF NOT EXISTS files_seq START 1;
CREATE INDEX IF NOT EXISTS idx_files_status ON files(status);
CREATE INDEX IF NOT EXISTS idx_files_extension ON files(extension);
CREATE INDEX IF NOT EXISTS idx_files_type ON files(file_type);
```

### parse_results

Phase 1 output. One row per file per scan run.

```sql
CREATE TABLE IF NOT EXISTS parse_results (
    id              INTEGER PRIMARY KEY,
    file_id         INTEGER NOT NULL REFERENCES files(file_id),
    run_id          INTEGER NOT NULL REFERENCES scan_runs(run_id),
    program_id      VARCHAR NOT NULL,
    source_format   VARCHAR NOT NULL,           -- 'fixed', 'free'
    valid           BOOLEAN NOT NULL,
    line_count      INTEGER NOT NULL,
    paragraphs      INTEGER NOT NULL DEFAULT 0,
    sections        INTEGER NOT NULL DEFAULT 0,
    calls           INTEGER NOT NULL DEFAULT 0,
    file_ops        INTEGER NOT NULL DEFAULT 0,
    sql_statements  INTEGER NOT NULL DEFAULT 0,
    is_subprogram   BOOLEAN NOT NULL DEFAULT false,
    has_linkage     BOOLEAN NOT NULL DEFAULT false,
    has_using       BOOLEAN NOT NULL DEFAULT false,
    data_items      INTEGER NOT NULL DEFAULT 0,
    error_count     INTEGER NOT NULL DEFAULT 0,
    warning_count   INTEGER NOT NULL DEFAULT 0,
    parse_time_ms   INTEGER NOT NULL DEFAULT 0,
    UNIQUE(file_id, run_id)
);

CREATE SEQUENCE IF NOT EXISTS parse_results_seq START 1;
CREATE INDEX IF NOT EXISTS idx_parse_results_run ON parse_results(run_id);
CREATE INDEX IF NOT EXISTS idx_parse_results_file ON parse_results(file_id);
CREATE INDEX IF NOT EXISTS idx_parse_results_valid ON parse_results(valid);
```

### diagnostics

Errors and warnings from both Phase 1 (parse) and Phase 2 (coverage).

```sql
CREATE TABLE IF NOT EXISTS diagnostics (
    id              INTEGER PRIMARY KEY,
    file_id         INTEGER NOT NULL REFERENCES files(file_id),
    run_id          INTEGER NOT NULL REFERENCES scan_runs(run_id),
    phase           INTEGER NOT NULL,           -- 1 or 2
    severity        VARCHAR NOT NULL,           -- 'error', 'warning', 'info'
    line            INTEGER,                    -- NULL if not line-specific
    code            VARCHAR NOT NULL,           -- 'E001', 'W001', 'C-ERR', etc.
    message         VARCHAR NOT NULL,
    category        VARCHAR                     -- 'parse', 'coverage', 'unsupported'
);

CREATE SEQUENCE IF NOT EXISTS diagnostics_seq START 1;
CREATE INDEX IF NOT EXISTS idx_diagnostics_file ON diagnostics(file_id);
CREATE INDEX IF NOT EXISTS idx_diagnostics_run ON diagnostics(run_id);
CREATE INDEX IF NOT EXISTS idx_diagnostics_code ON diagnostics(code);
CREATE INDEX IF NOT EXISTS idx_diagnostics_severity ON diagnostics(severity);
```

### coverage_results

Phase 2 output. One row per file per scan run.

```sql
CREATE TABLE IF NOT EXISTS coverage_results (
    id                  INTEGER PRIMARY KEY,
    file_id             INTEGER NOT NULL REFERENCES files(file_id),
    run_id              INTEGER NOT NULL REFERENCES scan_runs(run_id),
    total_statements    INTEGER NOT NULL,
    mapped_statements   INTEGER NOT NULL,
    coverage_pct        DOUBLE NOT NULL,
    total_data_entries  INTEGER NOT NULL,
    unhandled_count     INTEGER NOT NULL DEFAULT 0,
    coverage_time_ms    INTEGER NOT NULL DEFAULT 0,
    UNIQUE(file_id, run_id)
);

CREATE SEQUENCE IF NOT EXISTS coverage_results_seq START 1;
CREATE INDEX IF NOT EXISTS idx_coverage_run ON coverage_results(run_id);
CREATE INDEX IF NOT EXISTS idx_coverage_file ON coverage_results(file_id);
CREATE INDEX IF NOT EXISTS idx_coverage_pct ON coverage_results(coverage_pct);
```

### copybooks

Copybook cross-reference: which programs reference which copybooks, and
where those copybooks resolved to on disk.

```sql
CREATE TABLE IF NOT EXISTS copybooks (
    id              INTEGER PRIMARY KEY,
    run_id          INTEGER NOT NULL REFERENCES scan_runs(run_id),
    name            VARCHAR NOT NULL,           -- COPY target name (e.g., 'EMPLOYEE-COPY')
    resolved_path   VARCHAR,                    -- NULL if unresolved
    referenced_by   INTEGER NOT NULL REFERENCES files(file_id),
    resolved        BOOLEAN NOT NULL DEFAULT false
);

CREATE SEQUENCE IF NOT EXISTS copybooks_seq START 1;
CREATE INDEX IF NOT EXISTS idx_copybooks_name ON copybooks(name);
CREATE INDEX IF NOT EXISTS idx_copybooks_run ON copybooks(run_id);
CREATE INDEX IF NOT EXISTS idx_copybooks_ref ON copybooks(referenced_by);
```

### call_graph

Inter-program CALL relationships extracted during Phase 1.

```sql
CREATE TABLE IF NOT EXISTS call_graph (
    id              INTEGER PRIMARY KEY,
    run_id          INTEGER NOT NULL REFERENCES scan_runs(run_id),
    caller_file_id  INTEGER NOT NULL REFERENCES files(file_id),
    callee_name     VARCHAR NOT NULL,           -- target program name from CALL literal
    callee_file_id  INTEGER REFERENCES files(file_id),  -- NULL if target not found in scan
    is_dynamic      BOOLEAN NOT NULL DEFAULT false       -- true if CALL identifier (not literal)
);

CREATE SEQUENCE IF NOT EXISTS call_graph_seq START 1;
CREATE INDEX IF NOT EXISTS idx_call_graph_run ON call_graph(run_id);
CREATE INDEX IF NOT EXISTS idx_call_graph_caller ON call_graph(caller_file_id);
CREATE INDEX IF NOT EXISTS idx_call_graph_callee ON call_graph(callee_name);
```

---

## Phase Execution Flow

### Phase 1: Inventory (Parse)

```
1. Create scan_runs row (status='running')
2. Discover files recursively under root-dir
3. Insert/update files table (new files get 'pending', existing files update mtime)
4. Build work queue:
   - If --resume: SELECT file_id FROM files WHERE status = 'pending'
                  AND file_id NOT IN (SELECT file_id FROM parse_results WHERE run_id = <run_id>)
   - If --incremental: exclude files where files.mtime = stored mtime AND status IN ('parsed','covered')
   - Otherwise: all source files
5. Process work queue in parallel (rayon):
   a. Read file from disk
   b. detect_source_format()
   c. parse_cobol()
   d. collect_stats() (paragraphs, calls, file_ops, sql_statements, is_subprogram)
   e. extract_copy_targets()
   f. Build FileResult (reusing check.rs logic)
6. Batch-commit results to DuckDB every --batch-size files:
   a. INSERT INTO parse_results
   b. INSERT INTO diagnostics (for errors/warnings)
   c. INSERT INTO copybooks (for COPY targets)
   d. INSERT INTO call_graph (for CALL statements)
   e. UPDATE files SET status = 'parsed' (or 'failed'), last_scan_run = <run_id>
   f. UPDATE scan_runs SET processed_files = processed_files + batch_count
7. After all files: UPDATE scan_runs SET status='completed', finished_at=now()
```

### Phase 2: Coverage Analysis

```
1. If no Phase 1 for this run: error "run Phase 1 first"
   (unless --resume finds an existing run with Phase 1 complete)
2. Build work queue: files WHERE status = 'parsed' AND valid = true
   AND file_id NOT IN (SELECT file_id FROM coverage_results WHERE run_id = <run_id>)
3. Process work queue in parallel:
   a. Read file from disk
   b. transpile_with_diagnostics()
   c. Collect CoverageInfo
4. Batch-commit:
   a. INSERT INTO coverage_results
   b. INSERT INTO diagnostics (phase=2, for unhandled constructs)
   c. UPDATE files SET status = 'covered'
   d. UPDATE scan_runs counters
5. Finalize scan_runs row
```

### Phase 3: Reporting

```
1. Validate database has results (error if empty)
2. Determine run_id to report on:
   - Default: most recent completed run
   - Could add --run-id flag for historical comparison
3. Execute report queries (see Report Queries section)
4. Format output (text/json/csv)
5. Write to stdout
```

### Resumability Logic

The key invariant: each file result is committed to DuckDB within its batch
transaction. On crash, the database is consistent up to the last committed batch.

Resume algorithm:

```
1. Find most recent scan_runs row with status != 'completed'
2. If none found: error "nothing to resume"
3. Determine which phase was interrupted:
   - If processed_files < total_files for Phase 1: resume Phase 1
   - If Phase 1 complete but Phase 2 incomplete: resume Phase 2
4. Build work queue excluding already-processed files
5. Continue processing
6. On completion: update scan_runs status to 'completed'
```

---

## Parallelism Strategy

### Architecture

```
Main thread                 Worker pool (rayon)            DuckDB writer thread
-----------                 --------------------           -------------------
discover files
  |
build work queue
  |
partition into chunks       <-- par_iter() -->
  |                         process file 1
  |                         process file 2
  |                         process file 3
  |                              |
  |                         send batch via channel  -----> receive batch
  |                                                        BEGIN TRANSACTION
  |                                                        INSERT rows
  |                                                        COMMIT
  |                                                        update progress
  |
  |                         process file 4
  |                         ...
  |
wait for completion
```

### Key Design Decisions

1. **Single DuckDB writer thread**: DuckDB supports concurrent reads but writes
   are serialized internally. Rather than contend on the write lock from N worker
   threads, use a dedicated writer thread that receives batches via a bounded
   crossbeam channel. This eliminates lock contention and allows workers to
   continue processing while writes happen.

2. **Rayon par_iter for CPU work**: File parsing and transpilation are CPU-bound.
   Rayon's work-stealing scheduler distributes these evenly across `--jobs` threads.

3. **Batch commits**: Accumulate `--batch-size` results before sending to the
   writer thread. This amortizes transaction overhead (DuckDB transaction cost is
   non-trivial for single-row inserts).

4. **Progress tracking**: The writer thread updates an `AtomicU64` counter after
   each batch commit. The progress bar (indicatif) reads this counter. The
   `--status` flag reads `scan_runs.processed_files` from the database.

5. **Memory budget**: Each FileResult is small (a few KB). With 100-file batches,
   the channel buffer holds at most 2-3 batches (configurable), keeping memory
   under ~10MB regardless of corpus size.

### Thread Count Guidance

| Phase | Bottleneck | Recommended --jobs |
|---|---|---|
| Phase 1 (parse) | CPU (parsing) | num_cpus (default) |
| Phase 2 (coverage) | CPU (transpilation) | num_cpus (default) |
| Phase 3 (report) | I/O (DuckDB queries) | 1 (single-threaded) |

---

## Copybook Discovery Algorithm

Copybooks are resolved for two purposes:

1. **Inventory**: Record which copybooks each program references (Phase 1)
2. **Coverage**: Provide copybook content to the transpiler for accurate
   coverage analysis (Phase 2)

### Search Order

For a COPY statement `COPY EMPLOYEE-COPY`, search in this order:

```
1. Explicit --copy-path directories (left to right)
2. Global -C / --copybook-path directories (left to right)
3. Same directory as the source file
4. Sibling directories of the source file's parent:
   - ../copybooks/
   - ../copy/
   - ../cpy/
   - ../COPYBOOKS/
   - ../COPY/
   - ../CPY/
5. Well-known subdirectories under root-dir:
   - <root>/copybooks/
   - <root>/copy/
   - <root>/cpy/
   - <root>/COPYBOOKS/
   - <root>/COPY/
   - <root>/CPY/
   - <root>/COPYLIB/
   - <root>/copylib/
```

### File Name Matching

For `COPY EMPLOYEE-COPY`, try these filenames in each search directory:

```
1. EMPLOYEE-COPY.cpy
2. EMPLOYEE-COPY.CPY
3. EMPLOYEE-COPY.cbl
4. EMPLOYEE-COPY.CBL
5. EMPLOYEE-COPY.copy
6. EMPLOYEE-COPY.COPY
7. EMPLOYEE-COPY         (no extension)
8. employee-copy.cpy     (lowercase)
9. employee_copy.cpy     (underscored)
```

### Caching

Build a copybook resolution cache at scan start:

```
1. Walk all --copy-path and auto-discovered copybook directories
2. Build HashMap<String, PathBuf> mapping uppercase names to paths
3. Share cache (Arc<HashMap>) across worker threads
4. Cache is read-only during scan -- no synchronization needed
```

### Recording

For each resolved copybook, insert into the `copybooks` table:
- `name`: the COPY target as written
- `resolved_path`: absolute path if found, NULL if not
- `referenced_by`: file_id of the referencing program
- `resolved`: true/false

---

## File Discovery Logic

### Extensions

Default extensions (case-insensitive matching):

| Extension | File Type |
|---|---|
| `.cbl`, `.cob`, `.cobol` | `source` |
| `.cpy`, `.copy`, `.cpylib` | `copybook` |
| `.jcl`, `.proc` | `jcl` |

Override with `--extensions` flag.

### Exclusions

Default exclusions (applied to the relative path from root-dir):

```
**/backup/**
**/bak/**
**/archive/**
**/old/**
**/deprecated/**
**/.git/**
**/.svn/**
**/node_modules/**
```

Additional exclusions via `--exclude` flag.

### Discovery Algorithm

```
1. Walk root-dir recursively using std::fs::read_dir (not walkdir -- avoid
   adding a dependency; the existing codebase uses manual recursion)
2. For each entry:
   a. Skip if matches any exclusion pattern
   b. If directory: recurse
   c. If file: classify by extension
   d. Collect: path, file_size, mtime (from fs::metadata)
3. Sort by path for deterministic ordering
4. Insert/update files table
```

### Symlink Handling

Follow symlinks but track visited inodes to prevent infinite loops. If a
symlink target has already been visited (by inode), skip it and emit a
warning diagnostic.

---

## Report Queries

### Summary Report (`--report summary`)

```sql
-- File counts by type and status
SELECT
    file_type,
    status,
    COUNT(*) as file_count,
    SUM(file_size) as total_bytes
FROM files
WHERE last_scan_run = ?
GROUP BY file_type, status
ORDER BY file_type, status;

-- Parse success rate
SELECT
    COUNT(*) as total,
    SUM(CASE WHEN valid THEN 1 ELSE 0 END) as valid,
    SUM(CASE WHEN NOT valid THEN 1 ELSE 0 END) as invalid,
    ROUND(100.0 * SUM(CASE WHEN valid THEN 1 ELSE 0 END) / COUNT(*), 2) as success_pct
FROM parse_results
WHERE run_id = ?;

-- Line count statistics
SELECT
    SUM(line_count) as total_lines,
    AVG(line_count) as avg_lines,
    MIN(line_count) as min_lines,
    MAX(line_count) as max_lines,
    PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY line_count) as median_lines
FROM parse_results
WHERE run_id = ? AND valid = true;

-- Phase timing
SELECT
    phase,
    started_at,
    finished_at,
    processed_files,
    skipped_files,
    failed_files
FROM scan_runs
WHERE run_id = ?;

-- Program type breakdown
SELECT
    CASE
        WHEN is_subprogram THEN 'subprogram'
        ELSE 'main'
    END as program_type,
    COUNT(*) as count
FROM parse_results
WHERE run_id = ? AND valid = true
GROUP BY is_subprogram;

-- Feature usage summary
SELECT
    SUM(CASE WHEN sql_statements > 0 THEN 1 ELSE 0 END) as programs_with_sql,
    SUM(CASE WHEN file_ops > 0 THEN 1 ELSE 0 END) as programs_with_fileio,
    SUM(CASE WHEN calls > 0 THEN 1 ELSE 0 END) as programs_with_calls,
    SUM(sql_statements) as total_sql_stmts,
    SUM(file_ops) as total_file_ops,
    SUM(calls) as total_calls
FROM parse_results
WHERE run_id = ? AND valid = true;
```

### Coverage Report (`--report coverage`)

```sql
-- Coverage distribution histogram (10% buckets)
SELECT
    FLOOR(coverage_pct / 10) * 10 as bucket_low,
    FLOOR(coverage_pct / 10) * 10 + 10 as bucket_high,
    COUNT(*) as file_count,
    ROUND(100.0 * COUNT(*) / SUM(COUNT(*)) OVER (), 2) as pct_of_total
FROM coverage_results
WHERE run_id = ?
GROUP BY FLOOR(coverage_pct / 10)
ORDER BY bucket_low;

-- Overall coverage statistics
SELECT
    COUNT(*) as files_analyzed,
    ROUND(AVG(coverage_pct), 2) as avg_coverage,
    ROUND(PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY coverage_pct), 2) as median_coverage,
    MIN(coverage_pct) as min_coverage,
    MAX(coverage_pct) as max_coverage,
    SUM(total_statements) as total_stmts,
    SUM(mapped_statements) as total_mapped,
    ROUND(100.0 * SUM(mapped_statements) / NULLIF(SUM(total_statements), 0), 2) as weighted_coverage
FROM coverage_results
WHERE run_id = ?;

-- Top 20 lowest-coverage files (migration risk)
SELECT
    f.path,
    pr.program_id,
    cr.coverage_pct,
    cr.total_statements,
    cr.mapped_statements,
    cr.unhandled_count
FROM coverage_results cr
JOIN files f ON cr.file_id = f.file_id
JOIN parse_results pr ON cr.file_id = pr.file_id AND cr.run_id = pr.run_id
WHERE cr.run_id = ?
ORDER BY cr.coverage_pct ASC
LIMIT 20;

-- Top 20 highest-coverage files (quick wins)
SELECT
    f.path,
    pr.program_id,
    cr.coverage_pct,
    cr.total_statements,
    cr.mapped_statements
FROM coverage_results cr
JOIN files f ON cr.file_id = f.file_id
JOIN parse_results pr ON cr.file_id = pr.file_id AND cr.run_id = pr.run_id
WHERE cr.run_id = ?
ORDER BY cr.coverage_pct DESC
LIMIT 20;

-- Coverage by program size tier
SELECT
    CASE
        WHEN pr.line_count < 100 THEN 'tiny (<100 lines)'
        WHEN pr.line_count < 500 THEN 'small (100-499)'
        WHEN pr.line_count < 2000 THEN 'medium (500-1999)'
        WHEN pr.line_count < 10000 THEN 'large (2000-9999)'
        ELSE 'massive (10000+)'
    END as size_tier,
    COUNT(*) as file_count,
    ROUND(AVG(cr.coverage_pct), 2) as avg_coverage,
    SUM(pr.line_count) as total_lines
FROM coverage_results cr
JOIN parse_results pr ON cr.file_id = pr.file_id AND cr.run_id = pr.run_id
WHERE cr.run_id = ?
GROUP BY size_tier
ORDER BY MIN(pr.line_count);
```

### Errors Report (`--report errors`)

```sql
-- Top 20 most common error codes
SELECT
    code,
    severity,
    COUNT(*) as occurrences,
    COUNT(DISTINCT file_id) as affected_files,
    ROUND(100.0 * COUNT(DISTINCT file_id) /
        (SELECT COUNT(*) FROM files WHERE last_scan_run = ?), 2) as pct_of_files
FROM diagnostics
WHERE run_id = ?
GROUP BY code, severity
ORDER BY occurrences DESC
LIMIT 20;

-- Most common error messages (normalized -- first 80 chars)
SELECT
    LEFT(message, 80) as message_prefix,
    code,
    COUNT(*) as occurrences,
    COUNT(DISTINCT file_id) as affected_files
FROM diagnostics
WHERE run_id = ? AND severity = 'error'
GROUP BY LEFT(message, 80), code
ORDER BY occurrences DESC
LIMIT 20;

-- Files with most errors (trouble spots)
SELECT
    f.path,
    pr.program_id,
    COUNT(*) as error_count
FROM diagnostics d
JOIN files f ON d.file_id = f.file_id
LEFT JOIN parse_results pr ON d.file_id = pr.file_id AND d.run_id = pr.run_id
WHERE d.run_id = ? AND d.severity = 'error'
GROUP BY f.path, pr.program_id
ORDER BY error_count DESC
LIMIT 20;

-- Unresolved copybooks
SELECT
    name,
    COUNT(*) as reference_count,
    COUNT(DISTINCT referenced_by) as referencing_files
FROM copybooks
WHERE run_id = ? AND resolved = false
GROUP BY name
ORDER BY reference_count DESC;

-- Unhandled constructs from coverage (Phase 2 diagnostics)
SELECT
    category,
    LEFT(message, 80) as construct,
    COUNT(*) as occurrences,
    COUNT(DISTINCT file_id) as affected_files
FROM diagnostics
WHERE run_id = ? AND phase = 2
GROUP BY category, LEFT(message, 80)
ORDER BY occurrences DESC
LIMIT 30;
```

### Complexity Report (`--report complexity`)

```sql
-- Complexity score: paragraphs + 2*calls + 3*file_ops + 3*sql_statements
-- Weighted to reflect migration difficulty
SELECT
    f.path,
    pr.program_id,
    pr.line_count,
    pr.paragraphs,
    pr.calls,
    pr.file_ops,
    pr.sql_statements,
    pr.data_items,
    (pr.paragraphs + 2 * pr.calls + 3 * pr.file_ops + 3 * pr.sql_statements) as complexity_score
FROM parse_results pr
JOIN files f ON pr.file_id = f.file_id
WHERE pr.run_id = ? AND pr.valid = true
ORDER BY complexity_score DESC
LIMIT 50;

-- Complexity distribution
SELECT
    CASE
        WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 10 THEN 'trivial (0-9)'
        WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 50 THEN 'simple (10-49)'
        WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 200 THEN 'moderate (50-199)'
        WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 500 THEN 'complex (200-499)'
        ELSE 'very complex (500+)'
    END as complexity_tier,
    COUNT(*) as file_count,
    ROUND(100.0 * COUNT(*) / SUM(COUNT(*)) OVER (), 2) as pct_of_total,
    SUM(line_count) as total_lines
FROM parse_results
WHERE run_id = ? AND valid = true
GROUP BY complexity_tier
ORDER BY MIN(paragraphs + 2*calls + 3*file_ops + 3*sql_statements);

-- SQL usage breakdown
SELECT
    f.path,
    pr.program_id,
    pr.sql_statements,
    pr.line_count
FROM parse_results pr
JOIN files f ON pr.file_id = f.file_id
WHERE pr.run_id = ? AND pr.sql_statements > 0
ORDER BY pr.sql_statements DESC
LIMIT 30;

-- Call graph fan-out (programs that CALL the most targets)
SELECT
    f.path,
    pr.program_id,
    COUNT(DISTINCT cg.callee_name) as call_targets,
    SUM(CASE WHEN cg.callee_file_id IS NOT NULL THEN 1 ELSE 0 END) as resolved_targets,
    SUM(CASE WHEN cg.is_dynamic THEN 1 ELSE 0 END) as dynamic_calls
FROM call_graph cg
JOIN files f ON cg.caller_file_id = f.file_id
JOIN parse_results pr ON cg.caller_file_id = pr.file_id AND cg.run_id = pr.run_id
WHERE cg.run_id = ?
GROUP BY f.path, pr.program_id
ORDER BY call_targets DESC
LIMIT 30;

-- Most-called programs (highest fan-in)
SELECT
    callee_name,
    COUNT(DISTINCT caller_file_id) as called_by_count,
    CASE WHEN callee_file_id IS NOT NULL THEN 'resolved' ELSE 'external' END as resolution
FROM call_graph
WHERE run_id = ?
GROUP BY callee_name, CASE WHEN callee_file_id IS NOT NULL THEN 'resolved' ELSE 'external' END
ORDER BY called_by_count DESC
LIMIT 30;
```

### Full Report (`--report full`)

Combines all four reports above in sequence, separated by section headers.

---

## Status Display Format

`cobol2rust scan --status` reads the database and prints:

```
cobol2rust scan -- Status
=========================

Database: /data/scan_results.duckdb
Root dir: /data/cobol-mainframe

Run #3 (started 2026-03-10 14:23:05, running)
  Phase: 1 (Inventory)
  Workers: 24
  Batch size: 200

  Files discovered:    269,477
  Files processed:     142,831  (53.0%)
  Files skipped:         8,412  (incremental)
  Files failed:            234

  Throughput: ~1,247 files/sec
  Estimated remaining: ~1m 35s

Previous completed runs:
  Run #2  2026-03-09 09:15:00  Phase: all  269,477 files  [completed]
  Run #1  2026-03-08 16:00:00  Phase: 1    269,477 files  [completed]
```

If no scan is running:

```
cobol2rust scan -- Status
=========================

Database: /data/scan_results.duckdb
No scan currently running.

Last completed run:
  Run #2  2026-03-09 09:15:00  Phase: all  269,477 files  [completed]
  Duration: 12m 43s
  Parse success: 98.7% (266,035/269,477)
  Coverage avg: 72.4%
```

### Status Query SQL

```sql
-- Current or most recent run
SELECT * FROM scan_runs ORDER BY run_id DESC LIMIT 5;

-- Progress for running scan
SELECT
    sr.run_id,
    sr.started_at,
    sr.status,
    sr.phase,
    sr.worker_count,
    sr.batch_size,
    sr.total_files,
    sr.processed_files,
    sr.skipped_files,
    sr.failed_files,
    EXTRACT(EPOCH FROM (current_timestamp - sr.started_at)) as elapsed_seconds
FROM scan_runs sr
WHERE sr.status = 'running'
ORDER BY sr.run_id DESC
LIMIT 1;
```

---

## Error Handling and Edge Cases

### File-Level Errors

| Scenario | Behavior |
|---|---|
| File unreadable (permissions) | Log error, INSERT diagnostic with code 'E-IO', mark file as 'failed', continue |
| File too large (>50MB) | Log warning, skip file, INSERT diagnostic with code 'W-SIZE' |
| Binary file detected | Skip file, do not insert parse_results, mark as 'unknown' type |
| Parse panic (parser bug) | Catch with `std::panic::catch_unwind`, mark as 'failed', INSERT diagnostic |
| Encoding error (non-UTF-8) | Try latin-1 fallback. If still fails, mark as 'failed' |

### Database Errors

| Scenario | Behavior |
|---|---|
| DB file locked | Retry 3 times with 1-second backoff, then fail with clear message |
| DB file corrupt | Fail with message suggesting `--db <new-path>` |
| Disk full | Fail immediately, last committed batch is safe |
| Schema mismatch (old DB) | Run ALTER TABLE migrations to add missing columns |

### System Errors

| Scenario | Behavior |
|---|---|
| SIGINT/SIGTERM | Catch signal, finish current batch commit, update scan_runs status to 'interrupted', exit cleanly |
| OOM (unlikely) | Rayon will panic; catch at top level, commit partial progress |
| Thread panic | Rayon catches panics per-task; log error, continue with other files |

### Edge Cases

| Scenario | Behavior |
|---|---|
| Empty directory | Report "no files found", exit 0 |
| Same file via symlink | Detect by canonical path, skip duplicate |
| Root-dir is a single file | Scan that one file (degenerate case) |
| --resume with no interrupted run | Error: "no interrupted scan to resume" |
| --resume with completed run | Error: "most recent run already completed; use --incremental for a new scan" |
| --phase 2 without Phase 1 data | Error: "no Phase 1 results found; run --phase 1 first" |
| Mixed case extensions (.CBL vs .cbl) | Case-insensitive matching (handled by lowercasing) |

---

## Performance Considerations

### Target Numbers

| Metric | Target | Basis |
|---|---|---|
| Phase 1 throughput | >2,000 files/sec | 20 cores, ~500us parse per file (small files) |
| Phase 2 throughput | >200 files/sec | 20 cores, transpilation ~100ms per file avg |
| Phase 1 total time | ~2-3 minutes | 269K files at 2K/sec |
| Phase 2 total time | ~20-25 minutes | 269K files at 200/sec |
| Phase 3 total time | <10 seconds | DuckDB analytical queries |
| DuckDB file size | ~500MB-1GB | 269K files, all tables |
| Peak memory | <2GB | Bounded by batch size and channel buffer |

### Optimization Strategies

1. **Batch inserts**: Use DuckDB's appender API (`duckdb::Appender`) for bulk
   inserts instead of individual INSERT statements. The appender bypasses the
   SQL parser and uses columnar batching internally. This is 10-100x faster
   than prepared statement inserts for bulk loads.

2. **File I/O**: Use `std::fs::read_to_string` (not mmap). For files >10MB,
   consider `read_to_string` with a pre-allocated buffer via `String::with_capacity`
   using the known file size.

3. **Avoid re-parsing for Phase 2**: Phase 2 calls `transpile_with_diagnostics`
   which re-parses internally. This is unavoidable given the current transpiler
   API, but the cost is acceptable since Phase 2 is dominated by code generation
   time, not parse time.

4. **DuckDB WAL mode**: Enable WAL mode for concurrent read access during writes:
   ```sql
   PRAGMA enable_progress_bar;
   ```
   DuckDB uses WAL by default for persistent databases, enabling `--status` to
   read while the scan writes.

5. **Memory-mapped I/O for DuckDB**: DuckDB handles this internally. No special
   configuration needed.

6. **Progress bar overhead**: Use `indicatif::ProgressBar` with a 100ms refresh
   interval (not per-file updates) to avoid terminal I/O bottleneck.

7. **String interning**: For frequently repeated strings (error codes, severity
   levels, file extensions), use static string slices rather than heap-allocated
   Strings in the hot path.

---

## Clap Args Definition (Reference)

This shows the intended clap struct layout for implementation:

```rust
/// Arguments for `cobol2rust scan`.
#[derive(Debug, Args)]
pub struct ScanArgs {
    /// Root directory to scan recursively.
    pub root_dir: PathBuf,

    /// Which phase to run.
    #[arg(long, default_value = "all")]
    pub phase: ScanPhase,

    /// DuckDB database file path.
    #[arg(long, default_value = "./cobol_scan.duckdb")]
    pub db: PathBuf,

    /// Copybook search directory (repeatable).
    #[arg(long = "copy-path")]
    pub copy_paths: Vec<PathBuf>,

    /// Number of parallel workers.
    #[arg(short = 'j', long, default_value_t = num_cpus::get())]
    pub jobs: usize,

    /// Skip files unchanged since last scan.
    #[arg(long)]
    pub incremental: bool,

    /// Resume an interrupted scan.
    #[arg(long)]
    pub resume: bool,

    /// Show current scan progress and exit.
    #[arg(long)]
    pub status: bool,

    /// Report type to generate.
    #[arg(long)]
    pub report: Option<ReportType>,

    /// Report output format.
    #[arg(long, default_value = "text")]
    pub format: ReportFormat,

    /// Files per DuckDB transaction.
    #[arg(long, default_value_t = 100)]
    pub batch_size: usize,

    /// Glob patterns to exclude (repeatable).
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Override file extensions to scan.
    #[arg(long, value_delimiter = ',')]
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ScanPhase {
    #[value(name = "1")]
    Inventory,
    #[value(name = "2")]
    Coverage,
    #[value(name = "3")]
    Report,
    All,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ReportType {
    Summary,
    Coverage,
    Errors,
    Complexity,
    Full,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ReportFormat {
    Text,
    Json,
    Csv,
}
```

---

## New Dependencies

| Crate | Purpose | Notes |
|---|---|---|
| `duckdb` | Result persistence and analytical queries | Already in workspace deps |
| `crossbeam-channel` | Bounded channel for writer thread | New dependency |
| `num_cpus` | Default worker count | New dependency |
| `ctrlc` | Graceful SIGINT handling | New dependency |
| `glob` | Exclusion pattern matching | New dependency (or use manual matching) |

All other dependencies (rayon, indicatif, clap, serde, serde_json, miette) are
already present in `cobol-cli`.

---

## Module Layout

```
crates/cobol-cli/src/
  main.rs              -- add Scan(scan::ScanArgs) to Command enum
  scan/
    mod.rs             -- pub mod declarations, run() entry point
    args.rs            -- ScanArgs, ScanPhase, ReportType, ReportFormat
    db.rs              -- DuckDB schema creation, migrations, insert helpers
    discover.rs        -- file discovery, extension matching, exclusion
    copybook.rs        -- copybook resolution cache and search algorithm
    phase1.rs          -- inventory (parse) phase logic
    phase2.rs          -- coverage analysis phase logic
    phase3.rs          -- reporting phase, SQL queries, formatters
    status.rs          -- --status display logic
    resume.rs          -- resume and incremental logic
    writer.rs          -- dedicated DuckDB writer thread
    progress.rs        -- progress bar management
```

This mirrors the existing pattern where each subcommand is a module (or file)
under `src/`, but uses a directory module for `scan` due to its size.

---

## Integration with Existing Code

### Reuse from check.rs

The following functions from `check.rs` contain logic that `scan` needs:

- `check_file()`: Parse + stats collection. Extract the core logic into a
  shared function in a new `crates/cobol-cli/src/analyze.rs` module, callable
  by both `check` and `scan`.
- `collect_stats()`: Program statistics from AST. Move to shared module.
- `run_coverage()`: Transpilation coverage. Move to shared module.
- `scan_warnings()`: Warning detection. Move to shared module.
- `extract_program_id_text()`: Fallback program-id extraction. Move to shared.

### Reuse from workspace.rs

- `discover_cobol_files()` / `collect_cobol_files_recursive()`: File discovery.
  The scan version extends this with extension filtering, exclusion patterns,
  and additional file types (copybooks, JCL).
- `discover_copybook_dirs()`: Copybook directory discovery. Extend with the
  broader search algorithm described above.
- `extract_copy_targets()`: Already in cobol-transpiler, used as-is.
- `collect_calls()`: CALL extraction from AST. Reuse directly.

### Proposed Refactor

Create `crates/cobol-cli/src/analyze.rs`:

```rust
/// Shared analysis logic used by both `check` and `scan` subcommands.

pub struct AnalysisResult {
    pub program_id: String,
    pub source_format: String,
    pub valid: bool,
    pub info: ProgramStats,
    pub errors: Vec<DiagnosticEntry>,
    pub warnings: Vec<DiagnosticEntry>,
    pub copy_targets: Vec<String>,
    pub coverage: Option<CoverageResult>,
    pub parse_time_ms: u64,
}

pub struct ProgramStats { /* same as current ProgramInfo */ }
pub struct DiagnosticEntry { /* same as current Diagnostic */ }
pub struct CoverageResult { /* same as current CoverageInfo */ }

/// Analyze a single COBOL source file.
pub fn analyze_file(source: &str, with_coverage: bool) -> AnalysisResult { ... }
```

Both `check::run()` and `scan::phase1` call `analyze_file()`. This avoids
code duplication and ensures consistent behavior.

---

## Exit Codes

| Code | Meaning |
|---|---|
| 0 | Scan completed successfully |
| 1 | Scan completed with errors (some files failed to parse) |
| 2 | Scan was interrupted and saved progress (--resume available) |
| 3 | Fatal error (database error, root-dir not found, etc.) |

---

## Future Extensions (Not In Scope)

These are noted for architectural awareness but are NOT part of this spec:

- **Web dashboard**: Serve DuckDB data via HTTP for interactive exploration
- **Diff between runs**: Compare two scan_runs to detect codebase drift
- **Dependency graph export**: Export call_graph as DOT/GraphML for visualization
- **CICS detection**: Flag EXEC CICS usage for separate migration planning
- **Batch migration planning**: Group files into migration waves based on
  dependency clusters and coverage tiers
- **JCL analysis**: Parse JCL to understand job flow and program execution order
