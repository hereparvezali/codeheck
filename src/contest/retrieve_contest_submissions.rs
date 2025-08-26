use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{dto::MyErr, entity::submissions, utils::app_state::AppState};

use super::dto::RetrieveContestSubmissionsQuery;

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveContestSubmissionsQuery>,
) -> Result<Json<Vec<submissions::Model>>, MyErr> {
    Ok(Json(
        submissions::Entity::find()
            .filter(submissions::Column::ContestId.eq(query.contest_id))
            .all(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
