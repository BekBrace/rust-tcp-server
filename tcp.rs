// Importing necessary modules from the Rust libraries
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // this line reads data from the stream and stores it in the buffer.
    stream.read(&mut buffer).expect("Failed to read from client!");
    // this line converts the data in the buffer into a UTF-8 enccoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

// Entry point
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            // stderr - standard error stream
            }
        }
    }
}
