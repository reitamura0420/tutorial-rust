use actix_web::{get, post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::items::tasks::register_tasks;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTask {
    pub name: String,
    pub user_id: i32,
    pub term: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTaskUpdateSchedule {
    pub id: i32,
}

#[get("/tasks")]
async fn get_tasks() -> impl Responder {
    HttpResponse::Ok().body("get ok")
}

#[post("/tasks")]
async fn post_tasks(task: Json<RequestTask>) -> impl Responder {
    let to_task = RequestTask {
        name: task.name.clone(),
        user_id: task.user_id,
        term: task.term.clone(),
    };
    register_tasks(to_task);
    HttpResponse::Ok().body("ok")
}

#[post("/tasks/schedules")]
async fn update_schedules(request: Json<RequestTaskUpdateSchedule>) -> impl Responder {
    crate::items::tasks::update_schedules(request.id);
    HttpResponse::Ok().body("ok")
}
