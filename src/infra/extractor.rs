use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Query, Request},
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::infra::error::AppError;

/// Extractor that deserializes a JSON body and validates it.
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync + 'static,
{
    type Rejection = AppError;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(|e| AppError::BadRequest(e.to_string()))?;
            value.validate().map_err(AppError::from)?;
            Ok(ValidatedJson(value))
        }
    }
}

/// Extractor that deserializes query parameters and validates them.
pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync + 'static,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;
        value.validate().map_err(AppError::from)?;
        Ok(ValidatedQuery(value))
    }
}

/// Extractor that deserializes URL path parameters and validates them.
pub struct ValidatedPath<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedPath<T>
where
    T: DeserializeOwned + Validate + Send + 'static,
    S: Send + Sync + 'static,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(value) = Path::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;
        value.validate().map_err(AppError::from)?;
        Ok(ValidatedPath(value))
    }
}
