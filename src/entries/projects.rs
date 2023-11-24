use ::serde::{Deserialize, Serialize};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Uuid,
    #[serde(flatten)]
    pub data: ProjectData,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProjectData {
    pub name_project: String,
    pub name_customer: String,
    pub name_performer: String,
    pub employee_id: Vec<Uuid>,
    pub employee_lid_id: Vec<Uuid>,
    pub performers: Uuid,

    pub priority: i32,
    #[serde(with = "ts_milliseconds")]
    pub date_start: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub date_end: DateTime<Utc>,
}
