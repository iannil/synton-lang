//! Synton CLI Tool
//!
//! Command-line interface for the Synton programming language.

#![warn(missing_docs, unused_crate_dependencies)]

use clap::{Parser, Subcommand};
use miette::{Diagnostic, Error, IntoDiagnostic, NarratableReportHandler, Result};
use std::path::PathBuf;
use tracing::debug;
use tracing_subscriber::EnvFilter;

mod commands;
mod repl;
mod output;

use commands::{ParseCommand, CheckCommand, RunCommand, DecompileCommand, LspCommand};

/// Synton - AI-native programming language
#[derive(Parser, Debug)]
#[command(name = "synton")]
#[command(author = "Synton Lang Contributors")]
#[command(version)]
#[command(about = "Synton programming language toolchain", long_about = None)]
struct Cli {
    /// Suppress output
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Verbose output (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parse source code and output AST
    Parse {
        /// Input file
        input: PathBuf,

        /// Output format (json, text)
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Type check source code
    Check {
        /// Input file
        input: PathBuf,

        /// Emit DSO (Debug State Object) on error
        #[arg(long)]
        emit_dso: bool,
    },

    /// Run a Synton program
    Run {
        /// Input file
        input: PathBuf,

        /// Input values (JSON)
        #[arg(short, long)]
        values: Option<String>,

        /// Show execution trace
        #[arg(long)]
        trace: bool,
    },

    /// Decompile to another language
    Decompile {
        /// Input file
        input: PathBuf,

        /// Target language (python, typescript)
        #[arg(short, long, default_value = "python")]
        lang: String,

        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Start REPL
    Repl {
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Start LSP server
    Lsp {
        /// Use stdio transport
        #[arg(long)]
        stdio: bool,
    },

    /// Build a project
    Build {
        /// Project directory
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,

        /// Output directory
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Set up miette for error reporting
    miette::set_hook(Box::new(|_| {
        Box::new(NarratableReportHandler::new().with_cause_chain())
    }))?;

    // Set up tracing
    let filter = match args.verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        _ => "debug",
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("SYNTON_LOG").add_directive(filter.parse().unwrap()))
        .with_writer(std::io::stderr)
        .init();

    debug!("synton CLI starting");

    // Run command
    match args.command {
        Commands::Parse { input, format, output } => {
            ParseCommand::new(input, format, output).run()?;
        }
        Commands::Check { input, emit_dso } => {
            CheckCommand::new(input, emit_dso).run()?;
        }
        Commands::Run { input, values, trace } => {
            RunCommand::new(input, values, trace).run()?;
        }
        Commands::Decompile { input, lang, output } => {
            DecompileCommand::new(input, lang, output).run()?;
        }
        Commands::Repl { verbose } => {
            repl::Repl::new(verbose).run()?;
        }
        Commands::Lsp { stdio } => {
            LspCommand::new(stdio).run()?;
        }
        Commands::Build { dir, out } => {
            println!("Build not yet implemented");
        }
    }

    Ok(())
}
