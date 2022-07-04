use dotenv::dotenv;
use std::env;

use crate::{model::model_tasks::Task, presentation::presentation_tasks::RequestTask};
use diesel::Connection;
use diesel::RunQueryDsl;

pub fn establish_connection() -> diesel::MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn post_tasks(request: RequestTask) -> bool {
    use crate::schema::tasks;
    let connection = establish_connection();

    let new_task = Task {
        id: None,
        name: request.name,
        user_id: request.user_id,
        term: request.term,
        update_term_count: 0,
        is_completed: false,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&connection)
        .expect("Error saving new post");
    true
}
