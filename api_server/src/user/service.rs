use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QuerySelect, QueryTrait,
};
use std::sync::Arc;

use super::dto::{
    CreateUserPayload, MessageResponse, ResendVerificationPayload, RetrieveUserResponse,
    RetrieveUserStatsQuery, RetrieveUserinfoQuery, SigninUserPayload, SigninUserResponse,
    SignupResponse, UserStatsResponse,
};
use crate::{
    entity::{problems, submissions, users},
    error::AppError,
    utils::{
        config::Config,
        mailer::Mailer,
        security::{self, Claim},
    },
};

pub struct UserService;

impl UserService {

    pub async fn signup(
        db: &DatabaseConnection,
        config: &Config,
        payload: CreateUserPayload,
    ) -> Result<SignupResponse, AppError> {
        let existing = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(&payload.username))
                    .add(users::Column::Email.eq(&payload.email)),
            )
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let hashed_password = security::hash_password(&payload.password)?;
        let verification_token = uuid::Uuid::new_v4().simple().to_string();

        let user = if let Some(existing_user) = existing {
            if existing_user.verified {
                return Err(AppError::conflict("Username or email already exists"));
            }
            let mut active_user: users::ActiveModel = existing_user.into();
            active_user.username = Set(payload.username.clone());
            active_user.email = Set(payload.email.clone());
            active_user.password = Set(hashed_password);
            active_user.verification_token = Set(Some(verification_token.clone()));
            active_user
                .update(db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?
        } else {
            users::ActiveModel {
                username: Set(payload.username.clone()),
                email: Set(payload.email.clone()),
                password: Set(hashed_password),
                verified: Set(false),
                verification_token: Set(Some(verification_token.clone())),
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
        };

        if let Err(mail_err) = Mailer::send_verification_email(
            &config.smtp,
            &user.email,
            &user.username,
            &verification_token,
        )
        .await
        {
            return Err(mail_err);
        }

        Ok(SignupResponse::new(
            user.id,
            user.username,
            user.email,
            "User registered successfully. Please check your email to verify your account.",
        ))
    }

    pub async fn verify_email(
        db: &DatabaseConnection,
        token: &str,
    ) -> Result<MessageResponse, AppError> {
        if token.trim().is_empty() {
            return Err(AppError::bad_request("Verification token cannot be empty"));
        }

        let user = users::Entity::find()
            .filter(users::Column::VerificationToken.eq(token))
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::bad_request("Invalid or expired verification token"))?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.verified = Set(true);
        active_user.verification_token = Set(None);

        active_user
            .update(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(MessageResponse::new("Email verified successfully! You can now sign in."))
    }

    pub async fn resend_verification(
        db: &DatabaseConnection,
        config: &Config,
        payload: ResendVerificationPayload,
    ) -> Result<MessageResponse, AppError> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&payload.email))
            .one(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("User with this email not found"))?;

        if user.verified {
            return Err(AppError::bad_request("Email is already verified"));
        }

        let new_token = uuid::Uuid::new_v4().simple().to_string();
        let mut active_user: users::ActiveModel = user.into();
        active_user.verification_token = Set(Some(new_token.clone()));

        let updated_user = active_user
            .update(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Mailer::send_verification_email(
            &config.smtp,
            &updated_user.email,
            &updated_user.username,
            &new_token,
        )
        .await?;

        Ok(MessageResponse::new("Verification email resent successfully"))
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

        if !user.verified {
            return Err(AppError::auth("Please verify your email before signing in"));
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
            user.verified,
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
            user.verified,
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
