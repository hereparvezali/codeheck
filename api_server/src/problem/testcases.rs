use crate::{
    entity::{problems, testcases},
    error::AppError,
    problem::dto::ProblemIdAuthorId,
    utils::{app_state::AppState, helpers::SetFromValue, security::Claim},
};
use axum::{
    Extension, Json, debug_handler,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, QueryFilter, QuerySelect, QueryTrait};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub input: Option<String>,
    pub output: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTestcasePayload {
    pub problem_id: i64,
    pub cases: Vec<Case>,
}
#[debug_handler]
pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<CreateTestcasePayload>,
) -> Result<Json<Value>, AppError> {
    if problems::Entity::find_by_id(payload.problem_id)
        .select_only()
        .columns([problems::Column::Id, problems::Column::AuthorId])
        .into_model::<ProblemIdAuthorId>()
        .one(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?
        .ok_or(AppError::not_found("problem_not_found".to_string()))?
        .author_id
        .ok_or(AppError::auth("not_allowed".to_string()))?
        != claim.id
    {
        return Err(AppError::auth("not_allowed".to_string()));
    }

    let models: Vec<testcases::ActiveModel> = payload
        .cases
        .iter()
        .map(|v| testcases::ActiveModel {
            problem_id: payload.problem_id.set(),
            input: v.input.clone().set(),
            output: v.output.clone().set(),
            ..Default::default()
        })
        .collect();

    testcases::Entity::insert_many(models)
        .exec(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    Ok(Json::from(json!({
        "status": 201,
        "msg": "inserted"
    })))
}

#[derive(Serialize, Deserialize)]
pub struct QueryByProblemId {
    pub problem_id: i64,
}
#[debug_handler]
pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<QueryByProblemId>,
) -> Result<Json<Vec<testcases::Model>>, AppError> {
    Ok(Json(
        testcases::Entity::find()
            // .join(
            //     sea_orm::JoinType::InnerJoin,
            //     testcases::Relation::Problems.def(),
            // )
            .inner_join(problems::Entity)
            .filter(
                testcases::Column::Id.eq(query.problem_id).and(
                    problems::Column::AuthorId
                        .eq(claim.id)
                        .or(problems::Column::IsPublic.eq(true)),
                ),
            )
            .all(stt.db.as_ref())
            .await
            .map_err(|e| AppError::internal(e.to_string()))?,
    ))
}

#[debug_handler]
pub async fn delete_all(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<QueryByProblemId>,
) -> Result<Json<Value>, AppError> {
    let res = testcases::Entity::delete_many()
        .filter(
            testcases::Column::ProblemId.in_subquery(
                problems::Entity::find()
                    .select_only()
                    .columns([problems::Column::Id])
                    .filter(
                        problems::Column::Id
                            .eq(query.problem_id)
                            .and(problems::Column::AuthorId.eq(claim.id)),
                    )
                    .into_query(),
            ),
        )
        .exec(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    Ok(Json(json!({
        "status": 200,
        "msg": format!("rows_affected: {}", res.rows_affected)
    })))
}
