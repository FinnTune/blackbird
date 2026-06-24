use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

pub fn bind_ephemeral() -> (TcpListener, SocketAddr) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let address = listener.local_addr().expect("local address");
    (listener, address)
}

pub fn connect_with_timeout(address: SocketAddr) -> TcpStream {
    let stream = TcpStream::connect(address).expect("connect to broker");
    set_timeouts(&stream);
    stream
}

pub fn set_timeouts(stream: &TcpStream) {
    let timeout = Duration::from_secs(2);
    stream
        .set_read_timeout(Some(timeout))
        .expect("set read timeout");
    stream
        .set_write_timeout(Some(timeout))
        .expect("set write timeout");
}

pub fn send_line(stream: &mut TcpStream, line: &str) {
    writeln!(stream, "{line}").expect("write line");
    stream.flush().expect("flush line");
}

pub fn read_line(stream: &mut TcpStream) -> String {
    let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));
    let mut line = String::new();
    reader.read_line(&mut line).expect("read line");
    line
}
