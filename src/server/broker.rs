use std::io::{self, BufRead, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use super::clients::ClientRegistry;
use crate::comms::parse_nickname_handshake;

pub struct Broker {
    registry: Arc<ClientRegistry>,
    shutdown: Arc<AtomicBool>,
}

impl Broker {
    pub fn spawn(listener: TcpListener) -> Self {
        let shutdown = Arc::new(AtomicBool::new(true));
        let registry = Arc::new(ClientRegistry::new());

        listener
            .set_nonblocking(true)
            .expect("set listener to non-blocking");

        let registry_accept = Arc::clone(&registry);
        let shutdown_accept = Arc::clone(&shutdown);
        thread::spawn(move || accept_connections(listener, registry_accept, shutdown_accept));

        Self { registry, shutdown }
    }

    pub fn registry(&self) -> Arc<ClientRegistry> {
        Arc::clone(&self.registry)
    }

    pub fn install_shutdown_handler(&self) -> Result<(), ctrlc::Error> {
        let registry = Arc::clone(&self.registry);
        let shutdown = Arc::clone(&self.shutdown);
        ctrlc::set_handler(move || {
            shutdown_broker(&registry, &shutdown);
            std::process::exit(0);
        })
    }

    pub fn shutdown(&self) {
        shutdown_broker(&self.registry, &self.shutdown);
    }
}

fn shutdown_broker(registry: &ClientRegistry, shutdown: &AtomicBool) {
    shutdown.store(false, Ordering::Relaxed);
    println!("[system] shutting down");
    registry.broadcast("[system] server shutting down");
    registry.disconnect_all();
}

fn accept_connections(
    listener: TcpListener,
    registry: Arc<ClientRegistry>,
    shutdown: Arc<AtomicBool>,
) {
    while shutdown.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => match registry.register(stream) {
                Ok((id, stream)) => {
                    println!("Client connected: {}", id);
                    spawn_client_relay(registry.clone(), stream, id);
                }
                Err(e) => eprintln!("Failed to register client: {}", e),
            },
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                if shutdown.load(Ordering::Relaxed) {
                    eprintln!("Connection failed: {}", e);
                }
                break;
            }
        }
    }
}

fn spawn_client_relay(registry: Arc<ClientRegistry>, stream: TcpStream, id: SocketAddr) {
    thread::spawn(move || relay_client_messages(registry, stream, id));
}

fn relay_client_messages(registry: Arc<ClientRegistry>, stream: TcpStream, id: SocketAddr) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(message) => {
                if let Some(nickname) = parse_nickname_handshake(&message) {
                    if registry.set_nickname(id, nickname) {
                        println!(
                            "[system] {} is now known as {}",
                            id,
                            registry.display_name(id)
                        );
                    }
                    continue;
                }

                let sender = registry.display_name(id);
                let formatted = format!("[{sender}] {message}");
                println!("{formatted}");
                registry.broadcast(&formatted);
            }
            Err(e) => {
                eprintln!("Client {} disconnected: {}", registry.display_name(id), e);
                break;
            }
        }
    }
    let name = registry.display_name(id);
    registry.remove(id);
    println!("Client disconnected: {name}");
}

pub fn spawn_broker(listener: TcpListener) -> Arc<ClientRegistry> {
    Broker::spawn(listener).registry()
}

pub fn start_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Listening on {}", address);

    let broker = Broker::spawn(listener);
    broker
        .install_shutdown_handler()
        .expect("install Ctrl+C handler");

    let registry = broker.registry();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        let message = format!("[server] {}", line);
        println!("{}", message);
        registry.broadcast(&message);
    }

    broker.shutdown();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn shutdown_notifies_connected_clients() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let address = listener.local_addr().expect("local address");
        let broker = Broker::spawn(listener);

        let mut client = TcpStream::connect(address).expect("connect client");
        client
            .set_read_timeout(Some(Duration::from_secs(2)))
            .expect("set read timeout");

        for _ in 0..50 {
            if broker.registry().len() == 1 {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(broker.registry().len(), 1);

        broker.shutdown();

        let mut line = String::new();
        BufReader::new(&mut client)
            .read_line(&mut line)
            .expect("read shutdown notice");
        assert_eq!(line, "[system] server shutting down\n");
        assert!(broker.registry().is_empty());
    }
}
