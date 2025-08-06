use axum::{Extension, Json, extract::State};
use sea_orm::EntityTrait;

use crate::{
    dto::MyErr,
    entity::users,
    utils::{app_state::AppState, jwt::Claim},
};

use super::dto::RetrieveUserResponse;

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
) -> Result<Json<RetrieveUserResponse>, MyErr> {
    Ok(Json(
        users::Entity::find_by_id(claim.id)
            .one(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
            .ok_or(MyErr::NotFound("user_not_found".to_string()))
            .map(|usr| {
                RetrieveUserResponse::new(usr.username, usr.email, usr.rating, usr.created_at)
            })?,
    ))
}
