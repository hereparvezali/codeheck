use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ExprTrait, Order, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    sea_query::{BinOper, Expr},
};

use super::dto::{
    CreateProblemPayload, CreateTestcasePayload, ProblemIdAuthorId, RetrieveProblemQuery,
    RetrieveProblemsQueryWithCursor, RetrieveProblemsResponse,
    RetrieveProblemsWithCursorResponse, UpdateProblemPayload,
};
use crate::{
    entity::{contest_problems, contest_registrations, contests, problems, submissions, testcases},
    error::AppError,
    utils::helpers::{SetFromOption, SetFromValue},
};

pub struct ProblemService;

impl ProblemService {

    pub async fn create_problem(
        db: &DatabaseConnection,
        author_id: i64,
        payload: CreateProblemPayload,
    ) -> Result<problems::Model, AppError> {
        let conflict = problems::Entity::find()
            .filter(problems::Column::Slug.eq(&payload.slug))
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if conflict.is_some() {
            return Err(AppError::conflict("A problem with this slug already exists"));
        }

        let model = problems::ActiveModel {
            title: payload.title.set(),
            slug: payload.slug.set(),
            statement: payload.statement.set(),
            input_spec: payload.input_spec.set(),
            output_spec: payload.output_spec.set(),
            sample_inputs: Set(payload.sample_inputs.map(serde_json::Value::String)),
            sample_outputs: Set(payload.sample_outputs.map(serde_json::Value::String)),
            time_limit: payload.time_limit.set(),
            memory_limit: payload.memory_limit.set(),
            difficulty: payload.difficulty.set(),
            is_public: Set(payload.is_public.unwrap_or(true)),
            author_id: Set(Some(author_id)),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(model)
    }


    pub async fn get_problem(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        query: RetrieveProblemQuery,
    ) -> Result<problems::Model, AppError> {
        let mut filter = problems::Entity::find();

        if let Some(id) = query.id {
            filter = filter.filter(problems::Column::Id.eq(id));
        } else if let Some(ref slug) = query.slug {
            filter = filter.filter(problems::Column::Slug.eq(slug));
        } else {
            return Err(AppError::bad_request("Problem ID or slug is required"));
        }

        let problem = filter
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Problem not found"))?;

        if problem.is_public {
            return Ok(problem);
        }


        if let Some(uid) = user_id {
            if problem.author_id == Some(uid) {
                return Ok(problem);
            }


            let contest_ids: Vec<i64> = contest_problems::Entity::find()
                .filter(contest_problems::Column::ProblemId.eq(problem.id))
                .select_only()
                .column(contest_problems::Column::ContestId)
                .into_tuple()
                .all(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

            if !contest_ids.is_empty() {

                let is_contest_author = contests::Entity::find()
                    .filter(
                        Condition::all()
                            .add(contests::Column::Id.is_in(contest_ids.clone()))
                            .add(contests::Column::AuthorId.eq(uid)),
                    )
                    .one(db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?
                    .is_some();

                if is_contest_author {
                    return Ok(problem);
                }


                let now = chrono::Utc::now();
                let active_contests: Vec<i64> = contests::Entity::find()
                    .filter(
                        Condition::all()
                            .add(contests::Column::Id.is_in(contest_ids))
                            .add(contests::Column::StartTime.lte(now))
                            .add(contests::Column::EndTime.gte(now)),
                    )
                    .select_only()
                    .column(contests::Column::Id)
                    .into_tuple()
                    .all(db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;

                if !active_contests.is_empty() {
                    let is_registered = contest_registrations::Entity::find()
                        .filter(
                            Condition::all()
                                .add(contest_registrations::Column::ContestId.is_in(active_contests))
                                .add(contest_registrations::Column::UserId.eq(uid)),
                        )
                        .one(db)
                        .await
                        .map_err(|e| AppError::internal(e.to_string()))?
                        .is_some();

                    if is_registered {
                        return Ok(problem);
                    }
                }
            }
        }

        Err(AppError::not_found("Problem not found or not accessible"))
    }


    pub async fn list_problems(
        db: &DatabaseConnection,
        current_user_id: Option<i64>,
        query: RetrieveProblemsQueryWithCursor,
    ) -> Result<RetrieveProblemsWithCursorResponse, AppError> {
        let limit = query.limit.unwrap_or(20);
        let mut condition = Condition::all();

        if let Some(ref author) = query.author_id {
            condition = condition.add(problems::Column::AuthorId.eq(*author));
            if current_user_id != Some(*author) {
                condition = condition.add(problems::Column::IsPublic.eq(true));
            }
        } else if let Some(uid) = current_user_id {
            condition = condition.add(
                Condition::any()
                    .add(problems::Column::IsPublic.eq(true))
                    .add(problems::Column::AuthorId.eq(uid)),
            );
        } else {
            condition = condition.add(problems::Column::IsPublic.eq(true));
        }

        if let Some(ref diff) = query.difficulty {
            condition = condition.add(problems::Column::Difficulty.eq(diff.to_lowercase()));
        }

        if let Some(ref search_str) = query.search {
            condition = condition.add(problems::Column::Title.contains(search_str));
        }

        if let Some(cursor) = query.cursor {
            condition = condition.add(problems::Column::Id.gt(cursor));
        }

        let active_user_id = query.user_id.or(current_user_id);

        let mut select = problems::Entity::find()
            .filter(condition)
            .order_by(problems::Column::Id, Order::Asc)
            .limit(limit);

        if let Some(offset) = query.offset {
            select = select.offset(offset);
        }

        let problem_list: Vec<RetrieveProblemsResponse> = if let Some(uid) = active_user_id {
            select
                .column_as(
                    Expr::expr(
                        submissions::Entity::find()
                            .select_only()
                            .column(submissions::Column::Status)
                            .filter(
                                Condition::all()
                                    .add(
                                        Expr::col((submissions::Entity, submissions::Column::ProblemId))
                                            .binary(
                                                BinOper::Equal,
                                                Expr::col((problems::Entity, problems::Column::Id)),
                                            ),
                                    )
                                    .add(submissions::Column::UserId.eq(uid)),
                            )
                            .order_by_desc(submissions::Column::Id)
                            .limit(1)
                            .into_query(),
                    ),
                    "status",
                )
                .into_model::<RetrieveProblemsResponse>()
                .all(db)
                .await
        } else {
            select
                .into_model::<RetrieveProblemsResponse>()
                .all(db)
                .await
        }
        .map_err(|e| AppError::internal(e.to_string()))?;

        let cursor = problem_list.last().map(|p| p.id);

        Ok(RetrieveProblemsWithCursorResponse {
            cursor,
            problems: problem_list,
        })
    }


    pub async fn update_problem(
        db: &DatabaseConnection,
        author_id: i64,
        payload: UpdateProblemPayload,
    ) -> Result<u64, AppError> {
        let active_model = problems::ActiveModel {
            title: payload.title.set_from_opt(),
            slug: payload.slug.set_from_opt(),
            statement: payload.statement.set(),
            input_spec: payload.input_spec.set(),
            output_spec: payload.output_spec.set(),
            sample_inputs: Set(payload.sample_inputs.map(serde_json::Value::String)),
            sample_outputs: Set(payload.sample_outputs.map(serde_json::Value::String)),
            time_limit: payload.time_limit.set_from_opt(),
            memory_limit: payload.memory_limit.set_from_opt(),
            difficulty: payload.difficulty.set(),
            is_public: payload.is_public.set_from_opt(),
            ..Default::default()
        };

        let res = problems::Entity::update_many()
            .filter(
                Condition::all()
                    .add(problems::Column::Id.eq(payload.id))
                    .add(problems::Column::AuthorId.eq(author_id)),
            )
            .set(active_model)
            .exec(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if res.rows_affected == 0 {
            return Err(AppError::not_found("Problem not found or permission denied"));
        }

        Ok(res.rows_affected)
    }


    pub async fn delete_problem(
        db: &DatabaseConnection,
        author_id: i64,
        problem_id: i64,
    ) -> Result<u64, AppError> {
        let res = problems::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(problems::Column::Id.eq(problem_id))
                    .add(problems::Column::AuthorId.eq(author_id)),
            )
            .exec(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if res.rows_affected == 0 {
            return Err(AppError::not_found("Problem not found or permission denied"));
        }

        Ok(res.rows_affected)
    }


    pub async fn create_testcases(
        db: &DatabaseConnection,
        author_id: i64,
        payload: CreateTestcasePayload,
    ) -> Result<usize, AppError> {
        let problem = problems::Entity::find_by_id(payload.problem_id)
            .select_only()
            .columns([problems::Column::Id, problems::Column::AuthorId])
            .into_model::<ProblemIdAuthorId>()
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Problem not found"))?;

        if problem.author_id != Some(author_id) {
            return Err(AppError::forbidden("You are not the author of this problem"));
        }

        let count = payload.cases.len();
        let models: Vec<testcases::ActiveModel> = payload
            .cases
            .into_iter()
            .map(|v| testcases::ActiveModel {
                problem_id: Set(payload.problem_id),
                input: Set(Some(v.input)),
                output: Set(Some(v.output)),
                ..Default::default()
            })
            .collect();

        if !models.is_empty() {
            testcases::Entity::insert_many(models)
                .exec(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        }

        Ok(count)
    }


    pub async fn get_testcases(
        db: &DatabaseConnection,
        author_id: i64,
        problem_id: i64,
    ) -> Result<Vec<testcases::Model>, AppError> {
        let cases = testcases::Entity::find()
            .inner_join(problems::Entity)
            .filter(
                Condition::all()
                    .add(testcases::Column::ProblemId.eq(problem_id))
                    .add(problems::Column::AuthorId.eq(author_id)),
            )
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(cases)
    }


    pub async fn delete_testcases(
        db: &DatabaseConnection,
        author_id: i64,
        problem_id: i64,
    ) -> Result<u64, AppError> {
        let problem = problems::Entity::find_by_id(problem_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Problem not found"))?;

        if problem.author_id != Some(author_id) {
            return Err(AppError::forbidden("You are not the author of this problem"));
        }

        let res = testcases::Entity::delete_many()
            .filter(testcases::Column::ProblemId.eq(problem_id))
            .exec(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(res.rows_affected)
    }
}
