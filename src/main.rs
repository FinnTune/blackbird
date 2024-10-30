fn main() {
    println!("-----------------------------------------");
    println!("-------------Hello, Nerds!---------------");
    println!("-----------------------------------------");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 5 {
        println!("Usage: cargo run . <arg1> <arg2> <arg3>");
        println!("Arg length: {}\nArgs: {:?}", args.len(), args);
        return;
    } else if args.len() == 5 {
        let arg1 = &args[0];
        let arg2 = &args[1];
        let arg3 = &args[2];
        let arg4 = &args[3];
        let arg5 = &args[4];
        println!("Arg1 = {}\nArg2 = {}\nArg3 = {}\nArg4 = {}\nArg5 = {}", arg1, arg2, arg3, arg4, arg5);
    }
}
