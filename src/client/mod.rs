use std::io::{self, Write};
use std::net::{Shutdown, TcpStream};

use crate::comms::{nickname_handshake, run_client_session};

pub fn start_client(address: &str, nickname: Option<&str>) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to server at {address}");

    if let Some(name) = nickname {
        writeln!(stream, "{}", nickname_handshake(name))?;
        stream.flush()?;
    }

    install_disconnect_handler(stream.try_clone()?);

    run_client_session(stream)
}

fn install_disconnect_handler(stream: TcpStream) {
    ctrlc::set_handler(move || {
        println!("\n[system] disconnecting");
        let _ = stream.shutdown(Shutdown::Write);
        std::process::exit(0);
    })
    .expect("install Ctrl+C handler");
}
