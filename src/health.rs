use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::{AppError, ErrorResponse};
use crate::routes::AppState;

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
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "SELECT 1".to_string(),
    ))
    .await
    .map_err(|e| AppError::ServiceUnavailable(format!("Database health check failed: {e}")))?;

    Ok(StatusCode::OK)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(health))
        .route("/ready", get(readiness))
}
