use super::dto::CreateUserPayload;
use crate::{
    dto::MyErr,
    entity::users,
    utils::{app_state::AppState, hashing::hash_password},
};
use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

pub async fn signup(
    State(state): State<AppState>,
    Json(mut usr): Json<CreateUserPayload>,
) -> Result<Json<users::Model>, MyErr> {
    usr.password = hash_password(&usr.password);
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
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    if let Some(model) = find_duplicate {
        if model.email == usr.email {
            return Err(MyErr::Conflict("conflict_email".to_string()));
        }
        if model.username == usr.username {
            return Err(MyErr::Conflict("conflict_username".to_string()));
        }
    }
    if usr.username.is_empty() || usr.email.is_empty() || usr.password.is_empty() {
        return Err(MyErr::BadRequest(
            "Username, email, and password cannot be empty".to_string(),
        ));
    }
    if usr.username.len() < 3 || usr.password.len() < 6 {
        return Err(MyErr::BadRequest(
            "Username must be at least 3 characters and password at least 6 characters".to_string(),
        ));
    }
    Ok(Json(
        usr_active_model
            .insert(state.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
