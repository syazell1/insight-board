use api::{
    app_state::AppState,
    config::get_config,
    startup::Application,
    telemetry::{get_subscriber, initialize_subscriber},
    workers::api_monitoring::run_api_monitoring_worker,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to parse configurations");

    let subscriber = get_subscriber("server", "info", std::io::stdout);
    initialize_subscriber(subscriber);
    let app_state = AppState::build(&config).expect("Failed to build app state");

    let app = Application::build(app_state, config.clone())
        .await
        .expect("Failed to build application");

    let server = tokio::spawn(app.run_server_until_stopped());
    let api_worker = tokio::spawn(run_api_monitoring_worker(config));

    let _ = tokio::join!(server, api_worker);

    Ok(())
}
