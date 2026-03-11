# Scan Performance: NDJSON Refactor Proposal

## Problem

Running `cobol2rust scan` on 280K files (13M+ lines COBOL) on a 24-core/32GB Linux machine:
- CPU usage: 5-6% (effectively 1 core)
- Memory: ~5GB of 32GB
- File registration: 3-4 minutes for 280K files
- Overall scan: estimated hours

The machine is barely utilized. Root cause: DuckDB is a columnar OLAP engine
optimized for bulk analytical queries, not 280K individual row INSERT operations.
Each prepared statement execution goes through the full DuckDB query engine per row,
even inside a transaction.

## Current Architecture

```
scan -> discover files -> INSERT each file into DuckDB (slow)
     -> parse files in parallel -> INSERT results into DuckDB per file (slow)
     -> report from DuckDB (fast, this is what DuckDB is good at)
```

DuckDB is used for both OLTP (write-heavy scanning) and OLAP (read-heavy reporting).
It excels at OLAP but is fundamentally poor at row-at-a-time OLTP inserts.

## Proposed Architecture: NDJSON + DuckDB

Separate write path (NDJSON) from query path (DuckDB):

```
scan -> discover files -> write files.ndjson (fast, pure I/O)
     -> parse in parallel -> append to parse_results.ndjson (fast)
     -> coverage analysis -> append to coverage.ndjson (fast)

report -> DuckDB reads NDJSON files via read_ndjson_auto() (fast, bulk columnar load)
       -> SQL queries on in-memory DuckDB (fast, OLAP sweet spot)
```

### Output Files

```
/data/scan_results/
  scan_meta.json          # run metadata (start time, root dir, config)
  files.ndjson            # one line per discovered file
  parse_results.ndjson    # one line per parsed file (Phase 1)
  diagnostics.ndjson      # errors/warnings
  coverage.ndjson         # Phase 2 coverage results
  copybooks.ndjson        # copybook references
```

### NDJSON Line Format (examples)

**files.ndjson**:
```json
{"path":"src/PROG1.cbl","absolute_path":"/repo/src/PROG1.cbl","extension":"cbl","file_size":12345,"mtime":1710000000,"file_type":"source"}
```

**parse_results.ndjson**:
```json
{"path":"src/PROG1.cbl","program_id":"PROG1","source_format":"fixed","valid":true,"line_count":500,"paragraphs":12,"sections":3,"calls":2,"file_ops":1,"sql_statements":0,"parse_time_ms":15}
```

## Why This Works

### Write Performance
- NDJSON append is pure sequential I/O -- as fast as disk allows
- No query engine overhead per record
- `serde_json::to_string()` + `writeln!()` -- trivial
- 280K lines can be written in seconds

### Resume/Incremental
- On resume: read existing NDJSON, collect processed paths into HashSet, skip those
- Reading 280K NDJSON lines into a HashSet is near-instant
- Incremental: mtime stored in each record, compare on next scan
- Crash recovery: truncate last incomplete line, keep appending

### Transactional Guarantees
- Each NDJSON line is a self-contained record (atomic at line level)
- Append-only semantics -- written or not, no partial state
- No cross-table foreign key consistency needed -- file path is the natural key

### Reporting
- DuckDB's `read_ndjson_auto()` bulk-loads all records in seconds
- In-memory DuckDB instance -- no persistent DB file needed
- Full SQL query capability for aggregation, filtering, joins
- This is exactly what DuckDB was designed for

## Benefits

1. **Performance**: 10-100x faster scan writes (pure I/O vs DB round-trips)
2. **Build simplicity**: DuckDB C++ compilation (4GB RAM, minutes) only needed
   for reporting, not scanning. Could make it optional/feature-gated.
3. **Debuggability**: NDJSON files are human-readable, greppable, diffable
4. **Portability**: Results are plain files -- easy to copy, share, version
5. **Simplicity**: No schema migrations, no sequences, no surrogate keys

## Trade-offs

- No SQL during scanning (but we don't need it -- scanning is write-only)
- Slightly larger on-disk footprint vs compressed DuckDB (but negligible)
- Report phase needs to load data each time (but DuckDB does this in seconds)

## Implementation Plan

1. Wait for current DuckDB-based scan to complete on 280K files
2. Analyze actual performance data (parse_time_ms distribution, wall time)
3. Refactor scan write path to NDJSON output
4. Refactor report/status to load NDJSON into in-memory DuckDB
5. Consider making `duckdb` dependency optional (feature = "reporting")

## Status

- [x] Problem identified (Session 46)
- [x] Architecture proposed and documented
- [ ] Current scan completing on production data (~280K files, overnight)
- [ ] Post-scan performance analysis
- [ ] Implementation
