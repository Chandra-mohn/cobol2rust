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
    }
}
