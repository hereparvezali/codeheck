use axum::{
    Json,
    extract::{Path, State},
};
use sea_orm::EntityTrait;

use crate::{dto::MyErr, entity::submissions, utils::app_state::AppState};

pub async fn retrieve(
    State(stt): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<submissions::Model>, MyErr> {
    Ok(Json(
        submissions::Entity::find_by_id(id)
            .one(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
            .ok_or(MyErr::NotFound("submission_not_fount".to_string()))?,
    ))
}
