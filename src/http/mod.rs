mod employee;
mod utils;
use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/employee", employee::router())
}
