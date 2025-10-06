use std::sync::Arc;

use crate::{
    entity::contest_registrations,
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{
    Extension, Json, debug_handler,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::{Value, json};

#[derive(Debug, serde::Deserialize)]
pub struct DeleteRegistrationQuery {
    pub registration_id: i64,
}

#[debug_handler]
pub async fn delete(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<DeleteRegistrationQuery>,
) -> Result<Json<Value>, AppError> {
    let res = contest_registrations::Entity::delete_by_id(query.registration_id)
        .filter(contest_registrations::Column::UserId.eq(claim.id))
        .exec(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
    if res.rows_affected == 0 {
        return Err(AppError::not_found("register_not_found".to_string()));
    }
    Ok(Json(json!({
        "rows_affected": res.rows_affected,
    })))
}
