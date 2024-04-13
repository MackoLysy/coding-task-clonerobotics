use std::error::Error;

mod args;
mod errors;
mod logger;
mod server;

use server::HttpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::enable_logger();
    let mut server = HttpServer::new("localhost".to_string(), 3000);
    server.start().await?;
    Ok(())
}
