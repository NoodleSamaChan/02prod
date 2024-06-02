use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zer02prod::telemetry::{get_subscriber, init_subscriber};
use zer02prod::{configuration::get_configuration, startup::run};
use zer02prod::email_client::{self, EmailClient};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zer02prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url, sender_email,);

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection, email_client)?.await?;
    Ok(())
}
