//! Phase 2: Coverage -- run transpilation analysis on parseable files.

#[cfg(feature = "duckdb")]
use std::fs;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use indicatif::{ProgressBar, ProgressStyle};
use miette::Result;
#[cfg(feature = "duckdb")]
use rayon::prelude::*;

#[cfg(feature = "duckdb")]
use crate::analyze;
#[cfg(feature = "duckdb")]
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
// NDJSON mode: Phase 2 with multi-process pipeline
// ---------------------------------------------------------------------------

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use crossbeam_channel::bounded;

use crate::scan::ndjson::{self, NdjsonWriter};

/// Run Phase 2 in NDJSON mode: multi-process pipeline for coverage analysis.
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

    // Multi-process pipeline: N workers with --with-coverage.
    let num_workers = rayon::current_num_threads();
    let result_cap = 1024;
    let (result_tx, result_rx) = bounded::<serde_json::Value>(result_cap);

    let exe_path = std::env::current_exe()
        .map_err(|e| miette::miette!("cannot find own executable: {e}"))?;

    let mut workers: Vec<(std::process::Child, BufWriter<std::process::ChildStdin>)> = Vec::new();
    let mut reader_threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 0..num_workers {
        let mut child = Command::new(&exe_path)
            .args([
                "scan-worker",
                "--run-id", &run_id.to_string(),
                "--with-coverage",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| miette::miette!("failed to spawn coverage worker {i}: {e}"))?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let tx = result_tx.clone();
        let reader = thread::Builder::new()
            .name(format!("cov-reader-{i}"))
            .spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(_) => break,
                    };
                    if line.is_empty() {
                        continue;
                    }
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&line) {
                        if tx.send(val).is_err() {
                            break;
                        }
                    }
                }
            })
            .map_err(|e| miette::miette!("failed to spawn reader thread {i}: {e}"))?;

        reader_threads.push(reader);
        workers.push((child, BufWriter::new(stdin)));
    }

    drop(result_tx);

    // Distribute files round-robin to workers.
    // work_items is Vec<(file_id, abs_path)>; we need rel_path too but Phase 2
    // doesn't use it -- send abs_path as both.
    let shutdown_feeder = shutdown.clone();
    let feeder = thread::Builder::new()
        .name("cov-feeder".into())
        .spawn(move || {
            for (idx, (file_id, abs_path)) in work_items.into_iter().enumerate() {
                if shutdown_feeder.load(Ordering::Relaxed) {
                    break;
                }

                let worker_idx = idx % num_workers;
                let line = format!("{}\t{}\t{}\n", file_id, abs_path, abs_path);
                let (_, ref mut stdin) = workers[worker_idx];
                if stdin.write_all(line.as_bytes()).is_err() {
                    break;
                }
            }

            for (mut child, stdin) in workers {
                drop(stdin);
                let _ = child.wait();
            }
        })
        .map_err(|e| miette::miette!("failed to spawn feeder thread: {e}"))?;

    // Writer loop: drain results from all workers.
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_secs(5);
    let mut results_since_flush = 0u64;

    while let Ok(val) = result_rx.recv() {
        let entry_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("");

        match entry_type {
            "coverage" => {
                writer.write_raw_coverage(&val)?;
                processed_counter.fetch_add(1, Ordering::Relaxed);
                results_since_flush += 1;
            }
            "diagnostic" => {
                writer.write_raw_diagnostic(&val)?;
            }
            "parse_result" | "copybook" => {
                // Phase 2 workers also emit parse_results (ignored here).
            }
            "error" => {
                let path = val.get("absolute_path").and_then(|v| v.as_str()).unwrap_or("?");
                let msg = val.get("error").and_then(|v| v.as_str()).unwrap_or("unknown");
                eprintln!("  [ERR] {path}: {msg}");
                failed_counter.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
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

    for reader in reader_threads {
        let _ = reader.join();
    }

    let total_processed = processed_counter.load(Ordering::Relaxed) as usize;
    let total_failed = failed_counter.load(Ordering::Relaxed) as usize;
    eprintln!(
        "  Phase 2 complete: {} analyzed, {} failed",
        total_processed, total_failed
    );

    Ok(())
}
