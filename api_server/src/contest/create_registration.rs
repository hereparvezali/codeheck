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
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

#[derive(Debug, serde::Deserialize)]
pub struct RegistrationQuery {
    pub contest_id: i64,
}
#[debug_handler]
pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RegistrationQuery>,
) -> Result<Json<contest_registrations::Model>, AppError> {
    Ok(Json(
        contest_registrations::ActiveModel {
            user_id: Set(claim.id),
            contest_id: Set(query.contest_id),
            ..Default::default()
        }
        .insert(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?,
    ))
}
