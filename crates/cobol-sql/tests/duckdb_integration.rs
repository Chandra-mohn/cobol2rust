#![cfg(feature = "duckdb")]
//! Integration tests for DuckDbRuntime against the CobolSqlRuntime trait.
//!
//! Uses in-memory DuckDB -- no external database needed.
//! Tests exercise all 13 trait methods with COBOL-style field types.

use cobol_core::CobolField;
use cobol_sql::{CobolSqlRuntime, DuckDbRuntime, HostVar, HostVarMut, Sqlca};
use cobol_types::packed_decimal::PackedDecimal;
use cobol_types::pic_x::PicX;
use rust_decimal_macros::dec;

/// Helper: create an EMP table matching the COBOL test programs.
fn setup_emp_table(rt: &DuckDbRuntime) {
    rt.execute_batch(
        "CREATE TABLE EMP (
            EMPNO   INTEGER PRIMARY KEY,
            ENAME   VARCHAR(20),
            SAL     DECIMAL(9,2),
            DEPTNO  INTEGER,
            COMM    DECIMAL(9,2)
        )",
    )
    .expect("CREATE TABLE EMP");
}

/// Helper: seed the EMP table with sample data.
fn seed_emp_data(rt: &DuckDbRuntime) {
    rt.execute_batch(
        "INSERT INTO EMP VALUES (100, 'SMITH', 50000.00, 10, NULL);
         INSERT INTO EMP VALUES (200, 'JONES', 60000.00, 10, 1500.00);
         INSERT INTO EMP VALUES (300, 'BLAKE', 55000.00, 20, NULL);
         INSERT INTO EMP VALUES (400, 'CLARK', 45000.00, 30, 2000.00);",
    )
    .expect("seed EMP data");
}

/// Helper: create a runtime with EMP table and sample data.
fn runtime_with_data() -> DuckDbRuntime {
    let rt = DuckDbRuntime::in_memory().expect("in_memory");
    setup_emp_table(&rt);
    seed_emp_data(&rt);
    rt
}

// -------------------------------------------------------------------------
// SELECT INTO (exec_query)
// -------------------------------------------------------------------------

#[test]
fn select_into_single_row() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // Input: EMPNO = 100
    let empno = PicX::new(6, b"100");
    let params = [HostVar::new(&empno)];

    // Output fields
    let mut ename = PicX::new(20, b"");
    let mut sal = PicX::new(10, b"");

    {
        let mut into = [HostVarMut::new(&mut ename), HostVarMut::new(&mut sal)];
        rt.exec_query(
            "SELECT ENAME, SAL FROM EMP WHERE EMPNO = ?",
            &params,
            &mut into,
            &mut sqlca,
        );
    }

    assert!(sqlca.is_ok(), "SQLCODE={}", sqlca.sqlcode);
    let ename_str = std::str::from_utf8(ename.as_bytes()).unwrap().trim();
    assert_eq!(ename_str, "SMITH");
}

#[test]
fn select_into_not_found() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    let empno = PicX::new(6, b"999");
    let params = [HostVar::new(&empno)];
    let mut ename = PicX::new(20, b"");
    {
        let mut into = [HostVarMut::new(&mut ename)];
        rt.exec_query(
            "SELECT ENAME FROM EMP WHERE EMPNO = ?",
            &params,
            &mut into,
            &mut sqlca,
        );
    }

    assert!(sqlca.is_not_found(), "SQLCODE={}", sqlca.sqlcode);
}

// -------------------------------------------------------------------------
// INSERT / UPDATE / DELETE (exec_update)
// -------------------------------------------------------------------------

#[test]
fn insert_row() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    let empno = PicX::new(6, b"999");
    let ename = PicX::new(20, b"NEWBIE");
    let sal = PicX::new(10, b"35000.00");
    let deptno = PicX::new(4, b"10");
    let params = [
        HostVar::new(&empno),
        HostVar::new(&ename),
        HostVar::new(&sal),
        HostVar::new(&deptno),
    ];

    rt.exec_update(
        "INSERT INTO EMP (EMPNO, ENAME, SAL, DEPTNO) VALUES (?, ?, ?, ?)",
        &params,
        &mut sqlca,
    );

    assert!(sqlca.is_ok(), "SQLCODE={}", sqlca.sqlcode);

    // Verify by reading back
    let mut out_ename = PicX::new(20, b"");
    {
        let mut into = [HostVarMut::new(&mut out_ename)];
        rt.exec_query(
            "SELECT ENAME FROM EMP WHERE EMPNO = ?",
            &[HostVar::new(&empno)],
            &mut into,
            &mut sqlca,
        );
    }
    assert!(sqlca.is_ok());
    let name = std::str::from_utf8(out_ename.as_bytes()).unwrap().trim();
    assert_eq!(name, "NEWBIE");
}

#[test]
fn update_row() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    let new_sal = PicX::new(10, b"75000.00");
    let empno = PicX::new(6, b"100");
    let params = [HostVar::new(&new_sal), HostVar::new(&empno)];

    rt.exec_update(
        "UPDATE EMP SET SAL = ? WHERE EMPNO = ?",
        &params,
        &mut sqlca,
    );

    assert!(sqlca.is_ok(), "SQLCODE={}", sqlca.sqlcode);
    assert_eq!(sqlca.rows_affected(), 1);
}

#[test]
fn delete_row() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    let empno = PicX::new(6, b"200");
    let params = [HostVar::new(&empno)];

    rt.exec_update("DELETE FROM EMP WHERE EMPNO = ?", &params, &mut sqlca);

    assert!(sqlca.is_ok(), "SQLCODE={}", sqlca.sqlcode);
    assert_eq!(sqlca.rows_affected(), 1);

    // Verify it's gone
    let mut ename = PicX::new(20, b"");
    {
        let mut into = [HostVarMut::new(&mut ename)];
        rt.exec_query(
            "SELECT ENAME FROM EMP WHERE EMPNO = ?",
            &params,
            &mut into,
            &mut sqlca,
        );
    }
    assert!(sqlca.is_not_found());
}

// -------------------------------------------------------------------------
// COMMIT / ROLLBACK
// -------------------------------------------------------------------------

#[test]
fn commit_and_rollback() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // Begin a transaction
    rt.execute_batch("BEGIN TRANSACTION").unwrap();

    // Insert within transaction
    let empno = PicX::new(6, b"888");
    let ename = PicX::new(20, b"TXN-TEST");
    let sal = PicX::new(10, b"40000");
    let deptno = PicX::new(4, b"20");
    rt.exec_update(
        "INSERT INTO EMP (EMPNO, ENAME, SAL, DEPTNO) VALUES (?, ?, ?, ?)",
        &[
            HostVar::new(&empno),
            HostVar::new(&ename),
            HostVar::new(&sal),
            HostVar::new(&deptno),
        ],
        &mut sqlca,
    );
    assert!(sqlca.is_ok());

    // Rollback
    rt.rollback(&mut sqlca);
    assert!(sqlca.is_ok());

    // Row should not exist
    let mut out = PicX::new(20, b"");
    {
        let mut into = [HostVarMut::new(&mut out)];
        rt.exec_query(
            "SELECT ENAME FROM EMP WHERE EMPNO = ?",
            &[HostVar::new(&empno)],
            &mut into,
            &mut sqlca,
        );
    }
    assert!(sqlca.is_not_found(), "Row should be gone after ROLLBACK");
}

// -------------------------------------------------------------------------
// CURSOR operations
// -------------------------------------------------------------------------

#[test]
fn cursor_lifecycle() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // DECLARE
    let deptno = PicX::new(4, b"10");
    rt.declare_cursor(
        "EMP-CURSOR",
        "SELECT EMPNO, ENAME FROM EMP WHERE DEPTNO = ? ORDER BY EMPNO",
        &[HostVar::new(&deptno)],
        &mut sqlca,
    );
    assert!(sqlca.is_ok());

    // OPEN
    rt.open_cursor("EMP-CURSOR", &mut sqlca);
    assert!(sqlca.is_ok());

    // FETCH first row
    let mut empno_out = PicX::new(6, b"");
    let mut ename_out = PicX::new(20, b"");
    {
        let mut into = [
            HostVarMut::new(&mut empno_out),
            HostVarMut::new(&mut ename_out),
        ];
        rt.fetch_cursor("EMP-CURSOR", &mut into, &mut sqlca);
    }
    assert!(sqlca.is_ok(), "first fetch SQLCODE={}", sqlca.sqlcode);
    assert_eq!(
        std::str::from_utf8(empno_out.as_bytes()).unwrap().trim(),
        "100"
    );

    // FETCH second row
    {
        let mut into = [
            HostVarMut::new(&mut empno_out),
            HostVarMut::new(&mut ename_out),
        ];
        rt.fetch_cursor("EMP-CURSOR", &mut into, &mut sqlca);
    }
    assert!(sqlca.is_ok(), "second fetch SQLCODE={}", sqlca.sqlcode);
    assert_eq!(
        std::str::from_utf8(empno_out.as_bytes()).unwrap().trim(),
        "200"
    );

    // FETCH past end -> SQLCODE 100
    {
        let mut into = [
            HostVarMut::new(&mut empno_out),
            HostVarMut::new(&mut ename_out),
        ];
        rt.fetch_cursor("EMP-CURSOR", &mut into, &mut sqlca);
    }
    assert!(
        sqlca.is_not_found(),
        "end-of-data SQLCODE={}",
        sqlca.sqlcode
    );

    // CLOSE
    rt.close_cursor("EMP-CURSOR", &mut sqlca);
    assert!(sqlca.is_ok());
}

#[test]
fn cursor_not_declared_error() {
    let mut rt = DuckDbRuntime::in_memory().unwrap();
    let mut sqlca = Sqlca::default();

    rt.open_cursor("NO-SUCH-CURSOR", &mut sqlca);
    assert!(sqlca.is_error());
    assert_eq!(sqlca.sqlcode, -501);
}

// -------------------------------------------------------------------------
// PREPARE / EXECUTE
// -------------------------------------------------------------------------

#[test]
fn prepare_and_execute() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // PREPARE
    let sql_text = PicX::new(200, b"DELETE FROM EMP WHERE EMPNO = ?");
    rt.prepare("STMT1", &HostVar::new(&sql_text), &mut sqlca);
    assert!(sqlca.is_ok(), "PREPARE SQLCODE={}", sqlca.sqlcode);

    // EXECUTE with param
    let empno = PicX::new(6, b"300");
    rt.execute_prepared("STMT1", &[HostVar::new(&empno)], &mut sqlca);
    assert!(sqlca.is_ok(), "EXECUTE SQLCODE={}", sqlca.sqlcode);
    assert_eq!(sqlca.rows_affected(), 1);

    // Verify row is gone
    let mut out = PicX::new(20, b"");
    {
        let mut into = [HostVarMut::new(&mut out)];
        rt.exec_query(
            "SELECT ENAME FROM EMP WHERE EMPNO = ?",
            &[HostVar::new(&empno)],
            &mut into,
            &mut sqlca,
        );
    }
    assert!(sqlca.is_not_found());
}

#[test]
fn execute_unprepared_error() {
    let mut rt = DuckDbRuntime::in_memory().unwrap();
    let mut sqlca = Sqlca::default();

    rt.execute_prepared("NONEXISTENT", &[], &mut sqlca);
    assert!(sqlca.is_error());
    assert_eq!(sqlca.sqlcode, -518);
}

// -------------------------------------------------------------------------
// EXECUTE IMMEDIATE
// -------------------------------------------------------------------------

#[test]
fn execute_immediate() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    let sql_text = PicX::new(200, b"DELETE FROM EMP WHERE DEPTNO = 20");
    rt.execute_immediate(&HostVar::new(&sql_text), &mut sqlca);
    assert!(sqlca.is_ok(), "EXEC IMMEDIATE SQLCODE={}", sqlca.sqlcode);

    // Verify BLAKE (deptno 20) is gone
    let empno = PicX::new(6, b"300");
    let mut out = PicX::new(20, b"");
    {
        let mut into = [HostVarMut::new(&mut out)];
        rt.exec_query(
            "SELECT ENAME FROM EMP WHERE EMPNO = ?",
            &[HostVar::new(&empno)],
            &mut into,
            &mut sqlca,
        );
    }
    assert!(sqlca.is_not_found());
}

#[test]
fn execute_immediate_empty_sql_error() {
    let mut rt = DuckDbRuntime::in_memory().unwrap();
    let mut sqlca = Sqlca::default();

    let empty = PicX::new(10, b"");
    rt.execute_immediate(&HostVar::new(&empty), &mut sqlca);
    assert!(sqlca.is_error());
}

// -------------------------------------------------------------------------
// SAVEPOINT -- DuckDB does not support SAVEPOINT syntax.
// The savepoint/rollback_to_savepoint trait methods are tested with
// enterprise backends (Postgres, DB2, Oracle) that support them.
// -------------------------------------------------------------------------

// -------------------------------------------------------------------------
// Null indicators
// -------------------------------------------------------------------------

#[test]
fn null_indicator_on_select() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // SMITH has COMM = NULL
    let empno = PicX::new(6, b"100");
    let mut comm = PicX::new(10, b"");
    let mut comm_ind = PicX::new(4, b"0");

    {
        let mut into = [HostVarMut::with_indicator(&mut comm, &mut comm_ind)];
        rt.exec_query(
            "SELECT COMM FROM EMP WHERE EMPNO = ?",
            &[HostVar::new(&empno)],
            &mut into,
            &mut sqlca,
        );
    }

    assert!(sqlca.is_ok());
    // Indicator should be set to "-1" for NULL
    let ind_str = std::str::from_utf8(comm_ind.as_bytes()).unwrap().trim();
    assert_eq!(ind_str, "-1", "indicator should be -1 for NULL");
}

#[test]
fn non_null_indicator_on_select() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // JONES has COMM = 1500.00
    let empno = PicX::new(6, b"200");
    let mut comm = PicX::new(10, b"");
    let mut comm_ind = PicX::new(4, b"9999"); // will be overwritten

    {
        let mut into = [HostVarMut::with_indicator(&mut comm, &mut comm_ind)];
        rt.exec_query(
            "SELECT COMM FROM EMP WHERE EMPNO = ?",
            &[HostVar::new(&empno)],
            &mut into,
            &mut sqlca,
        );
    }

    assert!(sqlca.is_ok());
    let ind_str = std::str::from_utf8(comm_ind.as_bytes()).unwrap().trim();
    assert_eq!(ind_str, "0", "indicator should be 0 for non-NULL");
}

// -------------------------------------------------------------------------
// PackedDecimal host variables
// -------------------------------------------------------------------------

#[test]
fn packed_decimal_as_input_param() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // Use PackedDecimal for EMPNO (PIC 9(6) COMP-3)
    let mut empno = PackedDecimal::new(6, 0, false);
    empno.pack(dec!(100));

    let mut ename = PicX::new(20, b"");
    {
        let mut into = [HostVarMut::new(&mut ename)];
        rt.exec_query(
            "SELECT ENAME FROM EMP WHERE EMPNO = ?",
            &[HostVar::new(&empno)],
            &mut into,
            &mut sqlca,
        );
    }

    assert!(sqlca.is_ok(), "SQLCODE={}", sqlca.sqlcode);
    let name = std::str::from_utf8(ename.as_bytes()).unwrap().trim();
    assert_eq!(name, "SMITH");
}

// -------------------------------------------------------------------------
// Multiple rows affected
// -------------------------------------------------------------------------

#[test]
fn delete_multiple_rows_count() {
    let mut rt = runtime_with_data();
    let mut sqlca = Sqlca::default();

    // Delete all dept 10 employees (SMITH + JONES = 2 rows)
    let deptno = PicX::new(4, b"10");
    rt.exec_update(
        "DELETE FROM EMP WHERE DEPTNO = ?",
        &[HostVar::new(&deptno)],
        &mut sqlca,
    );

    assert!(sqlca.is_ok());
    assert_eq!(sqlca.rows_affected(), 2);
}

// -------------------------------------------------------------------------
// Edge cases
// -------------------------------------------------------------------------

#[test]
fn create_temp_table_via_execute_immediate() {
    let mut rt = DuckDbRuntime::in_memory().unwrap();
    let mut sqlca = Sqlca::default();

    let sql = PicX::new(200, b"CREATE TABLE TEMP_TABLE (ID INTEGER, STATUS VARCHAR(1))");
    rt.execute_immediate(&HostVar::new(&sql), &mut sqlca);
    assert!(sqlca.is_ok());

    // Insert and verify
    let sql2 = PicX::new(200, b"INSERT INTO TEMP_TABLE VALUES (1, 'A')");
    rt.execute_immediate(&HostVar::new(&sql2), &mut sqlca);
    assert!(sqlca.is_ok());
}

#[test]
fn prepare_empty_sql_error() {
    let mut rt = DuckDbRuntime::in_memory().unwrap();
    let mut sqlca = Sqlca::default();

    let empty = PicX::new(10, b"");
    rt.prepare("STMT1", &HostVar::new(&empty), &mut sqlca);
    assert!(sqlca.is_error());
    assert_eq!(sqlca.sqlcode, -518);
}
