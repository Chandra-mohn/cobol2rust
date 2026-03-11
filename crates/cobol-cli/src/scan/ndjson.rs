//! NDJSON (Newline-Delimited JSON) writer and loader for scan results.
//!
//! Replaces DuckDB as the write path during scanning for dramatically better
//! throughput. NDJSON files are loaded into in-memory DuckDB for reporting.

use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};

use crate::analyze::{AnalysisResult, CoverageResult, DiagnosticEntry};
use crate::scan::discover::DiscoveredFile;

// ---------------------------------------------------------------------------
// NDJSON record types (field names match DuckDB schema for seamless loading)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRecord {
    pub file_id: i64,
    pub path: String,
    pub absolute_path: String,
    pub extension: String,
    pub file_size: i64,
    pub mtime: i64,
    pub first_seen_run: i64,
    pub last_scan_run: i64,
    pub file_type: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseResultRecord {
    pub file_id: i64,
    pub run_id: i64,
    pub path: String,
    pub program_id: String,
    pub source_format: String,
    pub valid: bool,
    pub line_count: i32,
    pub paragraphs: i32,
    pub sections: i32,
    pub calls: i32,
    pub file_ops: i32,
    pub sql_statements: i32,
    pub is_subprogram: bool,
    pub has_linkage: bool,
    pub has_using: bool,
    pub data_items: i32,
    pub error_count: i32,
    pub warning_count: i32,
    pub parse_time_ms: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticRecord {
    pub file_id: i64,
    pub run_id: i64,
    pub phase: i32,
    pub severity: String,
    pub line: Option<i32>,
    pub code: String,
    pub message: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoverageRecord {
    pub file_id: i64,
    pub run_id: i64,
    pub total_statements: i32,
    pub mapped_statements: i32,
    pub coverage_pct: f64,
    pub total_data_entries: i32,
    pub unhandled_count: i32,
    pub coverage_time_ms: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopybookRecord {
    pub run_id: i64,
    pub name: String,
    pub referenced_by: i64,
    pub resolved: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanMeta {
    pub run_id: i64,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub root_dir: String,
    pub phase: String,
    pub status: String,
    pub total_files: i64,
    pub processed_files: i64,
    pub skipped_files: i64,
    pub failed_files: i64,
    pub worker_count: i64,
    pub batch_size: i64,
    pub incremental: bool,
}

// ---------------------------------------------------------------------------
// NDJSON Writer
// ---------------------------------------------------------------------------

/// Manages NDJSON output files for a scan run.
pub struct NdjsonWriter {
    output_dir: PathBuf,
    files_writer: BufWriter<File>,
    parse_results_writer: BufWriter<File>,
    diagnostics_writer: BufWriter<File>,
    coverage_writer: BufWriter<File>,
    copybooks_writer: BufWriter<File>,
    next_file_id: i64,
}

impl NdjsonWriter {
    /// Create a new writer, creating the output directory if needed.
    /// If `append` is true, opens files in append mode (for resume).
    pub fn new(output_dir: &Path, append: bool) -> Result<Self> {
        fs::create_dir_all(output_dir)
            .map_err(|e| miette::miette!("failed to create output dir {}: {e}", output_dir.display()))?;

        let open = |name: &str| -> Result<BufWriter<File>> {
            let path = output_dir.join(name);
            let file = if append {
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
            } else {
                File::create(&path)
            }
            .map_err(|e| miette::miette!("failed to open {}: {e}", path.display()))?;
            Ok(BufWriter::new(file))
        };

        // Determine next_file_id from existing files.ndjson (for resume).
        let next_file_id = if append {
            count_max_file_id(&output_dir.join("files.ndjson")).unwrap_or(0) + 1
        } else {
            1
        };

        Ok(Self {
            output_dir: output_dir.to_path_buf(),
            files_writer: open("files.ndjson")?,
            parse_results_writer: open("parse_results.ndjson")?,
            diagnostics_writer: open("diagnostics.ndjson")?,
            coverage_writer: open("coverage.ndjson")?,
            copybooks_writer: open("copybooks.ndjson")?,
            next_file_id,
        })
    }

    /// Write scan metadata.
    pub fn write_meta(&self, meta: &ScanMeta) -> Result<()> {
        let path = self.output_dir.join("scan_meta.json");
        let json = serde_json::to_string_pretty(meta).into_diagnostic()?;
        fs::write(&path, json)
            .map_err(|e| miette::miette!("failed to write scan_meta.json: {e}"))
    }

    /// Register discovered files and return a path -> file_id map.
    pub fn register_files(
        &mut self,
        files: &[DiscoveredFile],
        run_id: i64,
    ) -> Result<HashMap<String, i64>> {
        let mut map = HashMap::with_capacity(files.len());

        for file in files {
            let file_id = self.next_file_id;
            self.next_file_id += 1;

            let record = FileRecord {
                file_id,
                path: file.relative_path.clone(),
                absolute_path: file.absolute_path.clone(),
                extension: file.extension.clone(),
                file_size: file.file_size as i64,
                mtime: file.mtime_epoch,
                first_seen_run: run_id,
                last_scan_run: run_id,
                file_type: file.file_type.as_str().to_string(),
                status: "pending".to_string(),
            };

            let line = serde_json::to_string(&record).into_diagnostic()?;
            writeln!(self.files_writer, "{}", line).into_diagnostic()?;
            map.insert(file.relative_path.clone(), file_id);
        }
        self.files_writer.flush().into_diagnostic()?;
        Ok(map)
    }

    /// Write a parse result record.
    pub fn write_parse_result(
        &mut self,
        file_id: i64,
        run_id: i64,
        path: &str,
        analysis: &AnalysisResult,
    ) -> Result<()> {
        let record = ParseResultRecord {
            file_id,
            run_id,
            path: path.to_string(),
            program_id: analysis.program_id.clone(),
            source_format: analysis.source_format.clone(),
            valid: analysis.valid,
            line_count: analysis.line_count as i32,
            paragraphs: analysis.info.paragraphs as i32,
            sections: analysis.info.sections as i32,
            calls: analysis.info.calls as i32,
            file_ops: analysis.info.file_ops as i32,
            sql_statements: analysis.info.sql_statements as i32,
            is_subprogram: analysis.info.is_subprogram,
            has_linkage: analysis.info.has_linkage,
            has_using: analysis.info.has_using,
            data_items: analysis.info.data_items as i32,
            error_count: analysis.errors.len() as i32,
            warning_count: analysis.warnings.len() as i32,
            parse_time_ms: analysis.parse_time_ms as i32,
        };

        let line = serde_json::to_string(&record).into_diagnostic()?;
        writeln!(self.parse_results_writer, "{}", line).into_diagnostic()?;
        Ok(())
    }

    /// Write diagnostic records.
    pub fn write_diagnostics(
        &mut self,
        file_id: i64,
        run_id: i64,
        phase: i32,
        entries: &[DiagnosticEntry],
        severity: &str,
    ) -> Result<()> {
        for entry in entries {
            let record = DiagnosticRecord {
                file_id,
                run_id,
                phase,
                severity: severity.to_string(),
                line: entry.line.map(|l| l as i32),
                code: entry.code.clone(),
                message: entry.message.clone(),
                category: entry.category.clone(),
            };
            let line = serde_json::to_string(&record).into_diagnostic()?;
            writeln!(self.diagnostics_writer, "{}", line).into_diagnostic()?;
        }
        Ok(())
    }

    /// Write copybook reference records.
    pub fn write_copybooks(
        &mut self,
        run_id: i64,
        file_id: i64,
        copy_targets: &[String],
    ) -> Result<()> {
        for name in copy_targets {
            let record = CopybookRecord {
                run_id,
                name: name.clone(),
                referenced_by: file_id,
                resolved: false,
            };
            let line = serde_json::to_string(&record).into_diagnostic()?;
            writeln!(self.copybooks_writer, "{}", line).into_diagnostic()?;
        }
        Ok(())
    }

    /// Write a coverage result record.
    pub fn write_coverage(
        &mut self,
        file_id: i64,
        run_id: i64,
        coverage: &CoverageResult,
    ) -> Result<()> {
        let record = CoverageRecord {
            file_id,
            run_id,
            total_statements: coverage.total_statements as i32,
            mapped_statements: coverage.mapped_statements as i32,
            coverage_pct: coverage.coverage_pct,
            total_data_entries: coverage.total_data_entries as i32,
            unhandled_count: coverage.unhandled.len() as i32,
            coverage_time_ms: coverage.coverage_time_ms as i32,
        };
        let line = serde_json::to_string(&record).into_diagnostic()?;
        writeln!(self.coverage_writer, "{}", line).into_diagnostic()?;
        Ok(())
    }

    /// Flush all writers.
    pub fn flush(&mut self) -> Result<()> {
        self.files_writer.flush().into_diagnostic()?;
        self.parse_results_writer.flush().into_diagnostic()?;
        self.diagnostics_writer.flush().into_diagnostic()?;
        self.coverage_writer.flush().into_diagnostic()?;
        self.copybooks_writer.flush().into_diagnostic()?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Resume helpers: read existing NDJSON to determine what was already processed
// ---------------------------------------------------------------------------

/// Read files.ndjson and return path -> file_id map.
pub fn load_file_id_map(output_dir: &Path) -> Result<HashMap<String, i64>> {
    let path = output_dir.join("files.ndjson");
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let file = File::open(&path)
        .map_err(|e| miette::miette!("failed to open {}: {e}", path.display()))?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line.into_diagnostic()?;
        if line.trim().is_empty() {
            continue;
        }
        let record: FileRecord = serde_json::from_str(&line).into_diagnostic()?;
        map.insert(record.path, record.file_id);
    }
    Ok(map)
}

/// Read parse_results.ndjson and return set of already-processed paths.
pub fn load_processed_paths(output_dir: &Path) -> Result<HashSet<String>> {
    let path = output_dir.join("parse_results.ndjson");
    if !path.exists() {
        return Ok(HashSet::new());
    }
    let file = File::open(&path)
        .map_err(|e| miette::miette!("failed to open {}: {e}", path.display()))?;
    let reader = BufReader::new(file);
    let mut set = HashSet::new();
    for line in reader.lines() {
        let line = line.into_diagnostic()?;
        if line.trim().is_empty() {
            continue;
        }
        let record: ParseResultRecord = serde_json::from_str(&line).into_diagnostic()?;
        set.insert(record.path);
    }
    Ok(set)
}

/// Read parse_results.ndjson and return (file_id, absolute_path) for valid,
/// uncovered files (for Phase 2 input).
pub fn load_parseable_files(output_dir: &Path) -> Result<Vec<(i64, String)>> {
    let parse_path = output_dir.join("parse_results.ndjson");
    if !parse_path.exists() {
        return Ok(Vec::new());
    }

    // Get already-covered file_ids.
    let covered: HashSet<i64> = {
        let cov_path = output_dir.join("coverage.ndjson");
        if cov_path.exists() {
            let file = File::open(&cov_path)
                .map_err(|e| miette::miette!("failed to open {}: {e}", cov_path.display()))?;
            let reader = BufReader::new(file);
            let mut set = HashSet::new();
            for line in reader.lines() {
                let line = line.into_diagnostic()?;
                if line.trim().is_empty() {
                    continue;
                }
                let record: CoverageRecord = serde_json::from_str(&line).into_diagnostic()?;
                set.insert(record.file_id);
            }
            set
        } else {
            HashSet::new()
        }
    };

    // Build file_id -> absolute_path map from files.ndjson.
    let file_map: HashMap<i64, String> = {
        let files_path = output_dir.join("files.ndjson");
        if !files_path.exists() {
            return Ok(Vec::new());
        }
        let file = File::open(&files_path)
            .map_err(|e| miette::miette!("failed to open {}: {e}", files_path.display()))?;
        let reader = BufReader::new(file);
        let mut map = HashMap::new();
        for line in reader.lines() {
            let line = line.into_diagnostic()?;
            if line.trim().is_empty() {
                continue;
            }
            let record: FileRecord = serde_json::from_str(&line).into_diagnostic()?;
            map.insert(record.file_id, record.absolute_path);
        }
        map
    };

    // Collect valid, uncovered files.
    let file = File::open(&parse_path)
        .map_err(|e| miette::miette!("failed to open {}: {e}", parse_path.display()))?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line.into_diagnostic()?;
        if line.trim().is_empty() {
            continue;
        }
        let record: ParseResultRecord = serde_json::from_str(&line).into_diagnostic()?;
        if record.valid && !covered.contains(&record.file_id) {
            if let Some(abs_path) = file_map.get(&record.file_id) {
                result.push((record.file_id, abs_path.clone()));
            }
        }
    }
    Ok(result)
}

/// Read scan_meta.json.
pub fn load_scan_meta(output_dir: &Path) -> Result<Option<ScanMeta>> {
    let path = output_dir.join("scan_meta.json");
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| miette::miette!("failed to read scan_meta.json: {e}"))?;
    let meta: ScanMeta = serde_json::from_str(&content).into_diagnostic()?;
    Ok(Some(meta))
}

/// Count lines in a file (for progress estimation).
#[allow(dead_code)]
pub fn count_lines(path: &Path) -> usize {
    if !path.exists() {
        return 0;
    }
    let Ok(file) = File::open(path) else {
        return 0;
    };
    BufReader::new(file).lines().count()
}

// ---------------------------------------------------------------------------
// DuckDB loader: load NDJSON files into in-memory DuckDB for reporting
// ---------------------------------------------------------------------------

/// Load all NDJSON files from output_dir into an in-memory DuckDB connection.
/// Returns the connection ready for SQL queries.
#[cfg(feature = "duckdb")]
pub fn load_into_duckdb(output_dir: &Path) -> Result<duckdb::Connection> {
    let conn = duckdb::Connection::open_in_memory()
        .map_err(|e| miette::miette!("failed to open in-memory DuckDB: {e}"))?;

    // Load scan_meta.json as scan_runs table.
    if let Some(meta) = load_scan_meta(output_dir)? {
        conn.execute(
            "CREATE TABLE scan_runs (
                run_id INTEGER, started_at VARCHAR, finished_at VARCHAR,
                root_dir VARCHAR, phase VARCHAR, status VARCHAR,
                total_files INTEGER, processed_files INTEGER,
                skipped_files INTEGER, failed_files INTEGER,
                worker_count INTEGER, batch_size INTEGER, incremental BOOLEAN
            )",
            [],
        )
        .map_err(|e| miette::miette!("failed to create scan_runs: {e}"))?;

        conn.execute(
            "INSERT INTO scan_runs VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            duckdb::params![
                meta.run_id,
                meta.started_at,
                meta.finished_at,
                meta.root_dir,
                meta.phase,
                meta.status,
                meta.total_files,
                meta.processed_files,
                meta.skipped_files,
                meta.failed_files,
                meta.worker_count,
                meta.batch_size,
                meta.incremental,
            ],
        )
        .map_err(|e| miette::miette!("failed to insert scan_runs: {e}"))?;
    }

    // Load each NDJSON file as a table.
    load_ndjson_table(&conn, output_dir, "files", "files.ndjson")?;
    load_ndjson_table(&conn, output_dir, "parse_results", "parse_results.ndjson")?;
    load_ndjson_table(&conn, output_dir, "diagnostics", "diagnostics.ndjson")?;
    load_ndjson_table(&conn, output_dir, "coverage_results", "coverage.ndjson")?;
    load_ndjson_table(&conn, output_dir, "copybooks", "copybooks.ndjson")?;

    Ok(conn)
}

/// Load a single NDJSON file as a DuckDB table.
#[cfg(feature = "duckdb")]
fn load_ndjson_table(
    conn: &duckdb::Connection,
    output_dir: &Path,
    table_name: &str,
    file_name: &str,
) -> Result<()> {
    let ndjson_path = output_dir.join(file_name);

    if !ndjson_path.exists() || fs::metadata(&ndjson_path).map_or(true, |m| m.len() == 0) {
        // Create empty table with expected schema.
        create_empty_table(conn, table_name)?;
        return Ok(());
    }

    let path_str = ndjson_path.to_string_lossy().replace('\'', "''");
    let sql = format!(
        "CREATE TABLE {} AS SELECT * FROM read_ndjson_auto('{}')",
        table_name, path_str
    );
    conn.execute(&sql, [])
        .map_err(|e| miette::miette!("failed to load {} from NDJSON: {e}", table_name))?;

    Ok(())
}

/// Create empty tables with the expected schema for when NDJSON files don't exist.
#[cfg(feature = "duckdb")]
fn create_empty_table(conn: &duckdb::Connection, table_name: &str) -> Result<()> {
    let sql = match table_name {
        "files" => {
            "CREATE TABLE files (
                file_id INTEGER, path VARCHAR, absolute_path VARCHAR,
                extension VARCHAR, file_size BIGINT, mtime BIGINT,
                first_seen_run INTEGER, last_scan_run INTEGER,
                file_type VARCHAR, status VARCHAR
            )"
        }
        "parse_results" => {
            "CREATE TABLE parse_results (
                file_id INTEGER, run_id INTEGER, path VARCHAR,
                program_id VARCHAR, source_format VARCHAR, valid BOOLEAN,
                line_count INTEGER, paragraphs INTEGER, sections INTEGER,
                calls INTEGER, file_ops INTEGER, sql_statements INTEGER,
                is_subprogram BOOLEAN, has_linkage BOOLEAN, has_using BOOLEAN,
                data_items INTEGER, error_count INTEGER, warning_count INTEGER,
                parse_time_ms INTEGER
            )"
        }
        "diagnostics" => {
            "CREATE TABLE diagnostics (
                file_id INTEGER, run_id INTEGER, phase INTEGER,
                severity VARCHAR, line INTEGER, code VARCHAR,
                message VARCHAR, category VARCHAR
            )"
        }
        "coverage_results" => {
            "CREATE TABLE coverage_results (
                file_id INTEGER, run_id INTEGER,
                total_statements INTEGER, mapped_statements INTEGER,
                coverage_pct DOUBLE, total_data_entries INTEGER,
                unhandled_count INTEGER, coverage_time_ms INTEGER
            )"
        }
        "copybooks" => {
            "CREATE TABLE copybooks (
                run_id INTEGER, name VARCHAR,
                referenced_by INTEGER, resolved BOOLEAN
            )"
        }
        _ => return Ok(()),
    };

    conn.execute(sql, [])
        .map_err(|e| miette::miette!("failed to create empty table {}: {e}", table_name))?;
    Ok(())
}

/// Find the maximum file_id in files.ndjson (for resume).
fn count_max_file_id(path: &Path) -> Option<i64> {
    if !path.exists() {
        return None;
    }
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);
    let mut max_id: i64 = 0;
    for line in reader.lines().flatten() {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(record) = serde_json::from_str::<FileRecord>(&line) {
            if record.file_id > max_id {
                max_id = record.file_id;
            }
        }
    }
    if max_id > 0 { Some(max_id) } else { None }
}
