use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: Uuid,
    #[serde(flatten)]
    pub data: EmployeeData,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeData {
    pub name: String,
    pub second_name: String,
    pub surname: String,
    pub email: String,
    pub is_manager: bool,
}
