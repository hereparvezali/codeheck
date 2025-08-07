use super::dto::CreateTestcasePayload;
use crate::{
    dto::MyErr,
    entity::{problems, testcases},
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{Extension, Json, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QuerySelect};

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<CreateTestcasePayload>,
) -> Result<Json<testcases::Model>, MyErr> {
    if let Some(author) = problems::Entity::find_by_id(payload.problem_id.clone())
        .select_only()
        .column(problems::Column::AuthorId)
        .one(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
        .ok_or(MyErr::NotFound("problem_not_found".to_string()))?
        .author_id
        && author == claim.id
    {
        return Ok(Json(
            testcases::ActiveModel {
                problem_id: Set(payload.problem_id),
                input: Set(payload.input),
                output: Set(payload.output),
                ..Default::default()
            }
            .insert(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
        ));
    } else {
        Err(MyErr::Unauthorized("you_arent_the_author".to_string()))
    }
}
