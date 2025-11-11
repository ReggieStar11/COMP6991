//! A simple web server
//! which serves some html at index.html
//! and replaces triple curly braces with the given variable
mod test;
use std::io::{Read, Write};

use std::net::{TcpListener, TcpStream};
// hint, hint
use std::sync::{Arc, Mutex};

struct State {
    counter: i32,
}

fn handle_client(mut stream: TcpStream, state: std::sync::Arc<std::sync::Mutex<State>>) {
    // setup buffer, and read from stream into buffer
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // convert request payload to string
    let string = String::from_utf8_lossy(&buffer);

    // extract header
    let mut split = string.split("\r\n");
    let header = split.next().unwrap();

    if header == "POST /counter HTTP/1.1" {
        if let Ok(mut guard) = state.lock() {
            guard.counter += 1;
        }
    }

    let template = include_bytes!("../index.html");


    let file: Vec<u8> = {
        let s = String::from_utf8_lossy(template);
        let counter_val = if let Ok(guard) = state.lock() {
            guard.counter
        } else {
            0
        };
        let replaced = s.replace("{{{ counter }}}", &counter_val.to_string());
        replaced.into_bytes()
    };

    // DONT CHANGE ME
    let response = format!(
        "HTTP/1.1 200 OK\r\nContentType: text/html; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n",
        file.len()
    );

    // converts response to &[u8], and writes them to the stream
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&file).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(1).unwrap_or("8081".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;

    println!("Server running on port {}", port);
    // create new state
    let state = Arc::new(Mutex::new(State { counter: 0 }));

    // accept connections and process them, spawning a thread per connection
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let state = Arc::clone(&state);
        std::thread::spawn(move || {
            handle_client(stream, state);
        });
    }
    Ok(())
}
