use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde_json::json;

use crate::{
    contest::dto::{DeleteProblemQueryParams, NoValue},
    entity::{contest_problems, contests},
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};

pub async fn delete(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<DeleteProblemQueryParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    contests::Entity::find()
        .filter(
            contests::Column::Id
                .eq(query.contest_id)
                .and(contests::Column::AuthorId.eq(claim.id)),
        )
        .select_only()
        // .columns([contests::Column::Id, contests::Column::AuthorId])
        .into_model::<NoValue>()
        .one(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?
        .ok_or(AppError::auth("User is not authorized"))?;

    let res = contest_problems::Entity::delete(contest_problems::ActiveModel {
        contest_id: Set(query.contest_id),
        problem_id: Set(query.problem_id),
        ..Default::default()
    })
    .exec(stt.db.as_ref())
    .await
    .map_err(|e| AppError::internal(e.to_string()))?;

    Ok(Json(json!({
        "rows_affected": res.rows_affected
    })))
}
