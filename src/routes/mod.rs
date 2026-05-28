use axum::{
    Router,
    extract::FromRef,
    middleware::from_fn,
    http::HeaderValue,
};
use sea_orm::DatabaseConnection;
use tower_http::set_header::SetResponseHeaderLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod swagger;
pub mod auth;
pub mod user;
pub mod health;

use crate::{
    infra::config::Config,
    middleware::request_id::request_id_middleware,
};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/auth", auth::router())
        .nest("/users", user::router(state.clone()))
        .nest("/health", health::router())
        .layer(from_fn(request_id_middleware))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::X_XSS_PROTECTION,
            HeaderValue::from_static("1; mode=block"),
        ));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", swagger::ApiDoc::openapi()))
        .nest("/api", api_routes)
        .with_state(state)
}
