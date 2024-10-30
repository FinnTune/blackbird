use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn handle_incoming_messages(stream: &mut TcpStream) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(msg) => println!("[Server]: {}", msg),
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}