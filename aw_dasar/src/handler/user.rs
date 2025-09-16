use actix_web::{HttpResponse, Responder, get, web};

#[get("")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hai dari users")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users") // semua route post ada di bawah /posts
            .service(get_users),
    );
}
