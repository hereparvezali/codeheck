use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemQuery {
    pub id: Option<i64>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub input: Option<String>,
    pub output: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTestcasePayload {
    pub problem_id: i64,
    pub cases: Vec<Case>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProblemIsPublicQuery {
    pub problem_id: i64,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemsQueryWithCursor {
    pub cursor: Option<i64>,
    pub limit: Option<u64>,
    pub difficulty: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct RetrieveProblemsResponse {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub difficulty: Option<String>,
    pub is_public: bool,
    pub created_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemsWithCursorResponse {
    pub cursor: Option<i64>,
    pub problems: Vec<RetrieveProblemsResponse>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct RetrieveProblemAuthorId {
    pub id: i64,
    pub author_id: Option<i64>,
}
