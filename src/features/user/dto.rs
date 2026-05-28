use crate::db::models::user;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<user::Model> for UserResponse {
    fn from(user: user::Model) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}
