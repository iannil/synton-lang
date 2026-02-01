//! LSP hover information

use tower_lsp::lsp_types::*;
use crate::DocumentState;

pub struct HoverImpl;

impl HoverImpl {
    pub fn hover(state: &DocumentState, pos: Position) -> Option<Hover> {
        let _line = state.content.lines().nth(pos.line as usize)?;
        // TODO: Implement actual hover logic
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Hover info".to_string(),
            }),
            range: None,
        })
    }
}
