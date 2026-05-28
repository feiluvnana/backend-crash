use axum::{routing::post, Router};
use crate::features::auth::handler as auth_handler;

use crate::routes::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
}
