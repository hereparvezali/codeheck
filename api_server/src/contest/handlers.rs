use std::{convert::Infallible, sync::Arc};
use axum::{
    Extension, Json,
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::{
        Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::Stream;
use serde_json::json;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use super::{
    dto::{
        AddContestProblemsPayload, ContestsResponse, CreateContestPayload, DeleteContestQuery,
        DeleteProblemQueryParams, DeleteRegistrationQuery, LeaderboardResponse, RegistrationBody,
        RegistrationQuery, RetrieveContestInfoQuery, RetrieveContestProblemsResponse,
        RetrieveContestsQuery, RetrieveContestsWithCursor, RetrieveLeaderboardQuery,
        UpdateContestPayload,
    },
    service::ContestService,
};
use crate::{
    entity::{contest_registrations, contests},
    error::AppError,
    utils::{
        app_state::AppState,
        security::Claim,
    },
};

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<CreateContestPayload>,
) -> Result<Json<contests::Model>, AppError> {
    let contest = ContestService::create_contest(&stt.db, claim.id, payload).await?;
    Ok(Json(contest))
}

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<ContestsResponse>, AppError> {
    let contest = ContestService::get_contest(&stt.db, Some(claim.id), query).await?;
    Ok(Json(contest))
}

pub async fn retrieve_many(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveContestsQuery>,
) -> Result<Json<RetrieveContestsWithCursor>, AppError> {
    let result = ContestService::list_contests(&stt.db, Some(claim.id), query).await?;
    Ok(Json(result))
}

pub async fn update(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<UpdateContestPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = ContestService::update_contest(&stt.db, claim.id, payload).await?;
    Ok(Json(json!({ "status": "updated", "rows_affected": rows })))
}

pub async fn delete(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<DeleteContestQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = ContestService::delete_contest(&stt.db, claim.id, query.id).await?;
    Ok(Json(json!({ "status": "deleted", "rows_affected": rows })))
}

pub async fn add_problems(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<AddContestProblemsPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let count = ContestService::add_problems(&stt.db, claim.id, payload).await?;
    Ok(Json(json!({ "status": "added", "count": count })))
}

pub async fn retrieve_problems(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveContestInfoQuery>,
) -> Result<Json<Vec<RetrieveContestProblemsResponse>>, AppError> {
    let contest_id = query
        .id
        .ok_or_else(|| AppError::bad_request("Contest ID is required"))?;
    let problems = ContestService::get_problems(&stt.db, Some(claim.id), contest_id).await?;
    Ok(Json(problems))
}

pub async fn delete_problem(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<DeleteProblemQueryParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = ContestService::delete_problem(&stt.db, claim.id, query).await?;
    Ok(Json(json!({ "status": "removed", "rows_affected": rows })))
}

pub async fn register(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RegistrationQuery>,
    body: Option<Json<RegistrationBody>>,
) -> Result<Json<contest_registrations::Model>, AppError> {
    let contest_id = query
        .contest_id
        .or_else(|| body.and_then(|b| b.0.contest_id))
        .ok_or_else(|| AppError::bad_request("contest_id is required"))?;

    let reg = ContestService::register(&stt.db, claim.id, contest_id).await?;
    Ok(Json(reg))
}

pub async fn unregister(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<DeleteRegistrationQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let reg_id = query
        .registration_id
        .or(query.id)
        .ok_or_else(|| AppError::bad_request("registration_id or id is required"))?;

    let rows = ContestService::unregister(&stt.db, claim.id, reg_id).await?;
    Ok(Json(json!({ "status": "unregistered", "rows_affected": rows })))
}

pub async fn retrieve_leaderboard(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveLeaderboardQuery>,
) -> Result<Json<LeaderboardResponse>, AppError> {
    let standings = ContestService::calculate_leaderboard(&stt.db, Some(claim.id), query.contest_id).await?;
    Ok(Json(standings))
}


pub async fn stream_leaderboard(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveLeaderboardQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = stt.leaderboard_broadcast.subscribe();
    let target_contest_id = query.contest_id;
    let user_id = Some(claim.id);
    let db = stt.db.clone();

    let stream = async_stream::stream! {

        if let Ok(standings) = ContestService::calculate_leaderboard(&db, user_id, target_contest_id).await {
            if let Ok(data) = serde_json::to_string(&standings) {
                yield Ok(Event::default().event("leaderboard_update").data(data));
            }
        }

        let mut b_stream = BroadcastStream::new(rx);
        while let Some(Ok(ev)) = b_stream.next().await {
            if ev.contest_id == target_contest_id {
                if let Ok(standings) = ContestService::calculate_leaderboard(&db, user_id, target_contest_id).await {
                    if let Ok(data) = serde_json::to_string(&standings) {
                        yield Ok(Event::default().event("leaderboard_update").data(data));
                    }
                }
            }
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}


pub async fn ws_leaderboard(
    ws: WebSocketUpgrade,
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveLeaderboardQuery>,
) -> Response {
    let user_id = Some(claim.id);
    ws.on_upgrade(move |socket| handle_leaderboard_socket(socket, stt, user_id, query.contest_id))
}

async fn handle_leaderboard_socket(
    mut socket: WebSocket,
    stt: AppState,
    user_id: Option<i64>,
    contest_id: i64,
) {

    if let Ok(standings) = ContestService::calculate_leaderboard(&stt.db, user_id, contest_id).await {
        if let Ok(msg) = serde_json::to_string(&standings) {
            let _ = socket.send(Message::Text(msg.into())).await;
        }
    }

    let mut rx = stt.leaderboard_broadcast.subscribe();

    while let Ok(ev) = rx.recv().await {
        if ev.contest_id != contest_id {
            continue;
        }

        if let Ok(standings) = ContestService::calculate_leaderboard(&stt.db, user_id, contest_id).await {
            if let Ok(msg) = serde_json::to_string(&standings) {
                if socket.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        }
    }
}
