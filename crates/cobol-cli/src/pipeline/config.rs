//! Pipeline configuration: resolve defaults -> .cobol2rust.toml -> CLI overrides.

use std::path::{Path, PathBuf};

use crate::workspace::{PipelineConfig, ProjectConfig};

/// Fully resolved configuration for a pipeline run.
///
/// Resolution order: built-in defaults -> .cobol2rust.toml -> CLI overrides.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// COBOL project root directory.
    pub project_dir: PathBuf,
    /// Rust output directory.
    pub output: PathBuf,
    /// Number of parallel workers.
    pub jobs: usize,
    /// Skip files that fail, report at end.
    pub continue_on_error: bool,
    /// Skip unchanged files based on mtime.
    pub incremental: bool,
    /// Path to cobol-runtime crate (for path dependency).
    pub runtime_path: Option<PathBuf>,
    /// Copybook search paths (absolute).
    pub copy_paths: Vec<PathBuf>,
    /// File extensions to scan.
    pub extensions: Vec<String>,
    /// Glob patterns to exclude.
    pub exclude: Vec<String>,
    /// Which phases to run.
    pub phase_range: PhaseRange,
    /// Verbosity level (0 = normal, 1+ = verbose).
    pub verbose: u8,
    /// Suppress non-error output.
    pub quiet: bool,
    /// Suppress phase 3/5 report output (used in corpus mode).
    pub suppress_reports: bool,
    /// Optional prefix for log messages (e.g. "github/repo-name").
    /// When set, all phase progress messages include a timestamp and this prefix.
    pub log_prefix: Option<String>,
}

/// Print a phase-progress message.
///
/// When `prefix` is `Some`, the message is formatted as:
///   `2026-03-14T08:23:15 [github/repo-name] Discovering files...`
///
/// When `prefix` is `None`, the message is printed as-is via `eprintln!`.
pub fn phase_log(prefix: &Option<String>, msg: &str) {
    if let Some(pfx) = prefix {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // Format as ISO 8601 (UTC approximate).
        let secs_per_day = 86400u64;
        let secs_per_hour = 3600u64;
        let secs_per_min = 60u64;
        let days_since_epoch = now / secs_per_day;
        let time_of_day = now % secs_per_day;
        let hours = time_of_day / secs_per_hour;
        let minutes = (time_of_day % secs_per_hour) / secs_per_min;
        let seconds = time_of_day % secs_per_min;
        // Compute year/month/day from days since epoch.
        let (year, month, day) = days_to_date(days_since_epoch);
        eprintln!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02} [{}] {}",
            year, month, day, hours, minutes, seconds, pfx, msg
        );
    } else {
        eprintln!("{}", msg);
    }
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_date(days: u64) -> (u64, u64, u64) {
    // Civil calendar algorithm (Euclidean affine).
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Which phases to run.
#[derive(Debug, Clone)]
pub enum PhaseRange {
    /// Run a single specific phase.
    Single(u8),
    /// Run phases from..=to (inclusive).
    Range { from: u8, to: u8 },
}

impl PhaseRange {
    /// Check if a given phase number is included in this range.
    pub fn includes(&self, phase: u8) -> bool {
        match self {
            PhaseRange::Single(p) => *p == phase,
            PhaseRange::Range { from, to } => phase >= *from && phase <= *to,
        }
    }
}

impl Default for PhaseRange {
    fn default() -> Self {
        PhaseRange::Range { from: 0, to: 5 }
    }
}

/// CLI overrides that take precedence over config file values.
#[derive(Debug, Default)]
pub struct CliOverrides {
    pub output: Option<PathBuf>,
    pub jobs: Option<usize>,
    pub phase: Option<u8>,
    pub from: Option<u8>,
    pub to: Option<u8>,
    pub verbose: u8,
    pub quiet: bool,
}

/// Resolve configuration from project config + CLI overrides.
///
/// Resolution order: built-in defaults -> .cobol2rust.toml -> CLI overrides.
pub fn resolve_config(
    project_dir: &Path,
    project_config: Option<&ProjectConfig>,
    overrides: &CliOverrides,
) -> ResolvedConfig {
    let empty_pipeline = PipelineConfig::default();
    let pipeline = project_config
        .map(|c| &c.pipeline)
        .unwrap_or(&empty_pipeline);

    // Output: CLI > config > default (./rust-out relative to project dir)
    let output = overrides
        .output
        .clone()
        .or_else(|| pipeline.output.clone())
        .unwrap_or_else(|| PathBuf::from("rust-out"));
    let output = if output.is_absolute() {
        output
    } else {
        project_dir.join(output)
    };

    // Jobs: CLI > config > num_cpus
    let jobs = overrides
        .jobs
        .or(pipeline.jobs)
        .unwrap_or_else(num_cpus::get);

    // Continue on error: config > default (true)
    let continue_on_error = pipeline.continue_on_error.unwrap_or(true);

    // Incremental: config > default (true)
    let incremental = pipeline.incremental.unwrap_or(true);

    // Runtime path: config only (no CLI override for this)
    let runtime_path = pipeline.runtime_path.clone();

    // Copy paths: resolve from workspace config
    let copy_paths = project_config
        .map(|c| {
            c.workspace
                .copy_paths
                .iter()
                .map(|p| {
                    if p.is_absolute() {
                        p.clone()
                    } else {
                        project_dir.join(p)
                    }
                })
                .filter(|p| p.is_dir())
                .collect()
        })
        .unwrap_or_default();

    // Extensions: config > default
    let extensions = project_config
        .and_then(|c| {
            if c.workspace.extensions.is_empty() {
                None
            } else {
                Some(c.workspace.extensions.clone())
            }
        })
        .unwrap_or_else(|| vec!["cbl".into(), "cob".into(), "cobol".into()]);

    // Exclude: config > default (empty)
    let exclude = project_config
        .map(|c| c.workspace.exclude.clone())
        .unwrap_or_default();

    // Phase range: CLI overrides
    let phase_range = if let Some(phase) = overrides.phase {
        PhaseRange::Single(phase)
    } else {
        PhaseRange::Range {
            from: overrides.from.unwrap_or(0),
            to: overrides.to.unwrap_or(5),
        }
    };

    ResolvedConfig {
        project_dir: project_dir.to_path_buf(),
        output,
        jobs,
        continue_on_error,
        incremental,
        runtime_path,
        copy_paths,
        extensions,
        exclude,
        phase_range,
        verbose: overrides.verbose,
        quiet: overrides.quiet,
        suppress_reports: false,
        log_prefix: None,
    }
}
