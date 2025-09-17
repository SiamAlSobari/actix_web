mod middlewares;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use aw_middleware::Response;
use sqlx::mysql::MySqlPoolOptions;

#[get("/")]
async fn index() -> impl Responder {
    let res = Response::<()> {
        status: "B".to_string(),
        message: "K".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(res)
}

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            actix_web::cookie::Cookie::build("user_id", "12")
                .http_only(true)
                .path("/")
                .finish(),
        )
        .body("succes")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("")
        .await
        .expect("Koneksi gagal");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
