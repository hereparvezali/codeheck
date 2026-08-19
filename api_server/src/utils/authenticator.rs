use crate::{
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::sync::Arc;

pub async fn authorizer(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token(&req)
        .ok_or_else(|| AppError::auth("Missing authorization token. Please sign in."))?;

    let claims = decode::<Claim>(
        &token,
        &DecodingKey::from_secret(state.config.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::auth("Invalid or expired authorization token. Please sign in."))?
    .claims;

    req.extensions_mut().insert(Arc::new(claims));
    Ok(next.run(req).await)
}

fn extract_token(req: &Request) -> Option<String> {
    if let Some(auth_header) = req.headers().get(axum::http::header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    if let Some(cookie_header) = req.headers().get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for part in cookie_str.split(';') {
                let part = part.trim();
                if let Some(token) = part.strip_prefix("access_token=") {
                    return Some(token.to_string());
                }
            }
        }
    }

    if let Some(protocol_header) = req.headers().get("Sec-WebSocket-Protocol") {
        if let Ok(protocol_str) = protocol_header.to_str() {
            let first_proto = protocol_str.split(',').next().unwrap_or("").trim();
            if !first_proto.is_empty() {
                return Some(first_proto.to_string());
            }
        }
    }

    if let Some(query) = req.uri().query() {
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
                if key == "token" || key == "access_token" {
                    return Some(val.to_string());
                }
            }
        }
    }

    None
}

