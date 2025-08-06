use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UsernameOrEmail {
    Email(String),
    Username(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserPayload {
    pub username_or_email: UsernameOrEmail,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResponse {
    id: i64,
    username: String,
    email: String,
    access_token: String,
}
impl LoginUserResponse {
    pub fn new(
        id: i64,
        username: String,
        email: String,
        access_token: String,
    ) -> LoginUserResponse {
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
pub struct RetrieveUserResponse {
    pub username: String,
    pub email: String,
    pub rating: i16,
    pub created_at: NaiveDateTime,
}
impl RetrieveUserResponse {
    pub fn new(username: String, email: String, rating: i16, created_at: NaiveDateTime) -> Self {
        Self {
            username,
            email,
            rating,
            created_at,
        }
    }
}
