use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use env::var;
use std::env;

#[get("/")]
pub async fn postgres_ready(_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Hello, postgres!")
}
pub fn main() -> Server {
    if let Ok(database_url) = var("DATABASE_URL") {
        if let Ok(hostname) = var("HOSTNAME") {
            let manager = ConnectionManager::<PgConnection>::new(database_url);
            if let Ok(pool) = Pool::builder().build(manager) {
                if let Ok(server) = HttpServer::new(move || {
                    App::new()
                        .app_data(Data::new(pool.clone()))
                        .service(postgres_ready)
                })
                .bind(("127.0.0.1", 5050))
                {
                    return server.server_hostname(hostname).run();
                }
            }
        }
    }
    panic!("Failed to start the postgresql server!");
}
