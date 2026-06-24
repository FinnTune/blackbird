mod client;
mod comms;
mod server;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "blackbird", about = "TCP line chat")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Listen for incoming connections
    Listen {
        /// Address and port to bind (e.g. 127.0.0.1:8989)
        address: String,
    },
    /// Connect to a server
    Connect {
        /// Address and port to connect to (e.g. 127.0.0.1:8989)
        address: String,
    },
}

fn main() {
    println!("-----------------------------------------");
    println!("-------------Hello, Nerds!---------------");
    println!("-----------------------------------------");

    let cli = Cli::parse();

    match cli.command {
        Command::Listen { address } => {
            server::start_server(&address).expect("Failed to start server");
        }
        Command::Connect { address } => {
            client::start_client(&address).expect("Failed to start client");
        }
    }
}
