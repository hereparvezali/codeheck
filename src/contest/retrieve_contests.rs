use super::dto::RetrieveContestsQuery;
use crate::{
    contest::dto::{ContestsResponse, RetrieveContestsWithCursor},
    dto::MyErr,
    entity::{contest_registrations, contests},
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
};

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Query(query): Query<RetrieveContestsQuery>,
) -> Result<Json<RetrieveContestsWithCursor>, MyErr> {
    let contests_vec = contests::Entity::find()
        .join(
            JoinType::LeftJoin,
            contests::Relation::ContestRegistrations
                .def()
                .on_condition(move |_, _| {
                    Condition::all().add(contest_registrations::Column::UserId.eq(claim.id))
                }),
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
        .column_as(contest_registrations::Column::Id, "register_id")
        .column_as(contest_registrations::Column::RegisteredAt, "registered_at")
        .filter(
            Condition::all()
                // .add(contest_registrations::Column::UserId.eq(claim.id))
                .add_option(query.cursor.map(|cursor| contests::Column::Id.lt(cursor)))
                .add(
                    contests::Column::IsPublic
                        .eq(true)
                        .or(contests::Column::AuthorId.eq(claim.id)),
                ),
        )
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
