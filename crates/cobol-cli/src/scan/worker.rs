//! Scan worker process: reads file paths from stdin, writes NDJSON results to stdout.
//!
//! Each worker process has its own copy of the ANTLR DFA caches, avoiding the
//! `RwLock<DFA>` contention that serializes thread-level parallelism.

use std::fs;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use crate::analyze;

use super::args::ScanWorkerArgs;

/// Run the scan-worker loop: read `file_id\trel_path\tabs_path` from stdin,
/// parse each file, write one NDJSON line per result to stdout.
pub fn run(args: &ScanWorkerArgs) -> miette::Result<ExitCode> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse: file_id\trel_path\tabs_path
        let parts: Vec<&str> = line.splitn(3, '\t').collect();
        if parts.len() != 3 {
            continue;
        }

        let file_id: i64 = match parts[0].parse() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let rel_path = parts[1];
        let abs_path = parts[2];

        // Read file.
        let source = match fs::read_to_string(abs_path) {
            Ok(s) => s,
            Err(e) => {
                let entry = serde_json::json!({
                    "type": "error",
                    "file_id": file_id,
                    "absolute_path": abs_path,
                    "error": format!("read error: {e}"),
                });
                let _ = writeln!(out, "{}", entry);
                continue;
            }
        };

        // Parse (catch panics).
        let result = std::panic::catch_unwind(|| {
            analyze::analyze_source(&source, args.with_coverage)
        });

        match result {
            Ok(analysis) => {
                write_analysis(&mut out, file_id, args.run_id, rel_path, &analysis);
            }
            Err(_) => {
                let entry = serde_json::json!({
                    "type": "error",
                    "file_id": file_id,
                    "absolute_path": abs_path,
                    "error": "parser panicked",
                });
                let _ = writeln!(out, "{}", entry);
            }
        }
    }

    out.flush().ok();
    Ok(ExitCode::SUCCESS)
}

/// Write analysis result as a single NDJSON line to stdout.
fn write_analysis(
    out: &mut impl Write,
    file_id: i64,
    run_id: i64,
    rel_path: &str,
    a: &analyze::AnalysisResult,
) {
    // Parse result.
    let parse_entry = serde_json::json!({
        "type": "parse_result",
        "file_id": file_id,
        "run_id": run_id,
        "relative_path": rel_path,
        "program_id": a.program_id,
        "source_format": a.source_format,
        "valid": a.valid,
        "line_count": a.line_count,
        "parse_time_ms": a.parse_time_ms,
        "paragraphs": a.info.paragraphs,
        "sections": a.info.sections,
        "calls": a.info.calls,
        "file_ops": a.info.file_ops,
        "sql_stmts": a.info.sql_statements,
        "data_items": a.info.data_items,
        "is_subprogram": a.info.is_subprogram,
    });
    let _ = writeln!(out, "{}", parse_entry);

    // Errors.
    for diag in &a.errors {
        let entry = serde_json::json!({
            "type": "diagnostic",
            "file_id": file_id,
            "run_id": run_id,
            "phase": 1,
            "severity": "error",
            "line": diag.line,
            "message": diag.message,
            "code": diag.code,
            "category": diag.category,
        });
        let _ = writeln!(out, "{}", entry);
    }

    // Warnings.
    for diag in &a.warnings {
        let entry = serde_json::json!({
            "type": "diagnostic",
            "file_id": file_id,
            "run_id": run_id,
            "phase": 1,
            "severity": "warning",
            "line": diag.line,
            "message": diag.message,
            "code": diag.code,
            "category": diag.category,
        });
        let _ = writeln!(out, "{}", entry);
    }

    // Copybooks.
    for target in &a.copy_targets {
        let entry = serde_json::json!({
            "type": "copybook",
            "file_id": file_id,
            "run_id": run_id,
            "target": target,
        });
        let _ = writeln!(out, "{}", entry);
    }

    // Coverage (Phase 2).
    if let Some(ref cov) = a.coverage {
        let entry = serde_json::json!({
            "type": "coverage",
            "file_id": file_id,
            "run_id": run_id,
            "total_statements": cov.total_statements,
            "mapped_statements": cov.mapped_statements,
            "coverage_pct": cov.coverage_pct,
            "total_data_entries": cov.total_data_entries,
            "coverage_time_ms": cov.coverage_time_ms,
        });
        let _ = writeln!(out, "{}", entry);

        for diag in &cov.unhandled {
            let entry = serde_json::json!({
                "type": "diagnostic",
                "file_id": file_id,
                "run_id": run_id,
                "phase": 2,
                "severity": "warning",
                "line": diag.line,
                "message": diag.message,
                "code": diag.code,
                "category": diag.category,
            });
            let _ = writeln!(out, "{}", entry);
        }
    }
}
