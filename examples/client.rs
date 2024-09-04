use std::{
    io::{self, Write},
    sync::mpsc,
    thread,
};

use tcp_chatter::connect_to_server;

fn main() {
    print!("Type port to connect to: ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || connect_to_server(input.trim(), tx));

    for msg in rx {
        print!(" - {}", msg);
    }
}
