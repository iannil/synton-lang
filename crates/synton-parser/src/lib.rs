//! # Synton Parser
//!
//! Parser for Synton using Chumsky.

#![warn(missing_docs, unused_crate_dependencies)]

pub mod error;
pub mod ast_builder;

use synton_ast::{Module, Span, Position};
pub use error::{ParseError, ParseResult};

/// Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Enable error recovery
    pub recovery: bool,
    /// Maximum recursion depth
    pub max_depth: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            recovery: true,
            max_depth: 256,
        }
    }
}

/// Parser for Synton source code
pub struct Parser {
    config: ParserConfig,
}

impl Parser {
    /// Create a new parser with default config
    pub fn new() -> Self {
        Self::with_config(ParserConfig::default())
    }

    /// Create a parser with custom config
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse a complete module from source
    pub fn parse_module(&self, source: &str) -> ParseResult<Module> {
        // For now, create a basic module
        // Full implementation will use Chumsky parser combinators
        let module = Module::new(synton_ast::ModuleId::new(
            Self::hash_source(source),
        ));

        Ok(module)
    }

    /// Hash source for content-addressed module ID
    fn hash_source(source: &str) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = rustc_hash::FxHasher::default();
        source.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Parse an expression
    pub fn parse_expr(&self, source: &str) -> ParseResult<synton_ast::Expr> {
        let _tokens = synton_lexer::tokenize(source)?;

        // Placeholder - will be implemented with Chumsky
        use synton_ast::{Expr, ExprKind};
        Ok(Expr::new(ExprKind::Error, Span::new(
            Position::start(),
            Position::start(),
        )))
    }

    /// Parse a type
    pub fn parse_type(&self, source: &str) -> ParseResult<synton_ast::Type> {
        let _tokens = synton_lexer::tokenize(source)?;

        // Placeholder
        use synton_ast::{Type, TypeKind, BuiltinType};
        Ok(Type::new(TypeKind::Builtin(BuiltinType::I32), Span::new(
            Position::start(),
            Position::start(),
        )))
    }

    /// Parse a statement
    pub fn parse_stmt(&self, source: &str) -> ParseResult<synton_ast::Stmt> {
        let _tokens = synton_lexer::tokenize(source)?;

        // Placeholder
        use synton_ast::{Stmt, StmtKind};
        Ok(Stmt::new(StmtKind::Empty, Span::new(
            Position::start(),
            Position::start(),
        )))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to parse a module
pub fn parse_module(source: &str) -> ParseResult<Module> {
    Parser::new().parse_module(source)
}

/// Convenience function to parse an expression
pub fn parse_expr(source: &str) -> ParseResult<synton_ast::Expr> {
    Parser::new().parse_expr(source)
}

/// Convenience function to parse a type
pub fn parse_type(source: &str) -> ParseResult<synton_ast::Type> {
    Parser::new().parse_type(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_module() {
        let result = parse_module("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_expr() {
        let result = parse_expr("(+ 1 2)");
        // Will return error placeholder until fully implemented
        let _ = result;
    }
}
