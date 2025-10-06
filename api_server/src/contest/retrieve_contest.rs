use super::dto::RetrieveContestInfoQuery;
use crate::{
    entity::contests,
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{
    Extension, Json, debug_handler,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use std::sync::Arc;

#[debug_handler]
pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<contests::Model>, AppError> {
    Ok(Json(
        contests::Entity::find()
            .filter(
                Condition::all()
                    .add(
                        Condition::any()
                            .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                            .add_option(query.slug.map(|slug| contests::Column::Slug.eq(slug))),
                    )
                    .add(
                        contests::Column::IsPublic
                            .eq(true)
                            .or(contests::Column::AuthorId.eq(claim.id)),
                    ),
            )
            .one(stt.db.as_ref())
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or(AppError::not_found("contest_not_found".to_string()))?,
    ))
}
