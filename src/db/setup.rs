use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::info;

pub async fn connect_db(
    database_url: &str,
    max_conn: u32,
    min_conn: u32,
) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(max_conn)
        .min_connections(min_conn)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800));

    let db = Database::connect(opt).await?;
    info!("Database connection established");

    Ok(db)
}
