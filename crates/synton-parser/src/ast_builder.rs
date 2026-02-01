//! AST builder utilities for constructing parsed AST nodes

use synton_ast::{Span, Position, NodeId};

/// Builder for creating AST nodes with proper spans
pub struct AstBuilder {
    source: String,
    line_offsets: Vec<usize>,
    node_counter: u32,
}

impl AstBuilder {
    pub fn new(source: String) -> Self {
        let line_offsets = Self::compute_line_offsets(&source);
        Self {
            source,
            line_offsets,
            node_counter: 0,
        }
    }

    fn compute_line_offsets(source: &str) -> Vec<usize> {
        let mut offsets = vec![0];
        let mut offset = 0;
        for ch in source.chars() {
            offset += ch.len_utf8();
            if ch == '\n' {
                offsets.push(offset);
            }
        }
        offsets
    }

    /// Convert byte offset to line/column position
    pub fn position(&self, offset: usize) -> Position {
        let line = self.line_offsets
            .partition_point(|&o| o <= offset)
            .saturating_sub(1);

        let line_start = self.line_offsets.get(line).copied().unwrap_or(0);
        let column = offset - line_start;

        Position::new(line as u32 + 1, column as u32 + 1, offset as u32)
    }

    /// Create a span from byte range
    pub fn span(&self, start: usize, end: usize) -> Span {
        Span::new(self.position(start), self.position(end))
    }

    /// Create a single-point span
    pub fn span_at(&self, offset: usize) -> Span {
        let pos = self.position(offset);
        Span::single(pos)
    }

    /// Generate a new unique node ID
    pub fn fresh_node_id(&mut self) -> NodeId {
        let id = NodeId(self.node_counter);
        self.node_counter += 1;
        id
    }
}
