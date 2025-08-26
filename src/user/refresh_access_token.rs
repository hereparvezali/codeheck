use super::dto::SigninUserResponse;
use crate::{
    dto::MyErr,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{Json, extract::State};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use tower_cookies::Cookies;

pub async fn refresh(
    State(stt): State<AppState>,
    cookies: Cookies,
) -> Result<Json<SigninUserResponse>, MyErr> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let refresh_token = cookies
        .get("refresh_token")
        .ok_or(MyErr::NotFound("refresh_token_not_found".to_string()))?
        .value()
        .to_string();

    let claim: Claim = decode(
        &refresh_token,
        &DecodingKey::from_secret(stt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| MyErr::Unauthorized("refresh_token_expired".to_string()))?
    .claims;

    let access_token = encode(
        &Header::default(),
        &Claim::new(claim.id, claim.username.clone(), claim.email.clone(), 10),
        &EncodingKey::from_secret(stt.secret.as_bytes()),
    )
    .expect("==> Encoding error in access token");

    Ok(Json(SigninUserResponse::new(
        claim.id,
        claim.username,
        claim.email,
        access_token,
    )))
}
