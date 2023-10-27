use crate::entries::employee::Employee;
use crate::entries::projects::Progect;
use anyhow::anyhow;
use anyhow::Result;
use sea_orm::ActiveValue;
use sea_orm::ColumnTrait;
use sea_orm::DbErr;
use sea_orm::QueryFilter;
use sea_orm::QueryTrait;
use sea_orm::TransactionTrait;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use sea_orm_migration::MigratorTrait;
use sea_query::Expr;
use uuid::Uuid;

use self::migrator::Migrator;

pub mod entries;
pub mod migrations;
pub mod migrator;

pub static mut PG: Option<Postgress> = None;

#[derive(Clone, Debug)]
pub struct Postgress {
    db: DatabaseConnection,
}

impl Postgress {
    pub async fn try_create_project(&self, progect: Progect) -> Result<()> {
        // проверка, что такое сотрудники существуют
        for id in progect.employee_id.clone() {
            self.get_employee(id).await?;
        }
        for id in progect.employee_lid_id.clone() {
            self.get_employee(id).await?;
        }

        self.create_project(progect).await
    }

    pub async fn try_update_project(&self, progect: Progect) -> Result<()> {
        // проверка, что такое сотрудники существуют
        for id in progect.employee_id.clone() {
            self.get_employee(id).await?;
        }
        for id in progect.employee_lid_id.clone() {
            self.get_employee(id).await?;
        }

        self.update_project(progect).await
    }

    pub async fn try_remove_employee(&self, uuid: Uuid) -> Result<()> {
        //UPDATE `Progects` SET array_remove(employee_id, <тут uuid>)
        let t = entries::Progects::update_many()
            .col_expr(
                entries::progect::Column::EmployeeId,
                Expr::cust(format!("array_remove(employee_id, '{}')", uuid.to_string())),
            )
            .filter(Expr::cust(format!(
                "array_contains(employee_id, '{}')",
                uuid.to_string()
            )));

        println!(
            "{}",
            t.build(sea_orm::DatabaseBackend::Postgres).to_string()
        );
        self.remote_employee(uuid).await
    }

    pub async fn new(postgresql: String) {
        let db = Database::connect(postgresql)
            .await
            .unwrap_or_else(|err| panic!("Не удалось подключиться к Postgresql: {err}"));

        Migrator::up(&db, None)
            .await
            .unwrap_or_else(|err| panic!("Не удалось провести миграции хранилища: {err}"));
        unsafe { PG = Some(Postgress { db: db }) }
    }

    pub async fn get_employee(&self, id: Uuid) -> Result<Employee> {
        let employee: Employee = entries::Employees::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(anyhow!("Сотрудник {id} не найден"))?
            .into();

        Ok(employee)
    }

    // pub async fn get_all_emplotee(&self) -> Result<Vec<Employee>> {
    //     let employee: Employee = entries::Employees::find().all(&self.db).await?
    // }

    pub async fn create_employee(&self, empluyee: Employee) -> Result<()> {
        let model = entries::employees::ActiveModel {
            id: ActiveValue::Set(empluyee.id),
            name: ActiveValue::Set(empluyee.data.name),
            second_name: ActiveValue::Set(empluyee.data.second_name),
            surname: ActiveValue::Set(empluyee.data.surname),
            email: ActiveValue::Set(empluyee.data.email),
            is_manager: ActiveValue::Set(empluyee.data.is_manager),
        };

        _ = entries::Employees::insert(model).exec(&self.db).await?;
        Ok(())
    }

    async fn remote_employee(&self, id: Uuid) -> Result<()> {
        self.db
            .transaction::<_, _, DbErr>(|txn| {
                Box::pin(async move {
                    entries::Employees::delete_many()
                        .filter(entries::employees::Column::Id.eq(id))
                        .exec(txn)
                        .await?;

                    Ok(())
                })
            })
            .await?;

        Ok(())
    }

    pub async fn update_employee(&self, empluyee: Employee) -> Result<()> {
        let model = entries::employees::ActiveModel {
            id: ActiveValue::Set(empluyee.id),
            name: ActiveValue::Set(empluyee.data.name),
            second_name: ActiveValue::Set(empluyee.data.second_name),
            surname: ActiveValue::Set(empluyee.data.surname),
            email: ActiveValue::Set(empluyee.data.email),
            is_manager: ActiveValue::Set(empluyee.data.is_manager),
        };

        _ = entries::Employees::update(model).exec(&self.db).await?;

        Ok(())
    }

    pub async fn create_project(&self, progect: Progect) -> Result<()> {
        let model = entries::progect::ActiveModel {
            id: ActiveValue::Set(progect.id),
            name_customer: ActiveValue::Set(progect.name_customer),
            name_performer: ActiveValue::Set(progect.name_performer),
            employee_id: ActiveValue::Set(progect.employee_id),
            employee_lid_id: ActiveValue::Set(progect.employee_lid_id),
            performers: ActiveValue::Set(progect.performers),
            date_start: ActiveValue::Set(progect.date_start),
            date_end: ActiveValue::Set(progect.date_end),
        };

        _ = entries::Progects::insert(model).exec(&self.db).await?;
        Ok(())
    }

    pub async fn get_project(&self, id: Uuid) -> Result<Progect> {
        let progect: Progect = entries::Progects::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(anyhow!("Сотрудник {id} не найден"))?
            .into();

        Ok(progect)
    }

    pub async fn update_project(&self, progect: Progect) -> Result<()> {
        let model = entries::progect::ActiveModel {
            id: ActiveValue::Set(progect.id),
            name_customer: ActiveValue::Set(progect.name_customer),
            name_performer: ActiveValue::Set(progect.name_performer),
            employee_id: ActiveValue::Set(progect.employee_id),
            employee_lid_id: ActiveValue::Set(progect.employee_lid_id),
            performers: ActiveValue::Set(progect.performers),
            date_start: ActiveValue::Set(progect.date_start),
            date_end: ActiveValue::Set(progect.date_end),
        };

        _ = entries::Progects::update(model).exec(&self.db).await?;

        Ok(())
    }

    pub async fn remote_project(&self, id: Uuid) -> Result<()> {
        self.db
            .transaction::<_, _, DbErr>(|txn| {
                Box::pin(async move {
                    entries::Progects::delete_many()
                        .filter(entries::progect::Column::Id.eq(id))
                        .exec(txn)
                        .await?;

                    Ok(())
                })
            })
            .await?;

        Ok(())
    }
}
