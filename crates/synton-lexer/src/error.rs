//! Lexer error types

use std::ops::Range;

/// Result type for lexer operations
pub type LexResult<T> = Result<T, LexError>;

/// Lexer error
#[derive(Debug, thiserror::Error)]
pub enum LexError {
    /// Invalid token encountered
    #[error("invalid token: '{text}' at offset {offset}")]
    InvalidToken {
        text: String,
        offset: usize,
    },

    /// Unterminated string literal
    #[error("unterminated string literal at offset {start}")]
    UnterminatedString {
        start: usize,
    },

    /// Invalid escape sequence
    #[error("invalid escape sequence: '\\{seq}' at offset {offset}")]
    InvalidEscape {
        seq: String,
        offset: usize,
    },

    /// Invalid character literal
    #[error("invalid character literal: '{text}' at offset {offset}")]
    InvalidChar {
        text: String,
        offset: usize,
    },

    /// Unexpected end of file
    #[error("unexpected end of file")]
    UnexpectedEof,

    /// Generic error with message
    #[error("{0}")]
    Other(String),
}

impl LexError {
    pub fn span(&self) -> Range<usize> {
        match self {
            Self::InvalidToken { offset, .. } => *offset..(*offset + 1),
            Self::UnterminatedString { start, .. } => *start..(*start + 1),
            Self::InvalidEscape { offset, .. } => *offset..(*offset + 2),
            Self::InvalidChar { offset, .. } => *offset..(*offset + 1),
            Self::UnexpectedEof => 0..0,
            Self::Other(_) => 0..0,
        }
    }
}
