extern crate rust_es;

use rust_es::message_broker::server;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let produce_addr = "127.0.0.1:7878".to_string();
    let consume_addr = "127.0.0.1:7879".to_string();

    server::handle(produce_addr, consume_addr).await?;

    Ok(())
}
