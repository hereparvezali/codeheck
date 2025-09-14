use crate::{
    contest::dto::{ContestsResponse, RetrieveContestsQuery, RetrieveContestsWithCursor},
    dto::MyErr,
    entity::contests,
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
    Query(query): Query<RetrieveContestsQuery>,
) -> Result<Json<RetrieveContestsWithCursor>, MyErr> {
    let contests_vec = contests::Entity::find()
        .filter(
            Condition::all()
                .add(contests::Column::AuthorId.eq(claim.id))
                .add_option(query.cursor.map(|cursor| contests::Column::Id.lt(cursor))),
        )
        .select_only()
        .columns([
            contests::Column::Id,
            contests::Column::Title,
            contests::Column::Slug,
            contests::Column::Description,
            contests::Column::StartTime,
            contests::Column::EndTime,
            contests::Column::IsPublic,
            contests::Column::AuthorId,
        ])
        .order_by_desc(contests::Column::Id)
        .limit(query.limit)
        .into_model::<ContestsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    Ok(Json(RetrieveContestsWithCursor {
        cursor: contests_vec.last().map(|x| x.id),
        contests: contests_vec,
    }))
}
