//! CLI commands

use std::path::PathBuf;
use std::fs;
use miette::{IntoDiagnostic, Result, WrapErr, miette};

pub struct ParseCommand {
    input: PathBuf,
    format: String,
    output: Option<PathBuf>,
}

impl ParseCommand {
    pub fn new(input: PathBuf, format: String, output: Option<PathBuf>) -> Self {
        Self { input, format, output }
    }

    pub fn run(self) -> Result<()> {
        let source = fs::read_to_string(&self.input)
            .into_diagnostic()
            .wrap_err("Failed to read input file")?;

        let module = synton_parser::parse_module(&source)
            .map_err(|e| miette!("Parse error: {}", e))?;

        let result = match self.format.as_str() {
            "json" => serde_json::to_string_pretty(&module)
                .into_diagnostic()
                .wrap_err("Failed to serialize AST")?,
            "text" => format!("{:#?}", module),
            _ => return Err(miette!("Unknown format: {}", self.format)),
        };

        match self.output {
            Some(path) => {
                fs::write(&path, result)
                    .into_diagnostic()
                    .wrap_err("Failed to write output")?;
                eprintln!("AST written to {}", path.display());
            }
            None => {
                println!("{}", result);
            }
        }

        Ok(())
    }
}

pub struct CheckCommand {
    input: PathBuf,
    emit_dso: bool,
}

impl CheckCommand {
    pub fn new(input: PathBuf, emit_dso: bool) -> Self {
        Self { input, emit_dso }
    }

    pub fn run(self) -> Result<()> {
        let source = fs::read_to_string(&self.input)
            .into_diagnostic()
            .wrap_err("Failed to read input file")?;

        let module = synton_parser::parse_module(&source)
            .map_err(|e| miette!("Parse error: {}", e))?;

        synton_typeck::check(&module)
            .map_err(|e| miette!("Type check error: {}", e))?;

        eprintln!("Type check passed!");
        Ok(())
    }
}

pub struct RunCommand {
    input: PathBuf,
    values: Option<String>,
    trace: bool,
}

impl RunCommand {
    pub fn new(input: PathBuf, values: Option<String>, trace: bool) -> Self {
        Self { input, values, trace }
    }

    pub fn run(self) -> Result<()> {
        let source = fs::read_to_string(&self.input)
            .into_diagnostic()
            .wrap_err("Failed to read input file")?;

        // Parse
        let module = synton_parser::parse_module(&source)
            .map_err(|e| miette!("Parse error: {}", e))?;

        // Type check
        synton_typeck::check(&module)
            .map_err(|e| miette!("Type check error: {}", e))?;

        // TODO: Compile and run
        eprintln!("Run not yet fully implemented");
        Ok(())
    }
}

pub struct DecompileCommand {
    input: PathBuf,
    lang: String,
    output: Option<PathBuf>,
}

impl DecompileCommand {
    pub fn new(input: PathBuf, lang: String, output: Option<PathBuf>) -> Self {
        Self { input, lang, output }
    }

    pub fn run(self) -> Result<()> {
        let source = fs::read_to_string(&self.input)
            .into_diagnostic()
            .wrap_err("Failed to read input file")?;

        let module = synton_parser::parse_module(&source)
            .map_err(|e| miette!("Parse error: {}", e))?;

        let lang = synton_decompiler::TargetLang::from_name(&self.lang)
            .ok_or_else(|| miette!("Unknown target language: {}", self.lang))?;

        let result = synton_decompiler::decompile(&module, lang)
            .map_err(|e| miette!("Decompile error: {}", e))?;

        let output = self.output.unwrap_or_else(|| {
            let mut out = self.input.clone();
            out.set_extension(lang.extension());
            out
        });

        fs::write(&output, result)
            .into_diagnostic()
            .wrap_err("Failed to write output")?;

        eprintln!("Decompiled to {}", output.display());
        Ok(())
    }
}

pub struct LspCommand {
    stdio: bool,
}

impl LspCommand {
    pub fn new(stdio: bool) -> Self {
        Self { stdio }
    }

    pub fn run(self) -> Result<()> {
        #[cfg(feature = "lsp")]
        {
            eprintln!("Starting Synton LSP server...");

            use tokio::runtime::Runtime;
            use tower_lsp::LspService;

            let runtime = Runtime::new()
                .into_diagnostic()
                .wrap_err("Failed to create async runtime")?;

            runtime.block_on(async {
                let (stdin, stdout) = tokio::io::split(tokio::io::stdin());
                let service = LspService::new(|client| synton_lsp::SyntonServer::new(client));
                tower_lsp::Server::new(stdin, stdout).serve(service).await;
            });

            Ok(())
        }

        #[cfg(not(feature = "lsp"))]
        {
            Err(miette!("LSP feature not enabled. Recompile with --features lsp"))
        }
    }
}
