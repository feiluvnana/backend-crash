use anyhow::Ok;
use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/", get(hello_world))
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new())
                .on_response(DefaultOnResponse::new()),
        )
        .layer(tower_http::compression::CompressionLayer::new());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello_world() -> (StatusCode, Json<HelloWorldResponse>) {
    (
        StatusCode::OK,
        Json(HelloWorldResponse {
            data: "Hello, World!".to_owned(),
        }),
    )
}

#[derive(Serialize)]
struct HelloWorldResponse {
    data: String,
}
