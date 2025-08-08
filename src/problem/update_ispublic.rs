use super::dto::UpdateProblemIsPublicQuery;
use crate::{dto::MyErr, entity::problems, utils::app_state::AppState};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

pub async fn update(
    State(stt): State<AppState>,
    Query(query): Query<UpdateProblemIsPublicQuery>,
) -> Result<Json<problems::Model>, MyErr> {
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
