use std::sync::LazyLock;

use api::{
    config::{DatabaseSettings, get_config},
    startup::Application,
    telemetry::{get_subscriber, initialize_subscriber},
};
use sqlx::{Executor, PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let app_name = "test";
    let log_level = "info";

    if std::env::var("TEST_LOG").is_ok() {
        let sub = get_subscriber(app_name, log_level, std::io::stdout);
        initialize_subscriber(sub);
    } else {
        let sub = get_subscriber(app_name, log_level, std::io::sink);
        initialize_subscriber(sub);
    }
});

pub struct TestApp {
    pub address: String,
    pub client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);
    let client = reqwest::Client::new();

    let c = {
        let mut config = get_config().expect("Failed to parse configurations.");
        config.application.port = 0;
        config.database.database_name = Uuid::now_v7().to_string();

        config
    };

    let _ = configure_db(&c.database).await;
    let app = Application::build(c)
        .await
        .expect("Failed to build application");
    let address = format!("http://localhost:{}/api", app.get_port());
    let _ = tokio::spawn(app.run_server_until_stopped());

    TestApp { address, client }
}

async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let pool = PgPoolOptions::new().connect_lazy_with(config.get_connection_options_without_db());

    pool.execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    let pool = PgPoolOptions::new().connect_lazy_with(config.get_connection_options_with_db());

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations.");
    pool
}

