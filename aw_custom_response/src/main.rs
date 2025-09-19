use actix_web::{
    App, Error, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder, dev::Payload,
    error::ErrorUnauthorized, get, post, web,
};
use futures::future::{Ready, ready};
use sqlx::mysql::MySqlPoolOptions;

// --- AuthUser struct ---
#[derive(Debug)]
pub struct AuthUser {
    pub user_id: i32,
}
// --- Middleware AuthUser ---
impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.cookie("user_id") {
            Some(c) => match c.value().parse::<i32>() {
                Ok(id) => ready(Ok(AuthUser { user_id: id })),
                Err(_) => ready(Err(ErrorUnauthorized("Invalid user_id cookie"))),
            },
            None => ready(Err(ErrorUnauthorized("Unauthorized: no user_id cookie"))),
        }
    }
}

// --- Custom Response ---
#[derive(serde::Serialize)]
pub struct Response<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

// --- Routes ---
#[get("/")]
async fn index() -> impl Responder {
    let response = Response::<()> {
        status: "success".to_string(),
        message: "berhasil".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(response)
}
#[get("/profile")]
async fn profile(user: AuthUser) -> impl Responder {
    let response = Response {
        status: "success".to_string(),
        message: format!("Hello user with id: {}", user.user_id),
        data: Some(user.user_id),
    };
    HttpResponse::Ok().json(response)
}

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            actix_web::cookie::Cookie::build("user_id", "42") // contoh
                .path("/")
                .http_only(true)
                .finish(),
        )
        .body("Login sukses, cookie diset!")
}

// --- Main ---
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:@localhost:3306/1")
        .await
        .expect("Gagal koneksi ke database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(login)
            .service(profile)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
