use jsonwebtoken::{EncodingKey, Header, encode};
use lapin::{BasicProperties, Channel, options::BasicPublishOptions};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    Order, QueryFilter, QueryOrder, QuerySelect,
};

use super::dto::{
    CreateSubmissionPayload, InputOutput, ResponseFromWorker, RetrieveSubmissionsQuery,
    RetrieveSubmissionsResponse, RetrieveSubmissionsWithCursor, SubmissionPublishQueue,
    TimeAndMemoryLimit,
};
use crate::{
    entity::{contest_problems, contest_registrations, contests, problems, submissions, testcases},
    error::AppError,
    utils::{config::Config, security::Claim},
};

pub struct SubmissionService;

impl SubmissionService {

    pub async fn create_submission(
        db: &DatabaseConnection,
        mq: &Channel,
        config: &Config,
        claim: &Claim,
        payload: CreateSubmissionPayload,
    ) -> Result<submissions::Model, AppError> {
        let problem = problems::Entity::find_by_id(payload.problem_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Problem not found"))?;

        let now = chrono::Utc::now();


        if !problem.is_public && problem.author_id != Some(claim.id) {
            let contest_id = payload.contest_id.ok_or_else(|| {
                AppError::forbidden(
                    "This problem is private and can only be submitted within an active contest",
                )
            })?;


            let in_contest = contest_problems::Entity::find()
                .filter(
                    Condition::all()
                        .add(contest_problems::Column::ContestId.eq(contest_id))
                        .add(contest_problems::Column::ProblemId.eq(problem.id)),
                )
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
                .is_some();

            if !in_contest {
                return Err(AppError::bad_request(
                    "Problem is not part of the specified contest",
                ));
            }

            let contest = contests::Entity::find_by_id(contest_id)
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
                .ok_or_else(|| AppError::not_found("Contest not found"))?;

            if now < contest.start_time {
                return Err(AppError::forbidden("Contest has not started yet"));
            }
            if now > contest.end_time {
                return Err(AppError::forbidden("Contest has already ended"));
            }

            let is_registered = contest_registrations::Entity::find()
                .filter(
                    Condition::all()
                        .add(contest_registrations::Column::ContestId.eq(contest_id))
                        .add(contest_registrations::Column::UserId.eq(claim.id)),
                )
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
                .is_some();

            if !is_registered && contest.author_id != Some(claim.id) {
                return Err(AppError::forbidden(
                    "You must be registered for this contest to submit solutions",
                ));
            }
        } else if let Some(contest_id) = payload.contest_id {

            let in_contest = contest_problems::Entity::find()
                .filter(
                    Condition::all()
                        .add(contest_problems::Column::ContestId.eq(contest_id))
                        .add(contest_problems::Column::ProblemId.eq(problem.id)),
                )
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
                .is_some();

            if !in_contest {
                return Err(AppError::bad_request(
                    "Problem is not part of the specified contest",
                ));
            }

            let contest = contests::Entity::find_by_id(contest_id)
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
                .ok_or_else(|| AppError::not_found("Contest not found"))?;

            if now < contest.start_time {
                return Err(AppError::forbidden("Contest has not started yet"));
            }
            if now > contest.end_time {
                return Err(AppError::forbidden("Contest has already ended"));
            }

            let is_registered = contest_registrations::Entity::find()
                .filter(
                    Condition::all()
                        .add(contest_registrations::Column::ContestId.eq(contest_id))
                        .add(contest_registrations::Column::UserId.eq(claim.id)),
                )
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
                .is_some();

            if !is_registered && contest.author_id != Some(claim.id) {
                return Err(AppError::forbidden(
                    "You must be registered for this contest to submit solutions",
                ));
            }
        }

        let insert_res = submissions::ActiveModel {
            user_id: Set(claim.id),
            problem_id: Set(payload.problem_id),
            language: Set(payload.language.clone()),
            code: Set(payload.code.clone()),
            contest_id: Set(payload.contest_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        let limits = problems::Entity::find_by_id(payload.problem_id)
            .select_only()
            .columns([problems::Column::TimeLimit, problems::Column::MemoryLimit])
            .into_model::<TimeAndMemoryLimit>()
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Problem not found"))?;

        let cases = testcases::Entity::find()
            .filter(testcases::Column::ProblemId.eq(payload.problem_id))
            .select_only()
            .columns([testcases::Column::Input, testcases::Column::Output])
            .into_model::<InputOutput>()
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let token = encode(
            &Header::default(),
            claim,
            &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
        )
        .map_err(|e| AppError::internal(format!("Worker token encoding failed: {}", e)))?;

        let queue_payload = serde_json::to_vec(&SubmissionPublishQueue {
            submission_id: insert_res.id,
            problem_id: payload.problem_id,
            language: payload.language,
            code: payload.code,
            time_limit: limits.time_limit,
            memory_limit: limits.memory_limit,
            inputs_outputs: cases,
            token,
        })
        .map_err(|e| AppError::internal(format!("Serialization error: {}", e)))?;

        mq.basic_publish(
            "".into(),
            config.rabbitmq.outgoing.clone().into(),
            BasicPublishOptions::default(),
            &queue_payload,
            BasicProperties::default(),
        )
        .await
        .map_err(|e| AppError::internal(format!("RabbitMQ publish error: {}", e)))?
        .await
        .map_err(|e| AppError::internal(format!("RabbitMQ confirmation error: {}", e)))?;

        Ok(insert_res)
    }


    pub async fn get_submission(
        db: &DatabaseConnection,
        user_id: Option<i64>,
        submission_id: i64,
    ) -> Result<submissions::Model, AppError> {
        let sub = submissions::Entity::find_by_id(submission_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("Submission not found"))?;


        if user_id.is_some() && sub.user_id == user_id.unwrap() {
            return Ok(sub);
        }


        if let Some(cid) = sub.contest_id {
            let contest = contests::Entity::find_by_id(cid)
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

            if let Some(c) = contest {

                if user_id.is_some() && c.author_id == user_id {
                    return Ok(sub);
                }


                let now = chrono::Utc::now();
                if now <= c.end_time {
                    return Err(AppError::forbidden(
                        "You cannot view another participant's code while the contest is active",
                    ));
                }
            }
        }


        let problem = problems::Entity::find_by_id(sub.problem_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if let Some(p) = problem {
            if user_id.is_some() && p.author_id == user_id {
                return Ok(sub);
            }
            if !p.is_public {
                return Err(AppError::forbidden(
                    "You do not have permission to view this submission",
                ));
            }
        }

        Ok(sub)
    }


    pub async fn list_submissions(
        db: &DatabaseConnection,
        current_user_id: Option<i64>,
        query: RetrieveSubmissionsQuery,
    ) -> Result<RetrieveSubmissionsWithCursor, AppError> {
        let limit = query.limit.unwrap_or(20);
        let mut condition = Condition::all();

        if let Some(id) = query.id {
            condition = condition.add(submissions::Column::Id.eq(id));
        }

        if let Some(contest_id) = query.contest_id {
            condition = condition.add(submissions::Column::ContestId.eq(contest_id));


            let contest = contests::Entity::find_by_id(contest_id)
                .one(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

            if let Some(c) = contest {
                let now = chrono::Utc::now();
                let is_contest_author = current_user_id.is_some() && c.author_id == current_user_id;

                if now <= c.end_time && !is_contest_author {

                    if let Some(uid) = current_user_id {
                        condition = condition.add(submissions::Column::UserId.eq(uid));
                    } else {
                        return Ok(RetrieveSubmissionsWithCursor {
                            cursor: None,
                            submissions: vec![],
                        });
                    }
                }
            }
        }

        if let Some(user_id) = query.user_id {
            condition = condition.add(submissions::Column::UserId.eq(user_id));
        }

        if let Some(problem_id) = query.problem_id {
            condition = condition.add(submissions::Column::ProblemId.eq(problem_id));
        }

        if let Some(ref status) = query.status {
            condition = condition.add(submissions::Column::Status.eq(status));
        }
        if let Some(ref language) = query.language {
            condition = condition.add(submissions::Column::Language.eq(language));
        }
        if let Some(cursor) = query.cursor {
            condition = condition.add(submissions::Column::Id.lt(cursor));
        }

        let mut select = submissions::Entity::find()
            .filter(condition)
            .order_by(submissions::Column::Id, Order::Desc)
            .limit(limit);

        if let Some(offset) = query.offset {
            select = select.offset(offset);
        }

        let list = select
            .into_model::<RetrieveSubmissionsResponse>()
            .all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let cursor = list.last().map(|s| s.id);

        Ok(RetrieveSubmissionsWithCursor {
            cursor,
            submissions: list,
        })
    }


    pub async fn update_verdict(
        db: &DatabaseConnection,
        response: ResponseFromWorker,
    ) -> Result<submissions::Model, AppError> {
        let active_model = submissions::ActiveModel {
            id: Set(response.id),
            status: Set(response.status),
            verdict: Set(response.verdict),
            time: Set(response.time),
            memory: Set(response.memory),
            ..Default::default()
        };

        let updated = active_model
            .update(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(updated)
    }
}
