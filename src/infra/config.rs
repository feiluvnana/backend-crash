use anyhow::{Context, anyhow};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
    pub bcrypt_cost: u32,
    pub cors_origin: String,
    pub host: String,
    pub port: u16,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
    pub postgres_host: String,
    pub postgres_port: u16,
}

impl Config {
    /// Initialize configuration from environment variables and .env file
    pub fn init() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let postgres_user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
        let postgres_password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "password".to_string());
        let postgres_db = env::var("POSTGRES_DB").unwrap_or_else(|_| "backend_db".to_string());
        let postgres_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let postgres_port = env::var("POSTGRES_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()
            .map_err(|e| anyhow!("POSTGRES_PORT must be a valid u16: {}", e))?;

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
        );

        let jwt_secret =
            env::var("JWT_SECRET").context("JWT_SECRET must be set in env or .env file")?;

        let jwt_expiry_hours = env::var("JWT_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<u64>()
            .map_err(|e| anyhow!("JWT_EXPIRY_HOURS must be a valid u64: {}", e))?;

        let bcrypt_cost = env::var("BCRYPT_COST")
            .unwrap_or_else(|_| "12".to_string())
            .parse::<u32>()
            .map_err(|e| anyhow!("BCRYPT_COST must be a valid u32: {}", e))?;

        let cors_origin = env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".to_string());
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| anyhow!("PORT must be a valid u16: {}", e))?;

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiry_hours,
            bcrypt_cost,
            cors_origin,
            host,
            port,
            postgres_user,
            postgres_password,
            postgres_db,
            postgres_host,
            postgres_port,
        })
    }
}
