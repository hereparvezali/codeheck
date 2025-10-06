use crate::{entity::submissions, error::AppError, utils::app_state::AppState};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::EntityTrait;

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(id): Query<i64>,
) -> Result<Json<submissions::Model>, AppError> {
    Ok(Json(
        submissions::Entity::find_by_id(id)
            .one(stt.db.as_ref())
            .await?
            .ok_or_else(|| AppError::not_found("Submission not found"))?,
    ))
}
