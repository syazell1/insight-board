use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    routing::get,
    serve::Serve,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    app_state::AppState,
    config::{DatabaseSettings, Settings},
    features::{auth::routes::auth_routes, health_check::health_check},
};

pub struct Application {
    pub server: Serve<TcpListener, Router, Router>,
    pub port: u16,
    pub server_addr: String,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self> {
        let listener = create_tcp_listener(&config.application.host, config.application.port)
            .await
            .context("Failed to bind TCP listener")?;

        let port = listener
            .local_addr()
            .context("Failed to get local address")?
            .port();

        let app_state = create_app_state(&config)?;
        let app_routes = build_router(app_state, &config.application.client_url)?;

        let server_addr = format!("{}:{}", config.application.host, port);
        let server = axum::serve(listener, app_routes);

        Ok(Self {
            server,
            port,
            server_addr,
        })
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub async fn run_server_until_stopped(self) -> Result<()> {
        tracing::info!("Server started running at {}", self.server_addr);
        self.server
            .await
            .context("Failed to run server until stopped")?;
        Ok(())
    }
}

async fn create_tcp_listener(host: &str, port: u16) -> Result<TcpListener> {
    let address = format!("{}:{}", host, port);
    TcpListener::bind(&address)
        .await
        .with_context(|| format!("Failed to bind to address: {}", address))
}

fn create_app_state(config: &Settings) -> Result<Arc<AppState>> {
    let pool = create_db_pool(&config.database)?;
    Ok(Arc::new(AppState {
        pool,
        jwt_settings: config.jwt.clone(),
        client_url: config.application.client_url.clone(),
    }))
}

fn create_db_pool(config: &DatabaseSettings) -> Result<PgPool> {
    Ok(PgPool::connect_lazy_with(config.get_connection_options()))
}

fn build_router(app_state: Arc<AppState>, client_url: &str) -> Result<Router> {
    let cors_layer = create_cors_layer(client_url)?;
    let trace_layer = TraceLayer::new_for_http();

    Ok(get_app_routes(app_state)
        .layer(cors_layer)
        .layer(trace_layer))
}

fn create_cors_layer(client_url: &str) -> Result<CorsLayer> {
    let origin = client_url
        .parse::<HeaderValue>()
        .with_context(|| format!("Failed to parse client URL as HeaderValue: {}", client_url))?;

    Ok(CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_origin(origin))
}

fn get_app_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/health_check", get(health_check))
                .nest("/auth", auth_routes()),
        )
        .with_state(app_state)
}
