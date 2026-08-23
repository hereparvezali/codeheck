use std::sync::Arc;
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use tower_cookies::{Cookie, Cookies};

use super::{
    dto::{
        CreateUserPayload, MessageResponse, ResendVerificationPayload, RetrieveUserResponse,
        RetrieveUserStatsQuery, RetrieveUserinfoQuery, SigninUserPayload, SigninUserResponse,
        SignupResponse, UserStatsResponse, VerifyEmailPayload, VerifyEmailQuery,
    },
    service::UserService,
};
use crate::{
    error::AppError,
    utils::{
        app_state::AppState,
        security::{self, Claim},
    },
};

pub async fn signup(
    State(stt): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<SignupResponse>, AppError> {
    let res = UserService::signup(&stt.db, &stt.config, payload).await?;
    Ok(Json(res))
}

pub async fn verify_email_get(
    State(stt): State<AppState>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Json<MessageResponse>, AppError> {
    let res = UserService::verify_email(&stt.db, &query.token).await?;
    Ok(Json(res))
}

pub async fn verify_email_post(
    State(stt): State<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<Json<MessageResponse>, AppError> {
    let res = UserService::verify_email(&stt.db, &payload.token).await?;
    Ok(Json(res))
}

pub async fn resend_verification(
    State(stt): State<AppState>,
    Json(payload): Json<ResendVerificationPayload>,
) -> Result<Json<MessageResponse>, AppError> {
    let res = UserService::resend_verification(&stt.db, &stt.config, payload).await?;
    Ok(Json(res))
}

pub async fn signin(
    State(stt): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<SigninUserPayload>,
) -> Result<Json<SigninUserResponse>, AppError> {
    let (res, access_token, refresh_token) = UserService::signin(&stt.db, &stt.config, payload).await?;

    cookies.add(
        Cookie::build(("access_token", access_token))
            .path("/")
            .http_only(true)
            .build(),
    );
    cookies.add(
        Cookie::build(("refresh_token", refresh_token))
            .path("/")
            .http_only(true)
            .build(),
    );

    Ok(Json(res))
}

pub async fn signout(cookies: Cookies) -> Result<Json<serde_json::Value>, AppError> {
    cookies.remove(Cookie::build(("access_token", "")).path("/").build());
    cookies.remove(Cookie::build(("refresh_token", "")).path("/").build());
    Ok(Json(serde_json::json!({ "status": "signed_out" })))
}

pub async fn refresh(
    State(stt): State<AppState>,
    cookies: Cookies,
) -> Result<Json<serde_json::Value>, AppError> {
    let refresh_cookie = cookies
        .get("refresh_token")
        .ok_or_else(|| AppError::auth("No refresh token provided"))?;

    let claims = security::validate_token(refresh_cookie.value(), &stt.config)
        .map_err(|_| AppError::auth("Invalid refresh token"))?;

    let user = UserService::get_user(&stt.db, claims.id).await?;
    let access_token = security::generate_access_token_from_claim(&claims, &stt.config)
        .map_err(|e| AppError::internal(format!("Token generation failed: {}", e)))?;

    cookies.add(
        Cookie::build(("access_token", access_token.clone()))
            .path("/")
            .http_only(true)
            .build(),
    );

    Ok(Json(serde_json::json!({
        "id": user.id,
        "access_token": access_token,
        "username": user.username,
        "email": user.email
    })))
}

pub async fn retrieve_me(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
) -> Result<Json<RetrieveUserResponse>, AppError> {
    let user = UserService::get_user(&stt.db, claim.id).await?;
    Ok(Json(user))
}

pub async fn retrieve_user_info(
    State(stt): State<AppState>,
    Extension(_claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveUserinfoQuery>,
) -> Result<Json<RetrieveUserResponse>, AppError> {
    let user = UserService::get_user_info(&stt.db, query).await?;
    Ok(Json(user))
}

pub async fn retrieve_stats(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveUserStatsQuery>,
) -> Result<Json<UserStatsResponse>, AppError> {
    let stats = UserService::get_stats(&stt.db, Some(claim), query).await?;
    Ok(Json(stats))
}
