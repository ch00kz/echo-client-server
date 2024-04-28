use std::{
    io::{Read, Write},
    net::TcpListener,
};

const ECHO_SERVER_ADDR: &str = "127.0.0.1:8000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(ECHO_SERVER_ADDR)?;
    println!("Echo Server Listening: {:?}", listener);

    for connection in listener.incoming() {
        let mut stream = connection?;

        // read incoming message
        let mut read_buffer = [0; 1024];
        let num_bytes_read = stream.read(&mut read_buffer)?;
        let message = String::from_utf8_lossy(&read_buffer[..num_bytes_read]);
        println!("[Received] ({num_bytes_read} bytes) {message}");

        // echo received messaged
        let num_bytes_written = stream.write(message.as_bytes())?;
        stream.flush()?;
        println!("[Sent] ({num_bytes_written} bytes) {message}");
    }
    Ok(())
}