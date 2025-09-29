use serde::{Deserialize, Serialize};

/// Represents a submission received from the queue.
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

/// Represents an input-output pair for a test case.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputOutput {
    pub input: Option<String>,
    pub output: Option<String>,
}

/// Represents the response sent back to the API after processing a submission.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFromWorker {
    pub id: i64,
    pub status: String,
    pub verdict: Option<String>,
    pub time: Option<i16>,
    pub memory: Option<i16>,
}

impl ResponseFromWorker {
    /// Creates a new ResponseFromWorker with default values.
    pub fn new(id: i64) -> Self {
        Self {
            id,
            status: "AC".to_string(),
            verdict: None,
            time: None,
            memory: None,
        }
    }
}
