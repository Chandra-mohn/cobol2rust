//! Phase 3: Reporting -- query DuckDB and produce aggregated reports.

use duckdb::{params, Connection};
use miette::{IntoDiagnostic, Result};

use crate::scan::args::{ReportFormat, ReportType};

/// Run Phase 3: generate reports from the database.
pub fn run_phase3(
    conn: &Connection,
    run_id: i64,
    report_type: ReportType,
    format: ReportFormat,
) -> Result<()> {
    match report_type {
        ReportType::Summary => print_summary(conn, run_id, format),
        ReportType::Coverage => print_coverage(conn, run_id, format),
        ReportType::Errors => print_errors(conn, run_id, format),
        ReportType::Complexity => print_complexity(conn, run_id, format),
        ReportType::Full => {
            print_summary(conn, run_id, format)?;
            println!();
            print_coverage(conn, run_id, format)?;
            println!();
            print_errors(conn, run_id, format)?;
            println!();
            print_complexity(conn, run_id, format)?;
            Ok(())
        }
    }
}

fn print_summary(conn: &Connection, run_id: i64, format: ReportFormat) -> Result<()> {
    match format {
        ReportFormat::Text => print_summary_text(conn, run_id),
        ReportFormat::Json => print_query_json(conn, SUMMARY_QUERIES, run_id),
        ReportFormat::Csv => print_query_csv(conn, SUMMARY_QUERIES, run_id),
    }
}

fn print_coverage(conn: &Connection, run_id: i64, format: ReportFormat) -> Result<()> {
    match format {
        ReportFormat::Text => print_coverage_text(conn, run_id),
        ReportFormat::Json => print_query_json(conn, COVERAGE_QUERIES, run_id),
        ReportFormat::Csv => print_query_csv(conn, COVERAGE_QUERIES, run_id),
    }
}

fn print_errors(conn: &Connection, run_id: i64, format: ReportFormat) -> Result<()> {
    match format {
        ReportFormat::Text => print_errors_text(conn, run_id),
        ReportFormat::Json => print_query_json(conn, ERROR_QUERIES, run_id),
        ReportFormat::Csv => print_query_csv(conn, ERROR_QUERIES, run_id),
    }
}

fn print_complexity(conn: &Connection, run_id: i64, format: ReportFormat) -> Result<()> {
    match format {
        ReportFormat::Text => print_complexity_text(conn, run_id),
        ReportFormat::Json => print_query_json(conn, COMPLEXITY_QUERIES, run_id),
        ReportFormat::Csv => print_query_csv(conn, COMPLEXITY_QUERIES, run_id),
    }
}

// ---------------------------------------------------------------------------
// Text formatters
// ---------------------------------------------------------------------------

fn print_summary_text(conn: &Connection, run_id: i64) -> Result<()> {
    println!("=== SUMMARY REPORT ===");
    println!();

    // File counts by type
    println!("--- File Inventory ---");
    let mut stmt = conn
        .prepare(
            "SELECT file_type, status, COUNT(*) as cnt, SUM(file_size) as total_bytes
             FROM files WHERE last_scan_run = ?
             GROUP BY file_type, status ORDER BY file_type, status",
        )
        .into_diagnostic()?;

    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .into_diagnostic()?;

    println!("{:<12} {:<10} {:>10} {:>15}", "Type", "Status", "Files", "Size (MB)");
    println!("{}", "-".repeat(50));
    for row in rows.flatten() {
        let (ftype, status, cnt, bytes) = row;
        let mb = bytes as f64 / 1_048_576.0;
        println!("{:<12} {:<10} {:>10} {:>14.1}", ftype, status, cnt, mb);
    }
    println!();

    // Parse success rate
    println!("--- Parse Results ---");
    let parse_stats: (i64, i64, i64) = conn
        .query_row(
            "SELECT COUNT(*) as total,
                    SUM(CASE WHEN valid THEN 1 ELSE 0 END),
                    SUM(CASE WHEN NOT valid THEN 1 ELSE 0 END)
             FROM parse_results WHERE run_id = ?",
            params![run_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .into_diagnostic()?;

    let (total, valid, invalid) = parse_stats;
    let pct = if total > 0 {
        100.0 * valid as f64 / total as f64
    } else {
        0.0
    };
    println!("Total files checked: {total}");
    println!("Valid:   {valid} ({pct:.1}%)");
    println!("Invalid: {invalid}");
    println!();

    // Line count stats
    if let Ok((total_lines, avg_lines, max_lines)) = conn.query_row(
        "SELECT SUM(line_count), AVG(line_count), MAX(line_count)
         FROM parse_results WHERE run_id = ? AND valid = true",
        params![run_id],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        },
    ) {
        println!("--- Line Counts ---");
        println!("Total lines:   {total_lines}");
        println!("Avg per file:  {avg_lines:.0}");
        println!("Largest file:  {max_lines} lines");
        println!();
    }

    // Feature usage
    println!("--- Feature Usage ---");
    if let Ok((sql_progs, fileio_progs, call_progs, total_sql, total_fileops, total_calls)) =
        conn.query_row(
            "SELECT
                SUM(CASE WHEN sql_statements > 0 THEN 1 ELSE 0 END),
                SUM(CASE WHEN file_ops > 0 THEN 1 ELSE 0 END),
                SUM(CASE WHEN calls > 0 THEN 1 ELSE 0 END),
                SUM(sql_statements), SUM(file_ops), SUM(calls)
             FROM parse_results WHERE run_id = ? AND valid = true",
            params![run_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                ))
            },
        )
    {
        println!("Programs with SQL:     {sql_progs} ({total_sql} total SQL statements)");
        println!("Programs with File IO: {fileio_progs} ({total_fileops} total file ops)");
        println!("Programs with CALLs:   {call_progs} ({total_calls} total CALL statements)");
    }

    // Program type breakdown
    println!();
    println!("--- Program Types ---");
    let mut stmt = conn
        .prepare(
            "SELECT
                CASE WHEN is_subprogram THEN 'subprogram' ELSE 'main' END as ptype,
                COUNT(*)
             FROM parse_results WHERE run_id = ? AND valid = true
             GROUP BY is_subprogram",
        )
        .into_diagnostic()?;

    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (ptype, cnt) = row;
        println!("  {ptype}: {cnt}");
    }

    Ok(())
}

fn print_coverage_text(conn: &Connection, run_id: i64) -> Result<()> {
    println!("=== COVERAGE REPORT ===");
    println!();

    // Overall stats
    if let Ok((files, avg, median, min, max, total_stmts, mapped, weighted)) = conn.query_row(
        "SELECT COUNT(*),
                ROUND(AVG(coverage_pct), 2),
                ROUND(PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY coverage_pct), 2),
                MIN(coverage_pct), MAX(coverage_pct),
                SUM(total_statements), SUM(mapped_statements),
                ROUND(100.0 * SUM(mapped_statements) / NULLIF(SUM(total_statements), 0), 2)
         FROM coverage_results WHERE run_id = ?",
        params![run_id],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, f64>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, f64>(7)?,
            ))
        },
    ) {
        println!("Files analyzed: {files}");
        println!("Average coverage:  {avg:.1}%");
        println!("Median coverage:   {median:.1}%");
        println!("Min / Max:         {min:.1}% / {max:.1}%");
        println!("Total statements:  {total_stmts} ({mapped} mapped = {weighted:.1}% weighted)");
        println!();
    }

    // Distribution histogram
    println!("--- Coverage Distribution ---");
    let mut stmt = conn
        .prepare(
            "SELECT
                FLOOR(coverage_pct / 10) * 10 as bucket_low,
                FLOOR(coverage_pct / 10) * 10 + 10 as bucket_high,
                COUNT(*) as cnt
             FROM coverage_results WHERE run_id = ?
             GROUP BY FLOOR(coverage_pct / 10)
             ORDER BY bucket_low",
        )
        .into_diagnostic()?;

    let rows: Vec<(f64, f64, i64)> = stmt
        .query_map(params![run_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .into_diagnostic()?
        .filter_map(|r| r.ok())
        .collect();

    let max_count = rows.iter().map(|r| r.2).max().unwrap_or(1);
    for (low, high, cnt) in &rows {
        let bar_len = (*cnt as f64 / max_count as f64 * 40.0) as usize;
        let bar: String = "#".repeat(bar_len);
        println!("{:>3}% - {:>3}%  {:>8}  {}", *low as i64, *high as i64, cnt, bar);
    }
    println!();

    // Top 20 lowest coverage
    println!("--- Lowest Coverage (migration risk) ---");
    let mut stmt = conn
        .prepare(
            "SELECT f.path, pr.program_id, cr.coverage_pct, cr.total_statements, cr.unhandled_count
             FROM coverage_results cr
             JOIN files f ON cr.file_id = f.file_id
             JOIN parse_results pr ON cr.file_id = pr.file_id AND cr.run_id = pr.run_id
             WHERE cr.run_id = ?
             ORDER BY cr.coverage_pct ASC LIMIT 20",
        )
        .into_diagnostic()?;

    println!("{:<50} {:<16} {:>8} {:>8} {:>8}", "File", "Program-ID", "Cov%", "Stmts", "Unhandled");
    println!("{}", "-".repeat(95));
    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (path, pid, cov, stmts, unhandled) = row;
        let short_path = truncate_path(&path, 48);
        println!("{:<50} {:<16} {:>7.1} {:>8} {:>8}", short_path, pid, cov, stmts, unhandled);
    }

    Ok(())
}

fn print_errors_text(conn: &Connection, run_id: i64) -> Result<()> {
    println!("=== ERROR REPORT ===");
    println!();

    // Top error codes
    println!("--- Top Error Codes ---");
    let mut stmt = conn
        .prepare(
            "SELECT code, severity, COUNT(*) as cnt, COUNT(DISTINCT file_id) as files
             FROM diagnostics WHERE run_id = ?
             GROUP BY code, severity ORDER BY cnt DESC LIMIT 20",
        )
        .into_diagnostic()?;

    println!("{:<10} {:<10} {:>10} {:>10}", "Code", "Severity", "Count", "Files");
    println!("{}", "-".repeat(45));
    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (code, severity, cnt, files) = row;
        println!("{:<10} {:<10} {:>10} {:>10}", code, severity, cnt, files);
    }
    println!();

    // Most common error messages
    println!("--- Most Common Error Messages ---");
    let mut stmt = conn
        .prepare(
            "SELECT LEFT(message, 80) as msg, COUNT(*) as cnt, COUNT(DISTINCT file_id) as files
             FROM diagnostics WHERE run_id = ? AND severity = 'error'
             GROUP BY LEFT(message, 80) ORDER BY cnt DESC LIMIT 15",
        )
        .into_diagnostic()?;

    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (msg, cnt, files) = row;
        println!("  [{cnt} occurrences, {files} files] {msg}");
    }
    println!();

    // Unresolved copybooks
    println!("--- Unresolved Copybooks ---");
    let mut stmt = conn
        .prepare(
            "SELECT name, COUNT(*) as refs, COUNT(DISTINCT referenced_by) as files
             FROM copybooks WHERE run_id = ? AND resolved = false
             GROUP BY name ORDER BY refs DESC LIMIT 20",
        )
        .into_diagnostic()?;

    println!("{:<30} {:>10} {:>10}", "Copybook", "References", "Files");
    println!("{}", "-".repeat(55));
    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (name, refs, files) = row;
        println!("{:<30} {:>10} {:>10}", name, refs, files);
    }

    Ok(())
}

fn print_complexity_text(conn: &Connection, run_id: i64) -> Result<()> {
    println!("=== COMPLEXITY REPORT ===");
    println!();

    // Complexity distribution
    println!("--- Complexity Distribution ---");
    let mut stmt = conn
        .prepare(
            "SELECT
                CASE
                    WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 10 THEN 'trivial (0-9)'
                    WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 50 THEN 'simple (10-49)'
                    WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 200 THEN 'moderate (50-199)'
                    WHEN (paragraphs + 2*calls + 3*file_ops + 3*sql_statements) < 500 THEN 'complex (200-499)'
                    ELSE 'very complex (500+)'
                END as tier,
                COUNT(*) as cnt,
                SUM(line_count) as total_lines
             FROM parse_results WHERE run_id = ? AND valid = true
             GROUP BY tier
             ORDER BY MIN(paragraphs + 2*calls + 3*file_ops + 3*sql_statements)",
        )
        .into_diagnostic()?;

    println!("{:<25} {:>10} {:>15}", "Tier", "Files", "Total Lines");
    println!("{}", "-".repeat(55));
    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (tier, cnt, lines) = row;
        println!("{:<25} {:>10} {:>15}", tier, cnt, lines);
    }
    println!();

    // Top 30 most complex programs
    println!("--- Top 30 Most Complex Programs ---");
    let mut stmt = conn
        .prepare(
            "SELECT f.path, pr.program_id, pr.line_count, pr.paragraphs,
                    pr.calls, pr.file_ops, pr.sql_statements,
                    (pr.paragraphs + 2*pr.calls + 3*pr.file_ops + 3*pr.sql_statements) as score
             FROM parse_results pr
             JOIN files f ON pr.file_id = f.file_id
             WHERE pr.run_id = ? AND pr.valid = true
             ORDER BY score DESC LIMIT 30",
        )
        .into_diagnostic()?;

    println!(
        "{:<40} {:<14} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
        "File", "Program-ID", "Lines", "Paras", "Calls", "FileIO", "SQL", "Score"
    );
    println!("{}", "-".repeat(95));
    let rows = stmt
        .query_map(params![run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, i64>(7)?,
            ))
        })
        .into_diagnostic()?;

    for row in rows.flatten() {
        let (path, pid, lines, paras, calls, fileio, sql, score) = row;
        let short_path = truncate_path(&path, 38);
        println!(
            "{:<40} {:<14} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
            short_path, pid, lines, paras, calls, fileio, sql, score
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// JSON / CSV formatters (generic query runner)
// ---------------------------------------------------------------------------

fn print_query_json(
    conn: &Connection,
    queries: &[(&str, &str)],
    run_id: i64,
) -> Result<()> {
    use std::collections::BTreeMap;

    let mut result = BTreeMap::new();

    for (name, sql) in queries {
        let rows = execute_query_to_rows(conn, sql, run_id)?;
        let json_rows: Vec<serde_json::Value> = rows
            .into_iter()
            .map(serde_json::Value::Object)
            .collect();
        result.insert(*name, json_rows);
    }

    let json = serde_json::to_string_pretty(&result)
        .into_diagnostic()
        .map_err(|e| miette::miette!("JSON serialization failed: {e}"))?;
    println!("{json}");
    Ok(())
}

fn print_query_csv(
    conn: &Connection,
    queries: &[(&str, &str)],
    run_id: i64,
) -> Result<()> {
    for (name, sql) in queries {
        println!("# {name}");
        // Use a wrapper query that DuckDB can introspect column names from.
        let rows = execute_query_to_rows(conn, sql, run_id)?;
        if let Some(first) = rows.first() {
            let keys: Vec<&str> = first.keys().map(String::as_str).collect();
            println!("{}", keys.join(","));
            for row in &rows {
                let vals: Vec<&str> = first
                    .keys()
                    .map(|k| row.get(k).and_then(|v| v.as_str()).unwrap_or(""))
                    .collect();
                println!("{}", vals.join(","));
            }
        }
        println!();
    }
    Ok(())
}

fn execute_query_to_rows(
    conn: &Connection,
    sql: &str,
    run_id: i64,
) -> Result<Vec<serde_json::Map<String, serde_json::Value>>> {
    // Wrap the query so all columns are cast to VARCHAR for easy extraction.
    let mut stmt = conn.prepare(sql).into_diagnostic()?;

    let mut rows_out = Vec::new();
    let mut duckdb_rows = stmt.query(params![run_id]).into_diagnostic()?;

    // Get column names after first step.
    let col_count = duckdb_rows.as_ref().map_or(0, |s| s.column_count());
    let col_names: Vec<String> = if col_count > 0 {
        (0..col_count)
            .map(|i| {
                duckdb_rows
                    .as_ref()
                    .and_then(|s| s.column_name(i).ok())
                    .map_or_else(|| format!("col{i}"), |s| s.clone())
            })
            .collect()
    } else {
        Vec::new()
    };

    while let Some(row) = duckdb_rows.next().into_diagnostic()? {
        let mut map = serde_json::Map::new();
        for (i, name) in col_names.iter().enumerate() {
            let val = get_column_as_string(row, i);
            map.insert(name.clone(), serde_json::Value::String(val));
        }
        rows_out.push(map);
    }

    Ok(rows_out)
}

/// Extract a column value as a string, trying common types.
fn get_column_as_string(row: &duckdb::Row<'_>, idx: usize) -> String {
    // Try string first.
    if let Ok(v) = row.get::<_, String>(idx) {
        return v;
    }
    // Try i64.
    if let Ok(v) = row.get::<_, i64>(idx) {
        return v.to_string();
    }
    // Try f64.
    if let Ok(v) = row.get::<_, f64>(idx) {
        return v.to_string();
    }
    // Try bool.
    if let Ok(v) = row.get::<_, bool>(idx) {
        return v.to_string();
    }
    String::new()
}

/// Truncate a path string for display.
fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - max_len + 3..])
    }
}

// ---------------------------------------------------------------------------
// Query constants for JSON/CSV output
// ---------------------------------------------------------------------------

const SUMMARY_QUERIES: &[(&str, &str)] = &[
    (
        "file_inventory",
        "SELECT file_type, status, COUNT(*) as cnt, SUM(file_size) as total_bytes
         FROM files WHERE last_scan_run = ?
         GROUP BY file_type, status ORDER BY file_type, status",
    ),
    (
        "parse_results",
        "SELECT COUNT(*) as total,
                SUM(CASE WHEN valid THEN 1 ELSE 0 END) as valid,
                SUM(CASE WHEN NOT valid THEN 1 ELSE 0 END) as invalid
         FROM parse_results WHERE run_id = ?",
    ),
    (
        "feature_usage",
        "SELECT
            SUM(CASE WHEN sql_statements > 0 THEN 1 ELSE 0 END) as sql_programs,
            SUM(CASE WHEN file_ops > 0 THEN 1 ELSE 0 END) as fileio_programs,
            SUM(CASE WHEN calls > 0 THEN 1 ELSE 0 END) as call_programs,
            SUM(sql_statements) as total_sql,
            SUM(file_ops) as total_fileops,
            SUM(calls) as total_calls
         FROM parse_results WHERE run_id = ? AND valid = true",
    ),
];

const COVERAGE_QUERIES: &[(&str, &str)] = &[
    (
        "overall",
        "SELECT COUNT(*) as files,
                ROUND(AVG(coverage_pct), 2) as avg_coverage,
                MIN(coverage_pct) as min, MAX(coverage_pct) as max,
                SUM(total_statements) as total_stmts,
                SUM(mapped_statements) as mapped_stmts
         FROM coverage_results WHERE run_id = ?",
    ),
    (
        "distribution",
        "SELECT FLOOR(coverage_pct / 10) * 10 as bucket,
                COUNT(*) as cnt
         FROM coverage_results WHERE run_id = ?
         GROUP BY FLOOR(coverage_pct / 10) ORDER BY bucket",
    ),
];

const ERROR_QUERIES: &[(&str, &str)] = &[
    (
        "top_error_codes",
        "SELECT code, severity, COUNT(*) as cnt, COUNT(DISTINCT file_id) as files
         FROM diagnostics WHERE run_id = ?
         GROUP BY code, severity ORDER BY cnt DESC LIMIT 20",
    ),
    (
        "unresolved_copybooks",
        "SELECT name, COUNT(*) as refs, COUNT(DISTINCT referenced_by) as files
         FROM copybooks WHERE run_id = ? AND resolved = false
         GROUP BY name ORDER BY refs DESC LIMIT 20",
    ),
];

const COMPLEXITY_QUERIES: &[(&str, &str)] = &[(
    "top_complex",
    "SELECT f.path, pr.program_id, pr.line_count, pr.paragraphs,
            pr.calls, pr.file_ops, pr.sql_statements,
            (pr.paragraphs + 2*pr.calls + 3*pr.file_ops + 3*pr.sql_statements) as score
     FROM parse_results pr
     JOIN files f ON pr.file_id = f.file_id
     WHERE pr.run_id = ? AND pr.valid = true
     ORDER BY score DESC LIMIT 30",
)];
