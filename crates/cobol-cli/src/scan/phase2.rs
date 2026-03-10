//! Phase 2: Coverage -- run transpilation analysis on parseable files.

use std::fs;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use duckdb::Connection;
use indicatif::{ProgressBar, ProgressStyle};
use miette::Result;
use rayon::prelude::*;

use crate::analyze;
use crate::analyze::AnalysisResult;
use crate::scan::db;

/// Result from a Phase 2 analysis.
struct Phase2Result {
    file_id: i64,
    analysis: AnalysisResult,
}

/// Run Phase 2: coverage analysis on files that passed Phase 1.
pub fn run_phase2(
    conn: &Connection,
    run_id: i64,
    processed_counter: &Arc<AtomicU64>,
    failed_counter: &Arc<AtomicU64>,
    shutdown: &Arc<AtomicBool>,
    batch_size: usize,
) -> Result<()> {
    let work_items = db::get_parseable_uncovered_files(conn, run_id)?;

    let total_work = work_items.len();
    eprintln!("  Phase 2: {} files for coverage analysis", total_work);

    if total_work == 0 {
        eprintln!("  No files to analyze (all covered or none parseable).");
        return Ok(());
    }

    // Reset counters for Phase 2.
    processed_counter.store(0, Ordering::Relaxed);
    failed_counter.store(0, Ordering::Relaxed);

    let pb = ProgressBar::new(total_work as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "  Phase 2 [{bar:40}] {pos}/{len} ({per_sec}) ETA: {eta}",
        )
        .unwrap()
        .progress_chars("=> "),
    );

    let chunks: Vec<&[(i64, String)]> = work_items.chunks(batch_size).collect();

    for chunk in chunks {
        if shutdown.load(Ordering::Relaxed) {
            eprintln!("\n  Scan interrupted. Use --resume to continue.");
            break;
        }

        // Parse + transpile in parallel.
        let results: Vec<Phase2Result> = chunk
            .par_iter()
            .filter_map(|(file_id, abs_path)| {
                if shutdown.load(Ordering::Relaxed) {
                    return None;
                }

                let source = match fs::read_to_string(abs_path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("  [ERR] Cannot read {abs_path}: {e}");
                        failed_counter.fetch_add(1, Ordering::Relaxed);
                        pb.inc(1);
                        return None;
                    }
                };

                let analysis = std::panic::catch_unwind(|| {
                    analyze::analyze_source(&source, true)
                });

                match analysis {
                    Ok(result) => {
                        pb.inc(1);
                        Some(Phase2Result {
                            file_id: *file_id,
                            analysis: result,
                        })
                    }
                    Err(_) => {
                        eprintln!("  [ERR] Transpiler panicked on: {abs_path}");
                        failed_counter.fetch_add(1, Ordering::Relaxed);
                        pb.inc(1);
                        None
                    }
                }
            })
            .collect();

        // Write batch to DB on main thread.
        for entry in &results {
            match write_phase2_entry(conn, run_id, entry) {
                Ok(()) => {
                    processed_counter.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    eprintln!("  [ERR] Coverage write failed for file_id {}: {e}", entry.file_id);
                    failed_counter.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    pb.finish_with_message("done");

    let total_processed = processed_counter.load(Ordering::Relaxed) as usize;
    let total_failed = failed_counter.load(Ordering::Relaxed) as usize;
    eprintln!(
        "  Phase 2 complete: {} analyzed, {} failed",
        total_processed, total_failed
    );

    Ok(())
}

fn write_phase2_entry(
    conn: &Connection,
    run_id: i64,
    entry: &Phase2Result,
) -> miette::Result<()> {
    let file_id = entry.file_id;
    let a = &entry.analysis;

    if let Some(ref coverage) = a.coverage {
        db::insert_coverage_result(conn, file_id, run_id, coverage)?;

        if !coverage.unhandled.is_empty() {
            db::insert_diagnostics(conn, file_id, run_id, 2, &coverage.unhandled, "warning")?;
        }
    }

    db::update_file_status(conn, file_id, "covered", run_id)?;
    Ok(())
}
