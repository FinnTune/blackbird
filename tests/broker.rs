mod common;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use blackbird::server::{spawn_broker, ClientRegistry};

use common::{bind_ephemeral, connect_with_timeout, read_line, send_line};

#[test]
fn client_message_is_broadcast_to_other_clients() {
    let (listener, address) = bind_ephemeral();
    let _registry = spawn_broker(listener);

    let mut alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);

    let alice_addr = alice.local_addr().expect("alice local address");
    thread::sleep(Duration::from_millis(20));

    send_line(&mut alice, "hello everyone");

    let bob_line = read_line(&mut bob);
    assert_eq!(bob_line, format!("[{alice_addr}] hello everyone\n"));

    let alice_line = read_line(&mut alice);
    assert_eq!(alice_line, format!("[{alice_addr}] hello everyone\n"));
}

#[test]
fn server_broadcast_reaches_all_clients() {
    let (listener, address) = bind_ephemeral();
    let registry: Arc<ClientRegistry> = spawn_broker(listener);

    let mut alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);
    thread::sleep(Duration::from_millis(20));

    registry.broadcast("[server] attention please");

    assert_eq!(read_line(&mut alice), "[server] attention please\n");
    assert_eq!(read_line(&mut bob), "[server] attention please\n");
}

#[test]
fn disconnected_client_is_removed_from_broker() {
    let (listener, address) = bind_ephemeral();
    let registry = spawn_broker(listener);

    let alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);
    thread::sleep(Duration::from_millis(20));

    assert_eq!(registry.len(), 2);

    drop(alice);
    thread::sleep(Duration::from_millis(50));

    assert_eq!(registry.len(), 1);

    send_line(&mut bob, "still here");
    let bob_line = read_line(&mut bob);
    let bob_addr = bob.local_addr().expect("bob local address");
    assert_eq!(bob_line, format!("[{bob_addr}] still here\n"));
}
