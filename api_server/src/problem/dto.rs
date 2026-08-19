use sea_orm::entity::prelude::DateTimeWithTimeZone;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemQuery {
    pub id: Option<i64>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProblemPayload {
    pub title: String,
    pub slug: String,
    pub statement: Option<String>,
    pub input_spec: Option<String>,
    pub output_spec: Option<String>,
    pub sample_inputs: Option<String>,
    pub sample_outputs: Option<String>,
    pub time_limit: i16,
    pub memory_limit: i16,
    pub difficulty: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProblemPayload {
    pub id: i64,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub statement: Option<String>,
    pub input_spec: Option<String>,
    pub output_spec: Option<String>,
    pub sample_inputs: Option<String>,
    pub sample_outputs: Option<String>,
    pub time_limit: Option<i16>,
    pub memory_limit: Option<i16>,
    pub difficulty: Option<String>,
    pub is_public: Option<bool>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub author_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemsQueryWithCursor {
    pub cursor: Option<i64>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub difficulty: Option<String>,
    pub author_id: Option<i64>,
    pub user_id: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteProblemQuery {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct RetrieveProblemsResponse {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub difficulty: Option<String>,
    pub is_public: bool,
    pub created_at: DateTimeWithTimeZone,
    pub author_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemsWithCursorResponse {
    pub cursor: Option<i64>,
    pub problems: Vec<RetrieveProblemsResponse>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct ProblemIdAuthorId {
    pub id: i64,
    pub author_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTestcasePayload {
    pub problem_id: i64,
    pub cases: Vec<Case>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryByProblemId {
    pub problem_id: i64,
}
