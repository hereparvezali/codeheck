use super::dto::CreateTestcasePayload;
use crate::{
    dto::MyErr,
    entity::{problems, testcases},
    problem::dto::RetrieveProblemAuthorId,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{Extension, Json, extract::State};
use sea_orm::{ActiveValue::Set, EntityTrait, QuerySelect};
use serde_json::json;

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<CreateTestcasePayload>,
) -> Result<Json<serde_json::Value>, MyErr> {
    if problems::Entity::find_by_id(payload.problem_id)
        .select_only()
        .columns([problems::Column::Id, problems::Column::AuthorId])
        .into_model::<RetrieveProblemAuthorId>()
        .one(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
        .ok_or(MyErr::NotFound("problem_not_found".to_string()))?
        .author_id
        .ok_or(MyErr::Unauthorized("not_allowed".to_string()))?
        != claim.id
    {
        return Err(MyErr::Unauthorized("not_allowed".to_string()));
    }
    let models: Vec<testcases::ActiveModel> = payload
        .cases
        .iter()
        .map(|v| testcases::ActiveModel {
            problem_id: Set(payload.problem_id),
            input: Set(v.input.clone()),
            output: Set(v.output.clone()),
            ..Default::default()
        })
        .collect();
    testcases::Entity::insert_many(models)
        .exec(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    Ok(Json(json!({
        "status": 201,
        "msg": "inserted"
    })))
}
