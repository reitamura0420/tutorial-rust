use chrono::NaiveDateTime;
use dotenv::dotenv;
use std::env;

use crate::{model::tasks::Task, presentation::tasks::RequestTask};
use diesel::prelude::*;
use diesel::Connection;
use diesel::RunQueryDsl;

pub fn establish_connection() -> diesel::MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_tasks(request: RequestTask) -> bool {
    let connection = establish_connection();
    let term_to_date_time =
        NaiveDateTime::parse_from_str(&request.term, "%Y/%m/%d %H:%M:%S").unwrap();

    let new_task = Task {
        id: None,
        name: request.name,
        user_id: request.user_id,
        term: term_to_date_time,
        update_term_count: 0,
        is_completed: false,
    };

    diesel::insert_into(crate::schema::tasks::table)
        .values(&new_task)
        .execute(&connection)
        .expect("Error saving new post");
    true
}

pub fn update_schedules(request: i32) -> bool {
    use crate::schema::tasks::dsl::tasks;
    use crate::schema::tasks::*;
    let connection = establish_connection();
    let term_data = tasks
        .select(term)
        .filter(id.eq(request))
        .load::<NaiveDateTime>(&connection)
        .unwrap();

    let to_date = term_data.get(0).unwrap().clone();

    let term_updated = to_date + chrono::Duration::days(1);

    diesel::update(tasks.find(request))
        .set((
            update_term_count.eq(update_term_count + 1),
            term.eq(term_updated),
        ))
        .execute(&connection)
        .expect("Error saving new post");
    true
}
