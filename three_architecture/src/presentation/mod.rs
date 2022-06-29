use actix_web::{get, post, put, delete, HttpResponse, Responder};

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("get ok")
}
#[post("/")]
async fn post() -> impl Responder {
    HttpResponse::Ok().body("post ok")
}
#[put("/")]
async fn put() -> impl Responder {
    HttpResponse::Ok().body("put ok")
}
#[delete("/")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete ok")
}
