use crate::entries::projects::Progect;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name_customer: String,
    pub name_performer: String,
    pub employee_id: Vec<Uuid>,
    pub employee_lid_id: Vec<Uuid>,
    pub performers: Uuid,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Progect {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name_customer: value.name_customer,
            name_performer: value.name_performer,
            employee_id: value.employee_id,
            employee_lid_id: value.employee_lid_id,
            performers: value.performers,
            date_start: value.date_start,
            date_end: value.date_end,
        }
    }
}
