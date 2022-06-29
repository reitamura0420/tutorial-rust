use actix_web::{App, HttpServer};
mod presentation;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(presentation::get)
        .service(presentation::post)
        .service(presentation::put)
        .service(presentation::delete)
    })
    // ローカルホストのport8080で起動
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
