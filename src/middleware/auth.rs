use axum::{
    extract::{FromRequestParts, Request, State, FromRef},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use sea_orm::DatabaseConnection;

use crate::{
    infra::{config::Config, error::AppError},
    db::models::user,
    features::auth::service::AuthService,
    utils::jwt::verify_token,
};

pub struct CurrentUser(pub user::Model);

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync + 'static,
    DatabaseConnection: axum::extract::FromRef<S>,
    Config: axum::extract::FromRef<S>,
{
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let db = DatabaseConnection::from_ref(state);
        let config = Config::from_ref(state);

        // Extract header value manually before entering async block to satisfy lifetime requirements
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok().map(|s| s.to_string()));

        async move {
            let auth_header = auth_header
                .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or_else(|| AppError::Unauthorized("Invalid authorization header format".to_string()))?;

            let claims = verify_token(token, &config.jwt_secret)
                .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

            let user = AuthService::find_by_id(&db, claims.sub)
                .await?
                .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

            Ok(CurrentUser(user))
        }
    }
}

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
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization header format".to_string()))?;

    let claims = verify_token(token, &config.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    let user = AuthService::find_by_id(&db, claims.sub)
        .await?
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
