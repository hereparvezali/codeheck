use std::sync::Arc;

use super::dto::RetrieveProblemsQueryWithCursor;
use crate::{
    dto::MyErr,
    entity::problems,
    problem::dto::{RetrieveProblemsResponse, RetrieveProblemsWithCursorResponse},
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
    let problems_vec = problems::Entity::find()
        .filter(
            Condition::all()
                .add_option(query.cursor.map(|cursor| problems::Column::Id.lt(cursor)))
                .add(
                    query
                        .author_id
                        .map_or(problems::Column::IsPublic.eq(true), |auid| {
                            if auid != claim.id {
                                problems::Column::AuthorId
                                    .eq(auid)
                                    .and(problems::Column::IsPublic.eq(true))
                            } else {
                                problems::Column::AuthorId.eq(auid)
                            }
                        }),
                )
                .add_option(query.id.map(|id| problems::Column::Id.eq(id)))
                .add_option(query.slug.map(|slug| problems::Column::Slug.eq(slug)))
                .add_option(
                    query
                        .difficulty
                        .map(|difficulty| problems::Column::Difficulty.eq(difficulty)),
                ),
        )
        .select_only()
        .columns([
            problems::Column::Id,
            problems::Column::Slug,
            problems::Column::Title,
            problems::Column::Difficulty,
            problems::Column::IsPublic,
            problems::Column::CreatedAt,
            problems::Column::AuthorId,
        ])
        .order_by_desc(problems::Column::Id)
        .limit(query.limit)
        .offset(query.offset)
        .into_model::<RetrieveProblemsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    Ok(Json(RetrieveProblemsWithCursorResponse {
        cursor: problems_vec.last().map(|x| x.id),
        problems: problems_vec,
    }))
}
