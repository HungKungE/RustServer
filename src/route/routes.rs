use actix_web::{web};

use crate::test::tests::{manual_hello, manual_hello2};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/hey", web::get().to(manual_hello))
            .route("/hey2", web::get().to(manual_hello2)),
    );
}