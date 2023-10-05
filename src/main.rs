use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};

mod route;
mod test;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8080";

#[get("/")]
  async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .wrap(middleware::Logger::default()) // Logger 미들웨어를 추가
        .configure(route::routes::configure_routes) // route 모듈의 라우트 구성 함수를 호출
  })
  .bind(format!("{}:{}", SERVER_IP, SERVER_PORT))?
  .run()
  .await
}
