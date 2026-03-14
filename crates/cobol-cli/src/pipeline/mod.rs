//! Pipeline module: unified phase engine for `project` and `corpus` subcommands.
//!
//! Runs phases 0-5 sequentially, with checkpoint detection to skip completed phases.

pub mod config;
pub mod phase0;

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use miette::{miette, Result};

use crate::scan::args::{ReportFormat, ReportType};
use crate::scan::discover::{self, FileType};
use crate::scan::ndjson::{self, NdjsonWriter, ScanMeta};
use crate::workspace::load_project_config;
use config::ResolvedConfig;

/// Run the full pipeline for a single project.
pub fn run_pipeline(config: &ResolvedConfig) -> Result<ExitCode> {
    let start = Instant::now();

    if !config.project_dir.is_dir() {
        return Err(miette!(
            "project directory does not exist: {}",
            config.project_dir.display()
        ));
    }

    if !config.quiet {
        eprintln!("cobol2rust project: {}", config.project_dir.display());
        eprintln!("  output:      {}", config.output.display());
        eprintln!("  jobs:        {}", config.jobs);
        eprintln!("  incremental: {}", config.incremental);
        eprintln!();
    }

    // Configure rayon thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.jobs)
        .build_global()
        .ok();

    // Phase 0: Configure
    if config.phase_range.includes(0) {
        run_phase(0, "Configure", config, |cfg| {
            phase0::run_phase0(&cfg.project_dir, cfg.quiet)?;
            Ok(())
        })?;
    }

    // Reload config after Phase 0 may have created .cobol2rust.toml
    let project_config = load_project_config(&config.project_dir)?;
    let mut config = config.clone();
    if let Some(ref pc) = project_config {
        let resolved = crate::workspace::resolve_copy_paths(&config.project_dir, pc);
        for p in resolved {
            if !config.copy_paths.contains(&p) {
                config.copy_paths.push(p);
            }
        }
    }

    // Set up scan infrastructure for phases 1-2
    let scan_dir = reports_dir(&config);
    fs::create_dir_all(&scan_dir)
        .map_err(|e| miette!("failed to create reports dir: {e}"))?;

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        eprintln!("\n  Received interrupt signal, finishing current batch...");
        shutdown_clone.store(true, Ordering::Relaxed);
    })
    .ok(); // Ignore if already set

    // Phase 1: Inventory
    if config.phase_range.includes(1) {
        let skip = check_phase1_complete(&config) && config.incremental;
        if skip {
            if !config.quiet {
                eprintln!("Phase 1: Inventory -- skipped (parse_results.ndjson exists)");
                eprintln!();
            }
        } else {
            run_phase(1, "Inventory", &config, |cfg| {
                run_phase1_impl(cfg, &scan_dir, &shutdown)?;
                Ok(())
            })?;
        }
    }

    if shutdown.load(Ordering::Relaxed) {
        eprintln!("Pipeline interrupted.");
        return Ok(ExitCode::from(2));
    }

    // Phase 2: Coverage
    if config.phase_range.includes(2) {
        let skip = check_phase2_complete(&config) && config.incremental;
        if skip {
            if !config.quiet {
                eprintln!("Phase 2: Coverage -- skipped (coverage.ndjson exists)");
                eprintln!();
            }
        } else {
            run_phase(2, "Coverage", &config, |cfg| {
                run_phase2_impl(cfg, &scan_dir, &shutdown)?;
                Ok(())
            })?;
        }
    }

    if shutdown.load(Ordering::Relaxed) {
        eprintln!("Pipeline interrupted.");
        return Ok(ExitCode::from(2));
    }

    // Phase 3: Pre-Transpile Report
    if config.phase_range.includes(3) && !config.suppress_reports {
        run_phase(3, "Report (pre-transpile)", &config, |cfg| {
            run_report_impl(cfg, &scan_dir)?;
            Ok(())
        })?;
    }

    // Phase 4: Transpile
    if config.phase_range.includes(4) {
        run_phase(4, "Transpile", &config, |cfg| {
            run_phase4_impl(cfg)?;
            Ok(())
        })?;
    }

    // Phase 5: Combined Report
    if config.phase_range.includes(5) && !config.suppress_reports {
        run_phase(5, "Report (combined)", &config, |cfg| {
            run_report_impl(cfg, &scan_dir)?;
            Ok(())
        })?;
    }

    if !config.quiet {
        let elapsed = start.elapsed();
        eprintln!();
        eprintln!("Pipeline complete in {:.1}s", elapsed.as_secs_f64());
    }

    Ok(ExitCode::SUCCESS)
}

/// Run a single phase with timing and logging.
fn run_phase<F>(phase: u8, name: &str, config: &ResolvedConfig, f: F) -> Result<()>
where
    F: FnOnce(&ResolvedConfig) -> Result<()>,
{
    if !config.quiet {
        eprintln!("Phase {}: {} -- running", phase, name);
    }
    let start = Instant::now();

    f(config)?;

    if !config.quiet {
        let elapsed = start.elapsed();
        eprintln!(
            "Phase {}: {} -- done ({:.1}s)",
            phase,
            name,
            elapsed.as_secs_f64()
        );
        eprintln!();
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Checkpoint detection
// ---------------------------------------------------------------------------

fn reports_dir(config: &ResolvedConfig) -> PathBuf {
    config.output.join("reports")
}

fn file_has_content(path: &std::path::Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.len() > 0)
        .unwrap_or(false)
}

fn check_phase1_complete(config: &ResolvedConfig) -> bool {
    file_has_content(&reports_dir(config).join("parse_results.ndjson"))
}

fn check_phase2_complete(config: &ResolvedConfig) -> bool {
    file_has_content(&reports_dir(config).join("coverage.ndjson"))
}

// ---------------------------------------------------------------------------
// Phase implementations
// ---------------------------------------------------------------------------

fn chrono_now() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    // ISO 8601 approximate
    format!("{}Z", now.as_secs())
}

/// Phase 1: Inventory -- discover and parse all COBOL files.
fn run_phase1_impl(
    config: &ResolvedConfig,
    scan_dir: &std::path::Path,
    shutdown: &Arc<AtomicBool>,
) -> Result<()> {
    // Discover files
    eprintln!("  Discovering files...");
    let files = discover::discover_files(
        &config.project_dir,
        &config.extensions,
        &config.exclude,
    )?;

    let source_count = files.iter().filter(|f| f.file_type == FileType::Source).count();
    let copybook_count = files.iter().filter(|f| f.file_type == FileType::Copybook).count();
    eprintln!(
        "  Found {} files ({} source, {} copybooks)",
        files.len(),
        source_count,
        copybook_count
    );

    let mut writer = NdjsonWriter::new(scan_dir, false)?;
    let processed = Arc::new(AtomicU64::new(0));
    let failed = Arc::new(AtomicU64::new(0));

    // Write initial metadata
    writer.write_meta(&ScanMeta {
        run_id: 1,
        started_at: chrono_now(),
        finished_at: None,
        root_dir: config.project_dir.to_string_lossy().to_string(),
        phase: "1".to_string(),
        status: "running".to_string(),
        total_files: files.len() as i64,
        processed_files: 0,
        skipped_files: 0,
        failed_files: 0,
        worker_count: config.jobs as i64,
        batch_size: 100,
        incremental: config.incremental,
    })?;

    crate::scan::phase1::run_phase1_ndjson(
        &mut writer,
        scan_dir,
        1, // run_id
        &files,
        &processed,
        &failed,
        shutdown,
        100, // batch_size
        false, // resume
    )?;

    writer.flush()?;

    // Update metadata
    let total_processed = processed.load(Ordering::Relaxed) as i64;
    let total_failed = failed.load(Ordering::Relaxed) as i64;
    writer.write_meta(&ScanMeta {
        run_id: 1,
        started_at: chrono_now(),
        finished_at: Some(chrono_now()),
        root_dir: config.project_dir.to_string_lossy().to_string(),
        phase: "1".to_string(),
        status: "completed".to_string(),
        total_files: files.len() as i64,
        processed_files: total_processed,
        skipped_files: 0,
        failed_files: total_failed,
        worker_count: config.jobs as i64,
        batch_size: 100,
        incremental: config.incremental,
    })?;

    Ok(())
}

/// Phase 2: Coverage -- statement coverage analysis.
fn run_phase2_impl(
    config: &ResolvedConfig,
    scan_dir: &std::path::Path,
    shutdown: &Arc<AtomicBool>,
) -> Result<()> {
    let mut writer = NdjsonWriter::new(scan_dir, true)?; // append mode
    let processed = Arc::new(AtomicU64::new(0));
    let failed = Arc::new(AtomicU64::new(0));

    crate::scan::phase2::run_phase2_ndjson(
        &mut writer,
        scan_dir,
        1, // run_id
        &processed,
        &failed,
        shutdown,
        100, // batch_size
    )?;

    writer.flush()?;

    // Update metadata
    let total_processed = processed.load(Ordering::Relaxed) as i64;
    let total_failed = failed.load(Ordering::Relaxed) as i64;
    writer.write_meta(&ScanMeta {
        run_id: 1,
        started_at: chrono_now(),
        finished_at: Some(chrono_now()),
        root_dir: config.project_dir.to_string_lossy().to_string(),
        phase: "2".to_string(),
        status: "completed".to_string(),
        total_files: 0,
        processed_files: total_processed,
        skipped_files: 0,
        failed_files: total_failed,
        worker_count: config.jobs as i64,
        batch_size: 100,
        incremental: config.incremental,
    })?;

    Ok(())
}

/// Phase 3 / Phase 5: Report from NDJSON data.
fn run_report_impl(config: &ResolvedConfig, scan_dir: &std::path::Path) -> Result<()> {
    if !scan_dir.join("parse_results.ndjson").exists()
        && !scan_dir.join("transpile_results.ndjson").exists()
    {
        if !config.quiet {
            eprintln!("  No data available -- skipping report");
        }
        return Ok(());
    }

    #[cfg(feature = "duckdb")]
    {
        let conn = ndjson::load_into_duckdb(scan_dir)?;
        let meta = ndjson::load_scan_meta(scan_dir)?;
        let run_id = meta.map(|m| m.run_id).unwrap_or(1);
        crate::scan::phase3::run_phase3(&conn, run_id, ReportType::Full, ReportFormat::Text)?;
    }

    #[cfg(not(feature = "duckdb"))]
    {
        if !config.quiet {
            eprintln!("  Reporting requires the `duckdb` feature.");
            eprintln!(
                "  Use `cobol2rust report --scan-dir {}` on a DuckDB-enabled build.",
                scan_dir.display()
            );
        }
    }

    Ok(())
}

/// Phase 4: Transpile -- generate Rust workspace.
fn run_phase4_impl(config: &ResolvedConfig) -> Result<()> {
    crate::transpile_cmd::run_workspace_pipeline(config)?;
    Ok(())
}
