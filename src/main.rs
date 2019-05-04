use actix_web::{web, App, HttpServer, Responder};

fn index() -> impl Responder {
    format!("Hello")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind("127.0.0.1:8080")?
        .run()
}
