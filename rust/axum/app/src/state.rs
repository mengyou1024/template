//! Application state shared across request handlers.

use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    #[allow(dead_code)]
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: AppConfig) -> Self {
        Self {
            db,
            config: Arc::new(config),
        }
    }
}
