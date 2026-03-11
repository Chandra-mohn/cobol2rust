//! `cobol2rust scan` -- enterprise codebase scanner with DuckDB persistence.
//!
//! Three-phase pipeline:
//! - Phase 1: Inventory (parse all files, collect stats)
//! - Phase 2: Coverage (transpilation readiness analysis)
//! - Phase 3: Reporting (aggregated queries from DuckDB)

pub mod args;
mod db;
mod discover;
mod phase1;
mod phase2;
mod phase3;
mod status;

use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use duckdb::Connection;
use miette::Result;

use args::{ReportArgs, ReportType, ScanArgs, ScanPhase, StatusArgs};
use discover::FileType;

use crate::Cli;

/// Run the `cobol2rust status` subcommand.
pub fn run_status(status_args: &StatusArgs) -> Result<ExitCode> {
    let db_path = status_args.db.to_string_lossy().to_string();
    let conn = Connection::open(&db_path)
        .map_err(|e| miette::miette!("failed to open database {db_path}: {e}"))?;
    db::create_schema(&conn)?;
    status::print_status(&conn).map(|()| ExitCode::SUCCESS)
}

/// Run the `cobol2rust report` subcommand.
pub fn run_report(report_args: &ReportArgs) -> Result<ExitCode> {
    let db_path = report_args.db.to_string_lossy().to_string();
    let conn = Connection::open(&db_path)
        .map_err(|e| miette::miette!("failed to open database {db_path}: {e}"))?;
    db::create_schema(&conn)?;
    let run_id = determine_report_run_id(&conn)?;
    phase3::run_phase3(&conn, run_id, report_args.r#type, report_args.format)
        .map(|()| ExitCode::SUCCESS)
}

/// Run the `cobol2rust scan` subcommand.
pub fn run(cli: &Cli, scan_args: &ScanArgs) -> Result<ExitCode> {
    // Validate root dir exists.
    if !scan_args.root_dir.exists() {
        return Err(miette::miette!(
            "root directory does not exist: {}",
            scan_args.root_dir.display()
        ));
    }

    // Open/create database.
    let db_path = scan_args.db.to_string_lossy().to_string();
    let conn = Connection::open(&db_path)
        .map_err(|e| miette::miette!("failed to open database {db_path}: {e}"))?;

    db::create_schema(&conn)?;

    // Set up signal handler for graceful shutdown.
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        eprintln!("\n  Received interrupt signal, finishing current batch...");
        shutdown_clone.store(true, Ordering::Relaxed);
    })
    .map_err(|e| miette::miette!("failed to set signal handler: {e}"))?;

    // Configure rayon thread pool.
    let num_jobs = scan_args.effective_jobs();
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_jobs)
        .build_global()
        .ok(); // Ignore if already built.

    if !cli.quiet {
        eprintln!(
            "cobol2rust scan: {} (workers: {}, batch: {})",
            scan_args.root_dir.display(),
            num_jobs,
            scan_args.batch_size
        );
    }

    // Determine run_id (new or resume).
    let (run_id, is_resume) = if scan_args.resume {
        match db::find_resumable_run(&conn)? {
            Some((id, _phase)) => {
                eprintln!("  Resuming run #{id}");
                (id, true)
            }
            None => {
                return Err(miette::miette!(
                    "no interrupted scan to resume; use --incremental for a new scan"
                ));
            }
        }
    } else {
        let phase_str = match scan_args.phase {
            ScanPhase::Inventory => "1",
            ScanPhase::Coverage => "2",
            ScanPhase::Report => "3",
            ScanPhase::All => "all",
        };
        let id = db::insert_scan_run(
            &conn,
            &scan_args.root_dir.to_string_lossy(),
            phase_str,
            num_jobs,
            scan_args.batch_size,
            scan_args.incremental,
        )?;
        (id, false)
    };

    // Discover files.
    let should_discover = matches!(
        scan_args.phase,
        ScanPhase::Inventory | ScanPhase::All
    ) || is_resume;

    let files = if should_discover {
        eprintln!("  Discovering files...");
        let discovered = discover::discover_files(
            &scan_args.root_dir,
            &scan_args.extensions,
            &scan_args.exclude,
        )?;

        let source_count = discovered.iter().filter(|f| f.file_type == FileType::Source).count();
        let copybook_count = discovered.iter().filter(|f| f.file_type == FileType::Copybook).count();
        let jcl_count = discovered.iter().filter(|f| f.file_type == FileType::Jcl).count();

        eprintln!(
            "  Found {} files ({} source, {} copybooks, {} JCL)",
            discovered.len(),
            source_count,
            copybook_count,
            jcl_count
        );
        discovered
    } else {
        Vec::new()
    };

    let processed = Arc::new(AtomicU64::new(0));
    let failed = Arc::new(AtomicU64::new(0));

    let mut had_error = false;

    // Phase 1: Inventory.
    if matches!(scan_args.phase, ScanPhase::Inventory | ScanPhase::All) || is_resume {
        if !shutdown.load(Ordering::Relaxed) {
            eprintln!();
            eprintln!("=== Phase 1: Inventory ===");
            if let Err(e) = phase1::run_phase1(
                &conn,
                run_id,
                &files,
                &processed,
                &failed,
                &shutdown,
                scan_args.batch_size,
                scan_args.incremental,
                is_resume,
            ) {
                eprintln!("  Phase 1 error: {e}");
                had_error = true;
            }
        }
    }

    // Phase 2: Coverage.
    if matches!(scan_args.phase, ScanPhase::Coverage | ScanPhase::All) {
        if !shutdown.load(Ordering::Relaxed) && !had_error {
            eprintln!();
            eprintln!("=== Phase 2: Coverage ===");
            if let Err(e) = phase2::run_phase2(
                &conn,
                run_id,
                &processed,
                &failed,
                &shutdown,
                scan_args.batch_size,
            ) {
                eprintln!("  Phase 2 error: {e}");
                had_error = true;
            }
        }
    }

    // Finalize scan run status.
    if shutdown.load(Ordering::Relaxed) {
        db::interrupt_scan_run(&conn, run_id)?;
        eprintln!();
        eprintln!("Scan interrupted. Run with --resume to continue.");
        return Ok(ExitCode::from(2));
    }

    // Update final counts.
    let total_processed = processed.load(Ordering::Relaxed) as usize;
    let total_failed = failed.load(Ordering::Relaxed) as usize;
    db::update_scan_run_counts(&conn, run_id, files.len(), total_processed, 0, total_failed)?;
    db::complete_scan_run(&conn, run_id)?;

    // Phase 3: Reporting (auto-show summary after scan completes).
    if matches!(scan_args.phase, ScanPhase::Report | ScanPhase::All) {
        if !had_error {
            eprintln!();
            eprintln!("=== Phase 3: Report ===");
            phase3::run_phase3(&conn, run_id, ReportType::Summary, args::ReportFormat::Text)?;
        }
    }

    if !cli.quiet {
        eprintln!();
        eprintln!(
            "Results saved to: {}",
            scan_args.db.display()
        );
    }

    if had_error || total_failed > 0 {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Determine which run_id to report on.
fn determine_report_run_id(conn: &Connection) -> Result<i64> {
    db::find_latest_completed_run(conn)?.ok_or_else(|| {
        miette::miette!("no completed scan runs found in database; run a scan first")
    })
}
