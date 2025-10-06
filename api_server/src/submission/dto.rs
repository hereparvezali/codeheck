use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubmissionPayload {
    pub user_id: i64,
    pub problem_id: i64,
    pub language: String,
    pub code: String,
    pub contest_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionPublishQueue {
    pub submission_id: i64,
    pub problem_id: i64,
    pub language: String,
    pub code: String,
    pub time_limit: i16,
    pub memory_limit: i16,
    pub inputs_outputs: Vec<InputOutput>,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct InputOutput {
    pub input: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct TimeAndMemoryLimit {
    pub time_limit: i16,
    pub memory_limit: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFromWorker {
    pub id: i64,
    pub status: String,
    pub verdict: Option<String>,
    pub time: Option<i16>,
    pub memory: Option<i16>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveSubmissionsQuery {
    pub cursor: Option<i64>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,

    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub problem_id: Option<i64>,
    pub contest_id: Option<i64>,
    pub status: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveSubmissionsWithCursor {
    pub cursor: Option<i64>,
    pub submissions: Vec<RetrieveSubmissionsResponse>,
}
#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct RetrieveSubmissionsResponse {
    pub id: i64,
    pub user_id: i64,
    pub problem_id: i64,
    pub language: String,
    pub status: String,
    pub verdict: Option<String>,
    pub time: Option<i16>,
    pub memory: Option<i16>,
    pub submitted_at: NaiveDateTime,
    pub contest_id: Option<i64>,
}
