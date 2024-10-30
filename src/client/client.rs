use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

use crate::comms::comms::handle_incoming_messages;

pub fn start_client(address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to server at {}", address);

    let mut stream_clone = stream.try_clone()?;
    thread::spawn(move || handle_incoming_messages(&mut stream_clone));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        writeln!(stream, "{}", line)?;
    }
    Ok(())
}

pub fn handle_client(mut stream: TcpStream) {
    let reader = BufReader::new(stream.try_clone().unwrap());

    // Handle incoming messages in a separate thread
    thread::spawn(move || {
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("[Client]: {}", msg),
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    // Send messages from server's stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        if writeln!(stream, "{}", line).is_err() {
            eprintln!("Failed to send message");
            break;
        }
    }
}
