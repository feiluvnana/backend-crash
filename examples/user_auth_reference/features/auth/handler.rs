use axum::{Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::{
    infra::{config::Config, error::AppError, error::ErrorResponse, extractor::ValidatedJson},
    features::auth::{
        dto::{AuthResponse, LoginRequest, LoginResponse, RegisterRequest},
        service::AuthService,
    },
    features::user::dto::UserResponse,
    utils::jwt::generate_token,
};

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 409, description = "Conflict", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn register(
    State(db): State<DatabaseConnection>,
    State(config): State<Config>,
    ValidatedJson(payload): ValidatedJson<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    // Check if user already exists by username
    if AuthService::find_by_username(&db, &payload.username)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    // Check if user already exists by email
    if AuthService::find_by_email(&db, &payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    let hashed = bcrypt::hash(&payload.password, config.bcrypt_cost)
        .map_err(|_| AppError::Internal("Failed to hash password".to_string()))?;

    let user = AuthService::create_user(&db, &payload, hashed).await?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            message: "User registered successfully".to_owned(),
            user: UserResponse::from(user),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = LoginResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn login(
    State(db): State<DatabaseConnection>,
    State(config): State<Config>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    let user = AuthService::find_by_username(&db, &payload.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = bcrypt::verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::Internal("Failed to verify password".to_string()))?;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = generate_token(user.id, &user.username, &config.jwt_secret, config.jwt_expiry_hours)
        .map_err(|_| AppError::Internal("Failed to generate token".to_string()))?;

    Ok((StatusCode::OK, Json(LoginResponse { token })))
}
