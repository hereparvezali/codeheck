use super::dto::RetrieveProblemsQueryWithCursor;
use crate::{
    dto::MyErr,
    entity::problems,
    problem::dto::{RetrieveProblemsResponse, RetrieveProblemsWithCursorResponse},
    utils::app_state::AppState,
};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveProblemsQueryWithCursor>,
) -> Result<Json<RetrieveProblemsWithCursorResponse>, MyErr> {
    let problems_vec = problems::Entity::find()
        .select_only()
        .columns([
            problems::Column::Id,
            problems::Column::Slug,
            problems::Column::Title,
            problems::Column::Difficulty,
            problems::Column::IsPublic,
            problems::Column::CreatedAt,
        ])
        .filter(
            Condition::all()
                .add_option(query.cursor.map(|cursor| problems::Column::Id.lt(cursor)))
                .add(problems::Column::IsPublic.eq(true))
                .add_option(
                    query
                        .difficulty
                        .map(|difficulty| problems::Column::Difficulty.eq(difficulty)),
                ),
        )
        .order_by_desc(problems::Column::Id)
        .limit(query.limit)
        .into_model::<RetrieveProblemsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;
    Ok(Json(RetrieveProblemsWithCursorResponse {
        cursor: problems_vec.last().map(|x| x.id),
        problems: problems_vec,
    }))
}
