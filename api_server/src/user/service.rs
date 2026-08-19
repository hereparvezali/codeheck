use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QuerySelect, QueryTrait,
};
use std::sync::Arc;

use super::dto::{
    CreateUserPayload, RetrieveUserResponse, RetrieveUserStatsQuery, RetrieveUserinfoQuery,
    SigninUserPayload, SigninUserResponse, UserStatsResponse,
};
use crate::{
    entity::{problems, submissions, users},
    error::AppError,
    utils::{
        config::Config,
        security::{self, Claim},
    },
};

pub struct UserService;

impl UserService {

    pub async fn signup(
        db: &DatabaseConnection,
        payload: CreateUserPayload,
    ) -> Result<SigninUserResponse, AppError> {
        let existing = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(&payload.username))
                    .add(users::Column::Email.eq(&payload.email)),
            )
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if existing.is_some() {
            return Err(AppError::conflict("Username or email already exists"));
        }

        let hashed_password = security::hash_password(&payload.password)?;

        let user = users::ActiveModel {
            username: Set(payload.username),
            email: Set(payload.email),
            password: Set(hashed_password),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(SigninUserResponse::new(
            user.id,
            user.username,
            user.email,
            "".to_string(),
        ))
    }


    pub async fn signin(
        db: &DatabaseConnection,
        config: &Config,
        payload: SigninUserPayload,
    ) -> Result<(SigninUserResponse, String, String), AppError> {
        let user = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(&payload.username_or_email))
                    .add(users::Column::Email.eq(&payload.username_or_email)),
            )
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::auth("Invalid username or password"))?;

        let valid = security::verify_password(&user.password, &payload.password)?;

        if !valid {
            return Err(AppError::auth("Invalid username or password"));
        }

        let access_token = security::generate_access_token(&user, config)
            .map_err(|e| AppError::internal(format!("Failed to generate access token: {}", e)))?;
        let refresh_token = security::generate_refresh_token(&user, config)
            .map_err(|e| AppError::internal(format!("Failed to generate refresh token: {}", e)))?;

        let response =
            SigninUserResponse::new(user.id, user.username, user.email, access_token.clone());

        Ok((response, access_token, refresh_token))
    }


    pub async fn get_user(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<RetrieveUserResponse, AppError> {
        let user = users::Entity::find_by_id(user_id)
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("User not found"))?;

        Ok(RetrieveUserResponse::new(
            user.id,
            user.username,
            user.email,
            user.rating,
            user.created_at,
        ))
    }


    pub async fn get_user_info(
        db: &DatabaseConnection,
        query: RetrieveUserinfoQuery,
    ) -> Result<RetrieveUserResponse, AppError> {
        let mut filter = users::Entity::find();

        if let Some(id) = query.id {
            filter = filter.filter(users::Column::Id.eq(id));
        } else if let Some(ref username) = query.username {
            filter = filter.filter(users::Column::Username.eq(username));
        } else if let Some(ref email) = query.email {
            filter = filter.filter(users::Column::Email.eq(email));
        } else {
            return Err(AppError::bad_request("id, username, or email is required"));
        }

        let user = filter
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("User not found"))?;

        Ok(RetrieveUserResponse::new(
            user.id,
            user.username,
            user.email,
            user.rating,
            user.created_at,
        ))
    }


    pub async fn get_stats(
        db: &DatabaseConnection,
        current_claim: Option<Arc<Claim>>,
        query: RetrieveUserStatsQuery,
    ) -> Result<UserStatsResponse, AppError> {
        let target_user = if let Some(uid) = query.user_id {
            users::Entity::find_by_id(uid).one(db).await
        } else if let Some(ref uname) = query.username {
            users::Entity::find()
                .filter(users::Column::Username.eq(uname))
                .one(db)
                .await
        } else if let Some(claim) = current_claim {
            users::Entity::find_by_id(claim.id).one(db).await
        } else {
            return Err(AppError::bad_request(
                "user_id, username, or authentication token is required",
            ));
        }
        .map_err(|e| AppError::internal(e.to_string()))?
        .ok_or_else(|| AppError::not_found("User not found"))?;

        let total_submissions = submissions::Entity::find()
            .filter(submissions::Column::UserId.eq(target_user.id))
            .count(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))? as i64;

        let accepted_submissions = submissions::Entity::find()
            .filter(
                Condition::all()
                    .add(submissions::Column::UserId.eq(target_user.id))
                    .add(submissions::Column::Status.eq("AC")),
            )
            .count(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            as i64;

        let solved_subquery = submissions::Entity::find()
            .select_only()
            .column(submissions::Column::ProblemId)
            .filter(
                Condition::all()
                    .add(submissions::Column::UserId.eq(target_user.id))
                    .add(submissions::Column::Status.eq("AC")),
            )
            .into_query();

        let total_solved = problems::Entity::find()
            .filter(problems::Column::Id.in_subquery(solved_subquery.clone()))
            .count(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))? as i64;

        let easy_solved = problems::Entity::find()
            .filter(
                Condition::all()
                    .add(problems::Column::Id.in_subquery(solved_subquery.clone()))
                    .add(problems::Column::Difficulty.eq("easy")),
            )
            .count(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))? as i64;

        let medium_solved = problems::Entity::find()
            .filter(
                Condition::all()
                    .add(problems::Column::Id.in_subquery(solved_subquery.clone()))
                    .add(problems::Column::Difficulty.eq("medium")),
            )
            .count(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))? as i64;

        let hard_solved = problems::Entity::find()
            .filter(
                Condition::all()
                    .add(problems::Column::Id.in_subquery(solved_subquery))
                    .add(problems::Column::Difficulty.eq("hard")),
            )
            .count(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))? as i64;

        Ok(UserStatsResponse {
            user_id: target_user.id,
            username: target_user.username,
            email: target_user.email,
            rating: target_user.rating,
            created_at: target_user.created_at,
            total_solved,
            easy_solved,
            medium_solved,
            hard_solved,
            total_submissions,
            accepted_submissions,
        })
    }
}
