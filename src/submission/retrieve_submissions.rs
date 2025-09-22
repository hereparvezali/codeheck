use crate::{
    dto::MyErr,
    entity::submissions,
    submission::dto::{
        RetrieveSubmissionsQuery, RetrieveSubmissionsResponse, RetrieveSubmissionsWithCursor,
    },
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use std::sync::Arc;

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveSubmissionsQuery>,
) -> Result<Json<RetrieveSubmissionsWithCursor>, MyErr> {
    let res = submissions::Entity::find()
        .filter(
            Condition::all()
                .add_option(
                    query
                        .cursor
                        .map(|cursor| submissions::Column::Id.lt(cursor)),
                )
                .add_option(query.id.map(|id| submissions::Column::Id.eq(id)))
                .add_option(
                    query
                        .user_id
                        .filter(|&user_id| user_id == claim.id)
                        .map(|user_id| submissions::Column::UserId.eq(user_id)),
                )
                .add_option(
                    query
                        .problem_id
                        .map(|problem_id| submissions::Column::ProblemId.eq(problem_id)),
                )
                .add_option(
                    query
                        .contest_id
                        .map(|contest_id| submissions::Column::ContestId.eq(contest_id)),
                )
                .add_option(
                    query
                        .status
                        .map(|status| submissions::Column::Status.eq(status)),
                )
                .add_option(
                    query
                        .language
                        .map(|language| submissions::Column::Language.eq(language)),
                ),
        )
        .order_by_desc(submissions::Column::Id)
        .limit(query.limit)
        .offset(query.offset)
        .select_only()
        .columns([
            submissions::Column::Id,
            submissions::Column::UserId,
            submissions::Column::ProblemId,
            submissions::Column::Language,
            submissions::Column::Status,
            submissions::Column::Verdict,
            submissions::Column::Time,
            submissions::Column::Memory,
            submissions::Column::SubmittedAt,
            submissions::Column::ContestId,
        ])
        .into_model::<RetrieveSubmissionsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?;

    Ok(Json(RetrieveSubmissionsWithCursor {
        cursor: res.last().map(|last| last.id),
        submissions: res,
    }))
}
