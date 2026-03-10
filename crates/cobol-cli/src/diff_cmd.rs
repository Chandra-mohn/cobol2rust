//! `cobol2rust diff` -- compare transpilation outputs between two COBOL files.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use miette::{miette, Result};
use similar::{Algorithm, ChangeTag, TextDiff};

use cobol_transpiler::transpile::{transpile_with_config, TranspileConfig};

use crate::Cli;

/// Output format for diff.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DiffFormat {
    /// Unified diff format (default).
    Unified,
    /// JSON diff output.
    Json,
}

/// Arguments for `cobol2rust diff`.
#[derive(Debug, Args)]
pub struct DiffArgs {
    /// First COBOL source file (old/before).
    pub old: PathBuf,

    /// Second COBOL source file (new/after).
    pub new: PathBuf,

    /// Compare AST structures instead of generated Rust.
    #[arg(long)]
    pub ast: bool,

    /// Number of context lines around differences.
    #[arg(long, default_value = "3")]
    pub context: usize,

    /// Output format.
    #[arg(long, default_value = "unified")]
    pub format: DiffFormat,

    /// Disable colored diff output.
    #[arg(long)]
    pub no_color: bool,

    /// COPY library mapping NAME=DIR (repeatable).
    #[arg(short = 'L', long = "library-map")]
    pub library_map: Vec<String>,
}

/// Run the diff subcommand.
pub fn run(cli: &Cli, args: &DiffArgs) -> Result<ExitCode> {
    let config = build_diff_config(cli, args)?;

    let old_source = fs::read_to_string(&args.old)
        .map_err(|e| miette!("failed to read {}: {e}", args.old.display()))?;
    let new_source = fs::read_to_string(&args.new)
        .map_err(|e| miette!("failed to read {}: {e}", args.new.display()))?;

    let (old_output, new_output) = if args.ast {
        let old_ast = transpile_to_ast_json(&old_source, &config, &args.old)?;
        let new_ast = transpile_to_ast_json(&new_source, &config, &args.new)?;
        (old_ast, new_ast)
    } else {
        let old_rust = transpile_with_config(&old_source, &config)
            .map_err(|e| miette!("failed to transpile {}: {e}", args.old.display()))?;
        let new_rust = transpile_with_config(&new_source, &config)
            .map_err(|e| miette!("failed to transpile {}: {e}", args.new.display()))?;
        (old_rust, new_rust)
    };

    if old_output == new_output {
        if !cli.quiet {
            eprintln!("No differences.");
        }
        return Ok(ExitCode::SUCCESS);
    }

    match args.format {
        DiffFormat::Unified => {
            print_unified_diff(args, &old_output, &new_output);
        }
        DiffFormat::Json => {
            print_json_diff(args, &old_output, &new_output)?;
        }
    }

    // Exit code 1 when differences exist (like standard diff)
    Ok(ExitCode::from(1))
}

/// Print a unified diff to stdout.
fn print_unified_diff(args: &DiffArgs, old: &str, new: &str) {
    let diff = TextDiff::configure()
        .algorithm(Algorithm::Patience)
        .diff_lines(old, new);

    let old_label = args.old.display().to_string();
    let new_label = args.new.display().to_string();
    let use_color = !args.no_color && atty_stdout();

    if use_color {
        println!("\x1b[1m--- {old_label}\x1b[0m");
        println!("\x1b[1m+++ {new_label}\x1b[0m");
        for (idx, group) in diff.grouped_ops(args.context).iter().enumerate() {
            if idx > 0 {
                println!("...");
            }
            for op in group {
                for change in diff.iter_changes(op) {
                    let line: &str = change.value();
                    match change.tag() {
                        ChangeTag::Delete => {
                            print!("\x1b[31m-{line}\x1b[0m");
                        }
                        ChangeTag::Insert => {
                            print!("\x1b[32m+{line}\x1b[0m");
                        }
                        ChangeTag::Equal => {
                            print!(" {line}");
                        }
                    }
                }
            }
        }
    } else {
        print!(
            "{}",
            diff.unified_diff()
                .header(&old_label, &new_label)
        );
    }
}

/// Print a JSON diff to stdout.
fn print_json_diff(args: &DiffArgs, old: &str, new: &str) -> Result<()> {
    let diff = TextDiff::configure()
        .algorithm(Algorithm::Patience)
        .diff_lines(old, new);

    let mut changes = Vec::new();

    for group in diff.grouped_ops(args.context) {
        for op in &group {
            for change in diff.iter_changes(op) {
                let tag = match change.tag() {
                    ChangeTag::Delete => "delete",
                    ChangeTag::Insert => "insert",
                    ChangeTag::Equal => "equal",
                };
                let line: &str = change.value();
                changes.push(serde_json::json!({
                    "tag": tag,
                    "old_index": change.old_index(),
                    "new_index": change.new_index(),
                    "value": line,
                }));
            }
        }
    }

    let output = serde_json::json!({
        "has_differences": true,
        "changes": changes,
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&output)
            .map_err(|e| miette!("JSON serialization error: {e}"))?
    );
    Ok(())
}

/// Transpile COBOL to AST JSON for comparison.
fn transpile_to_ast_json(
    source: &str,
    config: &TranspileConfig,
    path: &std::path::Path,
) -> Result<String> {
    use cobol_transpiler::parser::copy_expand::CopyExpander;
    use cobol_transpiler::parser::copybook::FileSystemResolver;
    use cobol_transpiler::parser::parse_cobol;

    let resolver = FileSystemResolver::new(config.copybook_paths.clone())
        .with_library_map(config.library_map.clone());
    let expander = CopyExpander::new(Box::new(resolver), config.max_copy_depth);
    let expanded = expander
        .expand(source)
        .map_err(|e| miette!("COPY expansion failed for {}: {e}", path.display()))?;

    let program = parse_cobol(&expanded)
        .map_err(|e| miette!("failed to parse {}: {e}", path.display()))?;

    serde_json::to_string_pretty(&program)
        .map_err(|e| miette!("failed to serialize AST for {}: {e}", path.display()))
}

/// Build a `TranspileConfig` from CLI flags for diff.
fn build_diff_config(cli: &Cli, args: &DiffArgs) -> Result<TranspileConfig> {
    let mut library_map = std::collections::HashMap::new();
    for mapping in &args.library_map {
        let (name, dir) = mapping
            .split_once('=')
            .ok_or_else(|| miette!("invalid library mapping '{mapping}': expected NAME=DIR"))?;
        library_map.insert(name.to_uppercase(), PathBuf::from(dir));
    }

    Ok(TranspileConfig {
        copybook_paths: cli.copybook_paths.clone(),
        library_map,
        max_copy_depth: 10,
    })
}

/// Check if stdout is a terminal (for color support).
fn atty_stdout() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}
