use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use uuid::Uuid;

use sqlx::PgPool;

use crate::config::{JwtSettings, Settings};
use crate::startup::create_db_pool;

pub struct AppState {
    pub pool: PgPool,
    pub jwt_settings: JwtSettings,
    pub client_url: String,
    pub api_metrics_tasks: Mutex<HashMap<Uuid, JoinHandle<()>>>,
}

impl AppState {
    pub fn build(config: &Settings) -> Result<Arc<Self>> {
        let pool = create_db_pool(&config.database)?;
        let api_metrics_tasks = Mutex::new(HashMap::new());

        Ok(Arc::new(AppState {
            pool,
            jwt_settings: config.jwt.clone(),
            client_url: config.application.client_url.clone(),
            api_metrics_tasks,
        }))
    }
}
