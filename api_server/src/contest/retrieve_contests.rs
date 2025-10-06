use super::dto::RetrieveContestsQuery;
use crate::{
    contest::dto::{ContestsResponse, RetrieveContestsWithCursor},
    entity::{contest_registrations, contests, problems},
    error::AppError,
    utils::{app_state::AppState, security::Claim},
};
use axum::{
    Extension, Json, debug_handler,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
};
use std::sync::Arc;

#[debug_handler]
pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveContestsQuery>,
) -> Result<Json<RetrieveContestsWithCursor>, AppError> {
    let coppied_claim = claim.clone();
    let contests_vec = contests::Entity::find()
        .join(
            JoinType::LeftJoin,
            contests::Relation::ContestRegistrations
                .def()
                .on_condition(move |_, _| {
                    Condition::all().add(contest_registrations::Column::UserId.eq(coppied_claim.id))
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
        .column_as(contest_registrations::Column::Id, "registration_id")
        .column_as(contest_registrations::Column::RegisteredAt, "registered_at")
        .filter(
            Condition::all()
                .add_option(query.cursor.map(|cursor| contests::Column::Id.lt(cursor)))
                .add(
                    query
                        .author_id
                        .map_or(contests::Column::IsPublic.eq(true), |auid| {
                            if auid != claim.id {
                                contests::Column::IsPublic
                                    .eq(true)
                                    .and(contests::Column::AuthorId.eq(auid))
                            } else {
                                contests::Column::AuthorId.eq(auid)
                            }
                        }),
                )
                .add_option(query.id.map(|id| problems::Column::Id.eq(id))),
        )
        .order_by_desc(contests::Column::Id)
        .limit(query.limit)
        .offset(query.offset)
        .into_model::<ContestsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    Ok(Json(RetrieveContestsWithCursor {
        cursor: contests_vec.last().map(|x| x.id),
        contests: contests_vec,
    }))
}
