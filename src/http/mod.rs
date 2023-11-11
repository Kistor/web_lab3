mod employee;
mod projects;
mod utils;
use axum::{
    http::{self, Method},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub fn router() -> Router {
    use http::header::CONTENT_TYPE;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .nest("/employee", employee::router())
        .nest("/projects", projects::router())
        .layer(cors)
}
