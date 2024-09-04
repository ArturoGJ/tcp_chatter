use std::{
    io::{stdin, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc::Sender, Arc},
    thread,
};

pub fn start_server(sender: Sender<String>) {
    let port = 0; // When using 0 the OS will provide an open port.
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).expect("Failed to get a listener");
    println!(
        "Listening on port: {}",
        listener
            .local_addr()
            .expect("Could not get local address of listener.")
    );
    println!(" ---------------- Server started ---------------- ");
    println!(" ---------------- Waiting for connections ---------------- ");
    for socket in listener.incoming() {
        let sender = sender.clone();
        match socket {
            Ok(socket) => {
                println!("Handling new client.");
                thread::spawn(move || {
                    start_client(socket, sender);
                });
            }
            Err(_) => eprintln!("Error getting socket stream."),
        }
    }
}

pub fn connect_to_server(port: &str, sender: Sender<String>) {
    let socket = TcpStream::connect(format!("127.0.0.1:{}", port))
        .expect(format!("Could not connect to port: {}", port).trim());
    start_client(socket, sender);
}

fn start_client(socket: TcpStream, sender: Sender<String>) {
    let socket = Arc::new(socket);
    let socket_clone = socket.clone();
    let addr = socket_clone.peer_addr().unwrap();
    println!("Messaging with: {}", addr);
    let receive_handle = thread::spawn(move || loop {
        let mut input_buffer = vec![0; 1024];
        socket_clone
            .as_ref()
            .read_exact(&mut input_buffer)
            .expect("Could not read into buffer.");

        let msg = input_buffer
            .iter()
            .take_while(|&&x| x != 0)
            .cloned()
            .collect();
        let msg = String::from_utf8(msg).expect("Failed to create message.");
        sender.send(msg).unwrap();
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
    receive_handle
        .join()
        .expect("Failed to create the receive handle.");
    println!("Receive handle: Start");
    send_handle
        .join()
        .expect("Failed to create the send handle.");
    println!("Send handle: END");
}
