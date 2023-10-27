use axum::{
    extract,
    routing::{delete, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    entries::employee::{Employee, EmployeeData},
    postgress::PG,
};

pub fn router() -> Router {
    Router::new()
        .route("/new", post(new_employee))
        .route("/remote", delete(remote_employee))
}

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

pub async fn remote_employee(
    extract::Json(RemoteEmployee { id }): extract::Json<RemoteEmployee>,
) -> Json<Value> {
    super::utils::response(unsafe { PG.clone().unwrap().try_remove_employee(id) }.await)
}
