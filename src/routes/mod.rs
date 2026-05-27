use axum::{
    extract::FromRef,
    middleware::from_fn_with_state,
    routing::get,
    routing::post,
    Router,
};
use sea_orm::DatabaseConnection;

use crate::{
    config::Config,
    handlers::{auth, user},
    middleware::auth::auth_middleware,
};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

pub fn create_router(state: AppState) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    let user_routes = Router::new()
        .route("/me", get(user::get_me))
        .route_layer(from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api/users", user_routes)
        .with_state(state)
}
