use std::sync::Arc;

use crate::{
    dto::MyErr,
    entity::{problems, submissions},
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{Extension, Json, extract::State};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
) -> Result<Json<Vec<(problems::Model, Option<submissions::Model>)>>, MyErr> {
    Ok(Json(
        problems::Entity::find()
            .join(JoinType::InnerJoin, problems::Relation::Submissions.def())
            .filter(
                Condition::all()
                    .add(submissions::Column::UserId.eq(claim.id))
                    .add(submissions::Column::Status.eq("AC")),
            )
            .select_also(submissions::Entity)
            .all(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
