use std::io::{self, Write};
use std::net::TcpStream;

use crate::comms::{nickname_handshake, run_client_session};

pub fn start_client(address: &str, nickname: Option<&str>) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to server at {address}");

    if let Some(name) = nickname {
        writeln!(stream, "{}", nickname_handshake(name))?;
        stream.flush()?;
    }

    run_client_session(stream)
}
