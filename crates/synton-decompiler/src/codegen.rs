//! Code generation utilities

use synton_ast::{Module, Expr, Stmt, Type, Span};
use crate::DecompilerResult;

/// Code generator configuration
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Indentation style
    pub indent: IndentStyle,
    /// Include type annotations
    pub types: bool,
    /// Include comments
    pub comments: bool,
    /// Pretty print
    pub pretty: bool,
    /// Line width (for pretty printing)
    pub line_width: usize,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            indent: IndentStyle::Spaces(4),
            types: true,
            comments: true,
            pretty: true,
            line_width: 100,
        }
    }
}

/// Indentation style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentStyle {
    Spaces(usize),
    Tabs,
}

impl IndentStyle {
    pub fn to_string(&self, level: usize) -> String {
        match self {
            Self::Spaces(n) => " ".repeat(level * n),
            Self::Tabs => "\t".repeat(level),
        }
    }
}

/// Base code generator trait
pub trait Codegen {
    fn config(&self) -> &CodegenConfig;

    fn indent(&self, level: usize) -> String {
        self.config().indent.to_string(level)
    }

    fn gen_module(&self, module: &Module) -> DecompilerResult<String>;
    fn gen_expr(&self, expr: &Expr) -> DecompilerResult<String>;
    fn gen_stmt(&self, stmt: &Stmt) -> DecompilerResult<String>;
    fn gen_type(&self, ty: &Type) -> DecompilerResult<String>;

    /// Helper to join items with separators
    fn join<T, F>(&self, items: &[T], sep: &str, f: F) -> String
    where
        F: Fn(&T) -> DecompilerResult<String>,
    {
        items.iter()
            .filter_map(|item| f(item).ok())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

/// Buffer for building code output
pub struct CodeBuffer {
    buf: String,
    indent_level: usize,
    indent: IndentStyle,
}

impl CodeBuffer {
    pub fn new(indent: IndentStyle) -> Self {
        Self {
            buf: String::new(),
            indent_level: 0,
            indent,
        }
    }

    pub fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn push_line(&mut self, s: &str) {
        self.buf.push_str(&self.indent.to_string(self.indent_level));
        self.buf.push_str(s);
        self.buf.push('\n');
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn finish(self) -> String {
        self.buf
    }
}
