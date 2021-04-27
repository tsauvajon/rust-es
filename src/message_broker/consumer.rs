use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum ConsumeError {
    ProtocolError,
    IOError(std::io::Error),
}

impl From<std::io::Error> for ConsumeError {
    fn from(err: std::io::Error) -> ConsumeError {
        ConsumeError::IOError(err)
    }
}

pub async fn consume_one(stream: &mut TcpStream) -> Result<Option<bytes::Bytes>, ConsumeError> {
    let (mut r, mut w) = stream.split();
    w.write_all(b"\0").await?;

    let mut buf = vec![0; 128];
    let mut value: Vec<u8> = vec![];
    loop {
        match r.read(&mut buf).await {
            Ok(0) => return Ok(None), // EOF
            Ok(n) => {
                if &buf[n - 1..n] == b"\0" {
                    value.append(&mut buf[1..n - 1].to_vec());

                    if buf[0] == b"+"[0] {
                        return Ok(Some(value.into()));
                    } else if buf[0] == b"-"[0] {
                        return Ok(None);
                    } else {
                        return Err(ConsumeError::ProtocolError);
                    }
                }

                value.append(&mut buf[..n].to_vec());
            }
            Err(err) => return Err(ConsumeError::IOError(err)),
        };
    }
}
