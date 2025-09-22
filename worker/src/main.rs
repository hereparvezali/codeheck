pub mod config;
pub mod handle;
pub mod languages;
pub mod run;
pub mod types;

use crate::handle::handle_delivery;
use futures_util::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable};
use std::{collections::VecDeque, env, sync::Arc};
use tokio::sync::{Mutex, Semaphore};

#[tokio::main]
async fn main() {
    let (api, channel) = config::load().await;
    let mut consumer = channel
        .basic_consume(
            "submissions",
            &env::var("HOSTNAME").expect("==> HOSTNAME not set??"),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Setup consumer failed??");

    let semaphore = Arc::new(Semaphore::new(num_cpus::get()));
    let core_pool = Arc::new(Mutex::new(
        (0..num_cpus::get()).collect::<VecDeque<usize>>(),
    ));

    while let Some(Ok(delivery)) = consumer.next().await {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let core_pool = core_pool.clone();
        tokio::spawn(handle_delivery(delivery, api.clone(), permit, core_pool));
    }
}
