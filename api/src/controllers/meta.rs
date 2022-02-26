use super::*;

#[get("/healthcheck")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("hi im alive")
}

#[get("/version")]
pub async fn version() -> impl Responder {
    HttpResponse::Ok().body("Mynt API Version 0.0.0")
}