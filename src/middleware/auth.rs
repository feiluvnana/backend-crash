use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    config::Config,
    models::user,
    utils::{error::AppError, jwt::verify_token},
};

pub async fn auth_middleware(
    State(db): State<DatabaseConnection>,
    State(config): State<Config>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Invalid authorization header format".to_string()))?;

    let claims = verify_token(token, &config.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    let user = user::Entity::find_by_id(claims.sub)
        .one(&db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::Unauthorized("User not found".to_string()))?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
