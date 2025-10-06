use crate::{
    contest::dto::{RetrieveContestInfoQuery, RetrieveContestProblemsResponse},
    entity::{contest_problems, contest_registrations, contests, problems},
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{
    Extension, Json, debug_handler,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QuerySelect, QueryTrait,
    RelationTrait,
};
use std::sync::Arc;

#[debug_handler]
pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<Vec<RetrieveContestProblemsResponse>>, AppError> {
    let now = chrono::Utc::now();

    let subq = contest_registrations::Entity::find()
        .select_only()
        .column(contest_registrations::Column::ContestId)
        .filter(
            Condition::all()
                .add(contest_registrations::Column::UserId.eq(claim.id))
                .add_option(
                    query
                        .id
                        .map(|id| contest_registrations::Column::ContestId.eq(id)),
                ),
        )
        .into_query();

    let probs = problems::Entity::find()
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
        .column(contest_problems::Column::Label)
        .filter(
            Condition::all()
                .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                .add_option(query.slug.map(|slug| contests::Column::Slug.eq(slug)))
                .add(
                    contests::Column::AuthorId
                        .eq(claim.id)
                        .or(contests::Column::IsPublic.eq(true).and(
                            contests::Column::EndTime.lt(now).or(contests::Column::Id
                                .in_subquery(subq)
                                .and(contests::Column::StartTime.lte(now))),
                        )),
                ),
        )
        .into_model::<RetrieveContestProblemsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    Ok(Json(probs))
}
