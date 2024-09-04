use std::{sync::mpsc, thread};

use tcp_chatter::start_server;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || start_server(tx));
    for msg in rx {
        print!(" - {}", msg);
    }
}
