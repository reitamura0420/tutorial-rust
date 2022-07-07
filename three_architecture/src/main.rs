use actix_web::{App, HttpServer};
#[macro_use]
extern crate diesel;
extern crate chrono;
use presentation::tasks::*;
use presentation::users::*;

mod data_access;
mod items;
mod model;
mod presentation;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_tasks)
            .service(post_tasks)
            .service(get_users)
            .service(post_users)
            .service(update_disabled)
            .service(update_schedules)
    })
    // ローカルホストのport8080で起動
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
