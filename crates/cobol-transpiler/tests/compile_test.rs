//! Compile-and-run integration tests.
//!
//! These tests generate Rust code from COBOL, write it to a temporary
//! Cargo project, compile it with `cargo check`, and optionally run it.
//!
//! These tests are slower (each spawns cargo) and are gated behind the
//! `compile_tests` feature flag. Run with:
//!   cargo test -p cobol-transpiler --test compile_test --features compile_tests
//!
//! They require cobol-runtime and all dependency crates to be available
//! at their workspace paths.

#![cfg(feature = "compile_tests")]

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use cobol_transpiler::transpile::transpile;

/// Get the workspace root directory.
fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/cobol-transpiler -> workspace root
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("cannot determine workspace root")
        .to_path_buf()
}

/// Create a temporary Cargo project with the generated Rust code.
fn create_temp_project(name: &str, rust_code: &str) -> tempfile::TempDir {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let project_dir = temp_dir.path().join(name);
    fs::create_dir_all(project_dir.join("src")).expect("failed to create src dir");

    let ws_root = workspace_root();

    // Write Cargo.toml with path dependencies
    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
cobol-runtime = {{ path = "{ws}/crates/cobol-runtime" }}
rust_decimal = "1.40"
rust_decimal_macros = "1.40"
"#,
        ws = ws_root.display()
    );
    fs::write(project_dir.join("Cargo.toml"), cargo_toml).expect("failed to write Cargo.toml");

    // Write the generated Rust code as main.rs
    fs::write(project_dir.join("src").join("main.rs"), rust_code)
        .expect("failed to write main.rs");

    temp_dir
}

/// Run `cargo check` on a temp project and return success/failure.
fn cargo_check(temp_dir: &tempfile::TempDir, name: &str) -> Result<(), String> {
    let project_dir = temp_dir.path().join(name);

    let output = Command::new("cargo")
        .arg("check")
        .current_dir(&project_dir)
        .output()
        .expect("failed to run cargo check");

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "cargo check failed:\n{stderr}"
        ))
    }
}

/// Run `cargo run` on a temp project and return stdout.
fn cargo_run(temp_dir: &tempfile::TempDir, name: &str) -> Result<String, String> {
    let project_dir = temp_dir.path().join(name);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .current_dir(&project_dir)
        .output()
        .expect("failed to run cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    // Note: STOP RUN calls process::exit(0) which is a clean exit
    if output.status.success() || output.status.code() == Some(0) {
        Ok(stdout)
    } else {
        Err(format!(
            "cargo run failed (exit code: {:?}):\nstdout: {stdout}\nstderr: {stderr}",
            output.status.code()
        ))
    }
}

// ---------------------------------------------------------------------------
// Compile tests
// ---------------------------------------------------------------------------

#[test]
fn compile_hello_world() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. HELLO.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY 'HELLO WORLD'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    let temp_dir = create_temp_project("hello", &rust_code);

    match cargo_check(&temp_dir, "hello") {
        Ok(()) => {}
        Err(e) => {
            println!("Generated code:\n{rust_code}");
            panic!("compile failed: {e}");
        }
    }
}

#[test]
fn compile_arithmetic() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. ARITH.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A PIC 9(5) VALUE 10.\n",
        "01  WS-B PIC 9(5) VALUE 20.\n",
        "01  WS-C PIC 9(5).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    ADD WS-A TO WS-B.\n",
        "    MOVE WS-B TO WS-C.\n",
        "    DISPLAY WS-C.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    let temp_dir = create_temp_project("arith", &rust_code);

    match cargo_check(&temp_dir, "arith") {
        Ok(()) => {}
        Err(e) => {
            println!("Generated code:\n{rust_code}");
            panic!("compile failed: {e}");
        }
    }
}

#[test]
fn compile_if_else() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTIF.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-X PIC 9(3) VALUE 5.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    IF WS-X > 0\n",
        "        DISPLAY 'POSITIVE'\n",
        "    ELSE\n",
        "        DISPLAY 'NOT POSITIVE'\n",
        "    END-IF.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    let temp_dir = create_temp_project("testif", &rust_code);

    match cargo_check(&temp_dir, "testif") {
        Ok(()) => {}
        Err(e) => {
            println!("Generated code:\n{rust_code}");
            panic!("compile failed: {e}");
        }
    }
}
