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
