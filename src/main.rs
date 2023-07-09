mod configuration;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::{startup::run, telemetry::{get_subscriber, init_subscriber}};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let db_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener, db_pool)?.await
}
