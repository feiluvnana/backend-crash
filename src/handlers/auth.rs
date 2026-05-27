use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    config::Config,
    models::user,
    utils::{
        crypto::{hash_password, verify_password},
        error::AppError,
        jwt::generate_token,
    },
};

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub user: user::Model,
}

pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    payload.validate().map_err(|err| AppError::BadRequest(err.to_string()))?;

    // Check if user or email already exists
    let existing_user = user::Entity::find()
        .filter(
            user::Column::Username
                .eq(&payload.username)
                .or(user::Column::Email.eq(&payload.email)),
        )
        .one(&db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if let Some(user) = existing_user {
        if user.username == payload.username {
            return Err(AppError::Conflict("Username already exists".to_string()));
        } else {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }
    }

    let password_hash = hash_password(&payload.password)
        .map_err(|_| AppError::Internal("Failed to hash password".to_string()))?;

    let new_user = user::ActiveModel {
        username: Set(payload.username),
        email: Set(payload.email),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    let user = new_user
        .insert(&db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            message: "User registered successfully".to_owned(),
            user,
        }),
    ))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(config): State<Config>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::Unauthorized("Invalid credentials".to_string()))?;

    if !verify_password(&payload.password, &user.password_hash) {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = generate_token(user.id, &user.username, &config.jwt_secret)
        .map_err(|_| AppError::Internal("Failed to generate token".to_string()))?;

    Ok((StatusCode::OK, Json(LoginResponse { token })))
}
