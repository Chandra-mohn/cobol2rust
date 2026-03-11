//! Phase 2: Coverage -- run transpilation analysis on parseable files.

use std::fs;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use indicatif::{ProgressBar, ProgressStyle};
use miette::Result;
#[cfg(feature = "duckdb")]
use rayon::prelude::*;

use crate::analyze;
use crate::analyze::AnalysisResult;

// ---------------------------------------------------------------------------
// DuckDB mode: Phase 2 with DuckDB persistence
// ---------------------------------------------------------------------------

#[cfg(feature = "duckdb")]
mod duckdb_phase2 {
    use super::*;
    use duckdb::Connection;
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
}

#[cfg(feature = "duckdb")]
pub use duckdb_phase2::run_phase2;

// ---------------------------------------------------------------------------
// NDJSON mode: Phase 2 with streaming pipeline
// ---------------------------------------------------------------------------

use std::path::Path;
use std::thread;

use crossbeam_channel::bounded;

use crate::scan::ndjson::{self, NdjsonWriter};

/// Job sent from feeder -> rayon parse pool (Phase 2).
struct CoverageJob {
    file_id: i64,
    absolute_path: String,
    source: String,
}

/// Result sent from rayon -> writer (Phase 2).
enum CoverageResult {
    Success { file_id: i64, analysis: AnalysisResult },
    Panic { absolute_path: String },
    ReadError { absolute_path: String },
}

/// Run Phase 2 in NDJSON mode: streaming pipeline for coverage analysis.
pub fn run_phase2_ndjson(
    writer: &mut NdjsonWriter,
    output_dir: &Path,
    run_id: i64,
    processed_counter: &Arc<AtomicU64>,
    failed_counter: &Arc<AtomicU64>,
    shutdown: &Arc<AtomicBool>,
    _batch_size: usize,
) -> Result<()> {
    // Get files that passed Phase 1 but have no coverage yet.
    let work_items = ndjson::load_parseable_files(output_dir)?;

    let total_work = work_items.len();
    eprintln!("  Phase 2: {} files for coverage analysis", total_work);

    if total_work == 0 {
        eprintln!("  No files to analyze (all covered or none parseable).");
        return Ok(());
    }

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

    // Streaming pipeline: feeder -> rayon pool -> writer (this thread).
    let num_threads = rayon::current_num_threads();
    let job_cap = num_threads * 2;
    let result_cap = 256;

    let (job_tx, job_rx) = bounded::<CoverageJob>(job_cap);
    let (result_tx, result_rx) = bounded::<CoverageResult>(result_cap);

    let shutdown_feeder = shutdown.clone();
    let shutdown_parser = shutdown.clone();

    // Stage 1: Feeder thread.
    let feeder_result_tx = result_tx.clone();
    let feeder = thread::Builder::new()
        .name("cov-feeder".into())
        .spawn(move || {
            for (file_id, abs_path) in work_items {
                if shutdown_feeder.load(Ordering::Relaxed) {
                    break;
                }

                match fs::read_to_string(&abs_path) {
                    Ok(source) => {
                        let job = CoverageJob {
                            file_id,
                            absolute_path: abs_path,
                            source,
                        };
                        if job_tx.send(job).is_err() {
                            break;
                        }
                    }
                    Err(_) => {
                        let _ = feeder_result_tx.send(CoverageResult::ReadError {
                            absolute_path: abs_path,
                        });
                    }
                }
            }
        })
        .map_err(|e| miette::miette!("failed to spawn feeder thread: {e}"))?;

    // Stage 2: Dispatcher thread -- spawns rayon tasks for coverage analysis.
    let dispatcher = thread::Builder::new()
        .name("cov-dispatcher".into())
        .spawn(move || {
            rayon::scope(|s| {
                while let Ok(job) = job_rx.recv() {
                    if shutdown_parser.load(Ordering::Relaxed) {
                        break;
                    }

                    let tx = result_tx.clone();
                    s.spawn(move |_| {
                        let outcome = std::panic::catch_unwind(|| {
                            analyze::analyze_source(&job.source, true)
                        });

                        let result = match outcome {
                            Ok(analysis) => CoverageResult::Success {
                                file_id: job.file_id,
                                analysis,
                            },
                            Err(_) => CoverageResult::Panic {
                                absolute_path: job.absolute_path,
                            },
                        };

                        let _ = tx.send(result);
                    });
                }
            });
        })
        .map_err(|e| miette::miette!("failed to spawn dispatcher thread: {e}"))?;

    // Stage 3: Writer loop on this thread.
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_secs(5);
    let mut results_since_flush = 0u64;

    while let Ok(result) = result_rx.recv() {
        match result {
            CoverageResult::Success { file_id, analysis } => {
                if let Some(ref coverage) = analysis.coverage {
                    writer.write_coverage(file_id, run_id, coverage)?;

                    if !coverage.unhandled.is_empty() {
                        writer.write_diagnostics(
                            file_id, run_id, 2, &coverage.unhandled, "warning",
                        )?;
                    }
                }
                processed_counter.fetch_add(1, Ordering::Relaxed);
                results_since_flush += 1;
            }
            CoverageResult::Panic { absolute_path } => {
                eprintln!("  [ERR] Transpiler panicked on: {absolute_path}");
                failed_counter.fetch_add(1, Ordering::Relaxed);
            }
            CoverageResult::ReadError { absolute_path } => {
                eprintln!("  [ERR] Cannot read: {absolute_path}");
                failed_counter.fetch_add(1, Ordering::Relaxed);
            }
        }

        pb.inc(1);

        if results_since_flush >= 500 || last_flush.elapsed() >= flush_interval {
            writer.flush()?;
            results_since_flush = 0;
            last_flush = Instant::now();
        }
    }

    writer.flush()?;
    pb.finish_with_message("done");

    feeder
        .join()
        .map_err(|_| miette::miette!("feeder thread panicked"))?;
    dispatcher
        .join()
        .map_err(|_| miette::miette!("dispatcher thread panicked"))?;

    let total_processed = processed_counter.load(Ordering::Relaxed) as usize;
    let total_failed = failed_counter.load(Ordering::Relaxed) as usize;
    eprintln!(
        "  Phase 2 complete: {} analyzed, {} failed",
        total_processed, total_failed
    );

    Ok(())
}
