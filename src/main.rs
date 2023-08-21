use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Define the address and the listener
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // Start the server
    run(listener)?.await
}
