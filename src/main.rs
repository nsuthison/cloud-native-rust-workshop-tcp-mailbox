use std::net::{TcpListener, TcpStream};

fn main() {
    let listerner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for connection_attempt in listerner.incoming() {
        match connection_attempt {
            Ok(stream) => {
                handle_client(stream)
            },
            Err(e) => {
                eprintln!("Error connecting: {}", e)
            }
        }
    }
}

fn handle_client(stream: TcpStream) -> () {
    println!("Client connected!!");

    drop(stream);
}