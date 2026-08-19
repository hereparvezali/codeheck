use sea_orm::entity::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninUserPayload {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninUserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub access_token: String,
}
impl SigninUserResponse {
    pub fn new(
        id: i64,
        username: String,
        email: String,
        access_token: String,
    ) -> SigninUserResponse {
        Self {
            id,
            username,
            email,
            access_token,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveUserinfoQuery {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveUserStatsQuery {
    pub user_id: Option<i64>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsResponse {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub rating: i16,
    pub created_at: DateTimeWithTimeZone,
    pub total_solved: i64,
    pub easy_solved: i64,
    pub medium_solved: i64,
    pub hard_solved: i64,
    pub total_submissions: i64,
    pub accepted_submissions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveUserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub rating: i16,
    pub created_at: DateTimeWithTimeZone,
}
impl RetrieveUserResponse {
    pub fn new(id: i64, username: String, email: String, rating: i16, created_at: DateTimeWithTimeZone) -> Self {
        Self {
            id,
            username,
            email,
            rating,
            created_at,
        }
    }
}

