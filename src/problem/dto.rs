use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProblemPayload {
    pub title: String,
    pub slug: String,
    pub statement: Option<String>,
    pub input_spec: Option<String>,
    pub output_spec: Option<String>,
    pub sample_inputs: Option<String>,
    pub time_limit: i16,
    pub memory_limit: i16,
    pub difficulty: Option<String>,
    pub author_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemQuery {
    pub id: Option<i64>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTestcasePayload {
    pub problem_id: i64,
    pub input: Option<String>,
    pub output: Option<String>,
}
