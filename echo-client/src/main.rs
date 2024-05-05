use std::{env, net::TcpStream, thread};

use common::message::{Message, MessageKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { name, addr } = parse_args()?;
    println!("[Connecting] {addr}");
    let mut stream = TcpStream::connect(addr.clone()).unwrap();
    let local_addr = stream.local_addr().unwrap();
    println!("[Connected] {}:{}", local_addr.ip(), local_addr.port());

    // Tell the server who you are
    Message::write_message(&mut stream, &Message::new(MessageKind::Iam, Some(name)));

    let read_stream = stream.try_clone().unwrap();
    thread::spawn(move || receive_messages_from_server(read_stream));

    loop {
        send_messages_to_server(&mut stream)?
    }
}

struct Args {
    name: String,
    addr: String,
}

fn receive_messages_from_server(mut stream: TcpStream) {
    loop {
        // reading response to message
        let received_message = Message::read_message(&mut stream).unwrap();
        match received_message.kind {
            MessageKind::Standard => print_message(&received_message),
            _ => {}
        }
    }
}

fn send_messages_to_server(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // accept message content from user
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    // writing message
    Message::write_message(
        stream,
        &Message::new(MessageKind::Standard, Some(buf.trim().to_string())),
    );

    Ok(())
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

fn print_message(message: &Message) {
    println!(
        "[{}] {}",
        message.created_at,
        message.content.clone().unwrap()
    )
}
