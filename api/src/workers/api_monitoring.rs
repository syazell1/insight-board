use std::{collections::HashSet, sync::Arc, time::Duration};

use reqwest::Client;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app_state::AppState, config::Settings, errors::AppError,
    features::api::models::ApiEndpointData, startup::create_db_pool,
};

const WORKER_SYNC_INTERVAL_SECS: u64 = 30;

pub async fn run_api_monitoring_worker(config: Settings, app_state: Arc<AppState>) {
    let pool = create_db_pool(&config.database).expect("Failed to connect to Database".into());

    execute_worker(pool, app_state).await;
}

async fn execute_worker(pool: PgPool, app_state: Arc<AppState>) {
    loop {
        match sync_monitoring_tasks(&pool, &app_state).await {
            Ok(_) => {
                tracing::debug!("Successfully synced monitoring tasks");
            }
            Err(e) => {
                tracing::error!("Error syncing monitoring tasks: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(WORKER_SYNC_INTERVAL_SECS)).await;
    }
}

async fn sync_monitoring_tasks(pool: &PgPool, app_state: &Arc<AppState>) -> Result<(), AppError> {
    let apis = sqlx::query_as!(
        ApiEndpointData,
        r#"
            SELECT id, name, url, interval_seconds, is_active, created_at
            FROM api_endpoints
            WHERE is_active = TRUE 
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut tasks = app_state.api_metrics_tasks.lock().await;

    // Get Ids of APIs that should be monitored
    let active_api_ids: HashSet<Uuid> = apis.iter().map(|api| api.id).collect();

    // Filter Ids that is not active within the tasks
    let tasks_to_remove: Vec<Uuid> = tasks
        .keys()
        .filter(|&id| !active_api_ids.contains(id))
        .copied()
        .collect();

    for api_id in tasks_to_remove {
        if let Some(handle) = tasks.remove(&api_id) {
            handle.abort();
            tracing::info!("Stopped monitoring task for API: {}", api_id);
        }
    }

    // Start tasks for new APIs
    for api in apis {
        if !tasks.contains_key(&api.id) {
            let handle =
                start_monitoring_task(api.id, api.url, api.interval_seconds.unwrap_or(60), pool);
            tasks.insert(api.id, handle);
            tracing::info!("Started monitoring task for API: {}", api.id);
        }
    }

    Ok(())
}

pub fn start_monitoring_task(
    api_id: Uuid,
    url: String,
    interval_seconds: i32,
    pool: &PgPool,
) -> tokio::task::JoinHandle<()> {
    let db = pool.clone();
    let client = Client::new();
    tokio::spawn(async move {
        loop {
            match check_api(&client, &url).await {
                Ok((status_code, latency)) => {
                    let status = status_code == 200;

                    add_api_metric_result(
                        api_id,
                        Some(status_code as i32),
                        Some(latency),
                        status,
                        None,
                        &db,
                    )
                    .await;

                    update_api_endpoint_status(api_id, status_code, latency, &db).await;
                }
                Err(e) => {
                    // Check if API still exists before continuing
                    if let Err(e) = check_api_by_id(api_id, &db).await {
                        tracing::warn!("API {} no longer exists or error occurred: {}", api_id, e);
                        break;
                    }

                    add_api_metric_result(api_id, None, None, false, Some(e.to_string()), &db)
                        .await;
                }
            }

            tokio::time::sleep(Duration::from_secs(interval_seconds as u64)).await;
        }
    })
}

pub async fn stop_monitoring_task(app_state: &Arc<AppState>, api_id: Uuid) {
    let mut tasks = app_state.api_metrics_tasks.lock().await;
    if let Some(handle) = tasks.remove(&api_id) {
        handle.abort();
        tracing::info!("Stopped monitoring task for API: {}", api_id);
    }
}

async fn check_api(client: &Client, url: &str) -> Result<(u16, i32), reqwest::Error> {
    let start = std::time::Instant::now();
    let resp = client.get(url).send().await?;
    let status = resp.status().as_u16();
    let latency = start.elapsed().as_millis() as i32;

    Ok((status, latency))
}

async fn check_api_by_id(api_id: Uuid, pool: &PgPool) -> Result<(), AppError> {
    let result = sqlx::query_scalar!(r#"SELECT id FROM api_endpoints WHERE id = $1"#, api_id)
        .fetch_optional(pool)
        .await?;

    match result {
        Some(_) => Ok(()),
        None => return Err(AppError::DataNotFoundError("API data was not found".into())),
    }
}

async fn update_api_endpoint_status(api_id: Uuid, status_code: u16, latency: i32, pool: &PgPool) {
    let _ = sqlx::query!(
        r#"
            UPDATE api_endpoints SET last_status_code = $1, last_latency_ms = $2
            WHERE id = $3
        "#,
        status_code as i32,
        latency,
        api_id
    )
    .execute(pool)
    .await;
}

async fn add_api_metric_result(
    api_id: Uuid,
    status_code: Option<i32>,
    latency: Option<i32>,
    is_success: bool,
    error_message: Option<String>,
    pool: &PgPool,
) {
    let id = Uuid::now_v7();
    let _ = sqlx::query!(
        r#"
            INSERT INTO api_metrics (id, api_id, status_code, latency_ms, is_success, error_message)
            VALUES
            ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        api_id,
        status_code,
        latency,
        is_success,
        error_message
    )
    .execute(pool)
    .await;
}
