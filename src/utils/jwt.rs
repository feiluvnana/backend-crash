use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,        // User ID
    pub username: String,
    pub exp: i64,        // Expiration timestamp
    pub iat: i64,        // Issued at timestamp
}

pub fn generate_token(user_id: i32, username: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    // Use chrono::Duration::try_hours or duration operations directly. Let's use Duration::try_hours.unwrap() or simply duration math.
    // Duration::try_hours(24) returns Option. Let's use try_hours.
    let expire = now + Duration::try_hours(24).expect("valid hours");
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
