#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpServer, Responder};

mod api;

fn create_study(study: web::Json<api::Study>) -> impl Responder {
    format!("Goal: {:?}", study.goal)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/CreateStudy").route(web::post().to(create_study)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
