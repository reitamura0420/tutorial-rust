use crate::schema::tasks;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub name: String,
    pub user_id: i32,
    pub term: NaiveDateTime,
    pub update_term_count: i32,
    pub is_completed: bool,
}
