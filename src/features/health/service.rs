use sea_orm::DatabaseConnection;
use crate::types::error::AppError;
use super::repository::HealthRepository;

pub struct HealthService;

impl HealthService {
    pub async fn check_readiness(db: &DatabaseConnection) -> Result<(), AppError> {
        HealthRepository::check_db(db)
            .await
            .map_err(|e| AppError::ServiceUnavailable(format!("Database health check failed: {e}")))?;
        Ok(())
    }
}
