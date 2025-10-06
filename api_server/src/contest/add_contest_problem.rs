use crate::{
    contest::dto::{AddContestProblemsPayload, ProblemId},
    entity::{contest_problems, problems},
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{Extension, Json, debug_handler, extract::State};
use sea_orm::{ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter, QuerySelect};
use std::sync::Arc;

#[debug_handler]
pub async fn add(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<AddContestProblemsPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let problem_ids = payload
        .problems
        .iter()
        .map(|id_label| id_label.problem_id)
        .collect::<Vec<i64>>();

    let fetched_ids = problems::Entity::find()
        .filter(
            Condition::all()
                .add(
                    problems::Column::IsPublic
                        .eq(true)
                        .or(problems::Column::AuthorId.eq(claim.id)),
                )
                .add(problems::Column::Id.is_in(problem_ids)),
        )
        .select_only()
        .column(problems::Column::Id)
        .into_model::<ProblemId>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
    let active_models = fetched_ids
        .iter()
        .enumerate()
        .map(|(i, problem_id)| contest_problems::ActiveModel {
            contest_id: Set(payload.contest_id),
            problem_id: Set(problem_id.id),
            label: Set(Some((b'A' + i as u8).to_string())),
        })
        .collect::<Vec<contest_problems::ActiveModel>>();

    let insert_result = contest_problems::Entity::insert_many(active_models)
        .exec(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "last_insert_id": insert_result.last_insert_id
    })))
}
