use super::dto::RetrieveProblemQuery;
use crate::{
    entity::problems,
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
    Query(query): Query<RetrieveProblemQuery>,
) -> Result<Json<problems::Model>, AppError> {
    Ok(Json(
        problems::Entity::find()
            .filter(
                Condition::all()
                    .add(
                        Condition::any()
                            .add_option(query.id.map(|id| problems::Column::Id.eq(id)))
                            .add_option(query.slug.map(|slug| problems::Column::Slug.eq(slug))),
                    )
                    .add(
                        problems::Column::IsPublic
                            .eq(true)
                            .or(problems::Column::AuthorId.eq(claim.id)),
                    ),
            )
            .one(stt.db.as_ref())
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or(AppError::not_found("not_available".to_string()))?,
    ))
}
