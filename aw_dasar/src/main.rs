use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
mod handler;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions}; // untuk bikin p
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("index page")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: MySqlPool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@localhost:3306/mydb")
        .await
    {
        Ok(p) => {
            println!("✅ Koneksi berhasil cihut");
            p // simpan ke variabel `pool`
        }
        Err(err) => {
            eprintln!("❌ Error koneksi: {}", err);
            return Ok(()); // keluar gracefully
        }
    };
    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(pool.clone()))
            .configure(handler::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
