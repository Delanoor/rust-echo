use std::env::args;
use std::{thread, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    // read args
    let delay = args()
        .nth(1)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();
    // starting
    println!("Katoo starting {}", SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    println!("Katoo listening {}", SERVER_ADDRESS);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket, delay).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, delay: u64) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);

    println!("received: {}", message);
    // delay
    thread::sleep(Duration::from_millis(delay));

    // write the message
    let _ = stream.write_all(message.as_bytes()).await;
    println!("sent: {}", message);
}
