use newslette_rs::{configuration::read_config, startup};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = read_config().expect("Failed to read configuration!");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to POstgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let _ = startup::run(listener, connection_pool)?.await;
    Ok(())
}
