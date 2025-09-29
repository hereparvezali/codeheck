use std::{env, sync::Arc};

use lapin::{
    Channel, Connection, ConnectionProperties, options::QueueDeclareOptions, types::FieldTable,
};

pub async fn load() -> (Arc<String>, Channel) {
    dotenvy::dotenv().ok();
    let api = Arc::new(env::var("SUBMISSION_API").expect("==> API must set??"));
    let channel = Connection::connect(
        &env::var("RABBITMQ_URL").expect("==> Set RABBITMQ_URL"),
        ConnectionProperties::default(),
    )
    .await
    .expect("==> Rabbitmq not connecting??")
    .create_channel()
    .await
    .expect("Channel cant create??");

    channel
        .queue_declare(
            "submissions",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue cant declare??");
    (api, channel)
}
