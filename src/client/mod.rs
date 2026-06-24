use std::io;
use std::net::TcpStream;

use crate::comms::run_client_session;

pub fn start_client(address: &str) -> io::Result<()> {
    let stream = TcpStream::connect(address)?;
    println!("Connected to server at {}", address);
    run_client_session(stream)
}
