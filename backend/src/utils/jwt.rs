use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub exp: usize,
}

impl Claim {
    pub fn new(id: i64, username: String, email: String, minutes: usize) -> Self {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize
            + (minutes * 60);
        Self {
            id,
            username,
            email,
            exp,
        }
    }
}
