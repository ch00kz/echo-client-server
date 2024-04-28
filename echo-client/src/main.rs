use std::{
    env,
    io::prelude::{Read, Write},
    net::TcpStream,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { addr } = parse_args()?;
    println!("[Connecting] {addr}");
    let mut stream = TcpStream::connect(addr.clone()).unwrap();
    let local_addr = stream.local_addr().unwrap();
    println!("[Connected] {}:{}", local_addr.ip(), local_addr.port());

    loop {
        // accept message from user
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        let message = buf.trim();

        // writing message
        let num_bytes_sent = stream.write(format!("{message}").as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("[Sent] ({num_bytes_sent} bytes): {}", message);
        
        // reading response to message
        let mut read_buffer: [u8; 1024] = [0; 1024];
        let num_bytes_read = stream.read(&mut read_buffer).unwrap();
        println!(
            "[Received] ({num_bytes_read} bytes): {}",
            String::from_utf8_lossy(&read_buffer[..num_bytes_read])
        );
    }
}

struct Args {
    addr: String,
}

fn parse_args() -> Result<Args, &'static str> {
    let args: Vec<String> = env::args().collect();
    if let Some(addr) = args.get(1) {
        Ok(Args { addr: addr.clone() })
    } else {
        Err("Socket address is missing from command line arguments")
    }
}
