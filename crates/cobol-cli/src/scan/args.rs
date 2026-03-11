//! CLI arguments for `cobol2rust scan`.

use std::path::PathBuf;

use clap::{Args, ValueEnum};

/// Arguments for `cobol2rust scan`.
#[derive(Debug, Args)]
pub struct ScanArgs {
    /// Root directory to scan recursively.
    pub root_dir: PathBuf,

    /// Which phase to run.
    #[arg(long, default_value = "all")]
    pub phase: ScanPhase,

    /// DuckDB database file path (default write mode).
    #[arg(long, default_value = "./cobol_scan.duckdb")]
    pub db: PathBuf,

    /// Scan results directory for NDJSON output (fast mode).
    /// When set, writes NDJSON files instead of DuckDB for 10-100x faster scans.
    #[arg(long)]
    pub scan_dir: Option<PathBuf>,

    /// Copybook search directory (repeatable).
    #[arg(long = "copy-path")]
    pub copy_paths: Vec<PathBuf>,

    /// Number of parallel workers.
    #[arg(long)]
    pub jobs: Option<usize>,

    /// Skip files unchanged since last scan.
    #[arg(long)]
    pub incremental: bool,

    /// Resume an interrupted scan.
    #[arg(long)]
    pub resume: bool,

    /// Files per transaction/write batch.
    #[arg(long, default_value_t = 100)]
    pub batch_size: usize,

    /// Glob patterns to exclude (repeatable).
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Override file extensions to scan (comma-separated).
    #[arg(long, value_delimiter = ',')]
    pub extensions: Vec<String>,
}

/// Arguments for `cobol2rust status`.
#[derive(Debug, Args)]
pub struct StatusArgs {
    /// DuckDB database file path.
    #[arg(long, default_value = "./cobol_scan.duckdb")]
    pub db: PathBuf,

    /// Scan results directory (NDJSON mode).
    #[arg(long)]
    pub scan_dir: Option<PathBuf>,
}

/// Arguments for `cobol2rust report`.
#[derive(Debug, Args)]
pub struct ReportArgs {
    /// DuckDB database file path.
    #[arg(long, default_value = "./cobol_scan.duckdb")]
    pub db: PathBuf,

    /// Scan results directory (NDJSON mode).
    #[arg(long)]
    pub scan_dir: Option<PathBuf>,

    /// Report type to generate.
    #[arg(long, default_value = "summary")]
    pub r#type: ReportType,

    /// Report output format.
    #[arg(long, default_value = "text")]
    pub format: ReportFormat,
}

impl ScanArgs {
    /// Effective number of worker threads.
    pub fn effective_jobs(&self) -> usize {
        self.jobs.unwrap_or_else(num_cpus::get)
    }

    /// Whether to use NDJSON mode (--scan-dir was specified).
    pub fn use_ndjson(&self) -> bool {
        self.scan_dir.is_some()
    }
}

/// Scan phase selector.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ScanPhase {
    /// Phase 1: Parse-only inventory.
    #[value(name = "1")]
    Inventory,
    /// Phase 2: Transpilation coverage analysis.
    #[value(name = "2")]
    Coverage,
    /// Phase 3: Reporting from DuckDB.
    #[value(name = "3")]
    Report,
    /// Run all phases sequentially.
    All,
}

/// Report type selector.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ReportType {
    Summary,
    Coverage,
    Errors,
    Complexity,
    Full,
}

/// Report output format.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ReportFormat {
    Text,
    Json,
    Csv,
}
