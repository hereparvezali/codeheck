use axum::{Extension, Json, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{
    dto::MyErr,
    entity::submissions,
    utils::{app_state::AppState, jwt::Claim},
};

use super::dto::CreateSubmissionPayload;

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Json(submitted): Json<CreateSubmissionPayload>,
) -> Result<Json<submissions::Model>, MyErr> {
    Ok(Json(
        submissions::ActiveModel {
            user_id: Set(claim.id),
            problem_id: Set(submitted.problem_id),
            language: Set(submitted.language),
            code: Set(submitted.code),
            contest_id: Set(submitted.contest_id),
            ..Default::default()
        }
        .insert(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
