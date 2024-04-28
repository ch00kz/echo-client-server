use std::{
    io::{
        self,
        prelude::{Read, Write},
    },
    net::TcpStream,
};

// const ECHO_SERVER: &str = "tcpbin.com:4242";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter addr of echo-server");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let addr = input.trim();
    println!("[Connecting] {addr}");
    let mut stream = TcpStream::connect(addr).unwrap();
    let local_addr = stream.local_addr().unwrap();
    println!("[Connected] {}:{}", local_addr.ip(), local_addr.port());

    // writing message
    let message = "Hello World!";
    let terminal_char = "\n";
    let num_bytes_sent = stream
        .write(format!("{message}{terminal_char}").as_bytes())
        .unwrap();
    stream.flush().unwrap();
    println!("[Sent] ({num_bytes_sent} bytes): {}", message);

    // reading message
    let mut read_buffer: [u8; 1024] = [0; 1024];
    let num_bytes_read = stream.read(&mut read_buffer).unwrap();
    println!(
        "[Received] ({num_bytes_read} bytes): {}",
        String::from_utf8_lossy(&read_buffer[..num_bytes_read])
    );

    Ok(())
}
