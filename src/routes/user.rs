use axum::{routing::get, Router, middleware::from_fn_with_state};

use crate::{
    features::user::handler as user_handler,
    middleware::auth::auth_middleware,
    routes::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(user_handler::get_me).patch(user_handler::update_me))
        .route("/", get(user_handler::list_users))
        .route("/{id}", get(user_handler::get_user_by_id))
        .route_layer(from_fn_with_state(state, auth_middleware))
}
