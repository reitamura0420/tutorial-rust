use actix_web::{App, HttpServer};
extern crate diesel;
use presentation::presentation_tasks::*;

mod data_access;
mod items;
mod model;
mod presentation;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(post)
            .service(put)
            .service(delete)
    })
    // ローカルホストのport8080で起動
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
