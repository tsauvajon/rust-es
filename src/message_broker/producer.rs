use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum ProduceError {
    ConnectionDroppedByServer,
    IoError(std::io::Error),
}

impl From<std::io::Error> for ProduceError {
    fn from(err: std::io::Error) -> ProduceError {
        ProduceError::IoError(err)
    }
}

pub async fn produce_one(stream: &mut TcpStream, msg: Vec<u8>) -> Result<(), ProduceError> {
    let (mut r, mut w) = stream.split();
    w.write_all(&msg[..]).await?;
    w.write_all(b"\0").await?;

    let mut buf = vec![0; 128];
    let mut value: Vec<u8> = vec![];
    loop {
        match r.read(&mut buf).await {
            Ok(0) => return Err(ProduceError::ConnectionDroppedByServer),
            Ok(n) => {
                if &buf[n - 1..n] == b"\0" {
                    value.append(&mut buf[..n - 1].to_vec());

                    let strval = std::str::from_utf8(&value[..]).unwrap();
                    println!("server response: {:?}", strval);
                    return Ok(());
                }

                value.append(&mut buf[..n].to_vec());
            }
            Err(err) => return Err(ProduceError::IoError(err)),
        };
    }
}
