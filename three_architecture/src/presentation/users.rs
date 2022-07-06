use actix_web::{get, post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUser {
    pub mail_address: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUserDisabled {
    pub id: i32,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("get ok")
}
#[post("/users")]
async fn post_users(request: Json<RequestUser>) -> impl Responder {
    let to_user = RequestUser {
        mail_address: request.mail_address.clone(),
        last_name: request.last_name.clone(),
    };
    crate::items::users::register_users(to_user);
    HttpResponse::Ok().body("ok")
}
#[post("/users/disabled")]
async fn update_disabled(request: Json<RequestUserDisabled>) -> impl Responder {
    crate::items::users::update_disabled(request.id);
    HttpResponse::Ok().body("ok")
}
