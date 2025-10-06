use std::sync::Arc;

use axum::{Extension, Json, extract::State};
use sea_orm::EntityTrait;

use super::dto::RetrieveUserResponse;
use crate::{
    entity::users,
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
) -> Result<Json<RetrieveUserResponse>, AppError> {
    Ok(Json(
        users::Entity::find_by_id(claim.id)
            .one(stt.db.as_ref())
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or(AppError::not_found("user_not_found".to_string()))
            .map(|usr| {
                RetrieveUserResponse::new(usr.username, usr.email, usr.rating, usr.created_at)
            })?,
    ))
}
