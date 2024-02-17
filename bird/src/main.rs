use std::str::FromStr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use uuid::Uuid;

const KATOO_SERVER_ADDRESS: &str = "127.0.0.1:8000";
const SERVER_ADDRESS: &str = "127.0.0.1:8001";

#[tokio::main]
async fn main() {
    println!("Bird starting {}", SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    println!("Bird listening {}", SERVER_ADDRESS);

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // create uuid
    let id = Uuid::new_v4();

    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);

    println!("received: {}", message);

    // call katoo
    let katoo_message = call_katoo(id, message.to_string()).await;
    let output = format!("Katoo says: {}", katoo_message);

    // send out message
    let _ = stream.write_all(output.as_bytes()).await;
    println!("{} - sent: {}", id, message);
}

async fn call_katoo(id: Uuid, message: String) -> String {
    // connection message
    println!("{} - connecting to Katoo {}", id, KATOO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(KATOO_SERVER_ADDRESS).await {
        // connected
        println!(
            "{} - connected to Katoo {}:{}",
            id,
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        let _ = stream.write_all(message.as_bytes()).await;
        println!("{} - sent {}", id, message);

        // read the result
        let mut buff = [0; 1024];
        let _len = stream.read(&mut buff).await.unwrap();
        let message = String::from_utf8_lossy(&buff);
        println!("{} - received from Katoo: {}", id, message);

        message.to_owned().to_string()
    } else {
        println!("failed to connect to Katoo server {}", KATOO_SERVER_ADDRESS);
        String::from_str("Katoo not available").unwrap()
    }
}
