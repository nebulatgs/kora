mod controllers;
mod realtime;
mod states;
mod structs;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use controllers::*;
use states::{channels::ChannelState, identity::IdentityState};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let channel_state = web::Data::new(ChannelState::new());
    let identity_state = web::Data::new(IdentityState::new());
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().supports_credentials().allow_any_origin())
            .app_data(channel_state.clone())
            .app_data(identity_state.clone())
            .service(hello)
            .service(echo)
            .service(meta::health_check)
            .service(meta::version)
            .service(channels::list)
            .service(channels::create)
            .service(channels::get)
            .service(channels::start)
            .service(channels::join)
            .service(identity::create)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
