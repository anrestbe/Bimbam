use crate::core::{session::Session, token::Token};
use lspower::lsp::{GotoDefinitionParams, GotoDefinitionResponse, Location, Url};
use std::sync::Arc;

pub fn go_to_definition(
    session: Arc<Session>,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let url = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let key = url.path();

    if let Some(document) = session.documents.get(key) {
        if let Some(token) = document.get_token_at_position(position) {
            if token.is_initial_declaration() {
                return Some(format_response(url, token));
            } else {
                return handle_go_to_definition(&session, &token);
            }
        }
    }

    None
}

fn handle_go_to_definition(session: &Session, token: &Token) -> Option<GotoDefinitionResponse> {
    if let Some(file) = &token.file {
        if let Some(document_ref) = session.documents.get(file) {
            if let Some(declared_token) = document_ref.get_declared_token(&token.name) {
                return match Url::from_file_path(document_ref.key()) {
                    Ok(url) => Some(format_response(url, declared_token)),
                    Err(_) => None,
                };
            }
        }
    }

    None
}

fn format_response(url: Url, token: &Token) -> GotoDefinitionResponse {
    GotoDefinitionResponse::Scalar(Location::new(url, token.range))
}
