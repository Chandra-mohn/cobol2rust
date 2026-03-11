//! Phase 1: Inventory -- parse all COBOL files and collect stats.

use std::collections::HashSet;
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
use crate::scan::discover::DiscoveredFile;

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

        // Write batch to DB on main thread (single connection).
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
