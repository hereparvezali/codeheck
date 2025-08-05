use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::Arc};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub secret: Arc<String>,
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            db: Arc::new(
                Database::connect(env::var("DATABASE_URL").expect("==> Setup your DATABASE_URL"))
                    .await
                    .expect("==> Database not working??"),
            ),
            secret: Arc::new(env::var("SECRET").expect("==> Setup your SECRET")),
        }
    }
}
