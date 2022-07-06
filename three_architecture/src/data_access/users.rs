use dotenv::dotenv;
use std::env;

use crate::{model::users::User, presentation::users::RequestUser};
use diesel::prelude::*;
use diesel::Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub fn establish_connection() -> diesel::MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_users(request: RequestUser) -> bool {
    let connection = establish_connection();

    let new_user = User {
        id: None,
        mail_address: request.mail_address,
        last_name: request.last_name,
        enable: true,
    };

    diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .execute(&connection)
        .expect("Error saving new post");
    true
}

pub fn update_disabled(request: i32) -> bool {
    use crate::schema::users::dsl::users;
    use crate::schema::users::*;
    let connection = establish_connection();

    diesel::update(users.find(request))
        .set(enable.eq(false))
        .execute(&connection)
        .expect("Error saving new post");
    true
}
