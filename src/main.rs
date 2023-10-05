use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8080";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn manual_hello2() -> impl Responder {
    HttpResponse::Ok().body("Hey there2!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello) // "/" 요청 처리
        .service(
            // "/api/~" 요청 처리
            web::scope("/api")
                .route("/hey",web::get().to(manual_hello))
                .route("/hey2",web::get().to(manual_hello2))
        )
  })
  .bind(SERVER_IP.to_owned()+":"+SERVER_PORT)?
  .run()
  .await
}
