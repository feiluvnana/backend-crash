use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use utoipa::ToSchema;

use crate::types::error::{AppError, ErrorResponse};
use crate::infra::routes::AppState;
use super::service::HealthService;

#[derive(Serialize, ToSchema)]
pub struct HealthStatus {
    pub status: &'static str,
    pub version: &'static str,
}

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is running", body = HealthStatus)
    )
)]
pub async fn health() -> (StatusCode, Json<HealthStatus>) {
    (
        StatusCode::OK,
        Json(HealthStatus {
            status: "ok",
            version: env!("CARGO_PKG_VERSION"),
        }),
    )
}

#[utoipa::path(
    get,
    path = "/api/health/ready",
    responses(
        (status = 200, description = "Service is ready"),
        (status = 503, description = "Service is unavailable", body = ErrorResponse)
    )
)]
pub async fn readiness(State(db): State<DatabaseConnection>) -> Result<StatusCode, AppError> {
    HealthService::check_readiness(&db).await?;
    Ok(StatusCode::OK)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(health))
        .route("/ready", get(readiness))
}
