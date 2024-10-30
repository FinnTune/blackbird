use std::io::{self};
use std::net::TcpListener;
use std::thread;

use crate::client::client::handle_client;

pub fn start_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}