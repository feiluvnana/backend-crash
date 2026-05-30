use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

pub struct HealthRepository;

impl HealthRepository {
    pub async fn check_db(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            "SELECT 1".to_string(),
        ))
        .await?;
        Ok(())
    }
}
