use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";

fn main() {
    // connection message
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        // connected
        println!(
            "connected to: {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // write a hello message
        let message = "Hello, world!";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("sent: {}", message);

        // read the result
        let mut buff = [0; 1024];
        let len = stream.read(&mut buff).unwrap();
        let message = String::from_utf8_lossy(&buff);
        println!("received: {}", message);
    } else {
        println!("failed to connect to echo serve {}", ECHO_SERVER_ADDRESS);
    }
}
