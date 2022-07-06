use crate::schema::users;
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub mail_address: String,
    pub last_name: String,
    pub enable: bool,
}
