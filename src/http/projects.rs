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

pub async fn remote_project(extract::Json(data): extract::Json<RemoteProject>) -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().remote_project(data.id) }.await)
}

pub async fn get_all() -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().get_all_projects().await })
}

pub async fn one(extract::Path(id): extract::Path<Uuid>) -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().get_project(id).await })
}

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
