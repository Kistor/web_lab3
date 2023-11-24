use axum::{
    extract,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    entries::employee::{Employee, EmployeeData},
    postgress::PG,
};

pub fn router() -> Router {
    Router::new()
        .route("/new", post(new_employee))
        .route("/remote", delete(remote_employee))
        .route("/:id", get(one).patch(update))
        .route("/", get(get_all))
}

#[utoipa::path(
    post,
    path = "/employee/new",
    request_body = EmployeeData
)]
pub async fn new_employee(extract::Json(data): extract::Json<EmployeeData>) -> Json<Value> {
    let emp = Employee {
        id: uuid::Uuid::new_v4(),
        data: EmployeeData {
            name: data.name,
            second_name: data.second_name,
            surname: data.surname,
            email: data.email,
            is_manager: data.is_manager,
        },
    };
    super::utils::response(unsafe { PG.clone().unwrap().create_employee(emp) }.await)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteEmployee {
    id: uuid::Uuid,
}

#[utoipa::path(
    delete,
    path = "/employee/remote",
    request_body = RemoteEmployee
)]
pub async fn remote_employee(
    extract::Json(RemoteEmployee { id }): extract::Json<RemoteEmployee>,
) -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().try_remove_employee(id) }.await)
}

#[utoipa::path(
    get,
    path = "/employee",
    responses(
            (status = 200, body = Vec<Employee>),
        ),

)]
pub async fn get_all() -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().get_all_employee().await })
}

#[utoipa::path(
    get,
    path = "/employee/:id",
    responses(
            (status = 200, body = Employee),
        ),
)]
pub async fn one(extract::Path(id): extract::Path<Uuid>) -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().get_employee(id).await })
}

#[utoipa::path(
    patch,
    path = "/employee/:id",
    request_body = EmployeeData,
    responses(
            (status = 200, body = Employee),
        ),

)]
pub async fn update(
    extract::Path(id): extract::Path<Uuid>,
    extract::Json(data): extract::Json<EmployeeData>,
) -> Json<Value> {
    super::utils::response(unsafe {
        PG.clone()
            .unwrap()
            .update_employee(Employee { id, data })
            .await
    })
}
