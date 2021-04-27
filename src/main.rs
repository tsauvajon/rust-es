use bytes::Bytes;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type Queue = Arc<Mutex<VecDeque<Bytes>>>;

#[tokio::main]
async fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    let produce_queue = queue.clone();
    // listen to producers
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let produce_queue = produce_queue.clone();

            tokio::spawn(async move {
                produce(socket, produce_queue).await;
            });
        }
    });

    // send to consumers
    let listener = TcpListener::bind("127.0.0.1:7879").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let mut queue = queue.clone();

        tokio::spawn(async move {
            consume(socket, queue).await;
        });
    }
}

async fn produce(stream: TcpStream, queue: Queue) {
    let (mut rd, mut wr) = tokio::io::split(stream);

    let mut value: Vec<u8> = vec![];

    let mut buf = vec![0; 128];
    // let mut buf = BytesMut::with_capacity(1024);
    loop {
        match rd.read(&mut buf).await {
            Ok(0) => break, // EOF
            Ok(n) => {
                println!("got some bytes: {:?}", &buf[..n]);
                value.append(&mut buf[..n].to_vec());
                wr.write_all(b"hello\r\n").await.unwrap(); // let mini-redis-cli stop their connection
            }
            Err(err) => {
                println!("read error: {:?}", err);
                return;
            }
        };
    }

    let strval = std::str::from_utf8(&value[..]).unwrap();
    println!("GOT MESSAGE {:?}", strval);
    let data = value.into();
    let mut queue = queue.lock().unwrap();
    queue.push_back(data);
}

async fn consume(stream: TcpStream, queue: Queue) {
    let (mut _rd, mut wr) = tokio::io::split(stream);

    // loop {
    let mut queue = queue.lock().unwrap();
    match queue.pop_front() {
        // Some(value) => println!("consumed msg: {:?}", value),
        Some(value) => wr.write_all(&value.to_vec()[..]).await.unwrap(),
        None => return,
    };
    // }
}
