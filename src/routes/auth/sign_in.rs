use actix_web::{web, HttpResponse};

use crate::controllers::auth::post_sign_in::post_sign_in;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(
        web::resource("/v1/signin")
            .route(web::get().to(post_sign_in))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
