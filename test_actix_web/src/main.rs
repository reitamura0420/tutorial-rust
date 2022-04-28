use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer};
use tera::Tera;

#[get("/html")]
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "kz_morita");
    let view = tmpl
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();

        App::new().data(templates).service(index)
    })
    .bind("127.0.0.1:7878")
    .expect("Can not bind to port 3000")
    .run()
    .await
}
