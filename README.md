# blackbird

A small TCP line-chat program written in Rust. Run a server, connect one or more clients, and broadcast messages to everyone in the room.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition)

## Build

```bash
cargo build --release
```

## Usage

Start a server:

```bash
cargo run -- listen 127.0.0.1:8989
```

Connect a client (in another terminal):

```bash
cargo run -- connect 127.0.0.1:8989
cargo run -- connect --name alice 127.0.0.1:8989
```

Type a message and press Enter to send. Type `exit` to leave a client session or stop the server. Press `Ctrl+C` on the server to shut down and notify connected clients.

Messages from clients are prefixed with their nickname when `--name` is provided, otherwise their address. Messages from the server operator are prefixed with `[server]`.

### Example session

**Server**

```
Listening on 127.0.0.1:8989
Client connected: 127.0.0.1:51234
Client connected: 127.0.0.1:51235
[alice] hello everyone
[server] welcome!
```

**Client**

```
Connected to server at 127.0.0.1:8989
[alice] hello everyone
[server] welcome!
```

## Development

```bash
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## License

Copyright (C) 2026 Andre Teetor

This project is licensed under the GNU General Public License v2.0 —
see the [LICENSE](LICENSE) file for details.
