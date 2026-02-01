//! Source location tracking

use serde::{Deserialize, Serialize};
use std::ops::Range;

/// A location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

impl Position {
    pub const fn new(line: u32, column: u32, offset: u32) -> Self {
        Self { line, column, offset }
    }

    pub const fn start() -> Self {
        Self { line: 1, column: 1, offset: 0 }
    }
}

/// A span in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub const fn single(pos: Position) -> Self {
        Self { start: pos, end: pos }
    }

    pub fn range(&self) -> Range<usize> {
        self.start.offset as usize..self.end.offset as usize
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: if self.start < other.start { self.start } else { other.start },
            end: if self.end > other.end { self.end } else { other.end },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start.offset == self.end.offset
    }
}

impl From<(Position, Position)> for Span {
    fn from((start, end): (Position, Position)) -> Self {
        Self::new(start, end)
    }
}

impl From<(u32, u32)> for Span {
    fn from((start, end): (u32, u32)) -> Self {
        Self::new(
            Position::new(0, 0, start),
            Position::new(0, 0, end),
        )
    }
}
