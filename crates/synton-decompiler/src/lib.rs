//! # Synton Decompiler
//!
//! Transpiler from Synton to human-readable languages like Python and TypeScript.
//!
//! Enables human-in-the-loop oversight by providing readable views of
//! AI-generated Synton code.

#![warn(missing_docs, unused_crate_dependencies)]

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use std::fmt;

pub mod python;
pub mod typescript;
pub mod codegen;

pub use python::PythonCodegen;
pub use typescript::TypeScriptCodegen;
pub use codegen::{Codegen, CodegenConfig, IndentStyle};

/// Target language for decompilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetLang {
    Python,
    TypeScript,
    JavaScript,
    Rust,
    C,
}

impl TargetLang {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::TypeScript | Self::JavaScript => "ts",
            Self::Rust => "rs",
            Self::C => "c",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Python => "Python",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Rust => "Rust",
            Self::C => "C",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "python" | "py" => Some(Self::Python),
            "typescript" | "ts" => Some(Self::TypeScript),
            "javascript" | "js" => Some(Self::JavaScript),
            "rust" | "rs" => Some(Self::Rust),
            "c" => Some(Self::C),
            _ => None,
        }
    }
}

/// Decompiler error
#[derive(Error, Debug, Diagnostic)]
pub enum DecompilerError {
    #[error("unsupported construct for {lang}: {construct}")]
    #[diagnostic(code(synton::decompiler::unsupported))]
    Unsupported {
        lang: String,
        construct: String,
        #[label("not supported in {lang}")]
        span: SourceSpan,
    },

    #[error("codegen error: {0}")]
    #[diagnostic(code(synton::decompiler::codegen))]
    Codegen(String, #[label] SourceSpan),

    #[error("invalid AST for decompilation")]
    #[diagnostic(code(synton::decompiler::invalid_ast))]
    InvalidAst {
        #[label("here")]
        span: SourceSpan,
    },
}

/// Result type for decompilation
pub type DecompilerResult<T> = Result<T, DecompilerError>;

/// Main decompiler
pub struct Decompiler {
    lang: TargetLang,
    config: CodegenConfig,
}

impl Decompiler {
    /// Create a new decompiler for the target language
    pub fn new(lang: TargetLang) -> Self {
        Self {
            lang,
            config: CodegenConfig::default(),
        }
    }

    /// Create with custom config
    pub fn with_config(lang: TargetLang, config: CodegenConfig) -> Self {
        Self { lang, config }
    }

    /// Decompile a module to a string
    pub fn decompile_module(&self, module: &synton_ast::Module) -> DecompilerResult<String> {
        match self.lang {
            TargetLang::Python => {
                let gen = PythonCodegen::new(self.config.clone());
                gen.gen_module(module)
            }
            TargetLang::TypeScript => {
                let gen = TypeScriptCodegen::new(self.config.clone());
                gen.gen_module(module)
            }
            _ => Err(DecompilerError::Unsupported {
                lang: self.lang.name().to_string(),
                construct: "module".to_string(),
                span: (0..0).into(),
            }),
        }
    }

    /// Decompile a single expression
    pub fn decompile_expr(&self, expr: &synton_ast::Expr) -> DecompilerResult<String> {
        match self.lang {
            TargetLang::Python => {
                let gen = PythonCodegen::new(self.config.clone());
                gen.gen_expr(expr)
            }
            TargetLang::TypeScript => {
                let gen = TypeScriptCodegen::new(self.config.clone());
                gen.gen_expr(expr)
            }
            _ => Err(DecompilerError::Unsupported {
                lang: self.lang.name().to_string(),
                construct: "expression".to_string(),
                span: (0..0).into(),
            }),
        }
    }

    /// Set indentation style
    pub fn with_indent(mut self, style: IndentStyle) -> Self {
        self.config.indent = style;
        self
    }
}

/// Convenience function to decompile a module
pub fn decompile(
    module: &synton_ast::Module,
    lang: TargetLang,
) -> DecompilerResult<String> {
    Decompiler::new(lang).decompile_module(module)
}
