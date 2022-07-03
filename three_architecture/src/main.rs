mod presentation;
use actix_web::{App, HttpServer};
use presentation::tasks::*;

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
