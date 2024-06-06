use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zer02prod::email_client::EmailClient;
use zer02prod::startup::Application;
use zer02prod::telemetry::{get_subscriber, init_subscriber};
use zer02prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
