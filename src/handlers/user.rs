use axum::{
    Extension,
    http::StatusCode,
    Json,
};
use crate::{models::user, utils::error::AppError};

pub async fn get_me(
    Extension(user): Extension<user::Model>,
) -> Result<(StatusCode, Json<user::Model>), AppError> {
    Ok((StatusCode::OK, Json(user)))
}
