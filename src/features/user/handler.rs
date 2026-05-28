use crate::{
    infra::{
        error::{AppError, ErrorResponse},
        extractor::ValidatedJson,
        pagination::{PaginationParams, PaginatedResponse},
    },
    features::user::{
        dto::{UserResponse, UpdateUserRequest},
        service::UserService,
    },
    middleware::auth::CurrentUser,
};
use axum::{extract::{Query, State}, Json, http::StatusCode};
use sea_orm::DatabaseConnection;

#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "Current user retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_me(
    CurrentUser(user): CurrentUser,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

#[utoipa::path(
    patch,
    path = "/api/users/me",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserResponse),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_me(
    State(db): State<DatabaseConnection>,
    CurrentUser(user): CurrentUser,
    ValidatedJson(payload): ValidatedJson<UpdateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let updated = UserService::update_user(&db, user.id, payload).await?;
    Ok((StatusCode::OK, Json(UserResponse::from(updated))))
}

#[utoipa::path(
    get,
    path = "/api/users",
    params(PaginationParams),
    responses(
        (status = 200, description = "Users retrieved successfully", body = PaginatedResponse<UserResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_users(
    State(db): State<DatabaseConnection>,
    CurrentUser(_user): CurrentUser,
    Query(page_params): Query<PaginationParams>,
) -> Result<(StatusCode, Json<PaginatedResponse<UserResponse>>), AppError> {
    let page = page_params.page();
    let per_page = page_params.per_page();

    let (users, total) = UserService::list_users(&db, page, per_page).await?;
    let data = users.into_iter().map(UserResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::new(data, page, per_page, total)),
    ))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User retrieved successfully", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_by_id(
    State(db): State<DatabaseConnection>,
    CurrentUser(_user): CurrentUser,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = UserService::find_by_id(&db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}
