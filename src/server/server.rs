use std::io;
use std::net::TcpListener;
use std::thread;

use crate::comms::run_interactive_session;

pub fn start_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Ok(addr) = stream.peer_addr() {
                    println!("Client connected: {}", addr);
                }
                thread::spawn(move || {
                    if let Err(e) = run_interactive_session(stream, "Client") {
                        eprintln!("Client session ended: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
