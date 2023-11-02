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
                    .table(Project::Table)
                    .col(ColumnDef::new(Project::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Project::NameCustomer).string())
                    .col(ColumnDef::new(Project::NamePerformer).string())
                    .col(
                        ColumnDef::new(Project::EmployeeId)
                            .array(ColumnType::Uuid)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::EmployeeLidId)
                            .array(ColumnType::Uuid)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Project::Performers).uuid().not_null())
                    .col(
                        ColumnDef::new(Project::DateStart)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Project::DateEnd)
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
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Project {
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
