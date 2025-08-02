use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    dto::{MyErr, RetrieveProblemQuery},
    entity::problems,
    utils::app_state::AppState,
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveProblemQuery>,
) -> Result<Json<problems::Model>, MyErr> {
    Ok(Json(
        problems::Entity::find()
            .filter(
                Condition::any()
                    .add_option(query.id.map(|id| problems::Column::Id.eq(id)))
                    .add_option(query.slug.map(|slug| problems::Column::Slug.eq(slug))),
            )
            .one(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
            .ok_or(MyErr::NotFound("not_available".to_string()))?,
    ))
}
