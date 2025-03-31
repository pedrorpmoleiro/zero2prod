use std::net::TcpListener;
use sqlx::PgPool;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use crate::routes::*;

pub fn run(
    listener: TcpListener,
    connection_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
