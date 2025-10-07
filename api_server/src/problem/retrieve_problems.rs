use crate::entity::submissions;
use crate::problem::dto::RetrieveProblemsQueryWithCursor;
use crate::{
    entity::problems,
    error::AppError,
    problem::dto::{RetrieveProblemsResponse, RetrieveProblemsWithCursorResponse},
    utils::{app_state::AppState, security::Claim},
};
use axum::{
    Extension, Json, debug_handler,
    extract::{Query, State},
};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{CaseStatement, SimpleExpr, SubQueryStatement};
// use sea_orm::sea_query::{CaseStatement, ExprTrait};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    sea_query,
};
use std::sync::Arc;

#[debug_handler]
pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveProblemsQueryWithCursor>,
) -> Result<Json<RetrieveProblemsWithCursorResponse>, AppError> {
    // for now keeping round trip to database. further improvement is possible using exists in subquery
    let probs = problems::Entity::find()
        .filter(
            Condition::all()
                .add_option(query.cursor.map(|cursor| problems::Column::Id.lt(cursor)))
                .add(
                    query
                        .author_id
                        .map_or(problems::Column::IsPublic.eq(true), |auid| {
                            if auid != claim.id {
                                problems::Column::AuthorId
                                    .eq(auid)
                                    .and(problems::Column::IsPublic.eq(true))
                            } else {
                                problems::Column::AuthorId.eq(auid)
                            }
                        }),
                )
                .add_option(query.id.map(|id| problems::Column::Id.eq(id)))
                .add_option(query.slug.map(|slug| problems::Column::Slug.eq(slug)))
                .add_option(
                    query
                        .difficulty
                        .map(|difficulty| problems::Column::Difficulty.eq(difficulty)),
                ),
        )
        .select_only()
        .columns([
            problems::Column::Id,
            problems::Column::Slug,
            problems::Column::Title,
            problems::Column::Difficulty,
            problems::Column::IsPublic,
            problems::Column::CreatedAt,
            problems::Column::AuthorId,
        ])
        .apply_if(
            query.user_id.zip(query.status.clone()),
            |sttmnt, (user_id, status)| {
                sttmnt.column_as(
                    {
                        let status_subq = sea_query::Query::select()
                            .column(submissions::Column::Status)
                            .from(submissions::Entity)
                            .and_where(
                                Expr::col(submissions::Column::ProblemId)
                                    .equals((problems::Entity, problems::Column::Id))
                                    .and(submissions::Column::UserId.eq(user_id))
                                    .and(submissions::Column::Status.eq(status)),
                            )
                            .limit(1)
                            .to_owned();

                        SimpleExpr::SubQuery(
                            None,
                            Box::new(SubQueryStatement::SelectStatement(status_subq)),
                        )
                    },
                    "status",
                )
            },
        )
        .apply_if(
            query.status.map_or(query.user_id, |_| None),
            |stt, user_id| {
                stt.column_as(
                    {
                        // Subquery for checking AC status
                        let ac_sub = sea_query::Query::select()
                            .expr(Expr::val(1))
                            .from(submissions::Entity)
                            .and_where(Expr::col(submissions::Column::Status).eq("AC"))
                            .and_where(
                                Expr::col((submissions::Entity, submissions::Column::ProblemId))
                                    .equals((problems::Entity, problems::Column::Id)),
                            )
                            .and_where(Expr::col(submissions::Column::UserId).eq(user_id))
                            .take();

                        // Subquery for last submission status
                        let last_sub = sea_query::Query::select()
                            .column(submissions::Column::Status)
                            .from(submissions::Entity)
                            .and_where(
                                Expr::col((submissions::Entity, submissions::Column::ProblemId))
                                    .equals((problems::Entity, problems::Column::Id)),
                            )
                            .and_where(Expr::col(submissions::Column::UserId).eq(user_id))
                            .order_by(submissions::Column::Id, Order::Desc)
                            .limit(1)
                            .take();

                        SimpleExpr::Case(Box::new(
                            CaseStatement::new()
                                .case(Expr::exists(ac_sub), "AC")
                                .finally(SimpleExpr::SubQuery(
                                    None,
                                    Box::new(SubQueryStatement::SelectStatement(last_sub)),
                                )),
                        ))
                    },
                    "status",
                )
            },
        )
        .order_by_desc(problems::Column::Id)
        .limit(query.limit)
        .offset(query.offset)
        .into_model::<RetrieveProblemsResponse>()
        .all(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    Ok(Json(RetrieveProblemsWithCursorResponse {
        cursor: probs.last().map(|x| x.id),
        problems: probs,
    }))
}
