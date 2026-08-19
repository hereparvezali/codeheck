use futures_util::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};

use super::{dto::ResponseFromWorker, service::SubmissionService};
use crate::utils::app_state::{AppState, LeaderboardEvent, SubmissionEvent};

pub struct VerdictsConsumer;

impl VerdictsConsumer {

    pub fn start(state: AppState) {
        tokio::spawn(async move {
            let queue_name = state.config.rabbitmq.incomming.clone();
            println!(
                "[Verdict Consumer] Starting listener on queue '{}'...",
                queue_name
            );

            loop {
                let consumer_result = state
                    .mq
                    .basic_consume(
                        queue_name.as_str().into(),
                        "api_server_verdicts_consumer".into(),
                        BasicConsumeOptions::default(),
                        FieldTable::default(),
                    )
                    .await;

                match consumer_result {
                    Ok(mut consumer) => {
                        println!(
                            "[Verdict Consumer] Successfully subscribed to '{}' queue.",
                            queue_name
                        );
                        while let Some(delivery_res) = consumer.next().await {
                            match delivery_res {
                                Ok(delivery) => {
                                    match serde_json::from_slice::<ResponseFromWorker>(
                                        &delivery.data,
                                    ) {
                                        Ok(response) => {
                                            println!(
                                                "[Verdict Consumer] Processing submission #{} verdict: status={}, verdict={:?}",
                                                response.id, response.status, response.verdict
                                            );

                                            match SubmissionService::update_verdict(
                                                &state.db, response,
                                            )
                                            .await
                                            {
                                                Ok(updated) => {
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

                                                    let _ = state.submission_broadcast.send(event);

                                                    if let Some(cid) = updated.contest_id {
                                                        let _ = state.leaderboard_broadcast.send(
                                                            LeaderboardEvent {
                                                                contest_id: cid,
                                                                submission_id: updated.id,
                                                                user_id: updated.user_id,
                                                            },
                                                        );
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "[Verdict Consumer Error] Database update failed: {:?}",
                                                        e
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "[Verdict Consumer Error] Failed to deserialize verdict: {:?}",
                                                e
                                            );
                                        }
                                    }

                                    let _ = delivery.ack(BasicAckOptions::default()).await;
                                }
                                Err(e) => {
                                    eprintln!("[Verdict Consumer Error] Delivery error: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "[Verdict Consumer Error] Failed to register consumer on '{}': {:?}. Retrying in 5s...",
                            queue_name, e
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });
    }
}
