mod client;
mod server;
mod comms;
fn main() {
    println!("-----------------------------------------");
    println!("-------------Hello, Nerds!---------------");
    println!("-----------------------------------------");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        eprintln!("Usage: {} <listen|connect> <address:port>", args[0]);
        eprintln!("Arg length: {}\nArgs: {:?}", args.len(), args);
        return;
    } else if args.len() == 4 {
        let arg1 = &args[0];
        let arg2 = &args[1];
        let arg3 = &args[2];
        let arg4 = &args[3];
        println!("Arg1 = {}\nArg2 = {}\nArg3 = {}\nArg4 = {}", arg1, arg2, arg3, arg4);
    }

    match args[2].as_str() {
        "listen" => server::server::start_server(&args[3]).expect("Failed to start server"),
        "connect" => client::client::start_client(&args[3]).expect("Failed to start client"),
        _ => eprintln!("Invalid mode: choose 'listen' or 'connect'"),
    }
}
