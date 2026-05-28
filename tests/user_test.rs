use crate::common::spawn_app;
use serde_json::json;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_get_me_unauthorized() {
    let app = spawn_app().await;

    let res = app
        .client
        .get(&format!("{}/api/users/me", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status().as_u16(), 401);
}

#[tokio::test]
async fn test_get_me_authorized() {
    let app = spawn_app().await;

    let username = format!("user_{}", Uuid::new_v4().simple());
    let email = format!("{}@example.com", username);
    let password = "validpassword123";

    // Register user
    app.client
        .post(&format!("{}/api/auth/register", app.address))
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .send()
        .await
        .unwrap();

    // Login
    let login_res = app
        .client
        .post(&format!("{}/api/auth/login", app.address))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await
        .unwrap();

    let login_body: serde_json::Value = login_res.json().await.unwrap();
    let token = login_body.get("token").unwrap().as_str().unwrap();

    // Get me
    let me_res = app
        .client
        .get(&format!("{}/api/users/me", app.address))
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    assert_eq!(me_res.status().as_u16(), 200);

    let me_body: serde_json::Value = me_res.json().await.unwrap();
    assert_eq!(me_body.get("username").unwrap().as_str().unwrap(), username);
    assert_eq!(me_body.get("role").unwrap().as_str().unwrap(), "user");
}
