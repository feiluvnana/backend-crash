use crate::common::spawn_app;
use serde_json::json;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_register_and_login_success() {
    let app = spawn_app().await;

    let username = format!("user_{}", Uuid::new_v4().simple());
    let email = format!("{}@example.com", username);
    let password = "validpassword123";

    // 1. Register User
    let register_res = app
        .client
        .post(&format!("{}/api/auth/register", app.address))
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(register_res.status().as_u16(), 201);

    // 2. Login
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

    assert_eq!(login_res.status().as_u16(), 200);

    let body: serde_json::Value = login_res.json().await.unwrap();
    assert!(body.get("token").is_some());
}
