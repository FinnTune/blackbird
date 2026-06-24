use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn spawn_incoming_reader(stream: TcpStream, label: String) {
    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("[{}]: {}", label, msg),
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });
}

pub fn run_interactive_session(
    mut stream: TcpStream,
    incoming_label: impl Into<String>,
) -> io::Result<()> {
    let reader_stream = stream.try_clone()?;
    spawn_incoming_reader(reader_stream, incoming_label.into());

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
