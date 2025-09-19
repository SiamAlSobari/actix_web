use std::fmt::format;

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
struct CreateUser {
    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    username: String,
}

#[post("/")]
async fn create_user(
    user: web::Json<CreateUser>,
    pool: web::Data<sqlx::MySqlPool>,
) -> impl Responder {
    match user.validate() {
        Ok(_) => match sqlx::query("INSERT INTO users (username) VALUES (?)")
            .bind(&user.username)
            .execute(pool.get_ref())
            .await
        {
            Ok(_) => HttpResponse::Ok().body(format!("User berhasil dibuat , {}", &user.username)),
            Err(e) => {
                eprintln!("DB Error: {:?}", e);
                HttpResponse::InternalServerError().body("Gagal simpan user")
            }
        },
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("")
        .await
        .expect("bodo ga ada db kotak");
    HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
