use super::dto::RetrieveContestInfoQuery;
use crate::{
    dto::MyErr,
    entity::contests,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<contests::Model>, MyErr> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(Json(
        contests::Entity::find()
            .filter(
                Condition::all()
                    .add(
                        Condition::any()
                            .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                            .add_option(query.slug.map(|slug| contests::Column::Slug.eq(slug))),
                    )
                    .add(
                        contests::Column::IsPublic
                            .eq(true)
                            .or(contests::Column::AuthorId.eq(claim.id)),
                    ),
            )
            .one(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
            .ok_or(MyErr::NotFound("contest_not_found".to_string()))?,
    ))
}
