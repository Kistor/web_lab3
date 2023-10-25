use super::migrations::*;
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231018_000001_create_employee::Migration),
            Box::new(m20231018_000002_create_projects::Migration),
        ]
    }
}
