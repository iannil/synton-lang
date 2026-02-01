//! LSP document symbols

use tower_lsp::lsp_types::*;
use crate::DocumentState;

pub struct Symbols;

impl Symbols {
    pub fn document_symbols(state: &DocumentState) -> Vec<DocumentSymbol> {
        let mut symbols = Vec::new();

        if let Some(ref module) = state.ast {
            for stmt in &module.stmts {
                if let synton_ast::StmtKind::FnDecl(decl) = &stmt.kind {
                    symbols.push(DocumentSymbol {
                        name: decl.name.clone(),
                        kind: SymbolKind::FUNCTION,
                        range: Self::span_to_range(&stmt.span, &state.content),
                        selection_range: Self::span_to_range(&stmt.span, &state.content),
                        children: None,
                        detail: Some("function".to_string()),
                        tags: None,
                        deprecated: None,
                    });
                }
            }
        }

        symbols
    }

    fn span_to_range(span: &synton_ast::Span, _content: &str) -> Range {
        Range {
            start: Position {
                line: span.start.line.saturating_sub(1),
                character: span.start.column.saturating_sub(1),
            },
            end: Position {
                line: span.end.line.saturating_sub(1),
                character: span.end.column.saturating_sub(1),
            },
        }
    }
}
