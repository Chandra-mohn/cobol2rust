//! `cobol2rust project` -- single project pipeline.
//!
//! Runs the full phase engine (0-5) for a single COBOL project,
//! with checkpoint resumption and config-file-first CLI design.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use miette::Result;

use crate::pipeline::config::{CliOverrides, resolve_config};
use crate::workspace::load_project_config;
use crate::Cli;

/// Arguments for `cobol2rust project`.
#[derive(Debug, Args)]
pub struct ProjectArgs {
    /// COBOL project root directory.
    pub project_dir: PathBuf,

    /// Override output directory (default from .cobol2rust.toml or ./rust-out).
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Override parallel workers (default from .cobol2rust.toml or num CPUs).
    #[arg(short = 'j', long)]
    pub jobs: Option<usize>,

    /// Run a specific phase: 0,1,2,3,4,5.
    #[arg(long)]
    pub phase: Option<u8>,

    /// Start from phase (default: 0).
    #[arg(long)]
    pub from: Option<u8>,

    /// Stop after phase (default: 5).
    #[arg(long)]
    pub to: Option<u8>,
}

/// Run the `project` subcommand.
pub fn run(cli: &Cli, args: &ProjectArgs) -> Result<ExitCode> {
    // Load project config from .cobol2rust.toml
    let project_config = load_project_config(&args.project_dir)?;

    // Build CLI overrides
    let overrides = CliOverrides {
        output: args.output.clone(),
        jobs: args.jobs,
        phase: args.phase,
        from: args.from,
        to: args.to,
        verbose: cli.verbose,
        quiet: cli.quiet,
    };

    // Resolve: defaults -> config file -> CLI overrides
    let config = resolve_config(
        &args.project_dir,
        project_config.as_ref(),
        &overrides,
    );

    // Run the pipeline
    crate::pipeline::run_pipeline(&config)
}
