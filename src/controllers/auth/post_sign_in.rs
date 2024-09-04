use actix_web::{HttpResponse, Responder};

pub async fn post_sign_in() -> impl Responder {
    HttpResponse::Ok().body("btw i use rust i feel, oyyhahh daddy i am so fast ->> bomb")
}
