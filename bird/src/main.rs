use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";
const SERVER_ADDRESS: &str = "127.0.0.1:8001";

#[tokio::main]
async fn main() {
    println!("Karin starting {}", SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    println!("karin listening {}", SERVER_ADDRESS);

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);

    println!("received: {}", message);

    // call sirocco
    let sirocco_message = call_sirocco(message.to_string()).await;
    let output = format!("sirocco says: {}", sirocco_message);

    // write the message
    let _ = stream.write_all(output.as_bytes()).await;
    println!("sent: {}", output);
}

async fn call_sirocco(message: String) -> String {
    // connection message
    println!("connecting to sirocco {}", SIROCCO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(SIROCCO_SERVER_ADDRESS).await {
        // connected
        println!(
            "connected to sirocco {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent {} to Sirocco", message);

        // read the result
        let mut buff = [0; 1024];
        let _len = stream.read(&mut buff).await.unwrap();
        let message = String::from_utf8_lossy(&buff);
        println!("received from sirocco: {}", message);

        message.to_owned().to_string()
    } else {
        println!(
            "failed to connect to sirocco server {}",
            SIROCCO_SERVER_ADDRESS
        );
        "Failed".to_owned()
    }
}
