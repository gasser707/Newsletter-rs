use actix_web::{dev::Server, web, App, HttpServer};
use crate::routes::{health_check, subscribe};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let address = listener.local_addr().unwrap().to_string();
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_connection_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(web::Data::clone(&db_pool))
    })
    .listen(listener)?
    .run();
    // No .await here!
    println!("Server is now running at {}", address);
    Ok(server)
}
