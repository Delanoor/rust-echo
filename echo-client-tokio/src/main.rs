use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

#[tokio::main]
async fn main() {
    println!("connecting to: {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        // connected
        println!(
            "connected to echo server {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // send a message
        let message = "Hello, HWN";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);

        // read the result

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message);
    } else {
        println!("failed to connect to ")
    }
}
