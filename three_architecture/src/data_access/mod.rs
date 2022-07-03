use crate::DbConn;

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name="tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub name: String,
    pub user_id: i32,
    pub term: String,
    pub update_term_count: i32,
    pub is_completed: bool,
}

#[derive(Debug, FromForm)]
pub struct RequestTask {
    pub name: String,
    pub user_id: i32,
    pub term: String,
}

pub fn postTasks() {
    use web_diesel::schema::tasks::dsl::*;

    pub async fn insert(request: RequestTask, conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| {
            let t = Task { id: None, name: request.name, user_id: request.user_id, term: request.term, update_term_count: 0, is_completed: false };
            diesel::insert_into(tasks::table).values(&t).execute(c)
        }).await
    }
}
