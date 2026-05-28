use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // User ID
    pub username: String,
    pub exp: i64, // Expiration timestamp
    pub iat: i64, // Issued at timestamp
}

pub fn generate_token(
    user_id: i32,
    username: &str,
    secret: &str,
    expiry_hours: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = now + Duration::try_hours(expiry_hours as i64).expect("valid hours");
    let claims = Claims {
        sub: user_id,
        username: username.to_owned(),
        exp: expire.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}
