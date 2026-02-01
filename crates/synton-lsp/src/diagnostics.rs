//! LSP diagnostics

use tower_lsp::lsp_types::*;
use crate::DocumentState;

pub struct Diagnostics;

impl Diagnostics {
    pub fn from_document(state: &DocumentState) -> Vec<Diagnostic> {
        state.diagnostics.clone()
    }
}
