use uuid::Uuid;

pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub second_name: String,
    pub surname: String,
    pub email: String,
}
