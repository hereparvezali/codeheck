use crate::{
    contest::dto::{RetrieveContestInfoQuery, RetrieveContestProblemsResponse},
    dto::MyErr,
    entity::{contest_problems, contest_registrations, contests, problems},
    utils::app_state::AppState,
};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QuerySelect, QueryTrait,
    RelationTrait,
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<Vec<RetrieveContestProblemsResponse>>, MyErr> {
    let now = chrono::Utc::now();
    println!(
        "{}",
        problems::Entity::find()
            .join(
                JoinType::InnerJoin,
                problems::Relation::ContestProblems.def(),
            )
            .join(
                JoinType::InnerJoin,
                contest_problems::Relation::Contests.def(),
            )
            .join(
                JoinType::InnerJoin,
                contest_registrations::Relation::Contests.def(),
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
                Condition::all()
                    .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                    .add_option(
                        query
                            .slug
                            .clone()
                            .map(|slug| contests::Column::Slug.eq(slug))
                    )
                    .add(
                        query
                            .user_id
                            .map(|uid| {
                                contests::Column::StartTime
                                    .gte(now)
                                    .and(contest_registrations::Column::UserId.eq(uid))
                            })
                            .unwrap_or(contests::Column::EndTime.lt(now)),
                    ),
            )
            .build(sea_orm::DbBackend::Postgres)
            .to_string()
    );

    let probs = problems::Entity::find()
        .join(
            JoinType::InnerJoin,
            problems::Relation::ContestProblems.def(),
        )
        .join(
            JoinType::InnerJoin,
            contest_problems::Relation::Contests.def(),
        )
        .join(
            JoinType::InnerJoin,
            contests::Relation::ContestRegistrations.def(),
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
            Condition::all()
                .add_option(query.id.map(|id| contests::Column::Id.eq(id)))
                .add_option(query.slug.map(|slug| contests::Column::Slug.eq(slug)))
                .add(
                    query
                        .user_id
                        .map(|uid| {
                            contests::Column::StartTime
                                .gte(now)
                                .and(contest_registrations::Column::UserId.eq(uid))
                        })
                        .unwrap_or(contests::Column::EndTime.lt(now)),
                ),
        )
        .into_model::<RetrieveContestProblemsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;
    Ok(Json(probs))
}
