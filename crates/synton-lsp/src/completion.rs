//! LSP completion

use tower_lsp::lsp_types::*;
use crate::DocumentState;

pub struct Completion;

impl Completion {
    pub fn complete(state: &DocumentState, pos: Position) -> Option<Vec<CompletionItem>> {
        let mut items = Vec::new();

        // Keywords
        for kw in Self::keywords() {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }

        // Type keywords
        for ty in Self::types() {
            items.push(CompletionItem {
                label: ty.to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                ..Default::default()
            });
        }

        // Standard library functions
        for fn_name in Self::stdlib() {
            items.push(CompletionItem {
                label: fn_name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                ..Default::default()
            });
        }

        Some(items)
    }

    fn keywords() -> [&'static str; 20] {
        [
            "fn", "let", "set", "if", "else", "branch", "match",
            "loop", "while", "for", "in", "break", "continue", "return",
            "struct", "enum", "type", "const", "import", "export",
        ]
    }

    fn types() -> [&'static str; 13] {
        [
            "i32", "i64", "u32", "u64", "f32", "f64", "bool",
            "string", "str", "char", "list", "map", "maybe",
        ]
    }

    fn stdlib() -> [&'static str; 10] {
        [
            "print", "len", "abs", "min", "max", "clamp",
            "push", "pop", "first", "last",
        ]
    }
}
