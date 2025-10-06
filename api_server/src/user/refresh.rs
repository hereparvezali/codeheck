use super::dto::SigninUserResponse;
use crate::{
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{Json, extract::State};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use tower_cookies::Cookies;

pub async fn refresh(
    State(stt): State<AppState>,
    cookies: Cookies,
) -> Result<Json<SigninUserResponse>, AppError> {
    let refresh_token = cookies
        .get("refresh_token")
        .ok_or(AppError::auth("refresh_token_not_found".to_string()))?
        .value()
        .to_string();

    let claim: Claim = decode(
        &refresh_token,
        &DecodingKey::from_secret(stt.config.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::auth("refresh_token_expired".to_string()))?
    .claims;

    let access_token = encode(
        &Header::default(),
        &Claim::new(
            claim.id,
            claim.username.clone(),
            claim.email.clone(),
            stt.config.jwt.access_token_expiry_minutes,
        ),
        &EncodingKey::from_secret(stt.config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::validation(e.to_string()))?;

    Ok(Json(SigninUserResponse::new(
        claim.id,
        claim.username,
        claim.email,
        access_token,
    )))
}
