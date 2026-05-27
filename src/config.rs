use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        dotenvy::dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in env or .env file");
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set in env or .env file");
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid u16");

        Config {
            database_url,
            jwt_secret,
            host,
            port,
        }
    }
}
