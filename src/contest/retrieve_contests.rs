use super::dto::RetrieveContestsQuery;
use crate::{
    contest::dto::RetrieveContestsWithCursor,
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
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let contests_vec = contests::Entity::find()
        .filter(
            Condition::all()
                .add_option(query.cursor.map(|cursor| contests::Column::Id.lt(cursor)))
                .add(
                    contests::Column::IsPublic
                        .eq(true)
                        .or(contests::Column::AuthorId.eq(claim.id)),
                ),
        )
        .order_by_desc(contests::Column::Id)
        .limit(query.limit)
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    Ok(Json(RetrieveContestsWithCursor {
        cursor: contests_vec.last().map(|x| x.id),
        contests: contests_vec,
    }))
}
