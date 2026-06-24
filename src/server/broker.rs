use std::io::{self, BufRead, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use super::clients::ClientRegistry;

fn spawn_client_relay(registry: Arc<ClientRegistry>, stream: TcpStream, id: SocketAddr) {
    thread::spawn(move || relay_client_messages(registry, stream, id));
}

fn relay_client_messages(registry: Arc<ClientRegistry>, stream: TcpStream, id: SocketAddr) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(message) => {
                let formatted = format!("[{}] {}", id, message);
                println!("{}", formatted);
                registry.broadcast(&formatted);
            }
            Err(e) => {
                eprintln!("Client {} disconnected: {}", id, e);
                break;
            }
        }
    }
    registry.remove(id);
    println!("Client disconnected: {}", id);
}

pub fn spawn_broker(listener: TcpListener) -> Arc<ClientRegistry> {
    let registry = Arc::new(ClientRegistry::new());

    let registry_accept = Arc::clone(&registry);
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => match registry_accept.register(stream) {
                    Ok((id, stream)) => {
                        println!("Client connected: {}", id);
                        spawn_client_relay(registry_accept.clone(), stream, id);
                    }
                    Err(e) => eprintln!("Failed to register client: {}", e),
                },
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    });

    registry
}

pub fn start_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Listening on {}", address);

    let registry = spawn_broker(listener);

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

    Ok(())
}
