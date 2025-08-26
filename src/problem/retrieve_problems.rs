use super::dto::RetrieveProblemsQuery;
use crate::{
    dto::MyErr,
    entity::problems,
    problem::dto::{RetrieveProblemsPayload, RetrieveProblemsWithCursorPayload},
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Query(query): Query<RetrieveProblemsQuery>,
) -> Result<Json<RetrieveProblemsWithCursorPayload>, MyErr> {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

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
                .add(
                    problems::Column::IsPublic
                        .eq(true)
                        .or(problems::Column::AuthorId.eq(claim.id)),
                )
                .add_option(
                    query
                        .difficulty
                        .map(|difficulty| problems::Column::Difficulty.eq(difficulty)),
                ),
        )
        .order_by_desc(problems::Column::Id)
        .limit(query.limit)
        .into_model::<RetrieveProblemsPayload>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;
    Ok(Json(RetrieveProblemsWithCursorPayload {
        cursor: problems_vec.last().map(|x| x.id),
        problems: problems_vec,
    }))
}
