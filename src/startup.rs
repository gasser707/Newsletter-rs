use crate::{
    configuration::Settings,
    email_client::EmailClient,
    routes::{health_check, subscribe},
};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_connection_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let address = listener.local_addr().unwrap().to_string();
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_connection_pool);
    let email_client = web::Data::new(email_client);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            // Data uses an Arc
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    // No .await here!
    log::info!("Server is now running at {}", address);
    Ok(server)
}

pub async fn build(configuration: Settings, listener: TcpListener) -> std::io::Result<Server> {
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid email address");

    let email_client_timeout = configuration.email_client.timeout();

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        email_client_timeout,
    );

    run(listener, connection_pool, email_client)
}
