use super::dto::RetrieveContestSubmissionsQuery;
use crate::{
    dto::MyErr,
    entity::submissions,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Query(query): Query<RetrieveContestSubmissionsQuery>,
) -> Result<Json<Vec<submissions::Model>>, MyErr> {
    Ok(Json(
        submissions::Entity::find()
            .filter(
                Condition::all()
                    .add(submissions::Column::ContestId.eq(query.contest_id))
                    .add(submissions::Column::UserId.eq(claim.id)),
            )
            .all(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
