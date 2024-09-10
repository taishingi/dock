use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use env::var;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
pub fn main() -> Server {
    if let Ok(database_url) = var("DATABASE_URL") {
        if let Ok(hostname) = var("HOSTNAME") {
            let manager = ConnectionManager::<SqliteConnection>::new(database_url);
            if let Ok(mut pool) = Pool::builder().build(manager) {
                if let Ok(server) =
                    HttpServer::new(move || App::new().app_data(Data::new(&mut pool)))
                        .bind(("127.0.0.1", 4050))
                {
                    return server.server_hostname(hostname).run();
                }
            }
        }
    }
    panic!("Failed to start SQLite server!");
}
