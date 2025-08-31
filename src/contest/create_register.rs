use crate::{
    dto::MyErr,
    entity::contest_registrations,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Path(contest_id): Path<i64>,
) -> Result<Json<contest_registrations::Model>, MyErr> {
    Ok(Json(
        contest_registrations::ActiveModel {
            user_id: Set(claim.id),
            contest_id: Set(contest_id),
            ..Default::default()
        }
        .insert(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
