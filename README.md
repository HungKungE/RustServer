rust server

1. rust server project directory 만들기

```sh
 cargo new "directory name"
```

2. Cargo.toml에 actix-web 종속성 추가

```sh
[dependencies]

actix-web = "4"
```

3. main.rs 임시 추가 - 이후에 본인 맘대로 예제 변형

```sh
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
        .service(hello)
        .route("/hey", web::get().to(manual_hello))
        .route("/hey2", web::get().to(manual_hello2))
    })
    .bind(SERVER_IP.to_owned()+":"+SERVER_PORT)?
    .run()
    .await
  }
```
