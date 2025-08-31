use crate::{
    dto::MyErr,
    entity::contest_registrations,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::{Value, json};

pub async fn delete(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, MyErr> {
    let res = contest_registrations::Entity::delete_by_id(id)
        .filter(contest_registrations::Column::UserId.eq(claim.id))
        .exec(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;
    if res.rows_affected == 0 {
        return Err(MyErr::NotFound("register_not_found".to_string()));
    }
    Ok(Json(json!({
        "rows_affected": res.rows_affected,
    })))
}
