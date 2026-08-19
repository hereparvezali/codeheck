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
use sea_orm::EntityTrait;
use serde_json::json;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use super::{
    dto::{CreateSubmissionPayload, ResponseFromWorker, RetrieveSubmissionsQuery, RetrieveSubmissionsWithCursor},
    service::SubmissionService,
};
use crate::{
    entity::submissions,
    error::AppError,
    utils::{
        app_state::{AppState, LeaderboardEvent, SubmissionEvent},
        security::Claim,
    },
};

pub async fn create(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Json(payload): Json<CreateSubmissionPayload>,
) -> Result<Json<submissions::Model>, AppError> {
    let sub = SubmissionService::create_submission(&stt.db, &stt.mq, &stt.config, &claim, payload).await?;
    Ok(Json(sub))
}

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveSubmissionsQuery>,
) -> Result<Json<submissions::Model>, AppError> {
    let id = query
        .id
        .ok_or_else(|| AppError::bad_request("Submission ID is required"))?;
    let sub = SubmissionService::get_submission(&stt.db, Some(claim.id), id).await?;
    Ok(Json(sub))
}

pub async fn retrieve_many(
    State(stt): State<AppState>,
    Extension(claim): Extension<Arc<Claim>>,
    Query(query): Query<RetrieveSubmissionsQuery>,
) -> Result<Json<RetrieveSubmissionsWithCursor>, AppError> {
    let result = SubmissionService::list_submissions(&stt.db, Some(claim.id), query).await?;
    Ok(Json(result))
}

pub async fn update(
    State(stt): State<AppState>,
    Extension(_claim): Extension<Arc<Claim>>,
    Json(response): Json<ResponseFromWorker>,
) -> Result<Json<serde_json::Value>, AppError> {
    let updated = SubmissionService::update_verdict(&stt.db, response.clone()).await?;

    let event = SubmissionEvent {
        submission_id: updated.id,
        problem_id: updated.problem_id,
        user_id: updated.user_id,
        status: updated.status.clone(),
        verdict: updated.verdict.clone(),
        time: updated.time,
        memory: updated.memory,
        contest_id: updated.contest_id,
    };

    let _ = stt.submission_broadcast.send(event);

    if let Some(cid) = updated.contest_id {
        let _ = stt.leaderboard_broadcast.send(LeaderboardEvent {
            contest_id: cid,
            submission_id: updated.id,
            user_id: updated.user_id,
        });
    }

    Ok(Json(json!({ "status": "updated" })))
}

pub async fn stream_submission(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveSubmissionsQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = stt.submission_broadcast.subscribe();
    let target_id = query.id;
    let target_problem_id = query.problem_id;

    let stream = BroadcastStream::new(rx).filter_map(move |item| {
        let ev = item.ok()?;
        if let Some(id) = target_id {
            if ev.submission_id != id {
                return None;
            }
        }
        if let Some(pid) = target_problem_id {
            if ev.problem_id != pid {
                return None;
            }
        }
        let data = serde_json::to_string(&ev).ok()?;
        Some(Ok(Event::default().event("submission_update").data(data)))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn ws_submission(
    ws: WebSocketUpgrade,
    State(stt): State<AppState>,
    Query(query): Query<RetrieveSubmissionsQuery>,
) -> Response {
    ws.on_upgrade(move |socket| handle_submission_socket(socket, stt, query.id, query.problem_id))
}

async fn handle_submission_socket(
    mut socket: WebSocket,
    stt: AppState,
    target_id: Option<i64>,
    target_problem_id: Option<i64>,
) {
    if let Some(id) = target_id {
        if let Ok(Some(sub)) = submissions::Entity::find_by_id(id).one(stt.db.as_ref()).await {
            let ev = SubmissionEvent {
                submission_id: sub.id,
                problem_id: sub.problem_id,
                user_id: sub.user_id,
                status: sub.status.clone(),
                verdict: sub.verdict.clone(),
                time: sub.time,
                memory: sub.memory,
                contest_id: sub.contest_id,
            };
            if let Ok(msg) = serde_json::to_string(&ev) {
                let _ = socket.send(Message::Text(msg.into())).await;
            }
            if sub.status.to_uppercase() != "PENDING" {
                return;
            }
        }
    }

    let mut rx = stt.submission_broadcast.subscribe();

    while let Ok(ev) = rx.recv().await {
        if let Some(id) = target_id {
            if ev.submission_id != id {
                continue;
            }
        }
        if let Some(pid) = target_problem_id {
            if ev.problem_id != pid {
                continue;
            }
        }

        if let Ok(msg) = serde_json::to_string(&ev) {
            if socket.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
            if ev.status.to_uppercase() != "PENDING" {
                break;
            }
        }
    }
}

