use super::dto::CreateUserPayload;
use crate::{
    entity::users,
    error::AppError,
    utils::{app_state::AppState, security::hash_password},
};
use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

pub async fn signup(
    State(state): State<AppState>,
    Json(mut usr): Json<CreateUserPayload>,
) -> Result<Json<users::Model>, AppError> {
    usr.password = hash_password(&usr.password)?;
    let usr_active_model = users::ActiveModel {
        email: Set(usr.email.clone()),
        username: Set(usr.username.clone()),
        password: Set(usr.password.clone()),
        ..Default::default()
    };

    let find_duplicate = users::Entity::find()
        .filter(
            users::Column::Email
                .eq(usr.email.clone())
                .or(users::Column::Username.eq(usr.username.clone())),
        )
        .one(state.db.as_ref())
        .await?;

    if let Some(model) = find_duplicate {
        if model.email == usr.email {
            return Err(AppError::conflict("Email already registered"));
        }
        if model.username == usr.username {
            return Err(AppError::conflict("Username already taken"));
        }
    }
    if usr.username.is_empty() || usr.email.is_empty() || usr.password.is_empty() {
        return Err(AppError::bad_request(
            "Username, email, and password cannot be empty",
        ));
    }
    if usr.username.len() < 3 || usr.password.len() < 6 {
        return Err(AppError::bad_request(
            "Username must be at least 3 characters and password at least 6 characters",
        ));
    }
    Ok(Json(usr_active_model.insert(state.db.as_ref()).await?))
}
