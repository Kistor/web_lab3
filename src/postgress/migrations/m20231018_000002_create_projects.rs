use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20231018_000002_create_projects"
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
                    .col(ColumnDef::new(Employee::NameCustomer).string())
                    .col(ColumnDef::new(Employee::NamePerformer).string())
                    .col(
                        ColumnDef::new(Employee::EmployeeId)
                            .array(ColumnType::Uuid)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Employee::EmployeeLidId)
                            .array(ColumnType::Uuid)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Employee::Performers).uuid().not_null())
                    .col(
                        ColumnDef::new(Employee::DateStart)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Employee::DateEnd)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
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
    NameCustomer,
    NamePerformer,
    EmployeeId,
    EmployeeLidId,
    Performers,
    DateStart,
    DateEnd,
}
