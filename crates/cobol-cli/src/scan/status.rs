//! `--status` display: show current scan progress from the database.

use duckdb::{params, Connection};
use miette::{IntoDiagnostic, Result};

/// Print the current scan status and exit.
pub fn print_status(conn: &Connection) -> Result<()> {
    println!("cobol2rust scan -- Status");
    println!("=========================");
    println!();

    // Check for running/interrupted scans.
    let running = conn.query_row(
        "SELECT run_id, CAST(started_at AS VARCHAR), status, phase, total_files,
                processed_files, skipped_files, failed_files, worker_count, root_dir,
                EXTRACT(EPOCH FROM (current_timestamp - started_at)) as elapsed_secs
         FROM scan_runs
         WHERE status IN ('running', 'interrupted')
         ORDER BY run_id DESC LIMIT 1",
        [],
        |row| {
            Ok(RunInfo {
                run_id: row.get(0)?,
                started_at: row.get(1)?,
                status: row.get(2)?,
                phase: row.get(3)?,
                total_files: row.get(4)?,
                processed_files: row.get(5)?,
                skipped_files: row.get(6)?,
                failed_files: row.get(7)?,
                worker_count: row.get(8)?,
                root_dir: row.get(9)?,
                elapsed_secs: row.get(10)?,
            })
        },
    );

    match running {
        Ok(info) => {
            let db_display = conn
                .path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            println!("Database: {}", db_display);
            println!("Root dir: {}", info.root_dir);
            println!();
            println!(
                "Run #{} (started {}, {})",
                info.run_id, info.started_at, info.status
            );
            println!("  Phase:     {}", info.phase);
            println!("  Workers:   {}", info.worker_count);
            println!();

            let pct = if info.total_files > 0 {
                100.0 * info.processed_files as f64 / info.total_files as f64
            } else {
                0.0
            };

            println!("  Files discovered:  {:>10}", info.total_files);
            println!(
                "  Files processed:   {:>10}  ({:.1}%)",
                info.processed_files, pct
            );
            if info.skipped_files > 0 {
                println!(
                    "  Files skipped:     {:>10}  (incremental)",
                    info.skipped_files
                );
            }
            if info.failed_files > 0 {
                println!("  Files failed:      {:>10}", info.failed_files);
            }

            // Throughput estimate.
            if info.elapsed_secs > 0.0 && info.processed_files > 0 {
                let throughput = info.processed_files as f64 / info.elapsed_secs;
                let remaining = info.total_files - info.processed_files - info.skipped_files;
                let eta_secs = remaining as f64 / throughput;

                println!();
                println!("  Throughput: ~{:.0} files/sec", throughput);
                if eta_secs > 0.0 {
                    println!("  Estimated remaining: {}", format_duration(eta_secs));
                }
            }
        }
        Err(_) => {
            println!("No scan currently running.");
        }
    }

    // Previous completed runs.
    println!();
    println!("Recent completed runs:");

    let mut stmt = conn
        .prepare(
            "SELECT run_id, CAST(started_at AS VARCHAR), CAST(finished_at AS VARCHAR),
                    phase, total_files, processed_files, failed_files, status
             FROM scan_runs
             ORDER BY run_id DESC LIMIT 5",
        )
        .into_diagnostic()?;

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, String>(7)?,
            ))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (run_id, started, finished, phase, total, processed, failed, status) = row;
        let finished_str = finished.unwrap_or_else(|| "--".to_string());
        println!(
            "  Run #{run_id}  {started} -> {finished_str}  Phase: {phase}  \
             {processed}/{total} files ({failed} failed)  [{status}]"
        );
    }

    // Quick stats from most recent completed run.
    if let Ok(run_id) = conn.query_row(
        "SELECT run_id FROM scan_runs WHERE status = 'completed' ORDER BY run_id DESC LIMIT 1",
        [],
        |row| row.get::<_, i64>(0),
    ) {
        if let Ok((valid, invalid)) = conn.query_row(
            "SELECT SUM(CASE WHEN valid THEN 1 ELSE 0 END),
                    SUM(CASE WHEN NOT valid THEN 1 ELSE 0 END)
             FROM parse_results WHERE run_id = ?",
            params![run_id],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        ) {
            let total = valid + invalid;
            let pct = if total > 0 {
                100.0 * valid as f64 / total as f64
            } else {
                0.0
            };
            println!();
            println!("  Parse success: {:.1}% ({valid}/{total})", pct);
        }

        if let Ok(avg_cov) = conn.query_row(
            "SELECT ROUND(AVG(coverage_pct), 1) FROM coverage_results WHERE run_id = ?",
            params![run_id],
            |row| row.get::<_, f64>(0),
        ) {
            println!("  Coverage avg:  {avg_cov:.1}%");
        }
    }

    Ok(())
}

struct RunInfo {
    run_id: i64,
    started_at: String,
    status: String,
    phase: String,
    total_files: i64,
    processed_files: i64,
    skipped_files: i64,
    failed_files: i64,
    worker_count: i64,
    root_dir: String,
    elapsed_secs: f64,
}

fn format_duration(secs: f64) -> String {
    let s = secs as u64;
    if s < 60 {
        format!("{s}s")
    } else if s < 3600 {
        format!("{}m {}s", s / 60, s % 60)
    } else {
        format!("{}h {}m {}s", s / 3600, (s % 3600) / 60, s % 60)
    }
}
