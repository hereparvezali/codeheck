use std::sync::Arc;

use crate::{
    dto::MyErr,
    entity::problems,
    problem::dto::{
        RetrieveProblemsQueryWithCursor, RetrieveProblemsResponse,
        RetrieveProblemsWithCursorResponse,
    },
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveProblemsQueryWithCursor>,
) -> Result<Json<RetrieveProblemsWithCursorResponse>, MyErr> {
    let res = problems::Entity::find()
        .filter(
            Condition::all()
                .add(problems::Column::AuthorId.eq(claim.id))
                .add_option(query.cursor.map(|cursor| problems::Column::Id.lt(cursor)))
                .add_option(query.difficulty.map(|diff| problems::Column::Id.eq(diff))),
        )
        .select_only()
        .columns([
            problems::Column::Id,
            problems::Column::Slug,
            problems::Column::Title,
            problems::Column::Difficulty,
            problems::Column::IsPublic,
            problems::Column::CreatedAt,
        ])
        .order_by_desc(problems::Column::Id)
        .limit(query.limit)
        .into_model::<RetrieveProblemsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;
    Ok(Json(RetrieveProblemsWithCursorResponse {
        cursor: res.last().map(|x| x.id),
        problems: res,
    }))
}
