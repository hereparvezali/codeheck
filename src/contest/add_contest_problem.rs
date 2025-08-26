use super::dto::AddContestProblemPayload;
use crate::{
    dto::MyErr,
    entity::{contest_problems, problems},
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{Extension, Json, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QuerySelect};

pub async fn add(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<AddContestProblemPayload>,
) -> Result<Json<contest_problems::Model>, MyErr> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let problem_details = problems::Entity::find_by_id(payload.problem_id)
        .select_only()
        .columns([problems::Column::AuthorId, problems::Column::IsPublic])
        .one(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
        .ok_or(MyErr::NotFound("problem_not_found".to_string()))?;

    if !problem_details.is_public
        && let Some(author_id) = problem_details.author_id
        && author_id != claim.id
    {
        return Err(MyErr::Unauthorized("unauthorized".to_string()));
    }
    Ok(Json(
        contest_problems::ActiveModel {
            contest_id: Set(payload.contest_id),
            problem_id: Set(payload.problem_id),
            label: Set(payload.label),
        }
        .insert(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
