use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::entity::contests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContestPayload {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveContestInfoQuery {
    pub id: Option<i64>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddContestProblemPayload {
    pub contest_id: i64,
    pub problem_id: i64,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveContestSubmissionsQuery {
    pub contest_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveContestsQuery {
    pub cursor: Option<i64>,
    pub limit: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveContestsWithCursor {
    pub cursor: Option<i64>,
    pub contests: Vec<contests::Model>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct ContestProblems {
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct RetrieveContestProblemsResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub difficulty: Option<String>,
    #[sea_orm(nested)]
    pub contest_problems: Option<ContestProblems>,
}
