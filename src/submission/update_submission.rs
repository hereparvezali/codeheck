use crate::{entity::submissions, submission::dto::ResponseFromWorker, utils::app_state::AppState};
use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

pub async fn update(State(stt): State<AppState>, Json(response): Json<ResponseFromWorker>) {
    submissions::ActiveModel {
        id: Set(response.id),
        status: Set(response.status),
        verdict: Set(response.verdict),
        time: Set(response.time),
        memory: Set(response.memory),
        ..Default::default()
    }
    .update(stt.db.as_ref())
    .await
    .unwrap();
}
