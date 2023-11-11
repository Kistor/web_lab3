use crate::entries::projects::{Project, ProjectData};
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name_project: String,
    pub priority: i32,
    pub name_customer: String,
    pub name_performer: String,
    pub employee_id: Vec<Uuid>,
    pub employee_lid_id: Vec<Uuid>,
    pub performers: Uuid,
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Project {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            data: ProjectData {
                name_project: value.name_project,
                priority: value.priority,
                name_customer: value.name_customer,
                name_performer: value.name_performer,
                employee_id: value.employee_id,
                employee_lid_id: value.employee_lid_id,
                performers: value.performers,
                date_start: DateTime::from_utc(value.date_start, Utc),
                date_end: DateTime::from_utc(value.date_end, Utc),
            },
        }
    }
}
