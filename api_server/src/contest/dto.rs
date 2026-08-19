use sea_orm::entity::prelude::DateTimeWithTimeZone;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContestPayload {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_time: DateTimeWithTimeZone,
    pub end_time: DateTimeWithTimeZone,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContestPayload {
    pub id: i64,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<DateTimeWithTimeZone>,
    pub end_time: Option<DateTimeWithTimeZone>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveContestInfoQuery {
    pub id: Option<i64>,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteContestQuery {
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveContestsQuery {
    pub cursor: Option<i64>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub id: Option<i64>,
    pub slug: Option<String>,
    pub author_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct ContestsResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_time: DateTimeWithTimeZone,
    pub end_time: DateTimeWithTimeZone,
    pub is_public: bool,
    pub author_id: Option<i64>,
    pub registration_id: Option<i64>,
    pub registered_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveContestsWithCursor {
    pub cursor: Option<i64>,
    pub contests: Vec<ContestsResponse>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct RetrieveContestProblemsResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub difficulty: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddContestProblemsPayload {
    pub id: i64,
    pub problems: Vec<ProblemIdAndLabel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemIdAndLabel {
    pub problem_id: i64,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct ProblemId {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct DeleteProblemQueryParams {
    pub contest_id: i64,
    pub problem_id: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct RegistrationQuery {
    pub contest_id: Option<i64>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RegistrationBody {
    pub contest_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRegistrationQuery {
    pub registration_id: Option<i64>,
    pub id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveLeaderboardQuery {
    pub contest_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub user_id: i64,
    pub username: String,
    pub solved: i32,
    pub penalty: i64,
    pub problems: HashMap<i64, ProblemStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemStatus {
    pub solved: bool,
    pub attempts: i32,
    pub time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub standings: Vec<LeaderboardEntry>,
}
