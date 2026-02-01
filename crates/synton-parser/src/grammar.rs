//! Grammar module for Synton parser

use chumsky::prelude::*;
use synton_lexer::{Token, TokenKind};
use synton_ast::{Module, ModuleId, Stmt, StmtKind, Span, Position};

use super::stmt_parser::stmt_parser;

/// Module parser
pub fn module_parser<'a>() -> impl Parser<'a, &'a [TokenKind], Module> + Clone + 'a {
    // Parse a sequence of statements
    let stmts = stmt_parser()
        .repeated()
        .collect();

    stmts.then_ignore(end())
        .map(|stmts: Vec<Stmt>| Module {
            id: ModuleId::new("placeholder_hash".to_string()),
            stmts,
            imports: vec![],
            exports: vec![],
        })
        .boxed()
}
