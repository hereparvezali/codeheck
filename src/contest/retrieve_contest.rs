use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{dto::MyErr, entity::contests, utils::app_state::AppState};

use super::dto::RetrieveContestInfoQuery;

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<contests::Model>, MyErr> {
    Ok(Json(
        contests::Entity::find()
            .filter(
                Condition::any()
                    .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                    .add_option(query.slug.map(|slug| contests::Column::Slug.eq(slug))),
            )
            .one(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
            .ok_or(MyErr::NotFound("contest_not_found".to_string()))?,
    ))
}
