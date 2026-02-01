//! # Synton LSP Server
//!
//! Language Server Protocol implementation for Synton.
//!
//! Provides IDE features like syntax highlighting, autocomplete,
//! go-to-definition, hover info, and diagnostics.

#![warn(missing_docs, unused_crate_dependencies)]

use async_trait::async_trait;
use dashmap::DashMap;
use rustc_hash::FxHashMap;
use serde_json::Value;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub mod diagnostics;
pub mod completion;
pub mod hover;
pub mod symbols;

use diagnostics::Diagnostics;
use completion::Completion;
use hover::HoverImpl;
use symbols::Symbols;

/// LSP error
#[derive(Error, Debug)]
pub enum LspError {
    #[error("parse error: {0}")]
    Parse(String),

    #[error("type check error: {0}")]
    TypeCheck(String),
}

/// Main LSP server
#[derive(Debug, Clone)]
pub struct SyntonServer {
    client: Client,
    documents: Arc<RwLock<DashMap<String, DocumentState>>>,
}

#[derive(Debug, Clone)]
struct DocumentState {
    uri: String,
    version: i32,
    content: String,
    ast: Option<synton_ast::Module>,
    diagnostics: Vec<Diagnostic>,
}

impl SyntonServer {
    /// Create a new LSP server
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(DashMap::new())),
        }
    }

    /// Update document content and re-analyze
    async fn update_document(&self, uri: String, version: i32, content: String) {
        let mut state = DocumentState {
            uri: uri.clone(),
            version,
            content: content.clone(),
            ast: None,
            diagnostics: Vec::new(),
        };

        // Parse the document
        match synton_parser::parse_module(&content) {
            Ok(module) => {
                state.ast = Some(module);
            }
            Err(e) => {
                state.diagnostics.push(to_lsp_diagnostic(&e));
            }
        }

        // Type check if parsing succeeded
        if let Some(ref module) = state.ast {
            if let Err(e) = synton_typeck::check(module) {
                state.diagnostics.push(to_lsp_diagnostic(&e));
            }
        }

        self.documents.write().await.insert(uri.clone(), state);

        // Publish diagnostics
        let diagnostics = self.documents.read().await
            .get(&uri)
            .map(|s| s.diagnostics.clone())
            .unwrap_or_default();

        self.client.publish_diagnostics(
            Url::parse(&uri).unwrap(),
            diagnostics,
            None,
        ).await;
    }
}

#[async_trait]
impl LanguageServer for SyntonServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions::default(),
                            legend: SemanticTokensLegend {
                                token_types: semantic_token_types(),
                                token_modifiers: vec![],
                            },
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            ..Default::default()
                        },
                    ),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(
            MessageType::INFO,
            "Synton LSP server initialized",
        ).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.text_document.text;
        let version = params.text_document.version;

        self.update_document(uri, version, content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let version = params.text_document.version;

        let content = params.content_changes
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        self.update_document(uri, version, content).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.client.publish_diagnostics(
            params.text_document.uri,
            Vec::new(),
            None,
        ).await;
        self.documents.write().await.remove(&uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let pos = params.text_document_position.position;

        let docs = self.documents.read().await;
        let items = docs.get(&uri)
            .and_then(|state| Completion::complete(&state, pos))
            .unwrap_or_default();

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let pos = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let hover = docs.get(&uri)
            .and_then(|state| HoverImpl::hover(&state, pos));

        Ok(hover)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let pos = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        if let Some(_state) = docs.get(&uri) {
            // TODO: Implement go-to-definition
        }

        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.as_str();

        let docs = self.documents.read().await;
        let symbols = docs.get(uri)
            .map(|state| Symbols::document_symbols(&state))
            .unwrap_or_default();

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
}

fn semantic_token_types() -> Vec<SemanticTokenType> {
    vec![
        SemanticTokenType::COMMENT,
        SemanticTokenType::KEYWORD,
        SemanticTokenType::STRING,
        SemanticTokenType::NUMBER,
        SemanticTokenType::REGEXP,
        SemanticTokenType::OPERATOR,
        SemanticTokenType::FUNCTION,
        SemanticTokenType::VARIABLE,
        SemanticTokenType::TYPE,
        SemanticTokenType::PROPERTY,
        SemanticTokenType::NAMESPACE,
    ]
}

fn to_lsp_diagnostic(e: &dyn std::error::Error) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        },
        severity: Some(DiagnosticSeverity::ERROR),
        message: e.to_string(),
        ..Default::default()
    }
}
