use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zer02prod::telemetry::{get_subscriber, init_subscriber};
use zer02prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to create Postgres connection pool.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
