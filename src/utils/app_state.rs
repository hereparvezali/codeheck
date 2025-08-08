use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, sync::Arc};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub secret: Arc<String>,
}

impl AppState {
    pub async fn new() -> AppState {
        let mut opt =
            ConnectOptions::new(env::var("DATABASE_URL").expect("==> Setup your DATABASE_URL"));
        opt.max_connections(2)
            .min_connections(0)
            .connect_timeout(std::time::Duration::from_secs(2))
            .idle_timeout(std::time::Duration::from_secs(30));
        AppState {
            db: Arc::new(
                Database::connect(opt)
                    .await
                    .expect("==> Database not working??"),
            ),
            secret: Arc::new(env::var("SECRET").expect("==> Setup your SECRET")),
        }
    }
}
