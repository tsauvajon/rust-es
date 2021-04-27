use bytes::Bytes;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type Queue = Arc<Mutex<VecDeque<Bytes>>>;

pub async fn handle(produce_addr: String, consume_addr: String) -> Result<(), std::io::Error> {
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    // listen to producers
    let produce_queue = queue.clone();
    tokio::spawn(async move {
        let listener = TcpListener::bind(produce_addr).await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let produce_queue = produce_queue.clone();

            tokio::spawn(async move {
                process_produce(socket, produce_queue).await.unwrap();
            });
        }
    });

    // listen to consumers
    let listener = TcpListener::bind(consume_addr).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let queue = queue.clone();

        tokio::spawn(async move {
            process_consume(socket, queue).await.unwrap();
        });
    }
}

async fn process_produce(stream: TcpStream, queue: Queue) -> Result<(), std::io::Error> {
    let (mut rd, mut wr) = tokio::io::split(stream);

    let mut value: Vec<u8> = vec![];

    let mut buf = vec![0; 128];
    loop {
        match rd.read(&mut buf).await {
            Ok(0) => return Ok(()), // EOF
            Ok(n) => {
                if &buf[n - 1..n] == b"\0" {
                    value.append(&mut buf[..n - 1].to_vec());
                    let strval = std::str::from_utf8(&value[..]).unwrap();
                    println!("GOT MESSAGE {:?}", strval);
                    let data = value.clone().into();

                    {
                        let mut queue = queue.lock().unwrap();
                        queue.push_back(data);
                    }

                    wr.write_all(b"ok").await?;
                    wr.write_all(b"\0").await?; // let the client know we're done

                    continue;
                }

                value.append(&mut buf[..n].to_vec());
            }
            Err(err) => {
                println!("read error: {:?}", err);
                return Err(err);
            }
        };
    }
}

async fn process_consume(stream: TcpStream, queue: Queue) -> Result<(), std::io::Error> {
    let (mut rd, mut wr) = tokio::io::split(stream);

    let mut buf = vec![0; 128];
    loop {
        match rd.read(&mut buf).await {
            Ok(0) => return Ok(()), // EOF
            Ok(n) => {
                println!("consumer asking for a msg: {:?}", &buf[..n]);

                if &buf[n - 1..n] != b"\0" {
                    continue;
                }

                let val: Option<bytes::Bytes>;
                {
                    let mut queue = queue.lock().unwrap();
                    val = queue.pop_front();
                }

                match val {
                    Some(val) => {
                        wr.write_all(b"+").await?; // there is a message: '+'
                        wr.write_all(&val.to_vec()[..]).await?;
                    },
                    None => wr.write_all(b"-").await?, // there is no message: '-'
                };

                wr.write_all(b"\0").await?; // message done
            }
            Err(err) => {
                println!("read error: {:?}", err);
                return Err(err);
            }
        };
    }
}
