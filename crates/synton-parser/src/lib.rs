//! # Synton Parser
//!
//! Parser for Synton using Chumsky.

#![warn(missing_docs, unused_crate_dependencies)]

pub mod error;
pub mod ast_builder;
pub mod expr_parser;
pub mod stmt_parser;
pub mod grammar;

use chumsky::Parser;
use synton_lexer::TokenKind;
use synton_ast::{Module, Span, Position, Expr, Stmt};
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
pub struct SyntonParser {
    config: ParserConfig,
}

impl SyntonParser {
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
        let token_kinds = synton_lexer::tokenize(source)?;
        let parser = grammar::module_parser();
        parser.parse(&token_kinds)
            .into_result()
            .map_err(|err| ParseError::InvalidSyntax { message: format!("{:?}", err) })
    }

    /// Parse an expression
    pub fn parse_expr(&self, source: &str) -> ParseResult<Expr> {
        let token_kinds = synton_lexer::tokenize(source)?;
        let parser = expr_parser::expr_parser();
        parser.parse(&token_kinds)
            .into_result()
            .map_err(|err| ParseError::InvalidSyntax { message: format!("{:?}", err) })
    }

    /// Parse a type
    pub fn parse_type(&self, _source: &str) -> ParseResult<synton_ast::Type> {
        use synton_ast::{Type, TypeKind, BuiltinType};
        Ok(Type::new(TypeKind::Builtin(BuiltinType::I32), Span::new(
            Position::start(),
            Position::start(),
        )))
    }

    /// Parse a statement
    pub fn parse_stmt(&self, source: &str) -> ParseResult<Stmt> {
        let token_kinds = synton_lexer::tokenize(source)?;
        let parser = stmt_parser::stmt_parser();
        parser.parse(&token_kinds)
            .into_result()
            .map_err(|err| ParseError::InvalidSyntax { message: format!("{:?}", err) })
    }
}

impl Default for SyntonParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to parse a module
pub fn parse_module(source: &str) -> ParseResult<Module> {
    SyntonParser::new().parse_module(source)
}

/// Convenience function to parse an expression
pub fn parse_expr(source: &str) -> ParseResult<Expr> {
    SyntonParser::new().parse_expr(source)
}

/// Convenience function to parse a type
pub fn parse_type(source: &str) -> ParseResult<synton_ast::Type> {
    SyntonParser::new().parse_type(source)
}

/// Convenience function to parse a statement
pub fn parse_stmt(source: &str) -> ParseResult<Stmt> {
    SyntonParser::new().parse_stmt(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_module() {
        let result = parse_module("");
        // Empty module should succeed
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_expr() {
        let result = parse_expr("(+ 1 2)");
        // Should parse a binary addition expression
        assert!(result.is_ok());
        if let Ok(expr) = result {
            match &expr.kind {
                synton_ast::ExprKind::Binary { op, .. } => {
                    assert_eq!(op, &synton_ast::BinaryOp::Add);
                }
                _ => panic!("Expected Binary expression"),
            }
        }
    }

    #[test]
    fn test_literal_int() {
        let result = parse_expr("42");
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable() {
        let result = parse_expr("x");
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_binary() {
        let result = parse_expr("(+ (* 2 3) 4)");
        assert!(result.is_ok());
        if let Ok(expr) = result {
            // Should be: (+ (* 2 3) 4)
            match &expr.kind {
                synton_ast::ExprKind::Binary { op, left, right } => {
                    assert_eq!(op, &synton_ast::BinaryOp::Add);
                    // left should be (* 2 3)
                    match &left.kind {
                        synton_ast::ExprKind::Binary { op: mul_op, .. } => {
                            assert_eq!(mul_op, &synton_ast::BinaryOp::Mul);
                        }
                        _ => panic!("Expected Binary expression for left operand"),
                    }
                }
                _ => panic!("Expected Binary expression"),
            }
        }
    }

    #[test]
    fn test_boolean_literals() {
        let result1 = parse_expr("true");
        assert!(result1.is_ok());
        let result2 = parse_expr("false");
        assert!(result2.is_ok());
    }

    #[test]
    fn test_complex_expression() {
        let result = parse_expr("(+ (- 10 5) (* 2 3))");
        assert!(result.is_ok());
    }

    #[test]
    fn test_let_statement() {
        let result = parse_stmt("(let x = 42)");
        if let Err(e) = &result {
            eprintln!("Parse error: {:?}", e);
        }
        assert!(result.is_ok());
        if let Ok(stmt) = result {
            match &stmt.kind {
                synton_ast::StmtKind::Let { name, .. } => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Expected Let statement"),
            }
        }
    }

    #[test]
    fn test_let_without_init() {
        let result = parse_stmt("(let x)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_statement() {
        let result = parse_stmt("(branch true (+ 1 2) (+ 3 4))");
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_loop() {
        let result = parse_stmt("(while true (+ 1 2))");
        assert!(result.is_ok());
    }

    #[test]
    fn test_loop_statement() {
        let result = parse_stmt("(loop (+ 1 2))");
        assert!(result.is_ok());
    }

    #[test]
    fn test_break_statement() {
        let result1 = parse_stmt("(break)");
        assert!(result1.is_ok());
        let result2 = parse_stmt("(break 42)");
        assert!(result2.is_ok());
    }

    #[test]
    fn test_continue_statement() {
        let result = parse_stmt("(continue)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_expression_statement() {
        let result = parse_stmt("(+ 1 2)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_statements() {
        let result = parse_module("(let x = 42) (+ 1 2)");
        assert!(result.is_ok());
        if let Ok(module) = result {
            assert_eq!(module.stmts.len(), 2);
        }
    }

    #[test]
    fn test_complex_module() {
        let source = r#"
            (let x = 10)
            (let y = 20)
            (while true
                (+ x 1)
            )
        "#;
        let result = parse_module(source);
        assert!(result.is_ok());
        if let Ok(module) = result {
            assert_eq!(module.stmts.len(), 3);
        }
    }

    #[test]
    fn test_nested_control_flow() {
        let source = "(branch true (loop (+ 1 2)) (+ 3 4))";
        let result = parse_module(source);
        assert!(result.is_ok());
    }
}
