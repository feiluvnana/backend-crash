use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::auth::handler::register,
        crate::features::auth::handler::login,
        crate::features::user::handler::get_me,
        crate::features::user::handler::update_me,
        crate::features::user::handler::list_users,
        crate::features::user::handler::get_user_by_id,
        crate::features::health::handler::health,
        crate::features::health::handler::readiness,
    ),
    components(
        schemas(
            crate::features::auth::dto::RegisterRequest,
            crate::features::auth::dto::AuthResponse,
            crate::features::auth::dto::LoginRequest,
            crate::features::auth::dto::LoginResponse,
            crate::features::user::dto::UserResponse,
            crate::features::user::dto::UpdateUserRequest,
            crate::features::health::handler::HealthStatus,
            crate::infra::error::ErrorResponse,
            crate::infra::error::FieldError,
            crate::infra::pagination::PageMeta,
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
