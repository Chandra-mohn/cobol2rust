//! Phase 1: Inventory -- parse all COBOL files and collect stats.

use std::collections::HashSet;
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
// NDJSON mode: Multi-process pipeline (avoids ANTLR RwLock<DFA> contention)
// ---------------------------------------------------------------------------

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use crossbeam_channel::bounded;

use crate::scan::ndjson::{self, NdjsonWriter};

/// Run Phase 1 in NDJSON mode using worker child processes.
///
/// Architecture:
///   Main process spawns N `cobol2rust scan-worker` child processes.
///   Files are distributed round-robin to workers via stdin pipes.
///   Each worker reads files, parses them (with its own ANTLR DFA caches),
///   and writes NDJSON results to stdout.
///   Reader threads collect results into a channel; writer loop on this
///   thread drains the channel and writes NDJSON output files.
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
    // Multi-process pipeline: N worker processes, each with own ANTLR DFA
    // -----------------------------------------------------------------------

    let num_workers = rayon::current_num_threads();
    let result_cap = 1024;
    let (result_tx, result_rx) = bounded::<serde_json::Value>(result_cap);

    // Find the path to our own executable.
    let exe_path = std::env::current_exe()
        .map_err(|e| miette::miette!("cannot find own executable: {e}"))?;

    // Spawn N worker child processes.
    let mut workers: Vec<(std::process::Child, BufWriter<std::process::ChildStdin>)> = Vec::new();
    let mut reader_threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 0..num_workers {
        let mut child = Command::new(&exe_path)
            .args(["scan-worker", "--run-id", &run_id.to_string()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit()) // Worker errors go to parent stderr.
            .spawn()
            .map_err(|e| miette::miette!("failed to spawn worker {i}: {e}"))?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        // Reader thread: reads NDJSON lines from worker stdout, sends to result channel.
        let tx = result_tx.clone();
        let reader = thread::Builder::new()
            .name(format!("reader-{i}"))
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

    // Drop the original sender so channel closes when all reader threads finish.
    drop(result_tx);

    // Distribute files via shared work queue (demand-based, not round-robin).
    // Each worker gets a feeder thread that pulls from a shared channel.
    // Fast workers pull more files; slow workers don't block others.
    let (work_tx, work_rx) = bounded::<(i64, String, String)>(256);
    let shutdown_feeder = shutdown.clone();

    // Producer: feeds all work items into the shared channel.
    let producer = thread::Builder::new()
        .name("scan-producer".into())
        .spawn(move || {
            for item in work_items {
                if shutdown_feeder.load(Ordering::Relaxed) {
                    break;
                }
                if work_tx.send(item).is_err() {
                    break;
                }
            }
            // Channel closes when work_tx is dropped.
        })
        .map_err(|e| miette::miette!("failed to spawn producer thread: {e}"))?;

    // Per-worker feeder threads: each pulls from shared channel, writes to its worker's stdin.
    let mut feeder_threads: Vec<thread::JoinHandle<()>> = Vec::new();
    for (mut child, mut stdin) in workers {
        let rx = work_rx.clone();
        let t = thread::Builder::new()
            .name("scan-feeder".into())
            .spawn(move || {
                while let Ok((file_id, abs_path, rel_path)) = rx.recv() {
                    let line = format!("{}\t{}\t{}\n", file_id, rel_path, abs_path);
                    if stdin.write_all(line.as_bytes()).is_err() {
                        break;
                    }
                }
                drop(stdin);
                let _ = child.wait();
            })
            .expect("failed to spawn feeder thread");
        feeder_threads.push(t);
    }
    drop(work_rx);

    // Writer loop: drain results from all workers, write NDJSON output.
    let mut last_flush = Instant::now();
    let flush_interval = Duration::from_secs(5);
    let mut results_since_flush = 0u64;

    while let Ok(val) = result_rx.recv() {
        let entry_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("");

        match entry_type {
            "parse_result" => {
                // Write parse result directly as JSON (worker already serialized it).
                writer.write_raw_parse_result(&val)?;
                processed_counter.fetch_add(1, Ordering::Relaxed);
                results_since_flush += 1;
            }
            "diagnostic" => {
                writer.write_raw_diagnostic(&val)?;
            }
            "copybook" => {
                writer.write_raw_copybook(&val)?;
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

    // Final flush.
    writer.flush()?;
    pb.finish_with_message("done");

    producer
        .join()
        .map_err(|_| miette::miette!("producer thread panicked"))?;

    for t in feeder_threads {
        let _ = t.join();
    }

    for reader in reader_threads {
        let _ = reader.join();
    }

    let total_processed = processed_counter.load(Ordering::Relaxed) as usize;
    let total_failed = failed_counter.load(Ordering::Relaxed) as usize;
    eprintln!(
        "  Phase 1 complete: {} parsed, {} failed, {} skipped",
        total_processed, total_failed, skipped
    );

    Ok(())
}
