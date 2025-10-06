use super::dto::CreateProblemPayload;
use crate::{
    entity::problems,
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{Extension, Json, debug_handler, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::sync::Arc;

#[debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(problem_payload): Json<CreateProblemPayload>,
) -> Result<Json<problems::Model>, AppError> {
    let find_conflict = problems::Entity::find()
        .filter(problems::Column::Slug.eq(problem_payload.slug.clone()))
        .one(state.db.as_ref())
        .await
        .map_err(|_| {
            AppError::internal("Finding Conflict of Slug Problem in Database".to_string())
        })?;
    if find_conflict.is_some() {
        return Err(AppError::conflict("conflict_slug".to_string()));
    }
    Ok(Json(
        problems::ActiveModel {
            title: Set(problem_payload.title),
            slug: Set(problem_payload.slug),
            statement: Set(problem_payload.statement),
            input_spec: Set(problem_payload.input_spec),
            output_spec: Set(problem_payload.output_spec),
            sample_inputs: Set(Some(json!(problem_payload.sample_inputs))),
            sample_outputs: Set(Some(json!(problem_payload.sample_outputs))),
            time_limit: Set(problem_payload.time_limit),
            memory_limit: Set(problem_payload.memory_limit),
            difficulty: Set(problem_payload.difficulty),
            author_id: Set(Some(claim.id)),
            ..Default::default()
        }
        .insert(state.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?,
    ))
}
