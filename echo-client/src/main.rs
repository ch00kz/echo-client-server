use std::{env, net::TcpStream};

use common::message::{Message, MessageKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { name, addr } = parse_args()?;
    println!("[Connecting] {addr}");
    let mut stream = TcpStream::connect(addr.clone()).unwrap();
    let local_addr = stream.local_addr().unwrap();
    println!("[Connected] {}:{}", local_addr.ip(), local_addr.port());

    // tell the server who you are
    Message::write_message(&mut stream, Message::new(MessageKind::Iam, Some(name)));

    loop {
        // accept message from user
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;

        // writing message
        let message = Message::new(MessageKind::Standard, Some(buf.trim().to_string()));
        Message::write_message(&mut stream, message);

        // reading response to message
        let received_message = Message::read_message(&mut stream).unwrap();
        println!("{:?}", received_message);
    }
}

struct Args {
    name: String,
    addr: String,
}

fn parse_args() -> Result<Args, &'static str> {
    let args: Vec<String> = env::args().collect();
    if let (Some(name), Some(addr)) = (args.get(1), args.get(2)) {
        Ok(Args {
            name: name.clone(),
            addr: addr.clone(),
        })
    } else {
        Err("Please supply name and addr")
    }
}
