use crate::{
    entity::{submissions, users},
    error::AppError,
    utils::app_state::AppState,
};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct RetrieveLeaderboardQuery {
    pub contest_id: i32,
}

#[derive(Serialize)]
pub struct LeaderboardEntry {
    pub user_id: i64,
    pub username: String,
    pub solved: i32,
    pub penalty: i64, // in seconds
    pub problems: HashMap<i64, ProblemStatus>,
}

#[derive(Serialize)]
pub struct ProblemStatus {
    pub solved: bool,
    pub attempts: i32,
    pub time: Option<i16>, // time of first AC
}

#[derive(Serialize)]
pub struct LeaderboardResponse {
    pub standings: Vec<LeaderboardEntry>,
}

#[axum::debug_handler]
pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveLeaderboardQuery>,
) -> Result<Json<LeaderboardResponse>, AppError> {
    // Fetch all submissions for the contest
    let subs = submissions::Entity::find()
        .filter(submissions::Column::ContestId.eq(query.contest_id))
        .order_by_asc(submissions::Column::SubmittedAt)
        .all(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    // Group by user and problem
    let mut user_problems: HashMap<i64, HashMap<i64, Vec<submissions::Model>>> = HashMap::new();
    for sub in subs {
        user_problems
            .entry(sub.user_id)
            .or_insert_with(HashMap::new)
            .entry(sub.problem_id)
            .or_insert_with(Vec::new)
            .push(sub);
    }

    // Fetch users
    let user_ids: Vec<i64> = user_problems.keys().cloned().collect();
    let users_list = users::Entity::find()
        .filter(users::Column::Id.is_in(user_ids))
        .all(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
    let user_map: HashMap<i64, &users::Model> = users_list.iter().map(|u| (u.id, u)).collect();

    // Compute standings
    let mut standings: Vec<LeaderboardEntry> = Vec::new();
    for (user_id, problems) in user_problems {
        let mut solved = 0;
        let mut total_penalty = 0i64;
        let mut problem_statuses: HashMap<i64, ProblemStatus> = HashMap::new();

        for (prob_id, subs) in problems {
            let mut attempts = 0;
            let mut first_ac_time: Option<i16> = None;
            let mut solved_this = false;

            for sub in &subs {
                attempts += 1;
                if sub.status == "AC" && !solved_this {
                    solved_this = true;
                    first_ac_time = sub.time;
                    total_penalty += (attempts - 1) as i64 * 1200; // 20 min penalty
                    if let Some(time) = sub.time {
                        total_penalty += time as i64;
                    }
                }
            }

            if solved_this {
                solved += 1;
            }

            problem_statuses.insert(
                prob_id,
                ProblemStatus {
                    solved: solved_this,
                    attempts,
                    time: first_ac_time,
                },
            );
        }

        if let Some(user) = user_map.get(&user_id) {
            standings.push(LeaderboardEntry {
                user_id,
                username: user.username.clone(),
                solved,
                penalty: total_penalty,
                problems: problem_statuses,
            });
        }
    }

    // Sort: solved desc, penalty asc
    standings.sort_by(|a, b| b.solved.cmp(&a.solved).then(a.penalty.cmp(&b.penalty)));

    Ok(Json(LeaderboardResponse { standings }))
}
