use axum::{extract, routing::*, Json, Router};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    entries::projects::{Project, ProjectData},
    postgress::PG,
};

pub fn router() -> Router {
    Router::new()
        .route("/new", post(new_project))
        .route("/remote", delete(remote_project))
        .route("/:id", get(one).patch(update))
        .route("/", get(get_all))
}

#[utoipa::path(
    post,
    path = "/projects/new",
    request_body = ProjectData
)]
pub async fn new_project(extract::Json(data): extract::Json<ProjectData>) -> Json<Value> {
    let project = Project {
        id: uuid::Uuid::new_v4(),
        data: data,
    };

    super::utils::response(unsafe { PG.clone().unwrap().try_create_project(project) }.await)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProject {
    id: uuid::Uuid,
}

#[utoipa::path(
    delete,
    path = "/projects/remote",
    request_body = RemoteProject
)]
pub async fn remote_project(extract::Json(data): extract::Json<RemoteProject>) -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().remote_project(data.id) }.await)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltrAllProgects {
    pub search_string: Option<String>,
}

#[utoipa::path(
    get,
    path = "/projects",
    responses(
            (status = 200, body = Vec<Project>),
        ),

)]
//
pub async fn get_all(extract::Json(filtr): extract::Json<FiltrAllProgects>) -> Json<Value> {
    super::utils::response(unsafe {
        PG.clone()
            .unwrap()
            .get_all_projects(filtr.search_string)
            .await
    })
}

#[utoipa::path(
    get,
    path = "/projects/:id",
    responses(
            (status = 200, body = Project),
        ),
)]
pub async fn one(extract::Path(id): extract::Path<Uuid>) -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().get_project(id).await })
}

#[utoipa::path(
    patch,
    path = "/projects/:id",
    request_body = ProjectData,
    responses(
            (status = 200, body = Project),
        ),

)]
pub async fn update(
    extract::Path(id): extract::Path<Uuid>,
    extract::Json(data): extract::Json<ProjectData>,
) -> Json<Value> {
    super::utils::response(unsafe {
        PG.clone()
            .unwrap()
            .try_update_project(Project { id, data })
            .await
    })
}
