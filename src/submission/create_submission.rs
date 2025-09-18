use super::dto::CreateSubmissionPayload;
use crate::{
    dto::MyErr,
    entity::{problems, submissions, testcases},
    submission::dto::{InputOutput, SubmissionPublishQueue, TimeAndMemoryLimit},
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{Extension, Json, extract::State};
use lapin::{BasicProperties, options::BasicPublishOptions};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QuerySelect,
};
use std::sync::Arc;

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(submitted): Json<CreateSubmissionPayload>,
) -> Result<Json<submissions::Model>, MyErr> {
    let insert_res = submissions::ActiveModel {
        user_id: Set(claim.id),
        problem_id: Set(submitted.problem_id),
        language: Set(submitted.language.clone()),
        code: Set(submitted.code.clone()),
        contest_id: Set(submitted.contest_id),
        ..Default::default()
    }
    .insert(stt.db.as_ref())
    .await
    .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    let time_memory_limit = problems::Entity::find_by_id(submitted.problem_id)
        .select_only()
        .columns([problems::Column::TimeLimit, problems::Column::MemoryLimit])
        .into_model::<TimeAndMemoryLimit>()
        .one(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
        .ok_or(MyErr::NotFound("problem_not_found".to_string()))?;

    let cases = testcases::Entity::find()
        .filter(testcases::Column::ProblemId.eq(submitted.problem_id))
        .select_only()
        .columns([testcases::Column::Input, testcases::Column::Output])
        .into_model::<InputOutput>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    let payload = serde_json::to_vec(&SubmissionPublishQueue {
        submission_id: insert_res.id,
        problem_id: submitted.problem_id,
        language: submitted.language.clone(),
        code: submitted.code,
        time_limit: time_memory_limit.time_limit,
        memory_limit: time_memory_limit.memory_limit,
        inputs_outputs: cases,
    })
    .unwrap();

    stt.mq
        .basic_publish(
            "",
            "submissions",
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();

    Ok(Json::from(insert_res))
}
