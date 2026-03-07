//! `cobol2rust transpile` -- transpile COBOL source to Rust.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use miette::{miette, Context, IntoDiagnostic, Result};

use cobol_transpiler::transpile::{transpile_with_config, TranspileConfig};

use crate::workspace::{
    analyze_workspace, build_manifest, cobol_name_to_crate, discover_copybook_files,
    load_manifest_overrides, manifest_to_toml, ProgramType,
};
use crate::Cli;

/// Arguments for `cobol2rust transpile`.
#[derive(Debug, Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct TranspileArgs {
    /// COBOL source file or directory to transpile.
    pub input: PathBuf,

    /// Output file or directory (default: stdout for single file).
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Force main program output (generates `fn main()`).
    #[arg(long, conflicts_with = "lib")]
    pub main: bool,

    /// Force library output (no `fn main()`).
    #[arg(long, conflicts_with = "main")]
    pub lib: bool,

    /// COPY library mapping NAME=DIR (repeatable).
    #[arg(short = 'L', long = "library-map")]
    pub library_map: Vec<String>,

    /// Generate a Cargo workspace (required for directory input).
    #[arg(long)]
    pub workspace: bool,

    /// Skip files that fail, report at end.
    #[arg(long)]
    pub continue_on_error: bool,

    /// Read/write manifest for main/lib overrides.
    #[arg(long)]
    pub manifest: Option<PathBuf>,
}

/// Run the transpile subcommand.
pub fn run(cli: &Cli, args: &TranspileArgs) -> Result<ExitCode> {
    if args.input.is_dir() {
        if !args.workspace {
            return Err(miette!(
                "input is a directory; use --workspace for directory mode"
            ));
        }
        return run_workspace(cli, args);
    }

    // Single-file mode
    let source = fs::read_to_string(&args.input)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", args.input.display()))?;

    let config = build_config(cli, args)?;

    if !cli.quiet {
        eprintln!("Transpiling {}...", args.input.display());
    }

    let rust_source =
        transpile_with_config(&source, &config).map_err(|e| miette!("{e}"))?;

    match &args.output {
        Some(path) => {
            fs::write(path, &rust_source)
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to write {}", path.display()))?;
            if !cli.quiet {
                eprintln!("Wrote {}", path.display());
            }
        }
        None => {
            std::io::stdout()
                .write_all(rust_source.as_bytes())
                .into_diagnostic()
                .wrap_err("failed to write to stdout")?;
        }
    }

    Ok(ExitCode::SUCCESS)
}

/// Run workspace mode: transpile a directory of COBOL files into a Cargo workspace.
fn run_workspace(cli: &Cli, args: &TranspileArgs) -> Result<ExitCode> {
    let output_dir = args.output.as_ref().ok_or_else(|| {
        miette!("--output is required for workspace mode")
    })?;

    // Load overrides from existing manifest
    let overrides = load_manifest_overrides(args.manifest.as_deref())?;

    if !cli.quiet {
        eprintln!(
            "Analyzing {} for workspace transpilation...",
            args.input.display()
        );
    }

    // Analyze workspace
    let analysis =
        analyze_workspace(&args.input, &overrides, args.continue_on_error)?;

    for (path, err) in &analysis.errors {
        eprintln!("  warning: skipped {}: {err}", path.display());
    }

    if analysis.programs.is_empty() {
        return Err(miette!(
            "no programs found in {}",
            args.input.display()
        ));
    }

    let config = build_config(cli, args)?;

    // Create output directory structure
    fs::create_dir_all(output_dir)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to create {}", output_dir.display()))?;

    // Build workspace member list
    let has_copybooks = !analysis.all_copybooks.is_empty();
    let mut members = Vec::new();

    if has_copybooks {
        members.push("copybooks".to_string());
    }
    for (crate_name, info) in &analysis.programs {
        if info.program_type != ProgramType::Skip {
            members.push(format!("programs/{crate_name}"));
        }
    }

    // Write workspace Cargo.toml
    let workspace_toml = generate_workspace_cargo_toml(&members);
    fs::write(output_dir.join("Cargo.toml"), &workspace_toml)
        .into_diagnostic()
        .wrap_err("failed to write workspace Cargo.toml")?;

    // Create copybooks crate if needed
    if has_copybooks {
        create_copybooks_crate(output_dir, &analysis)?;
    }

    // Transpile each program
    let programs_dir = output_dir.join("programs");
    fs::create_dir_all(&programs_dir)
        .into_diagnostic()
        .wrap_err("failed to create programs/")?;

    let mut success_count = 0u32;
    let mut fail_count = 0u32;

    for (crate_name, info) in &analysis.programs {
        if info.program_type == ProgramType::Skip {
            continue;
        }

        let source_path = args.input.join(&info.source);
        let crate_dir = programs_dir.join(crate_name);
        let src_dir = crate_dir.join("src");

        fs::create_dir_all(&src_dir)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to create programs/{crate_name}/src")
            })?;

        // Generate program Cargo.toml
        let mut deps: Vec<String> = Vec::new();
        if has_copybooks {
            deps.push("copybooks".to_string());
        }
        for call_target in &info.calls {
            let target_crate = cobol_name_to_crate(call_target);
            if analysis.programs.contains_key(&target_crate) {
                deps.push(target_crate);
            }
        }

        let prog_cargo =
            generate_program_cargo_toml(crate_name, info.program_type, &deps);
        fs::write(crate_dir.join("Cargo.toml"), &prog_cargo)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to write programs/{crate_name}/Cargo.toml")
            })?;

        // Transpile the source file
        let entry_file = if info.program_type == ProgramType::Main {
            "main.rs"
        } else {
            "lib.rs"
        };

        match transpile_single(&source_path, &config) {
            Ok(rust_source) => {
                fs::write(src_dir.join(entry_file), &rust_source)
                    .into_diagnostic()
                    .wrap_err_with(|| {
                        format!(
                            "failed to write programs/{crate_name}/src/{entry_file}"
                        )
                    })?;
                success_count += 1;
                if cli.verbose > 0 && !cli.quiet {
                    eprintln!(
                        "  Transpiled {} -> programs/{crate_name}/src/{entry_file}",
                        info.source.display(),
                    );
                }
            }
            Err(e) => {
                if args.continue_on_error {
                    fail_count += 1;
                    eprintln!(
                        "  error: failed to transpile {}: {e}",
                        info.source.display(),
                    );
                } else {
                    return Err(e).wrap_err_with(|| {
                        format!("failed to transpile {}", info.source.display())
                    });
                }
            }
        }
    }

    // Write manifest
    let manifest = build_manifest(&analysis);
    let manifest_toml = manifest_to_toml(&manifest);
    let manifest_path = args
        .manifest
        .clone()
        .unwrap_or_else(|| output_dir.join("cobol2rust-manifest.toml"));
    fs::write(&manifest_path, &manifest_toml)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to write manifest {}", manifest_path.display())
        })?;

    if !cli.quiet {
        eprintln!(
            "Workspace transpiled: {success_count} succeeded, {fail_count} failed"
        );
        eprintln!("Output: {}", output_dir.display());
    }

    if fail_count > 0 {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Transpile a single source file and return the Rust source.
fn transpile_single(
    source_path: &std::path::Path,
    config: &TranspileConfig,
) -> Result<String> {
    let source = fs::read_to_string(source_path)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to read {}", source_path.display())
        })?;
    transpile_with_config(&source, config).map_err(|e| miette!("{e}"))
}

/// Build a `TranspileConfig` from CLI flags.
fn build_config(cli: &Cli, args: &TranspileArgs) -> Result<TranspileConfig> {
    let mut library_map = HashMap::new();
    for mapping in &args.library_map {
        let (name, dir) = mapping
            .split_once('=')
            .ok_or_else(|| {
                miette!("invalid library mapping '{mapping}': expected NAME=DIR")
            })?;
        library_map.insert(name.to_uppercase(), PathBuf::from(dir));
    }

    Ok(TranspileConfig {
        copybook_paths: cli.copybook_paths.clone(),
        library_map,
        max_copy_depth: 10,
    })
}

/// Generate workspace root `Cargo.toml`.
fn generate_workspace_cargo_toml(members: &[String]) -> String {
    let mut out = String::from("[workspace]\n");
    out.push_str("resolver = \"2\"\n");
    out.push_str("members = [\n");
    for m in members {
        let _ = writeln!(out, "    \"{m}\",");
    }
    out.push_str("]\n\n");
    out.push_str("[workspace.dependencies]\n");
    out.push_str("cobol-runtime = \"0.1\"\n");
    out
}

/// Generate a program crate `Cargo.toml`.
fn generate_program_cargo_toml(
    crate_name: &str,
    program_type: ProgramType,
    deps: &[String],
) -> String {
    let mut out = String::new();
    out.push_str("[package]\n");
    let _ = writeln!(out, "name = \"{crate_name}\"");
    out.push_str("version = \"0.1.0\"\n");
    out.push_str("edition = \"2021\"\n\n");

    if program_type == ProgramType::Main {
        out.push_str("[[bin]]\n");
        let _ = writeln!(out, "name = \"{crate_name}\"");
        out.push_str("path = \"src/main.rs\"\n\n");
    }

    out.push_str("[dependencies]\n");
    out.push_str("cobol-runtime = { workspace = true }\n");
    for dep in deps {
        let _ = writeln!(out, "{dep} = {{ path = \"../{dep}\" }}");
    }
    out
}

/// Create the copybooks crate with placeholder lib.rs.
fn create_copybooks_crate(
    output_dir: &std::path::Path,
    analysis: &crate::workspace::WorkspaceAnalysis,
) -> Result<()> {
    let cb_dir = output_dir.join("copybooks").join("src");
    fs::create_dir_all(&cb_dir)
        .into_diagnostic()
        .wrap_err("failed to create copybooks/src")?;

    let mut cargo = String::new();
    cargo.push_str("[package]\n");
    cargo.push_str("name = \"copybooks\"\n");
    cargo.push_str("version = \"0.1.0\"\n");
    cargo.push_str("edition = \"2021\"\n\n");
    cargo.push_str("[dependencies]\n");
    cargo.push_str("cobol-runtime = { workspace = true }\n");

    fs::write(output_dir.join("copybooks/Cargo.toml"), &cargo)
        .into_diagnostic()
        .wrap_err("failed to write copybooks/Cargo.toml")?;

    let mut lib = String::from("//! Shared copybook types.\n");
    lib.push_str("//! Auto-generated by `cobol2rust transpile --workspace`.\n\n");

    let mut all_cpy_files = Vec::new();
    for dir in &analysis.copybook_dirs {
        all_cpy_files.extend(discover_copybook_files(dir));
    }

    if all_cpy_files.is_empty() {
        lib.push_str("// No copybook files discovered.\n");
    } else {
        lib.push_str("// Copybook files to transpile:\n");
        for f in &all_cpy_files {
            let _ = writeln!(lib, "//   {f}");
        }
    }

    fs::write(cb_dir.join("lib.rs"), &lib)
        .into_diagnostic()
        .wrap_err("failed to write copybooks/src/lib.rs")?;

    Ok(())
}
