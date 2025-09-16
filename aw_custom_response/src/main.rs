use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use aw_custom_response::Response;
use sqlx::mysql::MySqlPoolOptions;

#[get("/")]
async fn index() -> impl Responder {
    let response = Response::<()> {
        status: "success".to_string(),
        message: "berhasil".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:@localhost:3306/1")
        .await
    {
        Err(err) => {
            eprint!("error koneksi {}", err)
        }
        Ok(_) => {
            println!("Koneksi berhasil")
        }
    };
    HttpServer::new(move || App::new().app_data(web::Data::new(pool)).service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
