use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tower_cookies::{Cookie, Cookies};

use crate::{
    dto::MyErr,
    entity::users,
    utils::{app_state::AppState, hashing::verify_password, jwt::Claim},
};

use super::dto::{SigninUserPayload, SigninUserResponse};

pub async fn signin(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(signin_info): Json<SigninUserPayload>,
) -> Result<Json<SigninUserResponse>, MyErr> {
    let model = users::Entity::find()
        .filter(
            users::Column::Username
                .eq(signin_info.username_or_email.clone())
                .or(users::Column::Email.eq(signin_info.username_or_email)),
        )
        .one(state.db.as_ref())
        .await
        .map_err(|_| MyErr::InternalServerError)?
        .ok_or(MyErr::NotFound("not_found".to_string()))?;

    if !verify_password(&model.password, &signin_info.password)
        .map_err(|_| MyErr::BadRequest("verifying_password_error".to_string()))?
    {
        return Err(MyErr::BadRequest("wrong_password".to_string()));
    }

    let access_token = encode(
        &Header::default(),
        &Claim::new(model.id, model.username.clone(), model.email.clone(), 10),
        &EncodingKey::from_secret(state.secret.as_bytes()),
    )
    .expect("==> Encoding error in access token");

    let refresh_token = encode(
        &Header::default(),
        &Claim::new(model.id, model.username.clone(), model.email.clone(), 10000),
        &EncodingKey::from_secret(state.secret.as_bytes()),
    )
    .expect("==> Encoding error in refresh token");

    let cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .build();
    cookies.add(cookie);

    let response = SigninUserResponse::new(model.id, model.username, model.email, access_token);
    Ok(Json(response))
}
