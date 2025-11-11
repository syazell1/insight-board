use crate::helpers::spawn_app;

#[tokio::test]
pub async fn health_check_should_return_200() {
    let app = spawn_app().await;

    let res = app
        .client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to send health_check request.");

    let status = res.status().as_u16();

    assert_eq!(status, 200);
}