use std::sync::Arc;

use axum::{Extension, Json, extract::State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    dto::MyErr,
    entity::contests,
    utils::{app_state::AppState, jwt::Claim},
};

use super::dto::CreateContestPayload;

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(contest): Json<CreateContestPayload>,
) -> Result<Json<contests::Model>, MyErr> {
    if contests::Entity::find()
        .filter(contests::Column::Slug.eq(contest.slug.clone()))
        .one(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
        .is_some()
    {
        return Err(MyErr::Conflict("slug_conflict".to_string()));
    }
    Ok(Json(
        contests::ActiveModel {
            title: Set(contest.title),
            slug: Set(contest.slug),
            description: Set(contest.description),
            start_time: Set(contest.start_time),
            end_time: Set(contest.end_time),
            is_public: Set(contest.is_public),
            author_id: Set(Some(claim.id)),
            ..Default::default()
        }
        .insert(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
