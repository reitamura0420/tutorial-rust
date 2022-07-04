use actix_web::{delete, get, post, put, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::items::item_tasks::register_tasks;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTask {
    pub name: String,
    pub user_id: i32,
    pub term: String,
}

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("get ok")
}
#[post("/")]
async fn post(task: Json<RequestTask>) -> impl Responder {
    let to_task = RequestTask {
        name: task.name.clone(),
        user_id: task.user_id,
        term: task.term.clone(),
    };
    register_tasks(to_task);
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
