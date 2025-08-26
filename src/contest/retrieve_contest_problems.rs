use crate::{
    contest::dto::{RetrieveContestInfoQuery, RetrieveContestProblemsResponse},
    dto::MyErr,
    entity::{contest_problems, contests, problems},
    utils::app_state::AppState,
};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, sea_query::Cond,
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<Vec<RetrieveContestProblemsResponse>>, MyErr> {
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
            .select_only()
            .columns([
                problems::Column::Id,
                problems::Column::Title,
                problems::Column::Slug,
                problems::Column::Difficulty,
            ])
            .columns([contest_problems::Column::Label])
            .filter(
                Cond::any()
                    .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                    .add_option(query.slug.map(|slug| contests::Column::Slug.eq(slug))),
            )
            .into_model::<RetrieveContestProblemsResponse>()
            .all(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?,
    ))
}
