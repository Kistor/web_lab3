use crate::entries::employee::Employee;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "employee")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub second_name: String,
    pub surname: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Employee {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            second_name: value.second_name,
            surname: value.surname,
            email: value.email,
        }
    }
}
