//! Expression parser for Synton - Polish notation

use chumsky::prelude::*;
use synton_lexer::{Token, TokenKind};
use synton_ast::{Expr, ExprKind, Literal, BinaryOp, CompareOp, Span, Position};

/// Expression parser
pub fn expr_parser<'a>() -> impl Parser<'a, &'a [TokenKind], Expr> + Clone + 'a {
    recursive(|expr| {
        // Integer literal
        let int_lit = select!(TokenKind { token: Token::Integer(n), .. } => n)
            .map(|n| Expr::new(ExprKind::Literal(Literal::Integer(n)), Span::new(Position::start(), Position::start())));

        // Boolean literals
        let bool_lit = select!(
            TokenKind { token: Token::True, .. } => true,
            TokenKind { token: Token::False, .. } => false
        )
        .map(|b| Expr::new(ExprKind::Literal(Literal::Bool(b)), Span::new(Position::start(), Position::start())));

        // Variable reference (excluding special call: syntax)
        let var = select!(TokenKind { token: Token::Identifier(s), .. } => s.clone())
            .map(|name| Expr::new(ExprKind::Var { id: None, name }, Span::new(Position::start(), Position::start())));

        // Atomic expressions (literals and variables)
        let atom = int_lit.or(bool_lit).or(var).boxed();

        // Binary operation in Polish notation: (+ 1 2)
        // Parse each operator explicitly with select!
        let op_plus = select!(TokenKind { token: Token::Plus, span: _ } => BinaryOp::Add);
        let op_minus = select!(TokenKind { token: Token::Minus, span: _ } => BinaryOp::Sub);
        let op_star = select!(TokenKind { token: Token::Star, span: _ } => BinaryOp::Mul);
        let op_slash = select!(TokenKind { token: Token::Slash, span: _ } => BinaryOp::Div);
        let op_percent = select!(TokenKind { token: Token::Percent, span: _ } => BinaryOp::Mod);
        let op_caret = select!(TokenKind { token: Token::Caret, span: _ } => BinaryOp::BitXor);
        let op_amp = select!(TokenKind { token: Token::Amp, span: _ } => BinaryOp::BitAnd);
        let op_pipe = select!(TokenKind { token: Token::Pipe, span: _ } => BinaryOp::BitOr);
        let op_shl = select!(TokenKind { token: Token::Shl, span: _ } => BinaryOp::Shl);
        let op_shr = select!(TokenKind { token: Token::Shr, span: _ } => BinaryOp::Shr);
        let op_and = select!(TokenKind { token: Token::AndAnd, span: _ } => BinaryOp::And);
        let op_or = select!(TokenKind { token: Token::OrOr, span: _ } => BinaryOp::Or);
        let op_pow = select!(TokenKind { token: Token::StarStar, span: _ } => BinaryOp::Pow);

        let binary_op = op_plus
            .or(op_minus)
            .or(op_star)
            .or(op_slash)
            .or(op_percent)
            .or(op_caret)
            .or(op_amp)
            .or(op_pipe)
            .or(op_shl)
            .or(op_shr)
            .or(op_and)
            .or(op_or)
            .or(op_pow)
            .boxed();

        let binary = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(
                // Parse operator
                binary_op
                // Parse left operand
                .then(expr.clone())
                // Parse right operand
                .then(expr.clone())
                // Parse closing paren
                .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            )
            .map(|((op, left), right)| {
                Expr::new(
                    ExprKind::Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    Span::new(Position::start(), Position::start()),
                )
            })
            .boxed();

        // Parenthesized expression for grouping
        let parenthesized = select!(TokenKind { token: Token::LParen, span: _ })
            .ignore_then(expr.clone())
            .then_ignore(select!(TokenKind { token: Token::RParen, span: _ }))
            .boxed();

        // Combine all expression types
        binary
            .or(parenthesized)
            .or(atom)
            .boxed()
    })
}
