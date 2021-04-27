extern crate rust_es;

use rust_es::message_broker::consumer;

use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), consumer::ConsumeError> {
    let mut stream = TcpStream::connect("127.0.0.1:7879").await?;

    loop {
        match consumer::consume_one(&mut stream).await? {
            None => {
                println!("no message available");
                std::thread::sleep(std::time::Duration::from_millis(5000));
            }
            Some(msg) => {
                let str_msg = std::str::from_utf8(&msg[..]).unwrap();
                {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    println!("consumed a message: {:?}", str_msg);
                }
            }
        };
    }
}
