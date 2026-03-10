//! EXEC SQL text analysis -- extract metadata from raw SQL text.
//!
//! No SQL parsing -- uses keyword matching and regex patterns to detect
//! statement type, host variable references, and cursor/prepared names.

use crate::ast::{ExecSqlStatement, HostVarRef, SqlStatementType};

/// Analyze raw SQL text and build an `ExecSqlStatement` AST node.
///
/// The `raw_sql` parameter is the normalized text between EXEC SQL and
/// END-EXEC (whitespace already collapsed by the preprocessor).
pub fn analyze_exec_sql(raw_sql: &str) -> ExecSqlStatement {
    let sql_type = detect_statement_type(raw_sql);
    let (input_vars, output_vars) = extract_host_vars(raw_sql, &sql_type);
    let cursor_name = extract_cursor_name(raw_sql, &sql_type);
    let prepared_name = extract_prepared_name(raw_sql, &sql_type);

    ExecSqlStatement {
        sql_type,
        raw_sql: raw_sql.to_string(),
        input_vars,
        output_vars,
        cursor_name,
        prepared_name,
    }
}

/// Detect the SQL statement type from the first keyword(s).
fn detect_statement_type(sql: &str) -> SqlStatementType {
    let upper = sql.trim().to_uppercase();
    let words: Vec<&str> = upper.split_whitespace().collect();

    if words.is_empty() {
        return SqlStatementType::Other(String::new());
    }

    match words[0] {
        "SELECT" => SqlStatementType::SelectInto,
        "INSERT" => SqlStatementType::Insert,
        "UPDATE" => SqlStatementType::Update,
        "DELETE" => SqlStatementType::Delete,
        "COMMIT" => SqlStatementType::Commit,
        "ROLLBACK" => SqlStatementType::Rollback,
        "SAVEPOINT" => SqlStatementType::Savepoint,
        "OPEN" => SqlStatementType::OpenCursor,
        "CLOSE" => SqlStatementType::CloseCursor,
        "FETCH" => SqlStatementType::FetchCursor,
        "DECLARE" => {
            // DECLARE cursor-name CURSOR FOR ...
            if words.len() >= 3 && words[2] == "CURSOR" {
                SqlStatementType::DeclareCursor
            } else {
                SqlStatementType::Other(words.join(" "))
            }
        }
        "PREPARE" => SqlStatementType::Prepare,
        "EXECUTE" => {
            // EXECUTE IMMEDIATE vs EXECUTE prepared-name
            if words.len() >= 2 && words[1] == "IMMEDIATE" {
                SqlStatementType::ExecuteImmediate
            } else {
                SqlStatementType::Execute
            }
        }
        "INCLUDE" => {
            if words.len() >= 2 && words[1] == "SQLCA" {
                SqlStatementType::IncludeSqlca
            } else {
                SqlStatementType::Other(words.join(" "))
            }
        }
        _ => SqlStatementType::Other(words[0].to_string()),
    }
}

/// Extract host variable references from SQL text.
///
/// Host variables are prefixed with `:` (e.g., `:WS-EMPNO`).
/// Returns (input_vars, output_vars) based on position relative to INTO.
fn extract_host_vars(sql: &str, sql_type: &SqlStatementType) -> (Vec<HostVarRef>, Vec<HostVarRef>) {
    let all_refs = find_host_var_refs(sql);

    if all_refs.is_empty() {
        return (Vec::new(), Vec::new());
    }

    match sql_type {
        SqlStatementType::SelectInto | SqlStatementType::FetchCursor => {
            // For SELECT INTO and FETCH, variables after INTO are outputs,
            // variables before INTO (in WHERE etc.) are inputs.
            split_at_into(sql, &all_refs)
        }
        _ => {
            // For INSERT/UPDATE/DELETE/etc., all host vars are inputs
            (all_refs, Vec::new())
        }
    }
}

/// Find all `:NAME` host variable references in SQL text.
///
/// Also detects null indicator variables: `:FIELD :INDICATOR` (space-separated,
/// no comma between them, second also starts with `:`).
fn find_host_var_refs(sql: &str) -> Vec<HostVarRef> {
    let mut refs = Vec::new();
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        if bytes[i] == b':' {
            // Found a host variable reference
            let start = i + 1;
            let mut end = start;
            while end < len && is_cobol_name_char(bytes[end]) {
                end += 1;
            }
            if end > start {
                let field_name = sql[start..end].to_uppercase();

                // Check for null indicator (next non-space token starting with :)
                let mut indicator = None;
                let mut j = end;
                // Skip spaces (not commas -- indicator follows without comma)
                while j < len && bytes[j] == b' ' {
                    j += 1;
                }
                if j < len && bytes[j] == b':' {
                    let ind_start = j + 1;
                    let mut ind_end = ind_start;
                    while ind_end < len && is_cobol_name_char(bytes[ind_end]) {
                        ind_end += 1;
                    }
                    if ind_end > ind_start {
                        indicator = Some(sql[ind_start..ind_end].to_uppercase());
                        end = ind_end; // skip past indicator
                    }
                }

                refs.push(HostVarRef {
                    field_name,
                    indicator,
                });
                i = end;
                continue;
            }
        }
        i += 1;
    }

    refs
}

/// Check if a byte is valid in a COBOL data name (A-Z, a-z, 0-9, hyphen).
fn is_cobol_name_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-'
}

/// Split host variable refs into (inputs, outputs) based on INTO keyword position.
fn split_at_into(sql: &str, refs: &[HostVarRef]) -> (Vec<HostVarRef>, Vec<HostVarRef>) {
    let upper = sql.to_uppercase();

    // Find the INTO keyword position (word boundary check)
    let into_pos = find_keyword_pos(&upper, "INTO");

    if let Some(into_byte_pos) = into_pos {
        // Find the FROM keyword position (marks end of INTO clause)
        let from_pos = find_keyword_pos(&upper, "FROM")
            .unwrap_or(sql.len());

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        for var_ref in refs {
            // Find where this variable appears in the SQL
            let search = format!(":{}", var_ref.field_name);
            if let Some(var_pos) = upper.find(&search) {
                if var_pos > into_byte_pos && var_pos < from_pos {
                    outputs.push(var_ref.clone());
                } else {
                    inputs.push(var_ref.clone());
                }
            } else {
                inputs.push(var_ref.clone());
            }
        }

        (inputs, outputs)
    } else {
        // No INTO found -- all are inputs
        (refs.to_vec(), Vec::new())
    }
}

/// Find keyword position with word boundary checking.
fn find_keyword_pos(upper_sql: &str, keyword: &str) -> Option<usize> {
    let bytes = upper_sql.as_bytes();
    let kw_bytes = keyword.as_bytes();
    let kw_len = kw_bytes.len();

    let mut pos = 0;
    while pos + kw_len <= bytes.len() {
        if &bytes[pos..pos + kw_len] == kw_bytes {
            let before_ok = pos == 0 || !bytes[pos - 1].is_ascii_alphanumeric();
            let after_ok = pos + kw_len >= bytes.len()
                || !bytes[pos + kw_len].is_ascii_alphanumeric();
            if before_ok && after_ok {
                return Some(pos);
            }
        }
        pos += 1;
    }
    None
}

/// Extract cursor name from DECLARE/OPEN/FETCH/CLOSE statements.
fn extract_cursor_name(sql: &str, sql_type: &SqlStatementType) -> Option<String> {
    let words: Vec<&str> = sql.split_whitespace().collect();

    match sql_type {
        SqlStatementType::DeclareCursor => {
            // DECLARE cursor-name CURSOR FOR ...
            words.get(1).map(|w| w.to_uppercase())
        }
        SqlStatementType::OpenCursor
        | SqlStatementType::CloseCursor
        | SqlStatementType::FetchCursor => {
            // OPEN cursor-name / CLOSE cursor-name / FETCH cursor-name
            words.get(1).map(|w| w.to_uppercase())
        }
        _ => None,
    }
}

/// Extract prepared statement name from PREPARE/EXECUTE statements.
fn extract_prepared_name(sql: &str, sql_type: &SqlStatementType) -> Option<String> {
    let words: Vec<&str> = sql.split_whitespace().collect();

    match sql_type {
        SqlStatementType::Prepare => {
            // PREPARE stmt-name FROM ...
            words.get(1).map(|w| w.to_uppercase())
        }
        SqlStatementType::Execute => {
            // EXECUTE stmt-name USING ...
            words.get(1).map(|w| w.to_uppercase())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_select_into() {
        let sql = "SELECT ENAME, SAL INTO :WS-ENAME, :WS-SAL FROM EMP WHERE EMPNO = :WS-EMPNO";
        assert_eq!(detect_statement_type(sql), SqlStatementType::SelectInto);
    }

    #[test]
    fn detect_insert() {
        let sql = "INSERT INTO EMP (EMPNO, ENAME) VALUES (:WS-EMPNO, :WS-ENAME)";
        assert_eq!(detect_statement_type(sql), SqlStatementType::Insert);
    }

    #[test]
    fn detect_declare_cursor() {
        let sql = "DECLARE C1 CURSOR FOR SELECT EMPNO FROM EMP";
        assert_eq!(detect_statement_type(sql), SqlStatementType::DeclareCursor);
    }

    #[test]
    fn detect_commit() {
        assert_eq!(detect_statement_type("COMMIT"), SqlStatementType::Commit);
    }

    #[test]
    fn detect_execute_immediate() {
        let sql = "EXECUTE IMMEDIATE :WS-SQL-STMT";
        assert_eq!(detect_statement_type(sql), SqlStatementType::ExecuteImmediate);
    }

    #[test]
    fn detect_include_sqlca() {
        assert_eq!(detect_statement_type("INCLUDE SQLCA"), SqlStatementType::IncludeSqlca);
    }

    #[test]
    fn extract_host_vars_select_into() {
        let sql = "SELECT ENAME, SAL INTO :WS-ENAME, :WS-SAL FROM EMP WHERE EMPNO = :WS-EMPNO";
        let stmt = analyze_exec_sql(sql);

        assert_eq!(stmt.sql_type, SqlStatementType::SelectInto);
        assert_eq!(stmt.output_vars.len(), 2);
        assert_eq!(stmt.output_vars[0].field_name, "WS-ENAME");
        assert_eq!(stmt.output_vars[1].field_name, "WS-SAL");
        assert_eq!(stmt.input_vars.len(), 1);
        assert_eq!(stmt.input_vars[0].field_name, "WS-EMPNO");
    }

    #[test]
    fn extract_host_vars_insert() {
        let sql = "INSERT INTO EMP (EMPNO, ENAME) VALUES (:WS-EMPNO, :WS-ENAME)";
        let stmt = analyze_exec_sql(sql);

        assert_eq!(stmt.sql_type, SqlStatementType::Insert);
        assert_eq!(stmt.input_vars.len(), 2);
        assert_eq!(stmt.output_vars.len(), 0);
    }

    #[test]
    fn extract_cursor_name_declare() {
        let sql = "DECLARE EMP-CURSOR CURSOR FOR SELECT EMPNO FROM EMP";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.cursor_name, Some("EMP-CURSOR".to_string()));
    }

    #[test]
    fn extract_cursor_name_fetch() {
        let sql = "FETCH C1 INTO :WS-EMPNO, :WS-ENAME";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.cursor_name, Some("C1".to_string()));
        assert_eq!(stmt.output_vars.len(), 2);
    }

    #[test]
    fn extract_prepared_name() {
        let sql = "PREPARE STMT1 FROM :WS-SQL-TEXT";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.prepared_name, Some("STMT1".to_string()));
    }

    #[test]
    fn host_var_with_indicator() {
        let sql = "SELECT ENAME INTO :WS-ENAME :WS-ENAME-IND FROM EMP WHERE EMPNO = :WS-EMPNO";
        let stmt = analyze_exec_sql(sql);

        assert_eq!(stmt.output_vars.len(), 1);
        assert_eq!(stmt.output_vars[0].field_name, "WS-ENAME");
        assert_eq!(
            stmt.output_vars[0].indicator,
            Some("WS-ENAME-IND".to_string())
        );
    }

    #[test]
    fn no_host_vars_commit() {
        let stmt = analyze_exec_sql("COMMIT");
        assert!(stmt.input_vars.is_empty());
        assert!(stmt.output_vars.is_empty());
    }

    // -- Statement type detection edge cases --

    #[test]
    fn detect_update() {
        let sql = "UPDATE EMP SET SAL = :WS-SAL WHERE EMPNO = :WS-EMPNO";
        assert_eq!(detect_statement_type(sql), SqlStatementType::Update);
    }

    #[test]
    fn detect_delete() {
        let sql = "DELETE FROM EMP WHERE EMPNO = :WS-EMPNO";
        assert_eq!(detect_statement_type(sql), SqlStatementType::Delete);
    }

    #[test]
    fn detect_rollback() {
        assert_eq!(detect_statement_type("ROLLBACK"), SqlStatementType::Rollback);
    }

    #[test]
    fn detect_savepoint() {
        assert_eq!(detect_statement_type("SAVEPOINT SP1"), SqlStatementType::Savepoint);
    }

    #[test]
    fn detect_open_cursor() {
        assert_eq!(detect_statement_type("OPEN C1"), SqlStatementType::OpenCursor);
    }

    #[test]
    fn detect_close_cursor() {
        assert_eq!(detect_statement_type("CLOSE C1"), SqlStatementType::CloseCursor);
    }

    #[test]
    fn detect_fetch_cursor() {
        let sql = "FETCH C1 INTO :WS-EMPNO";
        assert_eq!(detect_statement_type(sql), SqlStatementType::FetchCursor);
    }

    #[test]
    fn detect_execute_named() {
        let sql = "EXECUTE STMT1 USING :WS-PARAM";
        assert_eq!(detect_statement_type(sql), SqlStatementType::Execute);
    }

    #[test]
    fn detect_prepare() {
        let sql = "PREPARE STMT1 FROM :WS-SQL";
        assert_eq!(detect_statement_type(sql), SqlStatementType::Prepare);
    }

    #[test]
    fn detect_other_declare() {
        // DECLARE without CURSOR should be Other
        let sql = "DECLARE GLOBAL TEMPORARY TABLE T1";
        assert!(matches!(detect_statement_type(sql), SqlStatementType::Other(_)));
    }

    #[test]
    fn detect_other_unknown() {
        let sql = "GRANT SELECT ON EMP TO USER1";
        assert!(matches!(detect_statement_type(sql), SqlStatementType::Other(_)));
    }

    #[test]
    fn detect_empty_sql() {
        assert!(matches!(detect_statement_type(""), SqlStatementType::Other(_)));
    }

    #[test]
    fn detect_leading_whitespace() {
        assert_eq!(detect_statement_type("  COMMIT  "), SqlStatementType::Commit);
    }

    #[test]
    fn detect_case_insensitive() {
        assert_eq!(detect_statement_type("select a into :h from t"), SqlStatementType::SelectInto);
        assert_eq!(detect_statement_type("Insert Into T Values (:h)"), SqlStatementType::Insert);
    }

    // -- Host variable extraction edge cases --

    #[test]
    fn host_vars_update_all_inputs() {
        let sql = "UPDATE EMP SET SAL = :WS-SAL, COMM = :WS-COMM WHERE EMPNO = :WS-EMPNO";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.sql_type, SqlStatementType::Update);
        assert_eq!(stmt.input_vars.len(), 3);
        assert_eq!(stmt.output_vars.len(), 0);
        assert_eq!(stmt.input_vars[0].field_name, "WS-SAL");
        assert_eq!(stmt.input_vars[1].field_name, "WS-COMM");
        assert_eq!(stmt.input_vars[2].field_name, "WS-EMPNO");
    }

    #[test]
    fn host_vars_delete_all_inputs() {
        let sql = "DELETE FROM EMP WHERE EMPNO = :WS-EMPNO AND DEPTNO = :WS-DEPT";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.input_vars.len(), 2);
        assert_eq!(stmt.output_vars.len(), 0);
    }

    #[test]
    fn host_vars_multiple_into() {
        // Multiple fields after INTO, multiple in WHERE
        let sql = "SELECT A, B, C INTO :H1, :H2, :H3 FROM T WHERE X = :P1 AND Y = :P2";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.output_vars.len(), 3);
        assert_eq!(stmt.input_vars.len(), 2);
        assert_eq!(stmt.output_vars[0].field_name, "H1");
        assert_eq!(stmt.output_vars[1].field_name, "H2");
        assert_eq!(stmt.output_vars[2].field_name, "H3");
        assert_eq!(stmt.input_vars[0].field_name, "P1");
        assert_eq!(stmt.input_vars[1].field_name, "P2");
    }

    #[test]
    fn host_vars_select_no_where() {
        let sql = "SELECT COUNT(*) INTO :WS-COUNT FROM EMP";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.output_vars.len(), 1);
        assert_eq!(stmt.output_vars[0].field_name, "WS-COUNT");
        assert!(stmt.input_vars.is_empty());
    }

    #[test]
    fn host_vars_with_hyphens() {
        let sql = "SELECT EMP-NAME INTO :WS-EMP-FULL-NAME FROM EMP WHERE EMP-ID = :WS-EMP-ID";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.output_vars[0].field_name, "WS-EMP-FULL-NAME");
        assert_eq!(stmt.input_vars[0].field_name, "WS-EMP-ID");
    }

    #[test]
    fn host_vars_no_vars() {
        let sql = "SELECT COUNT(*) FROM EMP";
        let stmt = analyze_exec_sql(sql);
        assert!(stmt.input_vars.is_empty());
        assert!(stmt.output_vars.is_empty());
    }

    #[test]
    fn host_vars_multiple_indicators() {
        let sql = "SELECT A, B INTO :H1 :I1, :H2 :I2 FROM T";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.output_vars.len(), 2);
        assert_eq!(stmt.output_vars[0].field_name, "H1");
        assert_eq!(stmt.output_vars[0].indicator, Some("I1".to_string()));
        assert_eq!(stmt.output_vars[1].field_name, "H2");
        assert_eq!(stmt.output_vars[1].indicator, Some("I2".to_string()));
    }

    #[test]
    fn fetch_with_multiple_outputs() {
        let sql = "FETCH EMP-CURSOR INTO :WS-EMPNO, :WS-ENAME, :WS-SAL";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.sql_type, SqlStatementType::FetchCursor);
        assert_eq!(stmt.cursor_name, Some("EMP-CURSOR".to_string()));
        assert_eq!(stmt.output_vars.len(), 3);
        assert!(stmt.input_vars.is_empty());
    }

    // -- Cursor name extraction --

    #[test]
    fn cursor_name_open() {
        let stmt = analyze_exec_sql("OPEN MY-CURSOR");
        assert_eq!(stmt.cursor_name, Some("MY-CURSOR".to_string()));
    }

    #[test]
    fn cursor_name_close() {
        let stmt = analyze_exec_sql("CLOSE MY-CURSOR");
        assert_eq!(stmt.cursor_name, Some("MY-CURSOR".to_string()));
    }

    #[test]
    fn no_cursor_name_for_dml() {
        let stmt = analyze_exec_sql("INSERT INTO T VALUES (:H1)");
        assert!(stmt.cursor_name.is_none());
    }

    // -- Prepared statement name extraction --

    #[test]
    fn prepared_name_execute_with_using() {
        let stmt = analyze_exec_sql("EXECUTE STMT1 USING :WS-PARAM1, :WS-PARAM2");
        assert_eq!(stmt.prepared_name, Some("STMT1".to_string()));
        assert_eq!(stmt.input_vars.len(), 2);
    }

    #[test]
    fn no_prepared_name_for_execute_immediate() {
        let stmt = analyze_exec_sql("EXECUTE IMMEDIATE :WS-SQL");
        assert!(stmt.prepared_name.is_none());
        assert_eq!(stmt.sql_type, SqlStatementType::ExecuteImmediate);
    }

    #[test]
    fn no_prepared_name_for_dml() {
        let stmt = analyze_exec_sql("SELECT A INTO :H FROM T");
        assert!(stmt.prepared_name.is_none());
    }

    // -- raw_sql preservation --

    #[test]
    fn raw_sql_preserved() {
        let sql = "SELECT ENAME INTO :WS-ENAME FROM EMP WHERE EMPNO = :WS-EMPNO";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.raw_sql, sql);
    }

    // -- Declare cursor with host vars --

    #[test]
    fn declare_cursor_with_param() {
        let sql = "DECLARE C1 CURSOR FOR SELECT EMPNO FROM EMP WHERE DEPTNO = :WS-DEPT";
        let stmt = analyze_exec_sql(sql);
        assert_eq!(stmt.sql_type, SqlStatementType::DeclareCursor);
        assert_eq!(stmt.cursor_name, Some("C1".to_string()));
        assert_eq!(stmt.input_vars.len(), 1);
        assert_eq!(stmt.input_vars[0].field_name, "WS-DEPT");
    }
}
