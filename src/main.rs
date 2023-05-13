use newslette_rs::{
    configuration::{self, read_config},
    startup,
};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = read_config().expect("Failed to read configuration!");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener)?.await
}
