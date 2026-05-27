use anyhow::Ok;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use config::Config;
use database::connect_db;
use routes::{create_router, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::init();

    // Connect to Database & Run Migrations
    let db = connect_db(&config.database_url).await?;

    // Create AppState
    let state = AppState {
        db,
        config: config.clone(),
    };

    // Setup routes
    let app = create_router(state)
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new())
                .on_response(DefaultOnResponse::new()),
        )
        .layer(tower_http::compression::CompressionLayer::new());

    // Bind and serve
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
