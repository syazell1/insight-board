use crate::helpers::spawn_app;
use serde_json::json;

/// Helper function to register a user and get an access token
async fn get_access_token(app: &crate::helpers::TestApp, username: &str, password: &str) -> String {
    let register_body = json!({
        "username": username,
        "password": password
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

    register_json
        .get("access_token")
        .expect("No access token in response")
        .as_str()
        .expect("Access token is not a string")
        .to_string()
}

// GET /api/api/ tests

#[tokio::test]
async fn get_all_apis_returns_200_with_valid_token() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "getuser", "getpass").await;

    // Add an API endpoint first
    let add_body = json!({
        "name": "Test API",
        "url": "https://example.com/api",
        "interval_secs": 60,
        "is_active": true
    });

    let _ = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&add_body)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    // Get all APIs
    let response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let json: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(json.is_array());
    assert_eq!(json.as_array().unwrap().len(), 1);

    let api = &json.as_array().unwrap()[0];
    assert_eq!(api.get("name").unwrap().as_str().unwrap(), "Test API");
    assert_eq!(api.get("url").unwrap().as_str().unwrap(), "https://example.com/api");
}

#[tokio::test]
async fn get_all_apis_returns_empty_array_when_no_apis_exist() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "emptyuser", "emptypass").await;

    let response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let json: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(json.is_array());
    assert_eq!(json.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn get_all_apis_returns_401_without_token() {
    let app = spawn_app().await;

    let response = app
        .client
        .get(&format!("{}/api", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn get_all_apis_returns_only_user_own_apis() {
    let app = spawn_app().await;
    let access_token1 = get_access_token(&app, "user1", "pass1").await;
    let access_token2 = get_access_token(&app, "user2", "pass2").await;

    // User 1 adds an API
    let add_body1 = json!({
        "name": "User 1 API",
        "url": "https://user1.example.com/api",
        "interval_secs": 60,
        "is_active": true
    });

    let _ = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token1))
        .json(&add_body1)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    // User 2 adds an API
    let add_body2 = json!({
        "name": "User 2 API",
        "url": "https://user2.example.com/api",
        "interval_secs": 120,
        "is_active": false
    });

    let _ = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token2))
        .json(&add_body2)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    // User 1 should only see their own API
    let response1 = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token1))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response1.status().as_u16());
    let json1: serde_json::Value = response1.json().await.expect("Failed to parse response");
    assert_eq!(json1.as_array().unwrap().len(), 1);
    assert_eq!(json1.as_array().unwrap()[0].get("name").unwrap().as_str().unwrap(), "User 1 API");

    // User 2 should only see their own API
    let response2 = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token2))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response2.status().as_u16());
    let json2: serde_json::Value = response2.json().await.expect("Failed to parse response");
    assert_eq!(json2.as_array().unwrap().len(), 1);
    assert_eq!(json2.as_array().unwrap()[0].get("name").unwrap().as_str().unwrap(), "User 2 API");
}

// POST /api/api/ tests

#[tokio::test]
async fn add_api_returns_201_for_valid_data() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "adduser", "addpass").await;

    let body = json!({
        "name": "New API",
        "url": "https://api.example.com",
        "interval_secs": 30,
        "is_active": true
    });

    let response = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(201, response.status().as_u16());
}


// TODO: Add optional fields test
// #[tokio::test]
// async fn add_api_returns_201_with_optional_fields() {
//     let app = spawn_app().await;
//     let access_token = get_access_token(&app, "optionaluser", "optionalpass").await;

//     let body = json!({
//         "name": "Minimal API",
//         "url": "https://minimal.example.com",
//     });

//     let response = app
//         .client
//         .post(&format!("{}/api", app.address))
//         .header("Authorization", format!("Bearer {}", access_token))
//         .json(&body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     assert_eq!(201, response.status().as_u16());
// }

#[tokio::test]
async fn add_api_returns_401_without_token() {
    let app = spawn_app().await;

    let body = json!({
        "name": "Unauthorized API",
        "url": "https://unauthorized.example.com"
    });

    let response = app
        .client
        .post(&format!("{}/api", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn add_api_creates_api_that_appears_in_get_all() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "createuser", "createpass").await;

    let add_body = json!({
        "name": "Created API",
        "url": "https://created.example.com",
        "interval_secs": 45,
        "is_active": false
    });

    let add_response = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&add_body)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    assert_eq!(201, add_response.status().as_u16());

    // Verify it appears in get all
    let get_response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, get_response.status().as_u16());
    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    assert_eq!(json.as_array().unwrap().len(), 1);

    let api = &json.as_array().unwrap()[0];
    assert_eq!(api.get("name").unwrap().as_str().unwrap(), "Created API");
    assert_eq!(api.get("url").unwrap().as_str().unwrap(), "https://created.example.com");
    assert_eq!(api.get("interval_seconds").unwrap().as_i64().unwrap(), 45);
    assert_eq!(api.get("is_active").unwrap().as_bool().unwrap(), false);
}

// PATCH /api/api/{id} tests

#[tokio::test]
async fn update_api_returns_200_for_valid_data() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "updateuser", "updatepass").await;

    // First, add an API
    let add_body = json!({
        "name": "Original API",
        "url": "https://original.example.com",
        "interval_secs": 60,
        "is_active": true
    });

    let add_response = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&add_body)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    assert_eq!(201, add_response.status().as_u16());

    // Get the API ID
    let get_response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to get APIs.");

    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    let api_id = json.as_array().unwrap()[0].get("id").unwrap().as_str().unwrap();

    // Update the API
    let update_body = json!({
        "name": "Updated API",
        "url": "https://updated.example.com",
        "interval_secs": 120,
        "is_active": false
    });

    let update_response = app
        .client
        .patch(&format!("{}/api/{}", app.address, api_id))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&update_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, update_response.status().as_u16());

    // Verify the update
    let get_response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to get APIs.");

    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    let api = &json.as_array().unwrap()[0];
    assert_eq!(api.get("name").unwrap().as_str().unwrap(), "Updated API");
    assert_eq!(api.get("url").unwrap().as_str().unwrap(), "https://updated.example.com");
    assert_eq!(api.get("interval_seconds").unwrap().as_i64().unwrap(), 120);
    assert_eq!(api.get("is_active").unwrap().as_bool().unwrap(), false);
}

#[tokio::test]
async fn update_api_returns_400_for_invalid_uuid() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "invaliduser", "invalidpass").await;

    let update_body = json!({
        "name": "Test API",
        "url": "https://test.example.com"
    });

    let response = app
        .client
        .patch(&format!("{}/api/api/invalid-uuid", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&update_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn update_api_returns_404_for_nonexistent_api() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "notfounduser", "notfoundpass").await;

    // Generate a valid UUID that doesn't exist
    let fake_uuid = "01234567-89ab-cdef-0123-456789abcdef";

    let update_body = json!({
        "name": "Test API",
        "url": "https://test.example.com"
    });

    let response = app
        .client
        .patch(&format!("{}/api/{}", app.address, fake_uuid))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&update_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn update_api_returns_401_without_token() {
    let app = spawn_app().await;

    let update_body = json!({
        "name": "Test API",
        "url": "https://test.example.com"
    });

    let response = app
        .client
        .patch(&format!("{}/api/01234567-89ab-cdef-0123-456789abcdef", app.address))
        .json(&update_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn update_api_only_updates_own_apis() {
    let app = spawn_app().await;
    let access_token1 = get_access_token(&app, "owner1", "pass1").await;
    let access_token2 = get_access_token(&app, "owner2", "pass2").await;

    // User 1 adds an API
    let add_body = json!({
        "name": "User 1 API",
        "url": "https://user1.example.com",
        "interval_secs": 60,
        "is_active": true
    });

    let _ = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token1))
        .json(&add_body)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    // Get the API ID
    let get_response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token1))
        .send()
        .await
        .expect("Failed to get APIs.");

    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    let api_id = json.as_array().unwrap()[0].get("id").unwrap().as_str().unwrap();

    // User 2 tries to update User 1's API - should fail (404 or 403)
    let update_body = json!({
        "name": "Hacked API",
        "url": "https://hacked.example.com"
    });

    let response = app
        .client
        .patch(&format!("{}/api/{}", app.address, api_id))
        .header("Authorization", format!("Bearer {}", access_token2))
        .json(&update_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Should return 404 because the API doesn't exist for user 2
    assert_eq!(404, response.status().as_u16());
}

// DELETE /api/api/{id} tests

#[tokio::test]
async fn delete_api_returns_200_for_valid_id() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "deleteuser", "deletepass").await;

    // First, add an API
    let add_body = json!({
        "name": "To Delete API",
        "url": "https://delete.example.com",
        "interval_secs": 60,
        "is_active": true
    });

    let _ = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&add_body)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    // Get the API ID
    let get_response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to get APIs.");

    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    let api_id = json.as_array().unwrap()[0].get("id").unwrap().as_str().unwrap();

    // Delete the API
    let delete_response = app
        .client
        .delete(&format!("{}/api/{}", app.address, api_id))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, delete_response.status().as_u16());

    // Verify it's deleted
    let get_response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to get APIs.");

    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    assert_eq!(json.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn delete_api_returns_400_for_invalid_uuid() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "invaliduser2", "invalidpass2").await;

    let response = app
        .client
        .delete(&format!("{}/api/invalid-uuid", app.address))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn delete_api_returns_404_for_nonexistent_api() {
    let app = spawn_app().await;
    let access_token = get_access_token(&app, "notfounduser2", "notfoundpass2").await;

    // Generate a valid UUID that doesn't exist
    let fake_uuid = "01234567-89ab-cdef-0123-456789abcdef";

    let response = app
        .client
        .delete(&format!("{}/api/{}", app.address, fake_uuid))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn delete_api_returns_401_without_token() {
    let app = spawn_app().await;

    let response = app
        .client
        .delete(&format!("{}/api/01234567-89ab-cdef-0123-456789abcdef", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn delete_api_only_deletes_own_apis() {
    let app = spawn_app().await;
    let access_token1 = get_access_token(&app, "owner3", "pass3").await;
    let access_token2 = get_access_token(&app, "owner4", "pass4").await;

    // User 1 adds an API
    let add_body = json!({
        "name": "User 1 API",
        "url": "https://user1.example.com",
        "interval_secs": 60,
        "is_active": true
    });

    let _ = app
        .client
        .post(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token1))
        .json(&add_body)
        .send()
        .await
        .expect("Failed to add API endpoint.");

    // Get the API ID
    let get_response = app
        .client
        .get(&format!("{}/api", app.address))
        .header("Authorization", format!("Bearer {}", access_token1))
        .send()
        .await
        .expect("Failed to get APIs.");

    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    let api_id = json.as_array().unwrap()[0].get("id").unwrap().as_str().unwrap();

    // User 2 tries to delete User 1's API - should fail (404)
    let response = app
        .client
        .delete(&format!("{}/api/{}", app.address, api_id))
        .header("Authorization", format!("Bearer {}", access_token2))
        .send()
        .await
        .expect("Failed to execute request.");

    // Should return 404 because the API doesn't exist for user 2
    assert_eq!(404, response.status().as_u16());

    // Verify User 1's API still exists
    let get_response = app
        .client
        .get(&format!("{}/api/", app.address))
        .header("Authorization", format!("Bearer {}", access_token1))
        .send()
        .await
        .expect("Failed to get APIs.");

    let json: serde_json::Value = get_response.json().await.expect("Failed to parse response");
    assert_eq!(json.as_array().unwrap().len(), 1);
}

