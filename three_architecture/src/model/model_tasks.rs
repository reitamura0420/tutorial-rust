use diesel::prelude::*;
use serde::Deserialize;

// #[derive(Insertable, Deserialize)]
// #[diesel(table_name = tasks)]
pub struct Task {
    pub id: Option<i32>,
    pub name: String,
    pub user_id: i32,
    pub term: String,
    pub update_term_count: i32,
    pub is_completed: bool,
}
