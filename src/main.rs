use std::net::TcpListener;

use newsletter::configuration::get_configuration;
use newsletter::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
