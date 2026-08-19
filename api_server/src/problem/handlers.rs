use std::sync::Arc;
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use serde_json::json;

use super::{
    dto::{
        CreateProblemPayload, CreateTestcasePayload, DeleteProblemQuery, QueryByProblemId,
        RetrieveProblemQuery, RetrieveProblemsQueryWithCursor, RetrieveProblemsWithCursorResponse,
        UpdateProblemPayload,
    },
    service::ProblemService,
};
use crate::{
    entity::{problems, testcases},
    error::AppError,
    utils::{
        app_state::AppState,
        security::Claim,
    },
};

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<CreateProblemPayload>,
) -> Result<Json<problems::Model>, AppError> {
    let problem = ProblemService::create_problem(&stt.db, claim.id, payload).await?;
    Ok(Json(problem))
}

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveProblemQuery>,
) -> Result<Json<problems::Model>, AppError> {
    let problem = ProblemService::get_problem(&stt.db, Some(claim.id), query).await?;
    Ok(Json(problem))
}

pub async fn retrieve_many(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveProblemsQueryWithCursor>,
) -> Result<Json<RetrieveProblemsWithCursorResponse>, AppError> {
    let result = ProblemService::list_problems(&stt.db, Some(claim.id), query).await?;
    Ok(Json(result))
}

pub async fn update(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<UpdateProblemPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = ProblemService::update_problem(&stt.db, claim.id, payload).await?;
    Ok(Json(json!({ "status": "updated", "rows_affected": rows })))
}

pub async fn delete(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<DeleteProblemQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = ProblemService::delete_problem(&stt.db, claim.id, query.id).await?;
    Ok(Json(json!({ "status": "deleted", "rows_affected": rows })))
}

pub async fn create_testcases(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<CreateTestcasePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let count = ProblemService::create_testcases(&stt.db, claim.id, payload).await?;
    Ok(Json(json!({ "status": 201, "msg": "inserted", "count": count })))
}

pub async fn retrieve_testcases(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<QueryByProblemId>,
) -> Result<Json<Vec<testcases::Model>>, AppError> {
    let cases = ProblemService::get_testcases(&stt.db, claim.id, query.problem_id).await?;
    Ok(Json(cases))
}

pub async fn delete_testcases(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<QueryByProblemId>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = ProblemService::delete_testcases(&stt.db, claim.id, query.problem_id).await?;
    Ok(Json(json!({ "status": "deleted", "rows_affected": rows })))
}
