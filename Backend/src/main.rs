use actix_web::{App, HttpServer};
mod test;

mod controllers;
mod routes;
use crate::routes::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting on http://localhost:8080");

    HttpServer::new(|| App::new().configure(auth::sign_in::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
