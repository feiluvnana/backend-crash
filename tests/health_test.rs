use crate::common::spawn_app;

mod common;

#[tokio::test]
async fn test_health_check() {
    let app = spawn_app().await;

    let response = app
        .client
        .get(&format!("{}/api/health", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body.get("status").unwrap().as_str().unwrap(), "ok");
}
