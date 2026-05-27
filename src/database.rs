use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::time::Duration;
use migration::{Migrator, MigratorTrait};
use tracing::info;

pub async fn connect_db(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800));

    let db = Database::connect(opt).await?;
    info!("Database connection established");

    // Run migrations
    Migrator::up(&db, None).await?;
    info!("Database migrations run successfully");

    Ok(db)
}
