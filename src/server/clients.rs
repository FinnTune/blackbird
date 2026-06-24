use std::io::{self, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Mutex;

struct Client {
    id: SocketAddr,
    writer: TcpStream,
}

pub struct ClientRegistry {
    clients: Mutex<Vec<Client>>,
}

impl Default for ClientRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientRegistry {
    pub fn new() -> Self {
        Self {
            clients: Mutex::new(Vec::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.clients.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn register(&self, stream: TcpStream) -> io::Result<(SocketAddr, TcpStream)> {
        let id = stream.peer_addr()?;
        let writer = stream.try_clone()?;
        self.clients.lock().unwrap().push(Client { id, writer });
        Ok((id, stream))
    }

    pub fn remove(&self, id: SocketAddr) {
        self.clients
            .lock()
            .unwrap()
            .retain(|client| client.id != id);
    }

    pub fn broadcast(&self, message: &str) {
        self.clients
            .lock()
            .unwrap()
            .retain_mut(|client| writeln!(client.writer, "{}", message).is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::net::TcpListener;

    fn pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let address = listener.local_addr().expect("local address");
        let client = TcpStream::connect(address).expect("connect client");
        let (server, _) = listener.accept().expect("accept client");
        (client, server)
    }

    #[test]
    fn register_tracks_connected_clients() {
        let registry = ClientRegistry::new();
        let (client, server) = pair();

        let (id, _reader) = registry.register(server).expect("register client");

        assert_eq!(id, client.local_addr().expect("client local address"));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn broadcast_delivers_message_to_registered_client() {
        let registry = ClientRegistry::new();
        let (client, server) = pair();
        registry.register(server).expect("register client");

        registry.broadcast("hello, room");

        let mut reader = BufReader::new(client);
        let mut line = String::new();
        reader.read_line(&mut line).expect("read broadcast");
        assert_eq!(line, "hello, room\n");
    }

    #[test]
    fn remove_drops_client_from_registry() {
        let registry = ClientRegistry::new();
        let (_client, server) = pair();
        let (id, _) = registry.register(server).expect("register client");

        registry.remove(id);

        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn broadcast_prunes_disconnected_clients() {
        let registry = ClientRegistry::new();
        let (client, server) = pair();
        let (id, _) = registry.register(server).expect("register client");

        drop(client);

        registry.broadcast("anyone there?");

        assert_eq!(registry.len(), 0);
        registry.remove(id);
        assert_eq!(registry.len(), 0);
    }
}
