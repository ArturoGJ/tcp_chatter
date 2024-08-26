use std::{
    env,
    io::{stdin, Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    start_server(args.get(1));
    println!("Shutting down.");
}

fn start_server(port: Option<&String>) {
    if let Some(port) = port {
        let socket_stream = TcpStream::connect(format!("127.0.0.1:{}", 8137)).unwrap();
        println!("Connected to port: {}", port);
        start_jobs(socket_stream);
    } else {
        let port = 8137;
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(addr).expect("Failed to get a listener");
        println!("Listening on port: {}", port);
        for socket_stream in listener.incoming() {
            start_jobs(socket_stream.unwrap());
        }
    }
}

fn start_jobs(socket: TcpStream) {
    let socket = Arc::new(socket);
    let socket_clone = socket.clone();
    let addr = socket_clone.peer_addr().unwrap();
    println!("Messaging with: {}", addr);
    let receive_handle = thread::spawn(move || loop {
        let mut input_buffer = vec![0; 1024];
        socket_clone.as_ref().read_exact(&mut input_buffer).unwrap();
        let msg = input_buffer
            .iter()
            .take_while(|&&x| x != 0)
            .cloned()
            .collect();
        let msg = String::from_utf8(msg).unwrap();
        print!(" - {}", msg);
    });

    let socket_clone = socket.clone();
    let send_handle = thread::spawn(move || {
        let mut send_msg = String::new();

        while stdin().read_line(&mut send_msg).is_ok() {
            send_msg = send_msg.trim().to_string() + "\n";
            let mut buff = send_msg.clone().into_bytes();
            buff.resize(1024, 0);
            socket_clone.as_ref().write_all(&buff).unwrap();
            send_msg.clear();
        }
    });
    receive_handle.join().unwrap();
    println!("Receive handle: Start");
    send_handle.join().unwrap();
    println!("Send handle: END");
}
