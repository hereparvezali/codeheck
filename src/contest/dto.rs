use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

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
pub struct AddContestProblemsPayload {
    pub contest_id: i64,
    pub problems: Vec<ProblemIdAndLabel>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemIdAndLabel {
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
    pub offset: Option<u64>,

    pub id: Option<i64>,
    pub slug: Option<String>,
    pub author_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct ContestRegistrationsResponse {}
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct ContestsResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_public: bool,
    pub author_id: Option<i64>,

    pub register_id: Option<i64>,
    pub registered_at: Option<NaiveDateTime>,
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

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct ProblemId {
    pub id: i64,
}
