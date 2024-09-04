use actix_web::{HttpResponse, Responder};

pub async fn post_sign_in() -> impl Responder {
    HttpResponse::Ok().body("You're signed in. You can do whatever you want.")
}
