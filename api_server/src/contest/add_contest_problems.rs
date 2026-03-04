use crate::{
    contest::dto::{AddContestProblemsPayload, ProblemId}, entity::{contest_problems, problems}, error::AppError, utils::{app_state::AppState, security::Claim}
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
    let problem_labels = payload
        .problems
        .iter()
        .map(|id_label| id_label.label.clone())
        .collect::<Vec<Option<String>>>();

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
        .zip(problem_labels)
        .map(|(problem_id, label)| contest_problems::ActiveModel {
            contest_id: Set(payload.id),
            problem_id: Set(problem_id.id),
            label: Set(label),
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
