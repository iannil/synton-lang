//! Parser error types

use thiserror::Error;

/// Result type for parser operations
pub type ParseResult<T> = Result<T, ParseError>;

/// Parser error
#[derive(Error, Debug)]
pub enum ParseError {
    /// Error from lexer
    #[error(transparent)]
    LexError(#[from] synton_lexer::LexError),

    /// Unexpected token
    #[error("unexpected token: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
    },

    /// Unexpected end of input
    #[error("unexpected end of input, expected {expected}")]
    UnexpectedEof {
        expected: String,
    },

    /// Invalid syntax
    #[error("invalid syntax: {message}")]
    InvalidSyntax {
        message: String,
    },

    /// Type error during parsing
    #[error("type error: {message}")]
    TypeError {
        message: String,
    },

    /// Recursion limit exceeded
    #[error("recursion limit exceeded (max depth: {max_depth})")]
    RecursionLimit {
        max_depth: usize,
    },
}
