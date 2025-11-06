use api::{
    config::get_config,
    startup::Application,
    telemetry::{get_subscriber, initialize_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to parse configurations");

    let subscriber = get_subscriber("server", "info", std::io::stdout);
    initialize_subscriber(subscriber);

    let app = Application::build(config)
        .await
        .expect("Failed to build application");

    let server = tokio::spawn(app.run_server_until_stopped());

    tokio::select! {
        _ = server => {}
    }

    Ok(())
}
