//! Statement parser for Synton

use chumsky::prelude::*;
use synton_lexer::{Token, TokenKind};
use synton_ast::{Stmt, StmtKind, Span, Position, Expr};

/// Statement parser
pub fn stmt_parser<'a>() -> impl Parser<'a, &'a [TokenKind], Stmt> + Clone + 'a {
    // Import the expression parser
    use super::expr_parser::expr_parser;

    recursive(|stmt| {
        // Empty statement (semicolon)
        let empty = select!(TokenKind { token: Token::Semi, span: _ })
            .map(|_| Stmt::new(StmtKind::Empty, Span::new(Position::start(), Position::start())));

        // Let binding: (let x [= expr])
        // Simplified syntax for now
        let let_stmt = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(
                select!(TokenKind { token: Token::KwLet, span: _ })
                .ignore_then(
                    // Variable name
                    select!(TokenKind { token: Token::Identifier(s), .. } => s.clone())
                    // Optional initialization
                    .then(
                        select!(TokenKind { token: Token::Eq, span: _ })
                        .ignore_then(
                            expr_parser()
                        )
                        .or_not()
                    )
                )
                .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            )
            .map(|(name, init): (String, Option<Expr>)| {
                Stmt::new(
                    StmtKind::Let {
                        name,
                        id: None,
                        ty: None,
                        init: init.map(Box::new),
                        mutable: false,
                    },
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // If/branch statement: (branch cond then_branch [else_branch])
        let if_stmt = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(
                select!(TokenKind { token: Token::KwBranch, span: _ })
                .or(select!(TokenKind { token: Token::KwIf, span: _ }))
                .ignore_then(
                    expr_parser()
                    .then(stmt.clone())
                    .then(
                        stmt.clone().or_not()
                    )
                )
                .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            )
            .map(|((cond, then_branch), else_branch): ((Expr, Stmt), Option<Stmt>)| {
                Stmt::new(
                    StmtKind::If {
                        cond: Box::new(cond),
                        then_branch: Box::new(then_branch),
                        else_branch: else_branch.map(Box::new),
                    },
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // While loop: (while cond body)
        let while_stmt = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(
                select!(TokenKind { token: Token::KwWhile, span: _ })
                .ignore_then(
                    expr_parser()
                    .then(stmt.clone())
                )
                .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            )
            .map(|(cond, body): (Expr, Stmt)| {
                Stmt::new(
                    StmtKind::While {
                        cond: Box::new(cond),
                        body: Box::new(body),
                    },
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // Loop statement: (loop body)
        let loop_stmt = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(
                select!(TokenKind { token: Token::KwLoop, span: _ })
                .ignore_then(stmt.clone())
                .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            )
            .map(|body: Stmt| {
                Stmt::new(
                    StmtKind::Loop {
                        body: Box::new(body),
                    },
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // Break statement: (break) or (break expr)
        let break_stmt = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(
                select!(TokenKind { token: Token::KwBreak, span: _ })
                .ignore_then(
                    expr_parser().or_not()
                )
                .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            )
            .map(|value: Option<Expr>| {
                Stmt::new(
                    StmtKind::Break(value.map(Box::new)),
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // Continue statement: (continue)
        let continue_stmt = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(
                select!(TokenKind { token: Token::KwContinue, span: _ })
                .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            )
            .map(|_| {
                Stmt::new(
                    StmtKind::Continue,
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // Expression statement: just an expression
        let expr_stmt = expr_parser()
            .map(|expr: Expr| {
                Stmt::new(
                    StmtKind::Expr(Box::new(expr)),
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // Combine all statement types
        let_stmt
            .or(if_stmt)
            .or(while_stmt)
            .or(loop_stmt)
            .or(break_stmt)
            .or(continue_stmt)
            .or(expr_stmt)
            .or(empty)
            .boxed()
    })
}
