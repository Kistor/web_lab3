mod employee;
mod projects;
mod utils;
use axum::{
    http::{self, Method},
    Router,
};

use tower_http::cors::{Any, CorsLayer};
use utoipa_swagger_ui::SwaggerUi;

use crate::entries::employee::*;
use crate::entries::projects::*;

use utoipa::OpenApi;

use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};

#[derive(OpenApi)]
#[openapi(
    paths(
        self::employee::new_employee,
        self::employee::remote_employee,
        self::employee::get_all,
        self::employee::one,
        self::employee::update,


        self::projects::new_project,
        self::projects::remote_project,
        self::projects::get_all,
        self::projects::one,
        self::projects::update
    ),
    components(
        schemas(EmployeeData),
        schemas(Employee),
        schemas(ProjectData),
        schemas(Project),

    ),
    tags(
        (name = "empl", description = "try 1")
    )
)]
struct ApiDoc;

pub fn router() -> Router {
    use http::header::CONTENT_TYPE;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/employee", employee::router())
        .nest("/projects", projects::router())
        .layer(cors)
}
