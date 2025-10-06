use super::dto::{SigninUserPayload, SigninUserResponse};
use crate::{
    entity::users,
    error::{AppError, AppResult},
    utils::{
        app_state::AppState,
        security::{Claim, verify_password},
    },
};
use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tower_cookies::{Cookie, Cookies};

pub async fn signin(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(signin_info): Json<SigninUserPayload>,
) -> AppResult<Json<SigninUserResponse>> {
    let model = users::Entity::find()
        .filter(
            users::Column::Username
                .eq(signin_info.username_or_email.clone())
                .or(users::Column::Email.eq(signin_info.username_or_email)),
        )
        .one(state.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    if !verify_password(&model.password, &signin_info.password)? {
        return Err(AppError::auth("Invalid password"));
    }

    let access_token = encode(
        &Header::default(),
        &Claim::new(
            model.id,
            model.username.clone(),
            model.email.clone(),
            state.config.jwt.access_token_expiry_minutes,
        ),
        &EncodingKey::from_secret(state.config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::internal(format!("Failed to encode access token: {}", e)))?;

    let refresh_token = encode(
        &Header::default(),
        &Claim::new(
            model.id,
            model.username.clone(),
            model.email.clone(),
            state.config.jwt.refresh_token_expiry_minutes,
        ),
        &EncodingKey::from_secret(state.config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::internal(format!("Failed to encode refresh token: {}", e)))?;

    let cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .build();
    cookies.add(cookie);

    let response = SigninUserResponse::new(model.id, model.username, model.email, access_token);
    Ok(Json(response))
}
