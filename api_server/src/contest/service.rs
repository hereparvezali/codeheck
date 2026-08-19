use std::collections::HashMap;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ExprTrait, Order, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    sea_query::{BinOper, Expr},
};

use super::dto::{
    AddContestProblemsPayload, ContestsResponse, CreateContestPayload, DeleteProblemQueryParams,
    LeaderboardEntry, LeaderboardResponse, ProblemStatus, RetrieveContestInfoQuery,
    RetrieveContestProblemsResponse, RetrieveContestsQuery, RetrieveContestsWithCursor,
    UpdateContestPayload,
};
use crate::{
    entity::{contest_problems, contest_registrations, contests, problems, submissions, users},
    error::AppError,
    utils::helpers::SetFromOption,
};

pub struct ContestService;

impl ContestService {

    pub async fn create_contest(
        db: &DatabaseConnection,
        author_id: i64,
        payload: CreateContestPayload,
    ) -> Result<contests::Model, AppError> {
        let conflict = contests::Entity::find()
            .filter(contests::Column::Slug.eq(&payload.slug))
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if conflict.is_some() {
            return Err(AppError::conflict("A contest with this slug already exists"));
        }

        let model = contests::ActiveModel {
            title: Set(payload.title),
            slug: Set(payload.slug),
            description: Set(payload.description),
            start_time: Set(payload.start_time),
            end_time: Set(payload.end_time),
            is_public: Set(payload.is_public),
            author_id: Set(Some(author_id)),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(model)
    }


    pub async fn get_contest(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        query: RetrieveContestInfoQuery,
    ) -> Result<ContestsResponse, AppError> {
        let mut filter = contests::Entity::find();

        if let Some(id) = query.id {
            filter = filter.filter(contests::Column::Id.eq(id));
        } else if let Some(ref slug) = query.slug {
            filter = filter.filter(contests::Column::Slug.eq(slug));
        } else {
            return Err(AppError::bad_request("Contest ID or slug is required"));
        }

        let mut select = filter
            .columns([
                contests::Column::Id,
                contests::Column::Title,
                contests::Column::Slug,
                contests::Column::Description,
                contests::Column::StartTime,
                contests::Column::EndTime,
                contests::Column::IsPublic,
                contests::Column::AuthorId,
            ]);

        if let Some(uid) = user_id {
            select = select
                .column_as(
                    Expr::expr(
                        contest_registrations::Entity::find()
                            .select_only()
                            .column(contest_registrations::Column::Id)
                            .filter(
                                Condition::all()
                                    .add(
                                        Expr::col((
                                            contest_registrations::Entity,
                                            contest_registrations::Column::ContestId,
                                        ))
                                        .binary(
                                            BinOper::Equal,
                                            Expr::col((contests::Entity, contests::Column::Id)),
                                        ),
                                    )
                                    .add(contest_registrations::Column::UserId.eq(uid)),
                            )
                            .limit(1)
                            .into_query(),
                    ),
                    "registration_id",
                )
                .column_as(
                    Expr::expr(
                        contest_registrations::Entity::find()
                            .select_only()
                            .column(contest_registrations::Column::RegisteredAt)
                            .filter(
                                Condition::all()
                                    .add(
                                        Expr::col((
                                            contest_registrations::Entity,
                                            contest_registrations::Column::ContestId,
                                        ))
                                        .binary(
                                            BinOper::Equal,
                                            Expr::col((contests::Entity, contests::Column::Id)),
                                        ),
                                    )
                                    .add(contest_registrations::Column::UserId.eq(uid)),
                            )
                            .limit(1)
                            .into_query(),
                    ),
                    "registered_at",
                );
        }

        let contest = select
            .into_model::<ContestsResponse>()
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Contest not found"))?;

        Ok(contest)
    }


    pub async fn list_contests(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        query: RetrieveContestsQuery,
    ) -> Result<RetrieveContestsWithCursor, AppError> {
        let limit = query.limit.unwrap_or(20);
        let mut condition = Condition::all();

        if let Some(ref author) = query.author_id {
            condition = condition.add(contests::Column::AuthorId.eq(*author));
            if user_id != Some(*author) {
                condition = condition.add(contests::Column::IsPublic.eq(true));
            }
        } else if let Some(uid) = user_id {
            condition = condition.add(
                Condition::any()
                    .add(contests::Column::IsPublic.eq(true))
                    .add(contests::Column::AuthorId.eq(uid)),
            );
        } else {
            condition = condition.add(contests::Column::IsPublic.eq(true));
        }

        if let Some(cursor) = query.cursor {
            condition = condition.add(contests::Column::Id.gt(cursor));
        }

        let mut select = contests::Entity::find()
            .filter(condition)
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
            .order_by(contests::Column::Id, Order::Asc)
            .limit(limit);

        if let Some(offset) = query.offset {
            select = select.offset(offset);
        }

        if let Some(uid) = user_id {
            select = select
                .column_as(
                    Expr::expr(
                        contest_registrations::Entity::find()
                            .select_only()
                            .column(contest_registrations::Column::Id)
                            .filter(
                                Condition::all()
                                    .add(
                                        Expr::col((
                                            contest_registrations::Entity,
                                            contest_registrations::Column::ContestId,
                                        ))
                                        .binary(
                                            BinOper::Equal,
                                            Expr::col((contests::Entity, contests::Column::Id)),
                                        ),
                                    )
                                    .add(contest_registrations::Column::UserId.eq(uid)),
                            )
                            .limit(1)
                            .into_query(),
                    ),
                    "registration_id",
                )
                .column_as(
                    Expr::expr(
                        contest_registrations::Entity::find()
                            .select_only()
                            .column(contest_registrations::Column::RegisteredAt)
                            .filter(
                                Condition::all()
                                    .add(
                                        Expr::col((
                                            contest_registrations::Entity,
                                            contest_registrations::Column::ContestId,
                                        ))
                                        .binary(
                                            BinOper::Equal,
                                            Expr::col((contests::Entity, contests::Column::Id)),
                                        ),
                                    )
                                    .add(contest_registrations::Column::UserId.eq(uid)),
                            )
                            .limit(1)
                            .into_query(),
                    ),
                    "registered_at",
                );
        }

        let contest_list = select
            .into_model::<ContestsResponse>()
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let cursor = contest_list.last().map(|c| c.id);

        Ok(RetrieveContestsWithCursor {
            cursor,
            contests: contest_list,
        })
    }


    pub async fn update_contest(
        db: &DatabaseConnection,
        author_id: i64,
        payload: UpdateContestPayload,
    ) -> Result<u64, AppError> {
        let active_model = contests::ActiveModel {
            title: payload.title.set_from_opt(),
            slug: payload.slug.set_from_opt(),
            description: Set(payload.description),
            start_time: payload.start_time.set_from_opt(),
            end_time: payload.end_time.set_from_opt(),
            is_public: payload.is_public.set_from_opt(),
            ..Default::default()
        };

        let res = contests::Entity::update_many()
            .filter(
                Condition::all()
                    .add(contests::Column::Id.eq(payload.id))
                    .add(contests::Column::AuthorId.eq(author_id)),
            )
            .set(active_model)
            .exec(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if res.rows_affected == 0 {
            return Err(AppError::not_found("Contest not found or permission denied"));
        }

        Ok(res.rows_affected)
    }


    pub async fn delete_contest(
        db: &DatabaseConnection,
        author_id: i64,
        contest_id: i64,
    ) -> Result<u64, AppError> {
        let res = contests::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(contests::Column::Id.eq(contest_id))
                    .add(contests::Column::AuthorId.eq(author_id)),
            )
            .exec(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if res.rows_affected == 0 {
            return Err(AppError::not_found("Contest not found or permission denied"));
        }

        Ok(res.rows_affected)
    }


    pub async fn add_problems(
        db: &DatabaseConnection,
        author_id: i64,
        payload: AddContestProblemsPayload,
    ) -> Result<usize, AppError> {
        let contest = contests::Entity::find_by_id(payload.id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Contest not found"))?;

        if contest.author_id != Some(author_id) {
            return Err(AppError::auth("You are not the author of this contest"));
        }


        let requested_pids: Vec<i64> = payload.problems.iter().map(|p| p.problem_id).collect();
        let valid_problems = problems::Entity::find()
            .filter(
                Condition::all()
                    .add(problems::Column::Id.is_in(requested_pids.clone()))
                    .add(
                        Condition::any()
                            .add(problems::Column::IsPublic.eq(true))
                            .add(problems::Column::AuthorId.eq(author_id)),
                    ),
            )
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let valid_pids: std::collections::HashSet<i64> = valid_problems.into_iter().map(|p| p.id).collect();
        for pid in &requested_pids {
            if !valid_pids.contains(pid) {
                return Err(AppError::auth(format!(
                    "Problem ID {} is private and not authored by you",
                    pid
                )));
            }
        }


        let existing_pids: std::collections::HashSet<i64> = contest_problems::Entity::find()
            .filter(contest_problems::Column::ContestId.eq(payload.id))
            .select_only()
            .column(contest_problems::Column::ProblemId)
            .into_tuple()
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .into_iter()
            .collect();

        let models: Vec<contest_problems::ActiveModel> = payload
            .problems
            .into_iter()
            .filter(|p| !existing_pids.contains(&p.problem_id))
            .map(|p| contest_problems::ActiveModel {
                contest_id: Set(payload.id),
                problem_id: Set(p.problem_id),
                label: Set(p.label),
                ..Default::default()
            })
            .collect();

        let count = models.len();
        if !models.is_empty() {
            contest_problems::Entity::insert_many(models)
                .exec(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        }

        Ok(count)
    }


    pub async fn get_problems(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        contest_id: i64,
    ) -> Result<Vec<RetrieveContestProblemsResponse>, AppError> {
        let contest = contests::Entity::find_by_id(contest_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Contest not found"))?;

        let is_author = user_id.is_some() && contest.author_id == user_id;

        if !is_author {
            let now = chrono::Utc::now();
            if now < contest.start_time {
                return Err(AppError::auth(
                    "Contest has not started yet. Problems will be visible once the contest begins.",
                ));
            }

            let uid = user_id.ok_or_else(|| {
                AppError::auth("You must be registered for this contest to view problems")
            })?;

            let is_registered = contest_registrations::Entity::find()
                .filter(
                    Condition::all()
                        .add(contest_registrations::Column::ContestId.eq(contest_id))
                        .add(contest_registrations::Column::UserId.eq(uid)),
                )
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
                .is_some();

            if !is_registered {
                return Err(AppError::auth(
                    "You must be registered for this contest to view problems",
                ));
            }
        }

        let list = contest_problems::Entity::find()
            .filter(contest_problems::Column::ContestId.eq(contest_id))
            .inner_join(problems::Entity)
            .columns([
                problems::Column::Id,
                problems::Column::Title,
                problems::Column::Slug,
                problems::Column::Difficulty,
            ])
            .column(contest_problems::Column::Label)
            .order_by(contest_problems::Column::Label, Order::Asc)
            .into_model::<RetrieveContestProblemsResponse>()
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(list)
    }


    pub async fn delete_problem(
        db: &DatabaseConnection,
        author_id: i64,
        params: DeleteProblemQueryParams,
    ) -> Result<u64, AppError> {
        let contest = contests::Entity::find_by_id(params.contest_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Contest not found"))?;

        if contest.author_id != Some(author_id) {
            return Err(AppError::auth("You are not the author of this contest"));
        }

        let res = contest_problems::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(contest_problems::Column::ContestId.eq(params.contest_id))
                    .add(contest_problems::Column::ProblemId.eq(params.problem_id)),
            )
            .exec(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(res.rows_affected)
    }


    pub async fn register(
        db: &DatabaseConnection,
        user_id: i64,
        contest_id: i64,
    ) -> Result<contest_registrations::Model, AppError> {
        let contest = contests::Entity::find_by_id(contest_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Contest not found"))?;

        let now = chrono::Utc::now();
        if now > contest.end_time {
            return Err(AppError::bad_request(
                "Registration is closed. This contest has already ended.",
            ));
        }

        let existing = contest_registrations::Entity::find()
            .filter(
                Condition::all()
                    .add(contest_registrations::Column::ContestId.eq(contest_id))
                    .add(contest_registrations::Column::UserId.eq(user_id)),
            )
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if let Some(reg) = existing {
            return Ok(reg);
        }

        let reg = contest_registrations::ActiveModel {
            user_id: Set(user_id),
            contest_id: Set(contest_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(reg)
    }


    pub async fn unregister(
        db: &DatabaseConnection,
        user_id: i64,
        registration_id: i64,
    ) -> Result<u64, AppError> {
        let reg = contest_registrations::Entity::find_by_id(registration_id)
            .filter(contest_registrations::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Registration not found"))?;

        let contest = contests::Entity::find_by_id(reg.contest_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Contest not found"))?;

        let now = chrono::Utc::now();
        if now > contest.end_time {
            return Err(AppError::bad_request(
                "Cannot unregister from a contest that has already ended.",
            ));
        }

        let res = contest_registrations::Entity::delete_by_id(registration_id)
            .exec(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if res.rows_affected == 0 {
            return Err(AppError::not_found("Registration not found"));
        }

        Ok(res.rows_affected)
    }


    pub async fn calculate_leaderboard(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        contest_id: i64,
    ) -> Result<LeaderboardResponse, AppError> {
        let contest = contests::Entity::find_by_id(contest_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Contest not found"))?;


        if !contest.is_public {
            let is_author = user_id.is_some() && contest.author_id == user_id;
            if !is_author {
                let uid = user_id.ok_or_else(|| {
                    AppError::auth("You must be registered to view this private contest's leaderboard")
                })?;

                let is_reg = contest_registrations::Entity::find()
                    .filter(
                        Condition::all()
                            .add(contest_registrations::Column::ContestId.eq(contest_id))
                            .add(contest_registrations::Column::UserId.eq(uid)),
                    )
                    .one(db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?
                    .is_some();

                if !is_reg {
                    return Err(AppError::auth(
                        "You must be registered to view this private contest's leaderboard",
                    ));
                }
            }
        }

        let subs = submissions::Entity::find()
            .filter(
                Condition::all()
                    .add(submissions::Column::ContestId.eq(contest_id))
                    .add(submissions::Column::SubmittedAt.gte(contest.start_time))
                    .add(submissions::Column::SubmittedAt.lte(contest.end_time)),
            )
            .order_by_asc(submissions::Column::SubmittedAt)
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let mut user_ids: Vec<i64> = subs.iter().map(|s| s.user_id).collect();
        user_ids.sort_unstable();
        user_ids.dedup();

        let all_users = users::Entity::find()
            .filter(users::Column::Id.is_in(user_ids))
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let user_map: HashMap<i64, String> = all_users
            .into_iter()
            .map(|u| (u.id, u.username))
            .collect();

        let mut entries: HashMap<i64, LeaderboardEntry> = HashMap::new();

        for sub in subs {
            let entry = entries.entry(sub.user_id).or_insert_with(|| LeaderboardEntry {
                user_id: sub.user_id,
                username: user_map
                    .get(&sub.user_id)
                    .cloned()
                    .unwrap_or_else(|| format!("User {}", sub.user_id)),
                solved: 0,
                penalty: 0,
                problems: HashMap::new(),
            });

            let prob = entry.problems.entry(sub.problem_id).or_insert_with(|| ProblemStatus {
                solved: false,
                attempts: 0,
                time: None,
            });

            if prob.solved {
                continue;
            }

            prob.attempts += 1;

            if sub.status == "AC" {
                prob.solved = true;
                let raw_secs = (sub.submitted_at - contest.start_time).num_seconds();
                let elapsed_secs = if raw_secs > 0 { raw_secs } else { 0 };
                prob.time = Some(elapsed_secs);

                entry.solved += 1;
                let wrong_attempts_penalty = (prob.attempts - 1) as i64 * 1200;
                entry.penalty += elapsed_secs + wrong_attempts_penalty;
            }
        }

        let mut standings: Vec<LeaderboardEntry> = entries.into_values().collect();
        standings.sort_by(|a, b| {
            b.solved
                .cmp(&a.solved)
                .then_with(|| a.penalty.cmp(&b.penalty))
        });

        Ok(LeaderboardResponse { standings })
    }
}
