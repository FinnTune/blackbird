use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

pub fn run_client_session(stream: TcpStream) -> io::Result<()> {
    run_chat_session(stream, io::stdin().lock())
}

pub fn run_chat_session<R: BufRead>(mut stream: TcpStream, stdin: R) -> io::Result<()> {
    let reader_stream = stream.try_clone()?;
    thread::spawn(move || {
        let reader = BufReader::new(reader_stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("{}", msg),
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    run_stdin_loop(&mut stream, stdin)
}

fn run_stdin_loop<R: BufRead>(stream: &mut TcpStream, mut stdin: R) -> io::Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = stdin.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        if line.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        write!(stream, "{}", line)?;
        stream.flush()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn run_chat_session_sends_lines_until_exit() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let address = listener.local_addr().expect("local address");

        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept client");
            let mut received = String::new();
            BufReader::new(&mut stream)
                .read_line(&mut received)
                .expect("read line");
            received
        });

        let stream = TcpStream::connect(address).expect("connect");
        let input = Cursor::new(b"hello\nexit\n");
        run_chat_session(stream, input).expect("chat session");

        let received = server.join().expect("server thread");
        assert_eq!(received, "hello\n");
    }
}
