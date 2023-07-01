use std::net::TcpListener;

use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address, connection_pool)?.await
}
