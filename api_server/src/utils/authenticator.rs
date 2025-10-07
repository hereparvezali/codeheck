use crate::{
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{
    extract::{Request, State},
    middleware::Next,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::sync::Arc;

pub async fn authorizer(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<axum::http::Response<axum::body::Body>, AppError> {
    let claims = decode::<Claim>(
        token.token(),
        &DecodingKey::from_secret(state.config.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        tracing::warn!("JWT validation failed: {}", e);
        AppError::auth("Invalid or expired token")
    })?
    .claims;

    // tracing::debug!("User authenticated: {} ({})", claims.username, claims.id);

    req.extensions_mut().insert(Arc::new(claims));
    Ok(next.run(req).await)
}
