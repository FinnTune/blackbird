mod common;

use blackbird::server::spawn_broker;

use common::{
    bind_ephemeral, connect_with_timeout, read_chat_line, read_line, send_line, wait_for_clients,
};

#[test]
fn client_message_is_broadcast_to_other_clients() {
    let (listener, address) = bind_ephemeral();
    let registry = spawn_broker(listener);

    let mut alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);
    wait_for_clients(&registry, 2);

    let alice_addr = alice.local_addr().expect("alice local address");

    send_line(&mut alice, "hello everyone");

    assert_eq!(
        read_chat_line(&mut bob),
        format!("[{alice_addr}] hello everyone\n")
    );
}

#[test]
fn server_broadcast_reaches_all_clients() {
    let (listener, address) = bind_ephemeral();
    let registry = spawn_broker(listener);

    let mut alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);
    wait_for_clients(&registry, 2);

    registry.broadcast("[server] attention please");

    assert_eq!(read_line(&mut alice), "[server] attention please\n");
    assert_eq!(read_line(&mut bob), "[server] attention please\n");
}

#[test]
fn nickname_is_used_in_broadcast_messages() {
    let (listener, address) = bind_ephemeral();
    let registry = spawn_broker(listener);

    let mut alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);
    wait_for_clients(&registry, 2);

    send_line(&mut alice, "NICK:alice");
    send_line(&mut bob, "NICK:bob");
    std::thread::sleep(std::time::Duration::from_millis(50));

    send_line(&mut alice, "hello bob");

    assert_eq!(read_chat_line(&mut bob), "[alice] hello bob\n");
}

#[test]
fn join_and_leave_messages_are_broadcast() {
    let (listener, address) = bind_ephemeral();
    let registry = spawn_broker(listener);

    let mut alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);
    wait_for_clients(&registry, 2);

    send_line(&mut alice, "NICK:alice");
    assert_eq!(read_line(&mut bob), "[system] alice joined\n");

    drop(alice);
    for _ in 0..50 {
        if registry.len() == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    assert_eq!(read_line(&mut bob), "[system] alice left\n");
}

#[test]
fn disconnected_client_is_removed_from_broker() {
    let (listener, address) = bind_ephemeral();
    let registry = spawn_broker(listener);

    let alice = connect_with_timeout(address);
    let mut bob = connect_with_timeout(address);
    wait_for_clients(&registry, 2);

    drop(alice);
    for _ in 0..50 {
        if registry.len() == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    assert_eq!(registry.len(), 1);

    send_line(&mut bob, "still here");
    assert_eq!(registry.len(), 1);
}
