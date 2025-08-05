use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubmissionPayload {
    pub user_id: i64,
    pub problem_id: i64,
    pub language: String,
    pub code: Option<String>,
    pub contest_id: Option<i64>,
}
