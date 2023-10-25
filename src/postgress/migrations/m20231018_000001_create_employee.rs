use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20231018_000001_create_employee"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Employee::Table)
                    .col(ColumnDef::new(Employee::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Employee::Name).string())
                    .col(ColumnDef::new(Employee::SecondName).string())
                    .col(ColumnDef::new(Employee::Surname).string())
                    .col(ColumnDef::new(Employee::Email).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Employee::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Employee {
    Table,

    Id,
    Name,
    SecondName,
    Surname,
    Email,
}
