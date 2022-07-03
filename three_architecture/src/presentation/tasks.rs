use crate::item::tasks::*;
use actix_web::{delete, get, post, put, web::Json, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    name: String,
    user_id: i32,
    term: String,
}

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("get ok")
}
#[post("/")]
async fn post(task: Json<RequestTask>) -> impl Responder {
    registerTasks();
    HttpResponse::Ok().body("ok")
}
#[put("/")]
async fn put() -> impl Responder {
    HttpResponse::Ok().body("put ok")
}
#[delete("/")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete ok")
}
