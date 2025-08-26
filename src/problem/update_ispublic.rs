use super::dto::UpdateProblemIsPublicQuery;
use crate::{
    dto::MyErr,
    entity::problems,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub async fn update(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Query(query): Query<UpdateProblemIsPublicQuery>,
) -> Result<Json<problems::Model>, MyErr> {
    if let Some(author) = problems::Entity::find_by_id(query.problem_id)
        .one(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
        .ok_or(MyErr::NotFound("problem_not_found".to_string()))?
        .author_id
        && author != claim.id
    {
        return Err(MyErr::Unauthorized("unauthorized".to_string()));
    }
    Ok(Json(
        problems::ActiveModel {
            id: Set(query.problem_id),
            is_public: Set(query.is_public),
            ..Default::default()
        }
        .update(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
