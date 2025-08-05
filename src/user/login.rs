use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    dto::MyErr,
    entity::users,
    utils::{app_state::AppState, hashing::verify_password, jwt::Claim},
};

use super::dto::{LoginUserPayload, LoginUserResponse};

pub async fn login(
    State(state): State<AppState>,
    Json(login_info): Json<LoginUserPayload>,
) -> Result<Json<LoginUserResponse>, MyErr> {
    let model = match login_info.email_or_username {
        super::dto::EmailOrUsername::Email(email) => {
            users::Entity::find()
                .filter(users::Column::Email.eq(email))
                .one(state.db.as_ref())
                .await
        }
        super::dto::EmailOrUsername::Username(username) => {
            users::Entity::find()
                .filter(users::Column::Username.eq(username))
                .one(state.db.as_ref())
                .await
        }
    }
    .map_err(|_| MyErr::InternalServerError)?
    .ok_or(MyErr::NotFound("Credential did not match???".to_string()))?;

    if !verify_password(&model.password, &login_info.password)
        .map_err(|_| MyErr::BadRequest("Verifying password Error".to_string()))?
    {
        return Err(MyErr::BadRequest("Password didn't matched".to_string()));
    }

    let access_token = encode(
        &Header::default(),
        &Claim::new(
            model.id,
            model.username.clone(),
            model.email.clone(),
            10000000,
        ),
        &EncodingKey::from_secret(state.secret.as_bytes()),
    )
    .expect("==> Encoding error in access token");

    let response = LoginUserResponse::new(model.id, model.username, model.email, access_token);
    Ok(Json(response))
}
