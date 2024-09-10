use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use std::env::var;

#[get("/")]
async fn mariadb_ready() -> impl Responder {
    HttpResponse::Ok().body("Hello, mariadb!")
}

pub fn main() -> Server {
    if let Ok(database_url) = var("DATABASE_URL") {
        if let Ok(hostname) = var("HOSTNAME") {
            let manager = ConnectionManager::<MysqlConnection>::new(database_url);
            if let Ok(pool) = Pool::builder().build(manager) {
                if let Ok(server) = HttpServer::new(move || {
                    App::new()
                        .app_data(Data::new(pool.clone()))
                        .service(mariadb_ready)
                })
                    .bind(("127.0.0.1", 3050))
                {
                    return server.server_hostname(hostname.as_str()).run();
                }
            }
        }
    }
    panic!("Failed to start the mariadb database server!");
}
