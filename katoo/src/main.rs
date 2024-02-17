use std::env::args;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time::Duration};

const SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    // read args
    let delay = args()
        .nth(1)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();
    // starting
    println!("Katoo starting {}", SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    println!("Katoo listening {}", SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established");
        handle_connection(stream, delay);
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);

    println!("received: {}", message);
    // delay
    thread::sleep(Duration::from_millis(delay));

    // write the message
    let _ = stream.write_all(message.as_bytes());
    println!("sent: {}", message);
}
