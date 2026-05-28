use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // Sea-ORM migration CLI checks DATABASE_URL, construct it from parts if not explicitly defined
    if std::env::var("DATABASE_URL").is_err() {
        let user = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
        let password =
            std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "password".to_string());
        let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
        let db = std::env::var("POSTGRES_DB").unwrap_or_else(|_| "backend_db".to_string());
        let url = format!("postgres://{user}:{password}@{host}:{port}/{db}");
        unsafe {
            std::env::set_var("DATABASE_URL", url);
        }
    }
    cli::run_cli(migration::Migrator).await;
}
