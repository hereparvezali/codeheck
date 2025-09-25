use crate::{
    contest::dto::{RetrieveContestInfoQuery, RetrieveContestProblemsResponse},
    dto::MyErr,
    entity::{contest_problems, contest_registrations, contests, problems},
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::sync::Arc;

#[allow(unused_variables)]
pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<Vec<RetrieveContestProblemsResponse>>, MyErr> {
    let now = chrono::Utc::now();

    let probs = problems::Entity::find()
        .distinct()
        .join(
            JoinType::InnerJoin,
            problems::Relation::ContestProblems.def(),
        )
        .join(
            JoinType::InnerJoin,
            contest_problems::Relation::Contests.def(),
        )
        .join(
            JoinType::LeftJoin,
            contests::Relation::ContestRegistrations.def(),
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
                            contests::Column::EndTime.lt(now).or(
                                contest_registrations::Column::UserId
                                    .eq(claim.id)
                                    .and(contests::Column::StartTime.lte(now)),
                            ),
                        )),
                ),
        )
        .into_model::<RetrieveContestProblemsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    Ok(Json(probs))
}
