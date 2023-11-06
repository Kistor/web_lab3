mod employee;
mod projects;
mod utils;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/employee", employee::router())
        .nest("/projects", projects::router())
}
