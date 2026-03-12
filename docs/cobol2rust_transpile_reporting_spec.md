# Transpile Reporting Spec

## Overview

Enhance the `cobol2rust transpile --workspace` command to always produce NDJSON
reporting artifacts alongside the generated Rust workspace. Reuses the scan
infrastructure (NDJSON writer, DuckDB loader, Phase 3 reports) so that
`cobol2rust report` and `cobol2rust status` work on transpile output.

## Motivation

The current `transpile --workspace` produces a one-line stderr summary:
`"240000 succeeded, 312 failed"`. For enterprise-scale runs (240K files), users
need the same queryable reporting that `scan` provides -- error breakdowns,
coverage distributions, timing analysis, and structured per-file results.

The transpiler already computes all scan Phase 1 + Phase 2 data as a byproduct
(`transpile_with_diagnostics()` returns parse results, diagnostics, coverage
stats, and the generated Rust code). It just discards them. This spec captures
that data at zero additional cost.

## Design Principles

1. **Always-on**: No opt-in flag. Workspace transpile always writes reports.
2. **Reuse scan infra**: Same NDJSON record types, same DuckDB loader, same
   report commands.
3. **Single pass**: Parse + coverage + transpile + reporting data collected in
   one pass per file.
4. **Single-file mode unchanged**: `cobol2rust transpile foo.cbl` remains as-is
   (no NDJSON for single file).

## Output Structure

```
<output-dir>/
  Cargo.toml                      # workspace root (existing)
  cobol2rust-manifest.toml        # manifest (existing)
  programs/                       # generated Rust crates (existing)
    payroll/src/main.rs
    gl_post/src/main.rs
    ...
  reports/                        # NEW -- NDJSON reporting dir
    transpile_meta.json           # run metadata
    files.ndjson                  # discovered source files
    parse_results.ndjson          # per-file parse outcome
    diagnostics.ndjson            # errors + warnings (all phases)
    coverage.ndjson               # statement coverage per file
    transpile_results.ndjson      # NEW -- transpile outcome per file
```

The `reports/` directory lives inside the workspace output so everything is
self-contained.

## New Record Type: TranspileResultRecord

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TranspileResultRecord {
    pub file_id: i64,
    pub run_id: i64,
    pub path: String,              // relative COBOL source path
    pub success: bool,             // true if Rust code was generated
    pub output_path: String,       // relative path to generated .rs file (empty on failure)
    pub rust_lines: i32,           // lines in generated Rust file (0 on failure)
    pub duration_ms: i32,          // wall-clock time for this file
    pub error: Option<String>,     // error message if success=false
    pub coverage_pct: f64,         // statement coverage (from TranspileResult)
    pub total_statements: i32,     // total COBOL statements
    pub mapped_statements: i32,    // successfully mapped statements
    pub verbs_used: String,        // comma-separated list of COBOL verbs used
    pub verbs_unsupported: String, // comma-separated list of unsupported verbs
}
```

## Changes to Existing Code

### 1. cobol-transpiler: Add `transpile_with_config_and_diagnostics()`

```rust
/// Transpile with COPY expansion AND diagnostics/coverage stats.
pub fn transpile_with_config_and_diagnostics(
    source: &str,
    config: &TranspileConfig,
) -> Result<TranspileResult> {
    let resolver = FileSystemResolver::new(config.copybook_paths.clone())
        .with_library_map(config.library_map.clone());
    let copy_expander = CopyExpander::new(Box::new(resolver), config.max_copy_depth);
    let expanded = copy_expander.expand(source)?;

    let (program, diagnostics) = parse_cobol_with_diagnostics(&expanded)?;
    let code = generate_rust(&program)?;
    let total_statements = count_statements(&program);
    let unhandled_count = diagnostics.iter()
        .filter(|d| d.category == DiagCategory::UnhandledStatement)
        .count();

    let stats = TranspileStats {
        total_statements,
        mapped_statements: total_statements.saturating_sub(unhandled_count),
        total_data_entries: program.data_division.as_ref()
            .map_or(0, |dd| count_data_entries(&dd.working_storage)),
        ..Default::default()
    };

    Ok(TranspileResult::success(code, diagnostics, stats))
}
```

### 2. scan/ndjson.rs: Add TranspileResultRecord and writer method

Add `TranspileResultRecord` struct (shown above).

Extend `NdjsonWriter`:
- Add `transpile_results_writer: BufWriter<File>` field
- Open `transpile_results.ndjson` in constructor
- Add `write_transpile_result(&mut self, record: &TranspileResultRecord)`
- Flush it in `flush()`

Add `load_into_duckdb` support:
- Load `transpile_results.ndjson` as `transpile_results` table
- Add `create_empty_table` entry for `transpile_results`

### 3. transpile_cmd.rs: Workspace mode collects and writes NDJSON

Replace the per-job call from:
```rust
transpile_single(&job.source_path, &config)  // returns String
```

To:
```rust
transpile_with_config_and_diagnostics(&source, &config)  // returns TranspileResult
```

Workspace mode orchestration:

```
1. Create NdjsonWriter at <output-dir>/reports/
2. Discover files (reuse scan::discover or existing glob)
3. Register files -> get file_id map
4. Write transpile_meta.json
5. For each file (parallel via rayon):
   a. Read source
   b. Call transpile_with_config_and_diagnostics()
   c. If success: write .rs file as today
   d. Collect: TranspileResultRecord + parse diagnostics + coverage
6. After parallel phase: write all NDJSON records
   - parse_results.ndjson
   - diagnostics.ndjson
   - coverage.ndjson
   - transpile_results.ndjson
7. Update transpile_meta.json with final counts
8. Print stderr summary (as today)
```

Note: NDJSON writes happen on the main thread after collecting results from
the parallel phase (same pattern as scan Phase 1). This avoids needing
thread-safe writer access.

### 4. scan/phase3.rs: Add transpile report type

Add `ReportType::Transpile` variant.

Queries for `--type transpile`:

```sql
-- Transpile success rate
SELECT
    COUNT(*) as total,
    SUM(CASE WHEN success THEN 1 ELSE 0 END) as succeeded,
    SUM(CASE WHEN NOT success THEN 1 ELSE 0 END) as failed,
    ROUND(100.0 * SUM(CASE WHEN success THEN 1 ELSE 0 END) / COUNT(*), 1) as success_pct
FROM transpile_results;

-- Top errors
SELECT error, COUNT(*) as cnt
FROM transpile_results
WHERE NOT success
GROUP BY error ORDER BY cnt DESC LIMIT 20;

-- Generated Rust line counts
SELECT
    MIN(rust_lines) as min_lines,
    MAX(rust_lines) as max_lines,
    ROUND(AVG(rust_lines), 0) as avg_lines,
    SUM(rust_lines) as total_lines
FROM transpile_results WHERE success;

-- Timing distribution
SELECT
    CASE
        WHEN duration_ms < 100 THEN '<100ms'
        WHEN duration_ms < 500 THEN '100-500ms'
        WHEN duration_ms < 1000 THEN '500ms-1s'
        WHEN duration_ms < 5000 THEN '1-5s'
        ELSE '>5s'
    END as bucket,
    COUNT(*) as cnt
FROM transpile_results GROUP BY bucket ORDER BY MIN(duration_ms);

-- Unsupported verbs ranking
SELECT verb, COUNT(*) as cnt
FROM (
    SELECT UNNEST(string_split(verbs_unsupported, ',')) as verb
    FROM transpile_results
    WHERE verbs_unsupported != ''
)
GROUP BY verb ORDER BY cnt DESC LIMIT 20;

-- Coverage distribution for successful transpilations
SELECT
    CASE
        WHEN coverage_pct = 100 THEN '100%'
        WHEN coverage_pct >= 90 THEN '90-99%'
        WHEN coverage_pct >= 75 THEN '75-89%'
        WHEN coverage_pct >= 50 THEN '50-74%'
        ELSE '<50%'
    END as bucket,
    COUNT(*) as cnt
FROM transpile_results WHERE success
GROUP BY bucket ORDER BY MIN(coverage_pct) DESC;
```

### 5. scan/args.rs: Update ReportType enum

```rust
pub enum ReportType {
    Summary,
    Coverage,
    Errors,
    Complexity,
    Transpile,  // NEW
    Full,       // includes Transpile when data present
}
```

`Full` report auto-includes transpile section when `transpile_results.ndjson`
exists (non-empty).

### 6. Existing reports auto-enhance

When `transpile_results` table exists in DuckDB:
- **Summary report**: Add "Transpile Results" section (success/fail counts)
- **Errors report**: Include transpile errors alongside parse errors
- **Full report**: Include all of the above

No changes needed to Coverage or Complexity reports -- they already work from
parse_results/coverage tables which transpile populates.

## transpile_meta.json

Reuses `ScanMeta` struct with `phase: "transpile"`:

```json
{
    "run_id": 1,
    "started_at": "2026-03-12T10:00:00Z",
    "finished_at": "2026-03-12T14:30:00Z",
    "root_dir": "/data/repos",
    "phase": "transpile",
    "status": "completed",
    "total_files": 240000,
    "processed_files": 239688,
    "skipped_files": 0,
    "failed_files": 312,
    "worker_count": 20,
    "batch_size": 0,
    "incremental": false
}
```

## CLI Usage

```bash
# Transpile workspace (reports always written)
cobol2rust transpile repos/ --workspace --output /data/rust-output -j 20 --continue-on-error

# Check progress mid-run
cobol2rust status --scan-dir /data/rust-output/reports/

# Generate reports after completion
cobol2rust report --scan-dir /data/rust-output/reports/ --type transpile
cobol2rust report --scan-dir /data/rust-output/reports/ --type full
cobol2rust report --scan-dir /data/rust-output/reports/ --type errors --format json
```

## Incremental Behavior

When `--incremental` is used:
- Skip files where output .rs is newer than source .cbl (existing behavior)
- Append to existing NDJSON files (append mode)
- `transpile_meta.json` updated with cumulative counts
- Reports reflect all runs combined

## Thread Safety

Parallel processing collects results into thread-local `Vec<TranspileResultRecord>`
(and parse/coverage/diagnostic records). After the parallel phase, the main
thread writes all collected records to NDJSON sequentially. This matches the
scan Phase 1 pattern and avoids mutex contention on writers.

## Migration Path

- No breaking changes to existing `transpile` behavior
- Single-file mode unchanged
- `reports/` dir is new -- no conflict with existing output
- `report --type transpile` returns helpful error if no transpile data present
- Existing scan NDJSON dirs work as before (no `transpile_results.ndjson` = report skipped)

## Implementation Order

1. Add `transpile_with_config_and_diagnostics()` to cobol-transpiler
2. Add `TranspileResultRecord` to ndjson.rs, extend writer + loader
3. Modify `transpile_cmd.rs` workspace mode to use new function + write NDJSON
4. Add `ReportType::Transpile` to phase3.rs with queries
5. Update `Full` report to include transpile section
6. Test with small batch, then full 240K run
