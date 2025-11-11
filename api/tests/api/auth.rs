use crate::helpers::spawn_app;
use cookie::Cookie;
use serde_json::json;

#[tokio::test]
async fn register_returns_200_for_valid_data() {
    let app = spawn_app().await;

    let body = json!({
        "username": "testuser",
        "password": "testpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // Extract headers before consuming the response body
    let cookies: Vec<_> = response.headers().get_all("set-cookie").iter().collect();
    assert!(!cookies.is_empty());
    let cookie_str = cookies[0].to_str().unwrap();
    assert!(cookie_str.contains("rt="));
    assert!(cookie_str.contains("HttpOnly"));

    let json: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(json.get("id").is_some());
    assert!(json.get("access_token").is_some());
}

#[tokio::test]
async fn register_returns_400_for_empty_username() {
    let app = spawn_app().await;

    let body = json!({
        "username": "",
        "password": "testpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn register_returns_400_for_empty_password() {
    let app = spawn_app().await;

    let body = json!({
        "username": "testuser",
        "password": ""
    });

    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn register_returns_400_for_username_exceeding_max_length() {
    let app = spawn_app().await;

    let body = json!({
        "username": "a".repeat(13),
        "password": "testpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn register_returns_400_for_password_exceeding_max_length() {
    let app = spawn_app().await;

    let body = json!({
        "username": "testuser",
        "password": "a".repeat(13)
    });

    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn register_returns_400_for_username_with_special_chars() {
    let app = spawn_app().await;

    let body = json!({
        "username": "test!user",
        "password": "testpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn register_returns_400_for_duplicate_username() {
    let app = spawn_app().await;

    let body = json!({
        "username": "duplicate",
        "password": "testpass"
    });

    // First registration should succeed
    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // Second registration with same username should fail
    let response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn login_returns_200_for_valid_credentials() {
    let app = spawn_app().await;

    // First register a user
    let register_body = json!({
        "username": "loginuser",
        "password": "loginpass"
    });

    let _ = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&register_body)
        .send()
        .await
        .expect("Failed to register user.");

    // Then login
    let login_body = json!({
        "username": "loginuser",
        "password": "loginpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/login", app.address))
        .json(&login_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // Extract headers before consuming the response body
    let cookies: Vec<_> = response.headers().get_all("set-cookie").iter().collect();
    assert!(!cookies.is_empty());

    let json: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(json.get("id").is_some());
    assert!(json.get("access_token").is_some());
}

#[tokio::test]
async fn login_returns_401_for_invalid_username() {
    let app = spawn_app().await;

    // Register a user
    let register_body = json!({
        "username": "validuser",
        "password": "validpass"
    });

    let _ = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&register_body)
        .send()
        .await
        .expect("Failed to register user.");

    // Try to login with wrong username
    let login_body = json!({
        "username": "wronguser",
        "password": "validpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/login", app.address))
        .json(&login_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn login_returns_401_for_invalid_password() {
    let app = spawn_app().await;

    // Register a user
    let register_body = json!({
        "username": "validuser2",
        "password": "validpass"
    });

    let _ = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&register_body)
        .send()
        .await
        .expect("Failed to register user.");

    // Try to login with wrong password
    let login_body = json!({
        "username": "validuser2",
        "password": "wrongpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/login", app.address))
        .json(&login_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn login_returns_400_for_empty_username() {
    let app = spawn_app().await;

    let body = json!({
        "username": "",
        "password": "testpass"
    });

    let response = app
        .client
        .post(&format!("{}/auth/login", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn login_returns_400_for_empty_password() {
    let app = spawn_app().await;

    let body = json!({
        "username": "testuser",
        "password": ""
    });

    let response = app
        .client
        .post(&format!("{}/auth/login", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn get_current_user_returns_200_with_valid_token() {
    let app = spawn_app().await;

    // Register and get token
    let register_body = json!({
        "username": "currentuser",
        "password": "currentpass"
    });

    let register_response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&register_body)
        .send()
        .await
        .expect("Failed to register user.");

    let register_json: serde_json::Value = register_response
        .json()
        .await
        .expect("Failed to parse register response");
    let access_token = register_json
        .get("access_token")
        .expect("No access token in response")
        .as_str()
        .expect("Access token is not a string");

    // Get current user
    let response = app
        .client
        .get(&format!("{}/auth/current-user", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let json: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(json.get("id").is_some());
    assert_eq!(
        json.get("username").unwrap().as_str().unwrap(),
        "currentuser"
    );
}

#[tokio::test]
async fn get_current_user_returns_401_without_token() {
    let app = spawn_app().await;

    let response = app
        .client
        .get(&format!("{}/auth/current-user", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn get_current_user_returns_401_with_invalid_token() {
    let app = spawn_app().await;

    let response = app
        .client
        .get(&format!("{}/auth/current-user", app.address))
        .header("Authorization", "Bearer invalid_token_here")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}


#[tokio::test]
async fn refresh_token_returns_200_with_valid_refresh_token() {
    let app = spawn_app().await;

    // Register to get tokens
    let register_body = json!({
        "username": "refreshuser",
        "password": "refreshpass"
    });

    let register_response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&register_body)
        .send()
        .await
        .expect("Failed to register user.");

    // Extract refresh token from cookie
    let cookies: Vec<_> = register_response
        .headers()
        .get_all("set-cookie")
        .iter()
        .collect();
    assert!(!cookies.is_empty());
    let cookie_str = cookies[0].to_str().unwrap();
    let parsed = Cookie::parse(cookie_str).unwrap();
    let refresh_token = parsed.value();

    println!("{}", refresh_token);

    // Use refresh token to get new tokens
    let response = app
        .client
        .get(&format!("{}/auth/refresh", app.address))
        .header("Cookie", format!("rt={}", refresh_token))
        .send()
        .await
        .expect("Failed to execute request.");

    let status = response.status().as_u16();
    assert_eq!(200, status);

    // Extract headers before consuming the response body
    let new_cookies: Vec<_> = response.headers().get_all("set-cookie").iter().collect();
    assert!(!new_cookies.is_empty());

    let json: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(json.get("id").is_some());
    assert!(json.get("access_token").is_some());
}

#[tokio::test]
async fn refresh_token_returns_401_without_cookie() {
    let app = spawn_app().await;

    let response = app
        .client
        .get(&format!("{}/auth/refresh", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn refresh_token_returns_401_with_invalid_token() {
    let app = spawn_app().await;

    let response = app
        .client
        .get(&format!("{}/auth/refresh", app.address))
        .header("Cookie", "rt=invalid_refresh_token")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn refresh_token_prevents_reuse() {
    let app = spawn_app().await;

    // Register to get tokens
    let register_body = json!({
        "username": "reuseuser",
        "password": "reusepass"
    });

    let register_response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&register_body)
        .send()
        .await
        .expect("Failed to register user.");

    // Extract refresh token from cookie
    let cookies: Vec<_> = register_response
        .headers()
        .get_all("set-cookie")
        .iter()
        .collect();
    assert!(!cookies.is_empty());
    let cookie_str = cookies[0].to_str().unwrap();

    // First refresh should succeed
    let response1 = app
        .client
        .get(&format!("{}/auth/refresh", app.address))
        .header("Cookie", cookie_str)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response1.status().as_u16());

    // Try to reuse the old refresh token - should fail
    let response2 = app
        .client
        .get(&format!("{}/auth/refresh", app.address))
        .header("Cookie", cookie_str)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response2.status().as_u16());
}

#[tokio::test]
async fn login_cleans_up_old_refresh_tokens() {
    let app = spawn_app().await;

    // Register a user
    let register_body = json!({
        "username": "cleanupuser",
        "password": "cleanuppass"
    });

    let register_response = app
        .client
        .post(&format!("{}/auth/register", app.address))
        .json(&register_body)
        .send()
        .await
        .expect("Failed to register user.");

    // Get refresh token from first registration
    let cookies1: Vec<_> = register_response
        .headers()
        .get_all("set-cookie")
        .iter()
        .collect();
    let cookie_str1 = cookies1[0].to_str().unwrap();

    // Login again - this should clean up old tokens
    let login_body = json!({
        "username": "cleanupuser",
        "password": "cleanuppass"
    });

    let login_response = app
        .client
        .post(&format!("{}/auth/login", app.address))
        .header("Cookie", cookie_str1)
        .json(&login_body)
        .send()
        .await
        .expect("Failed to login.");

    assert_eq!(200, login_response.status().as_u16());

    // Get new refresh token
    let cookies2: Vec<_> = login_response
        .headers()
        .get_all("set-cookie")
        .iter()
        .collect();
    let cookie_str2 = cookies2[0].to_str().unwrap();

    // Old token should no longer work
    let response = app
        .client
        .get(&format!("{}/auth/refresh", app.address))
        .header("Cookie", cookie_str1)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());

    // New token should work
    let response = app
        .client
        .get(&format!("{}/auth/refresh", app.address))
        .header("Cookie", cookie_str2)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}
