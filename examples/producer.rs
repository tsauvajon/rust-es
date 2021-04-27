extern crate rust_es;

use rust_es::message_broker::producer;

use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), producer::ProduceError> {
    let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();

    producer::produce_one(&mut stream, "some message".into()).await?;

    Ok(())
}
