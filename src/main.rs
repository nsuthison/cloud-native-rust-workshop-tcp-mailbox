use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

#[derive(Debug, Eq, PartialEq)]
enum Request {
    Publish(String),
    Retrieve
}

fn main() {
    let listerner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for connection_attempt in listerner.incoming() {
        match connection_attempt {
            Ok(stream) => {
                handle_client(stream)
            }
            Err(e) => {
                eprintln!("Error connecting: {}", e)
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let line = read_line(&stream);
    let request = parse_request(line);

    match request {
        Request::Publish(msg) => {
            eprintln!("publishing message")
        }
        Request::Retrieve => {
            eprintln!("retrieving message")
        }
    }

    println!("Client connected!");
}

fn parse_request(line: String) -> Request {
    let trimmed = line.trim_end();

    if trimmed == "" {
        Request::Retrieve
    } else {
        Request::Publish(String::from(trimmed))
    }
}

fn read_line(stream: &TcpStream) -> String {
    let mut buffered_reader = BufReader::new(stream);

    let mut buf = String::new();
    buffered_reader.read_line(&mut buf).unwrap();

    buf
}