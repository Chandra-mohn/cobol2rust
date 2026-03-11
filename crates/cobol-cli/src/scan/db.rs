//! DuckDB schema creation and insert helpers for the scan database.

use std::collections::HashMap;

use duckdb::{params, Connection};
use miette::{IntoDiagnostic, Result};

use crate::analyze::{AnalysisResult, CoverageResult, DiagnosticEntry};
use crate::scan::discover::DiscoveredFile;

/// Create all tables if they do not exist.
pub fn create_schema(conn: &Connection) -> Result<()> {
    for stmt in SCHEMA_STATEMENTS {
        conn.execute(stmt, [])
            .map_err(|e| miette::miette!("failed to create schema: {e} (SQL: {})", &stmt[..stmt.len().min(60)]))?;
    }
    Ok(())
}

/// Insert a new scan run and return its run_id.
pub fn insert_scan_run(
    conn: &Connection,
    root_dir: &str,
    phase: &str,
    worker_count: usize,
    batch_size: usize,
    incremental: bool,
) -> Result<i64> {
    let run_id: i64 = conn
        .query_row(
            "INSERT INTO scan_runs (root_dir, phase, worker_count, batch_size, incremental)
             VALUES (?, ?, ?, ?, ?) RETURNING run_id",
            params![root_dir, phase, worker_count as i32, batch_size as i32, incremental],
            |row| row.get(0),
        )
        .into_diagnostic()?;

    Ok(run_id)
}

/// Update scan_runs counters.
pub fn update_scan_run_counts(
    conn: &Connection,
    run_id: i64,
    total: usize,
    processed: usize,
    skipped: usize,
    failed: usize,
) -> Result<()> {
    conn.execute(
        "UPDATE scan_runs SET total_files = ?, processed_files = ?,
         skipped_files = ?, failed_files = ? WHERE run_id = ?",
        params![
            total as i32,
            processed as i32,
            skipped as i32,
            failed as i32,
            run_id
        ],
    )
    .into_diagnostic()?;
    Ok(())
}

/// Mark a scan run as completed.
pub fn complete_scan_run(conn: &Connection, run_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE scan_runs SET status = 'completed', finished_at = current_timestamp WHERE run_id = ?",
        params![run_id],
    )
    .into_diagnostic()?;
    Ok(())
}

/// Mark a scan run as interrupted.
pub fn interrupt_scan_run(conn: &Connection, run_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE scan_runs SET status = 'interrupted', finished_at = current_timestamp WHERE run_id = ?",
        params![run_id],
    )
    .into_diagnostic()?;
    Ok(())
}

/// Bulk-register all discovered files using batch prepared statements.
/// Returns a map of relative_path -> file_id for all files.
pub fn bulk_register_files(
    conn: &Connection,
    files: &[DiscoveredFile],
    run_id: i64,
) -> Result<HashMap<String, i64>> {
    // Step 1: Get all existing paths in one query.
    let mut existing: HashMap<String, i64> = HashMap::new();
    {
        let mut stmt = conn
            .prepare("SELECT path, file_id FROM files")
            .into_diagnostic()?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })
            .into_diagnostic()?;
        for row in rows.flatten() {
            existing.insert(row.0, row.1);
        }
    }

    // Step 2: Batch-insert new files using a prepared statement.
    let mut insert_stmt = conn
        .prepare(
            "INSERT INTO files (path, absolute_path, extension, file_size, mtime,
             first_seen_run, last_scan_run, file_type, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'pending') RETURNING file_id",
        )
        .into_diagnostic()?;

    // Step 3: Prepare update statement for existing files.
    let mut update_stmt = conn
        .prepare(
            "UPDATE files SET file_size = ?, mtime = ?, last_scan_run = ? WHERE file_id = ?",
        )
        .into_diagnostic()?;

    let mut result_map: HashMap<String, i64> = HashMap::with_capacity(files.len());

    for file in files {
        if let Some(&file_id) = existing.get(&file.relative_path) {
            // Existing file: update metadata.
            update_stmt
                .execute(params![
                    file.file_size as i64,
                    file.mtime_epoch,
                    run_id,
                    file_id
                ])
                .into_diagnostic()?;
            result_map.insert(file.relative_path.clone(), file_id);
        } else {
            // New file: insert and get file_id.
            let file_id: i64 = insert_stmt
                .query_row(
                    params![
                        file.relative_path,
                        file.absolute_path,
                        file.extension,
                        file.file_size as i64,
                        file.mtime_epoch,
                        run_id,
                        run_id,
                        file.file_type.as_str(),
                    ],
                    |row| row.get(0),
                )
                .into_diagnostic()?;
            result_map.insert(file.relative_path.clone(), file_id);
        }
    }

    Ok(result_map)
}

/// Insert Phase 1 parse results for a file.
pub fn insert_parse_result(
    conn: &Connection,
    file_id: i64,
    run_id: i64,
    analysis: &AnalysisResult,
) -> Result<()> {
    conn.execute(
        "INSERT INTO parse_results (file_id, run_id, program_id, source_format, valid,
         line_count, paragraphs, sections, calls, file_ops, sql_statements,
         is_subprogram, has_linkage, has_using, data_items, error_count,
         warning_count, parse_time_ms)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            file_id,
            run_id,
            analysis.program_id,
            analysis.source_format,
            analysis.valid,
            analysis.line_count as i32,
            analysis.info.paragraphs as i32,
            analysis.info.sections as i32,
            analysis.info.calls as i32,
            analysis.info.file_ops as i32,
            analysis.info.sql_statements as i32,
            analysis.info.is_subprogram,
            analysis.info.has_linkage,
            analysis.info.has_using,
            analysis.info.data_items as i32,
            analysis.errors.len() as i32,
            analysis.warnings.len() as i32,
            analysis.parse_time_ms as i32,
        ],
    )
    .into_diagnostic()?;
    Ok(())
}

/// Insert diagnostics (errors/warnings) for a file.
pub fn insert_diagnostics(
    conn: &Connection,
    file_id: i64,
    run_id: i64,
    phase: i32,
    entries: &[DiagnosticEntry],
    severity: &str,
) -> Result<()> {
    let mut stmt = conn
        .prepare(
            "INSERT INTO diagnostics (file_id, run_id, phase, severity, line, code, message, category)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .into_diagnostic()?;

    for entry in entries {
        stmt.execute(params![
            file_id,
            run_id,
            phase,
            severity,
            entry.line.map(|l| l as i32),
            entry.code,
            entry.message,
            entry.category,
        ])
        .into_diagnostic()?;
    }
    Ok(())
}

/// Insert copybook references for a file.
pub fn insert_copybooks(
    conn: &Connection,
    run_id: i64,
    file_id: i64,
    copy_targets: &[String],
) -> Result<()> {
    let mut stmt = conn
        .prepare(
            "INSERT INTO copybooks (run_id, name, referenced_by, resolved)
             VALUES (?, ?, ?, false)",
        )
        .into_diagnostic()?;

    for name in copy_targets {
        stmt.execute(params![run_id, name, file_id])
            .into_diagnostic()?;
    }
    Ok(())
}

/// Insert Phase 2 coverage results for a file.
pub fn insert_coverage_result(
    conn: &Connection,
    file_id: i64,
    run_id: i64,
    coverage: &CoverageResult,
) -> Result<()> {
    conn.execute(
        "INSERT INTO coverage_results (file_id, run_id, total_statements, mapped_statements,
         coverage_pct, total_data_entries, unhandled_count, coverage_time_ms)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            file_id,
            run_id,
            coverage.total_statements as i32,
            coverage.mapped_statements as i32,
            coverage.coverage_pct,
            coverage.total_data_entries as i32,
            coverage.unhandled.len() as i32,
            coverage.coverage_time_ms as i32,
        ],
    )
    .into_diagnostic()?;
    Ok(())
}

/// Update a file's status.
pub fn update_file_status(
    conn: &Connection,
    file_id: i64,
    status: &str,
    run_id: i64,
) -> Result<()> {
    conn.execute(
        "UPDATE files SET status = ?, last_scan_run = ? WHERE file_id = ?",
        params![status, run_id, file_id],
    )
    .into_diagnostic()?;
    Ok(())
}

/// Get file IDs already processed in a given run (for resume).
pub fn get_processed_file_ids(conn: &Connection, run_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn
        .prepare("SELECT file_id FROM parse_results WHERE run_id = ?")
        .into_diagnostic()?;

    let ids: Vec<i64> = stmt
        .query_map(params![run_id], |row| row.get(0))
        .into_diagnostic()?
        .filter_map(|r| r.ok())
        .collect();
    Ok(ids)
}

/// Find the most recent interrupted or running scan run.
pub fn find_resumable_run(conn: &Connection) -> Result<Option<(i64, String)>> {
    let result = conn.query_row(
        "SELECT run_id, phase FROM scan_runs
         WHERE status IN ('running', 'interrupted')
         ORDER BY run_id DESC LIMIT 1",
        [],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
    );

    match result {
        Ok(row) => Ok(Some(row)),
        Err(duckdb::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(miette::miette!("failed to query resumable run: {e}")),
    }
}

/// Find the most recent completed run.
pub fn find_latest_completed_run(conn: &Connection) -> Result<Option<i64>> {
    let result = conn.query_row(
        "SELECT run_id FROM scan_runs WHERE status = 'completed' ORDER BY run_id DESC LIMIT 1",
        [],
        |row| row.get::<_, i64>(0),
    );

    match result {
        Ok(id) => Ok(Some(id)),
        Err(duckdb::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(miette::miette!("failed to query latest run: {e}")),
    }
}

/// Get files that passed Phase 1 but have no Phase 2 coverage yet.
pub fn get_parseable_uncovered_files(
    conn: &Connection,
    run_id: i64,
) -> Result<Vec<(i64, String)>> {
    let mut stmt = conn
        .prepare(
            "SELECT f.file_id, f.absolute_path
             FROM files f
             JOIN parse_results pr ON f.file_id = pr.file_id AND pr.run_id = ?
             WHERE pr.valid = true
             AND f.file_id NOT IN (SELECT file_id FROM coverage_results WHERE run_id = ?)
             ORDER BY f.file_id",
        )
        .into_diagnostic()?;

    let rows: Vec<(i64, String)> = stmt
        .query_map(params![run_id, run_id], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .into_diagnostic()?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

/// Schema SQL statements, executed one by one.
const SCHEMA_STATEMENTS: &[&str] = &[
    "CREATE SEQUENCE IF NOT EXISTS scan_runs_seq START 1",
    "CREATE SEQUENCE IF NOT EXISTS files_seq START 1",
    "CREATE SEQUENCE IF NOT EXISTS parse_results_seq START 1",
    "CREATE SEQUENCE IF NOT EXISTS diagnostics_seq START 1",
    "CREATE SEQUENCE IF NOT EXISTS coverage_results_seq START 1",
    "CREATE SEQUENCE IF NOT EXISTS copybooks_seq START 1",
    "CREATE SEQUENCE IF NOT EXISTS call_graph_seq START 1",
    "CREATE TABLE IF NOT EXISTS scan_runs (
        run_id          INTEGER PRIMARY KEY DEFAULT nextval('scan_runs_seq'),
        started_at      TIMESTAMP NOT NULL DEFAULT current_timestamp,
        finished_at     TIMESTAMP,
        root_dir        VARCHAR NOT NULL,
        phase           VARCHAR NOT NULL,
        status          VARCHAR NOT NULL DEFAULT 'running',
        total_files     INTEGER DEFAULT 0,
        processed_files INTEGER DEFAULT 0,
        skipped_files   INTEGER DEFAULT 0,
        failed_files    INTEGER DEFAULT 0,
        config_json     VARCHAR,
        worker_count    INTEGER NOT NULL,
        batch_size      INTEGER NOT NULL,
        incremental     BOOLEAN NOT NULL DEFAULT false
    )",
    "CREATE TABLE IF NOT EXISTS files (
        file_id         INTEGER PRIMARY KEY DEFAULT nextval('files_seq'),
        path            VARCHAR NOT NULL UNIQUE,
        absolute_path   VARCHAR NOT NULL,
        extension       VARCHAR NOT NULL,
        file_size       BIGINT NOT NULL,
        mtime           BIGINT NOT NULL,
        first_seen_run  INTEGER NOT NULL,
        last_scan_run   INTEGER NOT NULL,
        file_type       VARCHAR NOT NULL DEFAULT 'source',
        status          VARCHAR NOT NULL DEFAULT 'pending'
    )",
    "CREATE TABLE IF NOT EXISTS parse_results (
        id              INTEGER PRIMARY KEY DEFAULT nextval('parse_results_seq'),
        file_id         INTEGER NOT NULL,
        run_id          INTEGER NOT NULL,
        program_id      VARCHAR NOT NULL,
        source_format   VARCHAR NOT NULL,
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
    )",
    "CREATE TABLE IF NOT EXISTS diagnostics (
        id              INTEGER PRIMARY KEY DEFAULT nextval('diagnostics_seq'),
        file_id         INTEGER NOT NULL,
        run_id          INTEGER NOT NULL,
        phase           INTEGER NOT NULL,
        severity        VARCHAR NOT NULL,
        line            INTEGER,
        code            VARCHAR NOT NULL,
        message         VARCHAR NOT NULL,
        category        VARCHAR
    )",
    "CREATE TABLE IF NOT EXISTS coverage_results (
        id                  INTEGER PRIMARY KEY DEFAULT nextval('coverage_results_seq'),
        file_id             INTEGER NOT NULL,
        run_id              INTEGER NOT NULL,
        total_statements    INTEGER NOT NULL,
        mapped_statements   INTEGER NOT NULL,
        coverage_pct        DOUBLE NOT NULL,
        total_data_entries  INTEGER NOT NULL,
        unhandled_count     INTEGER NOT NULL DEFAULT 0,
        coverage_time_ms    INTEGER NOT NULL DEFAULT 0,
        UNIQUE(file_id, run_id)
    )",
    "CREATE TABLE IF NOT EXISTS copybooks (
        id              INTEGER PRIMARY KEY DEFAULT nextval('copybooks_seq'),
        run_id          INTEGER NOT NULL,
        name            VARCHAR NOT NULL,
        resolved_path   VARCHAR,
        referenced_by   INTEGER NOT NULL,
        resolved        BOOLEAN NOT NULL DEFAULT false
    )",
    "CREATE TABLE IF NOT EXISTS call_graph (
        id              INTEGER PRIMARY KEY DEFAULT nextval('call_graph_seq'),
        run_id          INTEGER NOT NULL,
        caller_file_id  INTEGER NOT NULL,
        callee_name     VARCHAR NOT NULL,
        callee_file_id  INTEGER,
        is_dynamic      BOOLEAN NOT NULL DEFAULT false
    )",
];
