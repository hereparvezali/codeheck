use crate::{
    contest::dto::RetrieveContestInfoQuery,
    dto::MyErr,
    entity::{contest_problems, contests, problems},
    utils::app_state::AppState,
};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<Vec<(problems::Model, Option<contest_problems::Model>)>>, MyErr> {
    Ok(Json(
        problems::Entity::find()
            .join(
                JoinType::InnerJoin,
                problems::Relation::ContestProblems.def(),
            )
            .join(
                JoinType::InnerJoin,
                contest_problems::Relation::Contests.def(),
            )
            .select_also(contest_problems::Entity)
            .filter(
                Condition::any()
                    .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                    .add_option(query.slug.map(|slug| contests::Column::Slug.eq(slug))),
            )
            .all(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
