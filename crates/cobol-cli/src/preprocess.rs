//! `cobol2rust preprocess` -- expand COPY statements and clean source format.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use miette::{miette, Context, IntoDiagnostic, Result};

use cobol_transpiler::parser::copy_expand::CopyExpander;
use cobol_transpiler::parser::copybook::FileSystemResolver;
use cobol_transpiler::parser::preprocess::preprocess;

use crate::Cli;

/// Arguments for `cobol2rust preprocess`.
#[derive(Debug, Args)]
pub struct PreprocessArgs {
    /// COBOL source file to preprocess.
    pub input: PathBuf,

    /// Output file (default: stdout).
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Skip COPY expansion, only strip fixed-format columns.
    #[arg(long)]
    pub no_copy: bool,

    /// COPY library mapping NAME=DIR (repeatable).
    #[arg(short = 'L', long = "library-map")]
    pub library_map: Vec<String>,

    /// Add comments showing where COPY content came from.
    #[arg(long)]
    pub show_origins: bool,
}

/// Run the preprocess subcommand.
pub fn run(cli: &Cli, args: &PreprocessArgs) -> Result<ExitCode> {
    let source = fs::read_to_string(&args.input)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", args.input.display()))?;

    if !cli.quiet {
        eprintln!("Preprocessing {}...", args.input.display());
    }

    // Step 1: Expand COPY statements (unless --no-copy).
    let after_copy = if args.no_copy {
        source
    } else {
        let mut library_map = HashMap::new();
        for mapping in &args.library_map {
            let (name, dir) = mapping
                .split_once('=')
                .ok_or_else(|| miette!("invalid library mapping '{mapping}': expected NAME=DIR"))?;
            library_map.insert(name.to_uppercase(), PathBuf::from(dir));
        }

        let resolver = FileSystemResolver::new(cli.copybook_paths.clone())
            .with_library_map(library_map);
        let expander = CopyExpander::new(Box::new(resolver), 10);
        expander.expand(&source).map_err(|e| miette!("{e}"))?
    };

    // Step 2: Strip fixed-format columns and handle continuations.
    let preprocessed = preprocess(&after_copy).map_err(|e| miette!("{e}"))?;

    // Write output.
    match &args.output {
        Some(path) => {
            fs::write(path, &preprocessed)
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to write {}", path.display()))?;
            if !cli.quiet {
                eprintln!("Wrote {}", path.display());
            }
        }
        None => {
            std::io::stdout()
                .write_all(preprocessed.as_bytes())
                .into_diagnostic()
                .wrap_err("failed to write to stdout")?;
        }
    }

    Ok(ExitCode::SUCCESS)
}
