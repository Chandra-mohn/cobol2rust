//! `cobol2rust corpus` -- multi-repo orchestrator.
//!
//! Walks a repos directory (2 levels deep), discovers repos with COBOL files,
//! and runs the pipeline on each. Merges all per-repo NDJSON into a single
//! reports directory for aggregate reporting.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use clap::Args;
use miette::{miette, IntoDiagnostic, Result};

use crate::pipeline;
use crate::pipeline::config::{PhaseRange, ResolvedConfig};
use crate::scan::ndjson::ScanMeta;
use crate::workspace::load_project_config;
use crate::Cli;

/// Arguments for `cobol2rust corpus`.
#[derive(Debug, Args)]
pub struct CorpusArgs {
    /// Root directory containing repo subdirectories (2 levels: source/repo/).
    pub repos_dir: PathBuf,

    /// Override output root directory.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Override total parallel workers.
    #[arg(short = 'j', long)]
    pub jobs: Option<usize>,

    /// Run a specific phase per repo: 0,1,2,3,4,5.
    #[arg(long)]
    pub phase: Option<u8>,

    /// Start from phase per repo (default: 0).
    #[arg(long)]
    pub from: Option<u8>,

    /// Stop after phase per repo (default: 5).
    #[arg(long)]
    pub to: Option<u8>,
}

/// A discovered repo within the corpus.
#[derive(Debug)]
struct RepoEntry {
    /// Relative path from repos_dir (e.g., "github/my-repo").
    rel_path: String,
    /// Absolute path to the repo directory.
    abs_path: PathBuf,
}

/// Run the `corpus` subcommand.
pub fn run(cli: &Cli, args: &CorpusArgs) -> Result<ExitCode> {
    let start = Instant::now();

    if !args.repos_dir.is_dir() {
        return Err(miette!(
            "repos directory does not exist: {}",
            args.repos_dir.display()
        ));
    }

    let output_root = args
        .output
        .clone()
        .unwrap_or_else(|| PathBuf::from("corpus-output"));
    let output_root = if output_root.is_absolute() {
        output_root
    } else {
        std::env::current_dir()
            .unwrap_or_default()
            .join(output_root)
    };

    let jobs = args.jobs.unwrap_or_else(num_cpus::get);

    if !cli.quiet {
        eprintln!("cobol2rust corpus: {}", args.repos_dir.display());
        eprintln!("  output:  {}", output_root.display());
        eprintln!("  jobs:    {}", jobs);
        eprintln!();
    }

    // Discover repos
    let repos = discover_repos(&args.repos_dir)?;

    if repos.is_empty() {
        eprintln!("No repos with COBOL files found in {}", args.repos_dir.display());
        return Ok(ExitCode::SUCCESS);
    }

    if !cli.quiet {
        eprintln!("Found {} repos with COBOL files", repos.len());
        eprintln!();
    }

    // Create merged reports directory
    let merged_reports = output_root.join("reports");
    fs::create_dir_all(&merged_reports)
        .into_diagnostic()
        .map_err(|e| miette!("failed to create merged reports dir: {e}"))?;

    // Set up log file
    let log_path = output_root.join("corpus.log");
    let mut log_file = fs::File::create(&log_path)
        .into_diagnostic()
        .map_err(|e| miette!("failed to create log file: {e}"))?;

    let phase_range = if let Some(phase) = args.phase {
        PhaseRange::Single(phase)
    } else {
        PhaseRange::Range {
            from: args.from.unwrap_or(0),
            to: args.to.unwrap_or(5),
        }
    };

    // Process each repo
    let total_repos = repos.len();
    let mut succeeded_repos: u32 = 0;
    let mut failed_repos: u32 = 0;
    let skipped_repos: u32 = 0;
    let mut total_files: u64 = 0;
    let mut total_succeeded: u64 = 0;
    let mut total_failed: u64 = 0;

    for (idx, repo) in repos.iter().enumerate() {
        let counter = idx + 1;
        let repo_output = output_root.join(&repo.rel_path);

        let msg = format!("[{}/{}] {}", counter, total_repos, repo.rel_path);
        if !cli.quiet {
            eprintln!("{}", msg);
        }
        let _ = writeln!(log_file, "{}", msg);

        // Load per-repo .cobol2rust.toml
        let project_config = load_project_config(&repo.abs_path).unwrap_or(None);

        // Build resolved config for this repo
        let config = ResolvedConfig {
            project_dir: repo.abs_path.clone(),
            output: repo_output.clone(),
            jobs,
            continue_on_error: true,
            incremental: true,
            runtime_path: None,
            copy_paths: project_config
                .as_ref()
                .map(|c| {
                    crate::workspace::resolve_copy_paths(&repo.abs_path, c)
                })
                .unwrap_or_default(),
            extensions: project_config
                .as_ref()
                .and_then(|c| {
                    if c.workspace.extensions.is_empty() {
                        None
                    } else {
                        Some(c.workspace.extensions.clone())
                    }
                })
                .unwrap_or_else(|| vec!["cbl".into(), "cob".into(), "cobol".into()]),
            exclude: project_config
                .as_ref()
                .map(|c| c.workspace.exclude.clone())
                .unwrap_or_default(),
            phase_range: phase_range.clone(),
            verbose: cli.verbose,
            quiet: true, // suppress per-repo noise in corpus mode
            suppress_reports: true, // reports only on merged data
        };

        // Run pipeline for this repo
        match pipeline::run_pipeline(&config) {
            Ok(_) => {
                succeeded_repos += 1;
            }
            Err(e) => {
                failed_repos += 1;
                let err_msg = format!("  FAILED: {e}");
                eprintln!("{}", err_msg);
                let _ = writeln!(log_file, "{}", err_msg);
            }
        }

        // Always merge NDJSON (even from failed repos)
        let repo_reports = repo_output.join("reports");
        if repo_reports.is_dir() {
            merge_ndjson(&repo_reports, &merged_reports, &mut log_file);
            // Extract counts from scan_meta.json
            if let Some(meta) = load_repo_meta(&repo_reports) {
                total_files += meta.total_files.max(0) as u64;
                total_succeeded += meta.processed_files.max(0) as u64;
                total_failed += meta.failed_files.max(0) as u64;
            }
        }

        // Progress estimate
        if !cli.quiet {
            let elapsed = start.elapsed().as_secs();
            if counter > 0 && elapsed > 0 {
                let remaining = (total_repos - counter) as u64 * elapsed / counter as u64;
                eprintln!(
                    "  Progress: {}/{} repos, ETA: {}s",
                    counter, total_repos, remaining
                );
            }
        }
    }

    // Write merged scan_meta.json
    let meta = ScanMeta {
        run_id: 1,
        started_at: format!("{}Z", start.elapsed().as_secs()),
        finished_at: Some(crate::scan::chrono_now()),
        root_dir: args.repos_dir.to_string_lossy().to_string(),
        phase: "corpus".to_string(),
        status: "completed".to_string(),
        total_files: total_files as i64,
        processed_files: total_succeeded as i64,
        skipped_files: 0,
        failed_files: total_failed as i64,
        worker_count: jobs as i64,
        batch_size: 0,
        incremental: true,
    };

    let meta_json = serde_json::to_string_pretty(&meta)
        .unwrap_or_else(|_| "{}".to_string());
    let _ = fs::write(merged_reports.join("scan_meta.json"), &meta_json);

    // Ensure empty NDJSON files exist for DuckDB loading
    for name in &["parse_results.ndjson", "copybooks.ndjson", "transpile_results.ndjson"] {
        let path = merged_reports.join(name);
        if !path.exists() {
            let _ = fs::File::create(&path);
        }
    }

    let elapsed = start.elapsed();

    if !cli.quiet {
        eprintln!();
        eprintln!("=========================================");
        eprintln!("Corpus complete");
        eprintln!("=========================================");
        eprintln!("Duration:        {:.1}s", elapsed.as_secs_f64());
        eprintln!(
            "Repos:           {} total, {} succeeded, {} failed, {} skipped",
            total_repos, succeeded_repos, failed_repos, skipped_repos
        );
        eprintln!(
            "Files:           {} total, {} succeeded, {} failed",
            total_files, total_succeeded, total_failed
        );
        eprintln!("Output:          {}", output_root.display());
        eprintln!("Merged reports:  {}", merged_reports.display());
        eprintln!();
        eprintln!("To view reports:");
        eprintln!(
            "  cobol2rust report --scan-dir {} --type full",
            merged_reports.display()
        );
    }

    let _ = writeln!(
        log_file,
        "\nCorpus complete: {} repos ({} ok, {} failed), {} files in {:.1}s",
        total_repos,
        succeeded_repos,
        failed_repos,
        total_files,
        elapsed.as_secs_f64()
    );

    if failed_repos > 0 {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Discover repos by walking 2 levels deep under repos_dir.
///
/// Structure: repos_dir/source/repo/ where repo contains COBOL files.
fn discover_repos(repos_dir: &Path) -> Result<Vec<RepoEntry>> {
    let mut repos = Vec::new();

    let source_dirs = fs::read_dir(repos_dir)
        .into_diagnostic()
        .map_err(|e| miette!("failed to read repos dir: {e}"))?;

    for source_entry in source_dirs.flatten() {
        let source_path = source_entry.path();
        if !source_path.is_dir() {
            continue;
        }
        let source_name = match source_path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        // Skip hidden directories
        if source_name.starts_with('.') {
            continue;
        }

        let repo_dirs = match fs::read_dir(&source_path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        for repo_entry in repo_dirs.flatten() {
            let repo_path = repo_entry.path();
            if !repo_path.is_dir() {
                continue;
            }
            let repo_name = match repo_path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if repo_name.starts_with('.') {
                continue;
            }

            // Check if this repo has COBOL files (search up to 5 levels deep)
            if has_cobol_files(&repo_path, 0, 5) {
                repos.push(RepoEntry {
                    rel_path: format!("{}/{}", source_name, repo_name),
                    abs_path: repo_path,
                });
            }
        }
    }

    // Sort for deterministic ordering
    repos.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    Ok(repos)
}

/// Check if a directory contains COBOL files (recursive up to max_depth).
fn has_cobol_files(dir: &Path, depth: usize, max_depth: usize) -> bool {
    if depth > max_depth {
        return false;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return false,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let lower = ext.to_lowercase();
                if lower == "cbl" || lower == "cob" || lower == "cobol" {
                    return true;
                }
            }
        } else if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !name.starts_with('.') && name != "node_modules" {
                if has_cobol_files(&path, depth + 1, max_depth) {
                    return true;
                }
            }
        }
    }
    false
}

/// Merge per-repo NDJSON files into the merged reports directory.
fn merge_ndjson(
    repo_reports: &Path,
    merged_reports: &Path,
    log_file: &mut fs::File,
) {
    let ndjson_files = [
        "transpile_results.ndjson",
        "files.ndjson",
        "diagnostics.ndjson",
        "coverage.ndjson",
        "parse_results.ndjson",
        "copybooks.ndjson",
    ];

    for name in &ndjson_files {
        let src = repo_reports.join(name);
        if !src.exists() {
            continue;
        }
        // Skip empty files
        if fs::metadata(&src).map(|m| m.len()).unwrap_or(0) == 0 {
            continue;
        }

        let dst = merged_reports.join(name);
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&dst)
        {
            Ok(mut dst_file) => {
                match fs::File::open(&src) {
                    Ok(src_file) => {
                        let reader = BufReader::new(src_file);
                        for line in reader.lines().flatten() {
                            if !line.is_empty() {
                                let _ = writeln!(dst_file, "{}", line);
                            }
                        }
                    }
                    Err(e) => {
                        let _ = writeln!(log_file, "  warning: failed to read {}: {e}", src.display());
                    }
                }
            }
            Err(e) => {
                let _ = writeln!(log_file, "  warning: failed to open {}: {e}", dst.display());
            }
        }
    }
}

/// Load scan_meta.json from a repo's reports directory.
fn load_repo_meta(repo_reports: &Path) -> Option<ScanMeta> {
    let meta_path = repo_reports.join("scan_meta.json");
    let content = fs::read_to_string(&meta_path).ok()?;
    serde_json::from_str(&content).ok()
}
