//! `cobol2rust compile` -- transpile COBOL and build in one step.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use miette::{Context, IntoDiagnostic, Result};

use crate::transpile_cmd::TranspileArgs;
use crate::Cli;

/// Arguments for `cobol2rust compile`.
#[derive(Debug, Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct CompileArgs {
    /// COBOL source file or directory to compile.
    pub input: PathBuf,

    /// Output directory (required).
    #[arg(short, long)]
    pub output: PathBuf,

    /// Build in release mode.
    #[arg(long)]
    pub release: bool,

    /// Cross-compilation target triple.
    #[arg(long)]
    pub target: Option<String>,

    /// Generate a Cargo workspace (required for directory input).
    #[arg(long)]
    pub workspace: bool,

    /// Force main program output.
    #[arg(long, conflicts_with = "lib")]
    pub main: bool,

    /// Force library output.
    #[arg(long, conflicts_with = "main")]
    pub lib: bool,

    /// COPY library mapping NAME=DIR (repeatable).
    #[arg(short = 'L', long = "library-map")]
    pub library_map: Vec<String>,

    /// Skip files that fail, report at end.
    #[arg(long)]
    pub continue_on_error: bool,

    /// Read/write manifest for main/lib overrides.
    #[arg(long)]
    pub manifest: Option<PathBuf>,

    /// Path to cobol-runtime crate (for workspace mode path dependency).
    #[arg(long)]
    pub runtime_path: Option<PathBuf>,
}

impl CompileArgs {
    /// Convert to `TranspileArgs` for the transpilation step.
    fn to_transpile_args(&self) -> TranspileArgs {
        TranspileArgs {
            input: self.input.clone(),
            output: Some(self.output.clone()),
            main: self.main,
            lib: self.lib,
            library_map: self.library_map.clone(),
            workspace: self.workspace,
            continue_on_error: self.continue_on_error,
            manifest: self.manifest.clone(),
            runtime_path: self.runtime_path.clone(),
            parallel: false,
            jobs: None,
            incremental: false,
        }
    }
}

/// Run the compile subcommand.
pub fn run(cli: &Cli, args: &CompileArgs) -> Result<ExitCode> {
    // Step 1: Transpile
    if !cli.quiet {
        eprintln!("Step 1/2: Transpiling...");
    }

    let transpile_args = args.to_transpile_args();
    let transpile_result = crate::transpile_cmd::run(cli, &transpile_args)?;
    if transpile_result != ExitCode::SUCCESS {
        return Ok(transpile_result);
    }

    // Step 2: Build
    if !cli.quiet {
        eprintln!("Step 2/2: Building...");
    }

    let build_dir = if args.workspace || args.input.is_dir() {
        // Workspace mode: build the entire workspace
        args.output.clone()
    } else {
        // Single file mode: the output is a .rs file, we need a buildable directory.
        // For single-file compile, we need to wrap it in a minimal Cargo project.
        let single_dir = args.output.parent().unwrap_or(&args.output).to_path_buf();
        ensure_single_file_project(&args.output, &single_dir, args.runtime_path.as_deref())?;
        single_dir
    };

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    cmd.current_dir(&build_dir);

    if args.release {
        cmd.arg("--release");
    }
    if let Some(ref triple) = args.target {
        cmd.arg("--target").arg(triple);
    }

    if cli.quiet {
        cmd.arg("--quiet");
    }

    if cli.verbose > 0 {
        for _ in 0..cli.verbose {
            cmd.arg("-v");
        }
    }

    let status = cmd
        .status()
        .into_diagnostic()
        .wrap_err("failed to run `cargo build`")?;

    if !status.success() {
        eprintln!("Build failed with exit code: {}", status.code().unwrap_or(-1));
        return Ok(ExitCode::from(1));
    }

    if !cli.quiet {
        let profile = if args.release { "release" } else { "debug" };
        let target_dir = build_dir.join("target").join(profile);
        eprintln!("Build succeeded. Binaries in: {}", target_dir.display());
    }

    Ok(ExitCode::SUCCESS)
}

/// For single-file compile mode, ensure a minimal Cargo project exists around
/// the transpiled .rs file so `cargo build` can work.
fn ensure_single_file_project(
    rs_file: &std::path::Path,
    project_dir: &std::path::Path,
    runtime_path: Option<&std::path::Path>,
) -> Result<()> {
    let cargo_toml = project_dir.join("Cargo.toml");
    if cargo_toml.exists() {
        // Already has a Cargo.toml, assume it's set up correctly
        return Ok(());
    }

    // Derive a crate name from the .rs filename
    let stem = rs_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("cobol-program");
    let crate_name = stem.replace('.', "-");

    let src_dir = project_dir.join("src");
    std::fs::create_dir_all(&src_dir)
        .into_diagnostic()
        .wrap_err("failed to create src/")?;

    // Move/copy the .rs file to src/main.rs if it's not already there
    let main_rs = src_dir.join("main.rs");
    if rs_file != main_rs {
        std::fs::copy(rs_file, &main_rs)
            .into_diagnostic()
            .wrap_err("failed to copy transpiled file to src/main.rs")?;
    }

    let runtime_dep = if let Some(path) = runtime_path {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        format!(
            "cobol-runtime = {{ path = \"{}\", features = [\"full\"] }}",
            canonical.display()
        )
    } else {
        "cobol-runtime = { version = \"0.1\", features = [\"full\"] }".to_string()
    };

    let toml_content = format!(
        "[package]\n\
         name = \"{crate_name}\"\n\
         version = \"0.1.0\"\n\
         edition = \"2021\"\n\n\
         [dependencies]\n\
         {runtime_dep}\n\
         rust_decimal = \"1\"\n"
    );

    std::fs::write(&cargo_toml, toml_content)
        .into_diagnostic()
        .wrap_err("failed to write Cargo.toml for single-file build")?;

    Ok(())
}
