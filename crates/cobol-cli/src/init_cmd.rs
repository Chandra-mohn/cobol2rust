//! `cobol2rust init` -- scaffold a Cargo workspace from COBOL sources.

use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use miette::{miette, Context, IntoDiagnostic, Result};

use crate::workspace::{
    analyze_workspace, build_manifest, cobol_name_to_crate, discover_copybook_files,
    load_manifest_overrides, manifest_to_toml, ProgramType,
};
use crate::Cli;

/// Arguments for `cobol2rust init`.
#[derive(Debug, Args)]
pub struct InitArgs {
    /// Directory containing COBOL source files.
    pub input: PathBuf,

    /// Output directory for the Rust workspace.
    #[arg(short, long)]
    pub output: PathBuf,

    /// Show what would be created without writing.
    #[arg(long)]
    pub dry_run: bool,

    /// Write/read a TOML manifest for main/lib overrides.
    #[arg(long)]
    pub manifest: Option<PathBuf>,
}

/// Run the init subcommand.
pub fn run(cli: &Cli, args: &InitArgs) -> Result<ExitCode> {
    if !args.input.is_dir() {
        return Err(miette!(
            "input must be a directory: {}",
            args.input.display()
        ));
    }

    // Load overrides from existing manifest if provided
    let overrides = load_manifest_overrides(args.manifest.as_deref())?;

    if !cli.quiet {
        eprintln!(
            "Scanning {} for COBOL sources...",
            args.input.display()
        );
    }

    // Analyze workspace
    let analysis = analyze_workspace(&args.input, &overrides, true)?;

    // Report errors
    for (path, err) in &analysis.errors {
        eprintln!("  warning: skipped {}: {err}", path.display());
    }

    if analysis.programs.is_empty() {
        return Err(miette!("no programs found in {}", args.input.display()));
    }

    // Print summary
    let main_count = analysis
        .programs
        .values()
        .filter(|p| p.program_type == ProgramType::Main)
        .count();
    let lib_count = analysis
        .programs
        .values()
        .filter(|p| p.program_type == ProgramType::Lib)
        .count();

    if !cli.quiet {
        eprintln!(
            "Found {} programs ({} main, {} lib), {} copybooks",
            analysis.programs.len(),
            main_count,
            lib_count,
            analysis.all_copybooks.len(),
        );

        if cli.verbose > 0 {
            for (crate_name, info) in &analysis.programs {
                eprintln!(
                    "  {} ({}) -> {} [{}]",
                    info.program_id,
                    crate_name,
                    info.program_type,
                    info.source.display(),
                );
            }
        }
    }

    // Build manifest
    let manifest = build_manifest(&analysis);
    let manifest_toml = manifest_to_toml(&manifest);

    if args.dry_run {
        // Dry run: just print what would be created
        eprintln!("\n--- Dry run: would create ---");
        print_dry_run(&args.output, &analysis);
        eprintln!("\n--- Manifest ---");
        print!("{manifest_toml}");
        return Ok(ExitCode::SUCCESS);
    }

    // Create the workspace skeleton
    create_workspace_skeleton(&args.output, &analysis, cli)?;

    // Write manifest
    let manifest_path = args
        .manifest
        .clone()
        .unwrap_or_else(|| args.output.join("cobol2rust-manifest.toml"));
    fs::write(&manifest_path, &manifest_toml)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write manifest {}", manifest_path.display()))?;

    if !cli.quiet {
        eprintln!("Wrote manifest: {}", manifest_path.display());
        eprintln!("Workspace created at: {}", args.output.display());
        eprintln!(
            "\nNext steps:\n  1. Review and edit {}\n  2. Run: cobol2rust transpile {} -o {} --workspace",
            manifest_path.display(),
            args.input.display(),
            args.output.display(),
        );
    }

    Ok(ExitCode::SUCCESS)
}

/// Print what would be created in dry-run mode.
fn print_dry_run(
    output: &std::path::Path,
    analysis: &crate::workspace::WorkspaceAnalysis,
) {
    eprintln!("  {}/", output.display());
    eprintln!("    Cargo.toml  (workspace)");
    eprintln!("    cobol2rust-manifest.toml");

    if !analysis.all_copybooks.is_empty() {
        eprintln!("    copybooks/");
        eprintln!("      Cargo.toml");
        eprintln!("      src/lib.rs  (placeholder)");
    }

    eprintln!("    programs/");
    for (crate_name, info) in &analysis.programs {
        if info.program_type == ProgramType::Skip {
            continue;
        }
        let entry_file = if info.program_type == ProgramType::Main {
            "main.rs"
        } else {
            "lib.rs"
        };
        eprintln!("      {crate_name}/");
        eprintln!("        Cargo.toml");
        eprintln!("        src/{entry_file}  (placeholder)");
    }
}

/// Create the workspace directory structure with placeholder files.
fn create_workspace_skeleton(
    output: &std::path::Path,
    analysis: &crate::workspace::WorkspaceAnalysis,
    cli: &Cli,
) -> Result<()> {
    // Create root directory
    fs::create_dir_all(output)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to create {}", output.display()))?;

    // Build workspace member list
    let mut members = Vec::new();
    let has_copybooks = !analysis.all_copybooks.is_empty();

    if has_copybooks {
        members.push("copybooks".to_string());
    }
    for (crate_name, info) in &analysis.programs {
        if info.program_type != ProgramType::Skip {
            members.push(format!("programs/{crate_name}"));
        }
    }

    // Write root Cargo.toml
    let workspace_toml = generate_workspace_cargo_toml(&members);
    fs::write(output.join("Cargo.toml"), &workspace_toml)
        .into_diagnostic()
        .wrap_err("failed to write workspace Cargo.toml")?;

    // Create copybooks crate if needed
    if has_copybooks {
        let cb_dir = output.join("copybooks").join("src");
        fs::create_dir_all(&cb_dir)
            .into_diagnostic()
            .wrap_err("failed to create copybooks/src")?;

        let cb_cargo = generate_copybook_cargo_toml();
        fs::write(output.join("copybooks/Cargo.toml"), &cb_cargo)
            .into_diagnostic()
            .wrap_err("failed to write copybooks/Cargo.toml")?;

        // Collect actual copybook file names from discovered directories
        let mut all_cpy_files = Vec::new();
        for dir in &analysis.copybook_dirs {
            all_cpy_files.extend(discover_copybook_files(dir));
        }

        let cb_lib = generate_copybook_lib_rs(&all_cpy_files);
        fs::write(cb_dir.join("lib.rs"), &cb_lib)
            .into_diagnostic()
            .wrap_err("failed to write copybooks/src/lib.rs")?;
    }

    // Create program crates
    let programs_dir = output.join("programs");
    fs::create_dir_all(&programs_dir)
        .into_diagnostic()
        .wrap_err("failed to create programs/")?;

    for (crate_name, info) in &analysis.programs {
        if info.program_type == ProgramType::Skip {
            continue;
        }

        let crate_dir = programs_dir.join(crate_name).join("src");
        fs::create_dir_all(&crate_dir)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to create programs/{crate_name}/src"))?;

        // Determine dependencies: cobol-runtime + copybooks + called programs
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
        fs::write(
            programs_dir.join(crate_name).join("Cargo.toml"),
            &prog_cargo,
        )
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to write programs/{crate_name}/Cargo.toml")
        })?;

        let entry_file = if info.program_type == ProgramType::Main {
            "main.rs"
        } else {
            "lib.rs"
        };
        let placeholder = generate_placeholder_rs(info, crate_name);
        fs::write(crate_dir.join(entry_file), &placeholder)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to write programs/{crate_name}/src/{entry_file}")
            })?;

        if cli.verbose > 0 && !cli.quiet {
            eprintln!(
                "  Created programs/{crate_name}/ ({}, from {})",
                info.program_type,
                info.source.display(),
            );
        }
    }

    Ok(())
}

/// Generate the workspace root `Cargo.toml`.
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

/// Generate `copybooks/Cargo.toml`.
fn generate_copybook_cargo_toml() -> String {
    let mut out = String::new();
    out.push_str("[package]\n");
    out.push_str("name = \"copybooks\"\n");
    out.push_str("version = \"0.1.0\"\n");
    out.push_str("edition = \"2021\"\n\n");
    out.push_str("[dependencies]\n");
    out.push_str("cobol-runtime = { workspace = true }\n");
    out
}

/// Generate `copybooks/src/lib.rs` placeholder.
fn generate_copybook_lib_rs(cpy_files: &[String]) -> String {
    let mut out = String::from("//! Shared copybook types.\n");
    out.push_str("//!\n");
    out.push_str("//! Auto-generated by `cobol2rust init`.\n");
    out.push_str("//! Transpile copybooks to fill in the actual types.\n\n");
    if cpy_files.is_empty() {
        out.push_str("// No copybook files discovered.\n");
    } else {
        out.push_str("// Copybook files to transpile:\n");
        for f in cpy_files {
            let _ = writeln!(out, "//   {f}");
        }
    }
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

/// Generate a placeholder `.rs` source file.
fn generate_placeholder_rs(
    info: &crate::workspace::ProgramInfo,
    crate_name: &str,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "// Placeholder for COBOL program: {}", info.program_id);
    let _ = writeln!(out, "// Source: {}", info.source.display());
    let _ = writeln!(
        out,
        "// Transpile with: cobol2rust transpile {} -o programs/{crate_name}/src/",
        info.source.display(),
    );
    out.push('\n');
    if info.program_type == ProgramType::Main {
        out.push_str("fn main() {\n");
        out.push_str("    // TODO: transpile COBOL source to fill in this file\n");
        out.push_str("}\n");
    }
    out
}
