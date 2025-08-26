use axum::{Extension, Json, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    dto::MyErr,
    entity::submissions,
    utils::{app_state::AppState, jwt::Claim},
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
) -> Result<Json<Vec<submissions::Model>>, MyErr> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(Json(
        submissions::Entity::find()
            .filter(submissions::Column::UserId.eq(claim.id))
            .all(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
