//! `cobol2rust scan` -- enterprise codebase scanner.
//!
//! Supports two write modes:
//! - DuckDB (default): `--db ./scan.duckdb` -- row-at-a-time inserts (requires `duckdb` feature)
//! - NDJSON (fast): `--scan-dir ./scan/` -- pure I/O, 10-100x faster
//!
//! Three-phase pipeline:
//! - Phase 1: Inventory (parse all files, collect stats)
//! - Phase 2: Coverage (transpilation readiness analysis)
//! - Phase 3: Reporting (aggregated queries from DuckDB)

pub mod args;
#[cfg(feature = "duckdb")]
mod db;
pub mod discover;
pub mod ndjson;
mod phase1;
mod phase2;
#[cfg(feature = "duckdb")]
mod phase3;
#[cfg(feature = "duckdb")]
mod status;
mod worker;

use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use miette::Result;

use args::{ScanArgs, ScanPhase};
use discover::FileType;

use crate::Cli;

/// Run the hidden `scan-worker` subcommand (called by main scan as child process).
pub fn run_scan_worker(args: &args::ScanWorkerArgs) -> Result<ExitCode> {
    worker::run(args)
}

/// Run the `cobol2rust status` subcommand.
pub fn run_status(status_args: &args::StatusArgs) -> Result<ExitCode> {
    #[cfg(feature = "duckdb")]
    {
        if let Some(ref scan_dir) = status_args.scan_dir {
            // NDJSON mode: load into in-memory DuckDB.
            if !scan_dir.exists() {
                return Err(miette::miette!(
                    "scan directory does not exist: {}",
                    scan_dir.display()
                ));
            }
            let conn = ndjson::load_into_duckdb(scan_dir)?;
            status::print_status(&conn).map(|()| ExitCode::SUCCESS)
        } else {
            // DuckDB mode (original).
            let db_path = status_args.db.to_string_lossy().to_string();
            let conn = duckdb::Connection::open(&db_path)
                .map_err(|e| miette::miette!("failed to open database {db_path}: {e}"))?;
            db::create_schema(&conn)?;
            status::print_status(&conn).map(|()| ExitCode::SUCCESS)
        }
    }

    #[cfg(not(feature = "duckdb"))]
    {
        let _ = status_args;
        Err(miette::miette!(
            "the `status` command requires the `duckdb` feature.\n\
             Rebuild with: cargo build --features duckdb"
        ))
    }
}

/// Run the `cobol2rust report` subcommand.
pub fn run_report(report_args: &args::ReportArgs) -> Result<ExitCode> {
    #[cfg(feature = "duckdb")]
    {
        if let Some(ref scan_dir) = report_args.scan_dir {
            // NDJSON mode: load into in-memory DuckDB.
            if !scan_dir.exists() {
                return Err(miette::miette!(
                    "scan directory does not exist: {}",
                    scan_dir.display()
                ));
            }
            let conn = ndjson::load_into_duckdb(scan_dir)?;
            let meta = ndjson::load_scan_meta(scan_dir)?
                .ok_or_else(|| miette::miette!("no scan_meta.json found"))?;
            phase3::run_phase3(&conn, meta.run_id, report_args.r#type, report_args.format)
                .map(|()| ExitCode::SUCCESS)
        } else {
            // DuckDB mode (original).
            let db_path = report_args.db.to_string_lossy().to_string();
            let conn = duckdb::Connection::open(&db_path)
                .map_err(|e| miette::miette!("failed to open database {db_path}: {e}"))?;
            db::create_schema(&conn)?;
            let run_id = determine_report_run_id(&conn)?;
            phase3::run_phase3(&conn, run_id, report_args.r#type, report_args.format)
                .map(|()| ExitCode::SUCCESS)
        }
    }

    #[cfg(not(feature = "duckdb"))]
    {
        let _ = report_args;
        Err(miette::miette!(
            "the `report` command requires the `duckdb` feature.\n\
             Rebuild with: cargo build --features duckdb"
        ))
    }
}

/// Run the `cobol2rust scan` subcommand.
pub fn run(cli: &Cli, scan_args: &ScanArgs) -> Result<ExitCode> {
    if scan_args.use_ndjson() {
        run_ndjson(cli, scan_args)
    } else {
        #[cfg(feature = "duckdb")]
        {
            run_duckdb(cli, scan_args)
        }

        #[cfg(not(feature = "duckdb"))]
        {
            let _ = cli;
            Err(miette::miette!(
                "DuckDB scan mode requires the `duckdb` feature.\n\
                 Use --scan-dir for NDJSON mode, or rebuild with: cargo build --features duckdb"
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// NDJSON scan mode (--scan-dir)
// ---------------------------------------------------------------------------

fn run_ndjson(cli: &Cli, scan_args: &ScanArgs) -> Result<ExitCode> {
    let scan_dir = scan_args.scan_dir.as_ref().unwrap();

    if !scan_args.root_dir.exists() {
        return Err(miette::miette!(
            "root directory does not exist: {}",
            scan_args.root_dir.display()
        ));
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        eprintln!("\n  Received interrupt signal, finishing current batch...");
        shutdown_clone.store(true, Ordering::Relaxed);
    })
    .map_err(|e| miette::miette!("failed to set signal handler: {e}"))?;

    let num_jobs = scan_args.effective_jobs();
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_jobs)
        .build_global()
        .ok();

    if !cli.quiet {
        eprintln!(
            "cobol2rust scan [NDJSON]: {} (workers: {}, batch: {})",
            scan_args.root_dir.display(),
            num_jobs,
            scan_args.batch_size
        );
        eprintln!("  Output: {}", scan_dir.display());
    }

    let is_resume = scan_args.resume;
    let mut writer = ndjson::NdjsonWriter::new(scan_dir, is_resume)?;

    let run_id: i64 = if is_resume {
        match ndjson::load_scan_meta(scan_dir)? {
            Some(meta) if meta.status == "running" || meta.status == "interrupted" => {
                eprintln!("  Resuming run #{}", meta.run_id);
                meta.run_id
            }
            _ => {
                return Err(miette::miette!(
                    "no interrupted scan to resume in {}",
                    scan_dir.display()
                ));
            }
        }
    } else {
        1
    };

    let started_at = chrono_now();
    let phase_str = match scan_args.phase {
        ScanPhase::Inventory => "1",
        ScanPhase::Coverage => "2",
        ScanPhase::Report => "3",
        ScanPhase::All => "all",
    };

    if !is_resume {
        writer.write_meta(&ndjson::ScanMeta {
            run_id,
            started_at: started_at.clone(),
            finished_at: None,
            root_dir: scan_args.root_dir.to_string_lossy().to_string(),
            phase: phase_str.to_string(),
            status: "running".to_string(),
            total_files: 0,
            processed_files: 0,
            skipped_files: 0,
            failed_files: 0,
            worker_count: num_jobs as i64,
            batch_size: scan_args.batch_size as i64,
            incremental: scan_args.incremental,
        })?;
    }

    // Discover files.
    let should_discover =
        matches!(scan_args.phase, ScanPhase::Inventory | ScanPhase::All) || is_resume;

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
            discovered.len(), source_count, copybook_count, jcl_count
        );
        discovered
    } else {
        Vec::new()
    };

    let processed = Arc::new(AtomicU64::new(0));
    let failed = Arc::new(AtomicU64::new(0));
    let mut had_error = false;

    // Phase 1.
    if matches!(scan_args.phase, ScanPhase::Inventory | ScanPhase::All) || is_resume {
        if !shutdown.load(Ordering::Relaxed) {
            eprintln!();
            eprintln!("=== Phase 1: Inventory ===");
            if let Err(e) = phase1::run_phase1_ndjson(
                &mut writer,
                scan_dir,
                run_id,
                &files,
                &processed,
                &failed,
                &shutdown,
                scan_args.batch_size,
                is_resume,
            ) {
                eprintln!("  Phase 1 error: {e}");
                had_error = true;
            }
        }
    }

    // Phase 2.
    if matches!(scan_args.phase, ScanPhase::Coverage | ScanPhase::All) {
        if !shutdown.load(Ordering::Relaxed) && !had_error {
            eprintln!();
            eprintln!("=== Phase 2: Coverage ===");
            if let Err(e) = phase2::run_phase2_ndjson(
                &mut writer,
                scan_dir,
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

    writer.flush()?;

    // Finalize metadata.
    let total_processed = processed.load(Ordering::Relaxed) as i64;
    let total_failed = failed.load(Ordering::Relaxed) as i64;
    let final_status = if shutdown.load(Ordering::Relaxed) {
        "interrupted"
    } else if had_error {
        "failed"
    } else {
        "completed"
    };

    writer.write_meta(&ndjson::ScanMeta {
        run_id,
        started_at,
        finished_at: Some(chrono_now()),
        root_dir: scan_args.root_dir.to_string_lossy().to_string(),
        phase: phase_str.to_string(),
        status: final_status.to_string(),
        total_files: files.len() as i64,
        processed_files: total_processed,
        skipped_files: 0,
        failed_files: total_failed,
        worker_count: num_jobs as i64,
        batch_size: scan_args.batch_size as i64,
        incremental: scan_args.incremental,
    })?;

    if shutdown.load(Ordering::Relaxed) {
        eprintln!();
        eprintln!("Scan interrupted. Run with --resume to continue.");
        return Ok(ExitCode::from(2));
    }

    // Phase 3: auto-summary (requires DuckDB).
    #[cfg(feature = "duckdb")]
    if matches!(scan_args.phase, ScanPhase::Report | ScanPhase::All) && !had_error {
        eprintln!();
        eprintln!("=== Phase 3: Report ===");
        let conn = ndjson::load_into_duckdb(scan_dir)?;
        phase3::run_phase3(&conn, run_id, args::ReportType::Summary, args::ReportFormat::Text)?;
    }

    #[cfg(not(feature = "duckdb"))]
    if matches!(scan_args.phase, ScanPhase::Report | ScanPhase::All) && !had_error {
        eprintln!();
        eprintln!("  Phase 3 (reporting) requires the `duckdb` feature.");
        eprintln!("  Use `cobol2rust report --scan-dir {}` on a DuckDB-enabled build.", scan_dir.display());
    }

    if !cli.quiet {
        eprintln!();
        eprintln!("Results saved to: {}", scan_dir.display());
    }

    if had_error || total_failed > 0 {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

// ---------------------------------------------------------------------------
// DuckDB scan mode (--db, original implementation)
// ---------------------------------------------------------------------------

#[cfg(feature = "duckdb")]
fn run_duckdb(cli: &Cli, scan_args: &ScanArgs) -> Result<ExitCode> {
    use args::ReportType;

    if !scan_args.root_dir.exists() {
        return Err(miette::miette!(
            "root directory does not exist: {}",
            scan_args.root_dir.display()
        ));
    }

    let db_path = scan_args.db.to_string_lossy().to_string();
    let conn = duckdb::Connection::open(&db_path)
        .map_err(|e| miette::miette!("failed to open database {db_path}: {e}"))?;

    db::create_schema(&conn)?;

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        eprintln!("\n  Received interrupt signal, finishing current batch...");
        shutdown_clone.store(true, Ordering::Relaxed);
    })
    .map_err(|e| miette::miette!("failed to set signal handler: {e}"))?;

    let num_jobs = scan_args.effective_jobs();
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_jobs)
        .build_global()
        .ok();

    if !cli.quiet {
        eprintln!(
            "cobol2rust scan: {} (workers: {}, batch: {})",
            scan_args.root_dir.display(),
            num_jobs,
            scan_args.batch_size
        );
    }

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

    let should_discover =
        matches!(scan_args.phase, ScanPhase::Inventory | ScanPhase::All) || is_resume;

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
            discovered.len(), source_count, copybook_count, jcl_count
        );
        discovered
    } else {
        Vec::new()
    };

    let processed = Arc::new(AtomicU64::new(0));
    let failed = Arc::new(AtomicU64::new(0));
    let mut had_error = false;

    // Phase 1.
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

    // Phase 2.
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

    if shutdown.load(Ordering::Relaxed) {
        db::interrupt_scan_run(&conn, run_id)?;
        eprintln!();
        eprintln!("Scan interrupted. Run with --resume to continue.");
        return Ok(ExitCode::from(2));
    }

    let total_processed = processed.load(Ordering::Relaxed) as usize;
    let total_failed = failed.load(Ordering::Relaxed) as usize;
    db::update_scan_run_counts(&conn, run_id, files.len(), total_processed, 0, total_failed)?;
    db::complete_scan_run(&conn, run_id)?;

    // Phase 3.
    if matches!(scan_args.phase, ScanPhase::Report | ScanPhase::All) && !had_error {
        eprintln!();
        eprintln!("=== Phase 3: Report ===");
        phase3::run_phase3(&conn, run_id, ReportType::Summary, args::ReportFormat::Text)?;
    }

    if !cli.quiet {
        eprintln!();
        eprintln!("Results saved to: {}", scan_args.db.display());
    }

    if had_error || total_failed > 0 {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Determine which run_id to report on (DuckDB mode).
#[cfg(feature = "duckdb")]
fn determine_report_run_id(conn: &duckdb::Connection) -> Result<i64> {
    db::find_latest_completed_run(conn)?.ok_or_else(|| {
        miette::miette!("no completed scan runs found in database; run a scan first")
    })
}

/// Get current timestamp as ISO 8601 string.
pub fn chrono_now() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let secs_per_day = 86400u64;
    let days = now / secs_per_day;
    let time = now % secs_per_day;
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;

    let mut y = 1970i64;
    let mut remaining_days = days as i64;
    loop {
        let days_in_year = if is_leap_year(y) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        y += 1;
    }
    let month_days = if is_leap_year(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 1u32;
    for &md in &month_days {
        if remaining_days < md {
            break;
        }
        remaining_days -= md;
        m += 1;
    }
    let d = remaining_days + 1;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hours, minutes, seconds
    )
}

fn is_leap_year(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
