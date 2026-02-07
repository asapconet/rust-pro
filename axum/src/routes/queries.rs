use crate::handlers::{get_user::get_user, health_check::health_check, list_users::list_users};

use axum::{routing::get, Router};

pub fn user_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user))
}
