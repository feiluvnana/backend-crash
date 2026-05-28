use crate::features::user::dto::UserResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct AuthResponse {
    pub message: String,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct LoginResponse {
    pub token: String,
}
