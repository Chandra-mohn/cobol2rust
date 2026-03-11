//! Phase 1: Inventory -- parse all COBOL files and collect stats.

use std::collections::HashSet;
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
use crate::scan::discover::DiscoveredFile;

// ---------------------------------------------------------------------------
// DuckDB mode: Phase 1 with DuckDB persistence
// ---------------------------------------------------------------------------

#[cfg(feature = "duckdb")]
mod duckdb_phase1 {
    use super::*;
    use duckdb::Connection;
    use crate::scan::db;

    /// Work item for Phase 1: a file to parse.
    #[derive(Debug, Clone)]
    pub struct Phase1WorkItem {
        pub file_id: i64,
        pub absolute_path: String,
    }

    /// Result of analyzing one file (to be sent back to main thread for DB write).
    struct Phase1Result {
        file_id: i64,
        analysis: AnalysisResult,
    }

    /// Run Phase 1: parse all source files in parallel, write results on main thread.
    pub fn run_phase1(
        conn: &Connection,
        run_id: i64,
        files: &[DiscoveredFile],
        processed_counter: &Arc<AtomicU64>,
        failed_counter: &Arc<AtomicU64>,
        shutdown: &Arc<AtomicBool>,
        batch_size: usize,
        incremental: bool,
        resume: bool,
    ) -> Result<()> {
        // Bulk-register all files in the DB using DuckDB Appender (fast path).
        eprintln!("  Registering {} files in database...", files.len());
        let file_id_map = db::bulk_register_files(conn, files, run_id)?;
        eprintln!("  Registration complete ({} files indexed)", file_id_map.len());

        let mut work_items = Vec::new();
        let mut skipped = 0usize;

        // Get already-processed file IDs for resume.
        let processed_ids: HashSet<i64> = if resume {
            db::get_processed_file_ids(conn, run_id)?
                .into_iter()
                .collect()
        } else {
            HashSet::new()
        };

        for file in files {
            // Only process source files in Phase 1.
            if file.file_type != crate::scan::discover::FileType::Source {
                continue;
            }

            let file_id = match file_id_map.get(&file.relative_path) {
                Some(&id) => id,
                None => {
                    eprintln!("  [WARN] No file_id for: {}", file.relative_path);
                    continue;
                }
            };

            // Skip if already processed (resume mode).
            if resume && processed_ids.contains(&file_id) {
                skipped += 1;
                continue;
            }

            // Skip if incremental and mtime unchanged.
            if incremental && !resume {
                let prev_mtime: Option<i64> = conn
                    .query_row(
                        "SELECT f.mtime FROM files f
                         JOIN parse_results pr ON f.file_id = pr.file_id
                         WHERE f.file_id = ? AND pr.valid = true
                         ORDER BY pr.run_id DESC LIMIT 1",
                        duckdb::params![file_id],
                        |row| row.get(0),
                    )
                    .ok();

                if prev_mtime == Some(file.mtime_epoch) {
                    skipped += 1;
                    continue;
                }
            }

            work_items.push(Phase1WorkItem {
                file_id,
                absolute_path: file.absolute_path.clone(),
            });
        }

        let total_work = work_items.len();
        eprintln!(
            "  Phase 1: {} files to parse ({} skipped)",
            total_work, skipped
        );

        if total_work == 0 {
            return Ok(());
        }

        db::update_scan_run_counts(conn, run_id, total_work + skipped, 0, skipped, 0)?;

        let pb = ProgressBar::new(total_work as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "  Phase 1 [{bar:40}] {pos}/{len} ({per_sec}) ETA: {eta}",
            )
            .unwrap()
            .progress_chars("=> "),
        );

        // Process in chunks: parallel parse -> sequential write.
        let chunks: Vec<&[Phase1WorkItem]> = work_items.chunks(batch_size).collect();

        for chunk in chunks {
            if shutdown.load(Ordering::Relaxed) {
                eprintln!("\n  Scan interrupted. Use --resume to continue.");
                break;
            }

            // Parse in parallel.
            let results: Vec<Phase1Result> = chunk
                .par_iter()
                .filter_map(|item| {
                    if shutdown.load(Ordering::Relaxed) {
                        return None;
                    }

                    let source = match fs::read_to_string(&item.absolute_path) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("  [ERR] Cannot read {}: {e}", item.absolute_path);
                            failed_counter.fetch_add(1, Ordering::Relaxed);
                            pb.inc(1);
                            return None;
                        }
                    };

                    let analysis = std::panic::catch_unwind(|| {
                        analyze::analyze_source(&source, false)
                    });

                    match analysis {
                        Ok(result) => {
                            pb.inc(1);
                            Some(Phase1Result {
                                file_id: item.file_id,
                                analysis: result,
                            })
                        }
                        Err(_) => {
                            eprintln!("  [ERR] Parser panicked on: {}", item.absolute_path);
                            failed_counter.fetch_add(1, Ordering::Relaxed);
                            pb.inc(1);
                            None
                        }
                    }
                })
                .collect();

            // Write entire batch in a single transaction (avoids per-row auto-commit).
            conn.execute("BEGIN TRANSACTION", [])
                .map_err(|e| miette::miette!("failed to begin transaction: {e}"))?;

            for entry in &results {
                match write_phase1_entry(conn, run_id, entry) {
                    Ok(()) => {
                        processed_counter.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(e) => {
                        eprintln!("  [ERR] DB write failed for file_id {}: {e}", entry.file_id);
                        failed_counter.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }

            conn.execute("COMMIT", [])
                .map_err(|e| miette::miette!("failed to commit transaction: {e}"))?;
        }

        pb.finish_with_message("done");

        let total_processed = processed_counter.load(Ordering::Relaxed) as usize;
        let total_failed = failed_counter.load(Ordering::Relaxed) as usize;
        eprintln!(
            "  Phase 1 complete: {} parsed, {} failed, {} skipped",
            total_processed, total_failed, skipped
        );

        Ok(())
    }

    fn write_phase1_entry(
        conn: &Connection,
        run_id: i64,
        entry: &Phase1Result,
    ) -> miette::Result<()> {
        let file_id = entry.file_id;
        let a = &entry.analysis;

        db::insert_parse_result(conn, file_id, run_id, a)?;

        if !a.errors.is_empty() {
            db::insert_diagnostics(conn, file_id, run_id, 1, &a.errors, "error")?;
        }
        if !a.warnings.is_empty() {
            db::insert_diagnostics(conn, file_id, run_id, 1, &a.warnings, "warning")?;
        }
        if !a.copy_targets.is_empty() {
            db::insert_copybooks(conn, run_id, file_id, &a.copy_targets)?;
        }

        let status = if a.valid { "parsed" } else { "failed" };
        db::update_file_status(conn, file_id, status, run_id)?;

        Ok(())
    }
}

#[cfg(feature = "duckdb")]
pub use duckdb_phase1::run_phase1;

// ---------------------------------------------------------------------------
// NDJSON mode: Streaming pipeline (no batch barrier)
// ---------------------------------------------------------------------------

use std::path::Path;
use std::thread;

use crossbeam_channel::bounded;

use crate::scan::ndjson::{self, NdjsonWriter};

/// Job sent from feeder -> rayon parse pool.
struct ParseJob {
    file_id: i64,
    absolute_path: String,
    relative_path: String,
    source: String,
}

/// Result sent from rayon parse pool -> writer.
enum ParseResult {
    Success {
        file_id: i64,
        relative_path: String,
        analysis: AnalysisResult,
    },
    ParsePanic {
        absolute_path: String,
    },
    ReadError {
        absolute_path: String,
    },
}

/// Run Phase 1 in NDJSON mode: streaming pipeline, no batch barrier.
///
/// Architecture:
///   [Feeder thread] --bounded(job_cap)--> [Rayon pool] --bounded(256)--> [Writer on caller thread]
///
/// Each file flows independently. A slow 28MB file ties up 1 core; the other N-1 keep working.
#[allow(clippy::too_many_arguments)]
pub fn run_phase1_ndjson(
    writer: &mut NdjsonWriter,
    output_dir: &Path,
    run_id: i64,
    files: &[DiscoveredFile],
    processed_counter: &Arc<AtomicU64>,
    failed_counter: &Arc<AtomicU64>,
    shutdown: &Arc<AtomicBool>,
    _batch_size: usize,
    resume: bool,
) -> Result<()> {
    // Register all files in NDJSON (fast: pure sequential I/O).
    eprintln!("  Registering {} files...", files.len());
    let file_id_map = if resume {
        let mut existing = ndjson::load_file_id_map(output_dir)?;
        let mut new_files: Vec<&DiscoveredFile> = Vec::new();
        for file in files {
            if !existing.contains_key(&file.relative_path) {
                new_files.push(file);
            }
        }
        if !new_files.is_empty() {
            let new_map = writer.register_files(
                &new_files.iter().copied().cloned().collect::<Vec<_>>(),
                run_id,
            )?;
            existing.extend(new_map);
        }
        existing
    } else {
        writer.register_files(files, run_id)?
    };
    eprintln!("  Registration complete ({} files indexed)", file_id_map.len());

    // Build work items (source files only, skip already-processed for resume).
    let processed_paths: HashSet<String> = if resume {
        ndjson::load_processed_paths(output_dir)?
    } else {
        HashSet::new()
    };

    let mut work_items: Vec<(i64, String, String)> = Vec::new(); // (file_id, abs_path, rel_path)
    let mut skipped = 0usize;

    for file in files {
        if file.file_type != crate::scan::discover::FileType::Source {
            continue;
        }

        let file_id = match file_id_map.get(&file.relative_path) {
            Some(&id) => id,
            None => continue,
        };

        if resume && processed_paths.contains(&file.relative_path) {
            skipped += 1;
            continue;
        }

        work_items.push((file_id, file.absolute_path.clone(), file.relative_path.clone()));
    }

    let total_work = work_items.len();
    eprintln!(
        "  Phase 1: {} files to parse ({} skipped)",
        total_work, skipped
    );

    if total_work == 0 {
        return Ok(());
    }

    let pb = ProgressBar::new(total_work as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "  Phase 1 [{bar:40}] {pos}/{len} ({per_sec}) ETA: {eta}",
        )
        .unwrap()
        .progress_chars("=> "),
    );

    // -----------------------------------------------------------------------
    // Streaming pipeline: feeder -> rayon pool -> writer (this thread)
    // -----------------------------------------------------------------------

    // Channel capacities: job_cap = 2x cores to keep rayon fed; result_cap = 256 for burst absorption.
    let num_threads = rayon::current_num_threads();
    let job_cap = num_threads * 2;
    let result_cap = 256;

    let (job_tx, job_rx) = bounded::<ParseJob>(job_cap);
    let (result_tx, result_rx) = bounded::<ParseResult>(result_cap);

    let shutdown_feeder = shutdown.clone();
    let shutdown_parser = shutdown.clone();

    // Stage 1: Feeder thread -- reads files from disk, sends source text to parse pool.
    let feeder_result_tx = result_tx.clone();
    let feeder = thread::Builder::new()
        .name("scan-feeder".into())
        .spawn(move || {
            for (file_id, absolute_path, relative_path) in work_items {
                if shutdown_feeder.load(Ordering::Relaxed) {
                    break;
                }

                match fs::read_to_string(&absolute_path) {
                    Ok(source) => {
                        let job = ParseJob {
                            file_id,
                            absolute_path,
                            relative_path,
                            source,
                        };
                        // Blocks if channel is full -- backpressure limits RAM.
                        if job_tx.send(job).is_err() {
                            break; // Receiver dropped, pipeline shutting down.
                        }
                    }
                    Err(_) => {
                        let _ = feeder_result_tx.send(ParseResult::ReadError {
                            absolute_path,
                        });
                    }
                }
            }
            // Dropping job_tx signals the dispatcher that no more jobs are coming.
        })
        .map_err(|e| miette::miette!("failed to spawn feeder thread: {e}"))?;

    // Stage 2: Dispatcher thread -- pulls jobs from channel, spawns rayon tasks.
    let dispatcher = thread::Builder::new()
        .name("scan-dispatcher".into())
        .spawn(move || {
            // Use a scope to wait for all in-flight rayon tasks to complete.
            rayon::scope(|s| {
                while let Ok(job) = job_rx.recv() {
                    if shutdown_parser.load(Ordering::Relaxed) {
                        break;
                    }

                    let tx = result_tx.clone();
                    s.spawn(move |_| {
                        let outcome = std::panic::catch_unwind(|| {
                            analyze::analyze_source(&job.source, false)
                        });

                        let result = match outcome {
                            Ok(analysis) => ParseResult::Success {
                                file_id: job.file_id,
                                relative_path: job.relative_path,
                                analysis,
                            },
                            Err(_) => ParseResult::ParsePanic {
                                absolute_path: job.absolute_path,
                            },
                        };

                        let _ = tx.send(result);
                        // job.source dropped here -- frees RAM immediately.
                    });
                }
            });
            // rayon::scope waits for all spawned tasks. Then result_tx is dropped,
            // closing the result channel and signaling the writer.
        })
        .map_err(|e| miette::miette!("failed to spawn dispatcher thread: {e}"))?;

    // Stage 3: Writer loop -- runs on THIS thread. Drains results as they arrive.
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_secs(5);
    let mut results_since_flush = 0u64;

    while let Ok(result) = result_rx.recv() {
        match result {
            ParseResult::Success { file_id, relative_path, analysis } => {
                writer.write_parse_result(file_id, run_id, &relative_path, &analysis)?;

                if !analysis.errors.is_empty() {
                    writer.write_diagnostics(file_id, run_id, 1, &analysis.errors, "error")?;
                }
                if !analysis.warnings.is_empty() {
                    writer.write_diagnostics(file_id, run_id, 1, &analysis.warnings, "warning")?;
                }
                if !analysis.copy_targets.is_empty() {
                    writer.write_copybooks(run_id, file_id, &analysis.copy_targets)?;
                }

                processed_counter.fetch_add(1, Ordering::Relaxed);
                results_since_flush += 1;
            }
            ParseResult::ParsePanic { absolute_path } => {
                eprintln!("  [ERR] Parser panicked on: {absolute_path}");
                failed_counter.fetch_add(1, Ordering::Relaxed);
            }
            ParseResult::ReadError { absolute_path } => {
                eprintln!("  [ERR] Cannot read: {absolute_path}");
                failed_counter.fetch_add(1, Ordering::Relaxed);
            }
        }

        pb.inc(1);

        // Flush every 500 results or every 5 seconds for crash recovery.
        if results_since_flush >= 500 || last_flush.elapsed() >= flush_interval {
            writer.flush()?;
            results_since_flush = 0;
            last_flush = Instant::now();
        }
    }

    // Final flush.
    writer.flush()?;
    pb.finish_with_message("done");

    // Wait for pipeline threads to finish (they should already be done since
    // result_rx.recv() returned Err, meaning all senders dropped).
    feeder
        .join()
        .map_err(|_| miette::miette!("feeder thread panicked"))?;
    dispatcher
        .join()
        .map_err(|_| miette::miette!("dispatcher thread panicked"))?;

    let total_processed = processed_counter.load(Ordering::Relaxed) as usize;
    let total_failed = failed_counter.load(Ordering::Relaxed) as usize;
    eprintln!(
        "  Phase 1 complete: {} parsed, {} failed, {} skipped",
        total_processed, total_failed, skipped
    );

    Ok(())
}
