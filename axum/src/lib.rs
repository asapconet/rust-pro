pub mod errors;
pub mod handlers;
pub mod hell_o;
pub mod routes;

use axum::Router;

pub fn create_app() -> Router {
    routes::router()
}
