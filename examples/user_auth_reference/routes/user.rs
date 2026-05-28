use axum::{routing::get, Router};

use crate::{
    features::user::handler as user_handler,
    routes::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_handler::get_me).patch(user_handler::update_me))
        .route("/", get(user_handler::list_users))
        .route("/{id}", get(user_handler::get_user_by_id))
}
