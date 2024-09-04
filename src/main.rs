use std::env;

use tcp_chatter::{start_client_mode, start_server};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <server | client>", args[0]);
        return;
    }
    match args[1].as_str() {
        "server" => start_server(),
        "client" => start_client_mode(),
        _ => eprintln!("Invalid mode. Use 'server' or 'client'"),
    }
    println!("Shutting down.");
}
