use rust_backend_boilerplate::{
    db::setup::connect_db,
    infra::config::Config,
    routes::{AppState, create_router},
};

pub struct TestApp {
    pub address: String,
    pub client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    let config = Config::init().expect("Failed to initialize test config");
    let db = connect_db(&config.database_url, 10, 2)
        .await
        .expect("Failed to connect to DB");
    let state = AppState {
        db: db.clone(),
        config,
    };
    let router = create_router(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    TestApp { address, client }
}
