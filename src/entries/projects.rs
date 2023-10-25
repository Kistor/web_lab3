use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Progect {
    pub id: Uuid,
    pub name_customer: String,
    pub name_performer: String,
    pub employee_id: Vec<Uuid>,
    pub employee_lid_id: Vec<Uuid>,
    pub performers: Uuid,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
}
