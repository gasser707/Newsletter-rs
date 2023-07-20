use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use zero2prod::{
    configuration,
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database_config.with_db());

    let sender_email = configuration
        .email_client_config
        .sender()
        .expect("Invalid email address");

    let email_client_timeout = configuration.email_client_config.timeout();

    let email_client = EmailClient::new(
        configuration.email_client_config.base_url,
        sender_email,
        configuration.email_client_config.authorization_token,
        email_client_timeout
    );

    let address = format!(
        "{}:{}",
        configuration.application_config.host, configuration.application_config.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener, connection_pool, email_client)?.await
}
