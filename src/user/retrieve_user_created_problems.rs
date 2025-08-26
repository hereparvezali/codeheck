use axum::{Extension, Json, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    dto::MyErr,
    entity::problems,
    utils::{app_state::AppState, jwt::Claim},
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
) -> Result<Json<Vec<problems::Model>>, MyErr> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(Json(
        problems::Entity::find()
            .filter(problems::Column::AuthorId.eq(claim.id))
            .all(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
