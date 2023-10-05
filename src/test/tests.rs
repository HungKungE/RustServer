use actix_web::{HttpResponse, Responder};

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn manual_hello2() -> impl Responder {
    HttpResponse::Ok().body("Hey there2!")
}