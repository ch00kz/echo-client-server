use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener},
    sync::{Arc, Mutex},
    thread,
};

use common::message::{Message, MessageKind};

const ECHO_SERVER_ADDR: &str = "127.0.0.1:8000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(ECHO_SERVER_ADDR)?;
    println!("Echo Server Listening: {:?}", listener);
    let address_book = Arc::new(Mutex::new(HashMap::<SocketAddr, String>::new()));
    // let connections = Arc::new(Mutex::new(Vec::<TcpStream>::new()));

    for conn in listener.incoming() {
        // clones the arc pointers, making another reference to the same allocation
        let address_book = Arc::clone(&address_book);
        // let connections = Arc::clone(&connections);

        // spawn thread, so we can accept new connections, otherwise the loop blocks
        thread::spawn(move || {
            let mut stream = conn.unwrap();
            // let mut all_connections = connections.lock().unwrap();
            // all_connections.push(stream.try_clone().unwrap());
            let peer_addr = stream.peer_addr().unwrap();
            println!("New connection from {peer_addr}");

            loop {
                // read incoming message
                let received_message = Message::read_message(&mut stream).unwrap();
                println!("{:?}", received_message);

                // Client is identifying themself
                if received_message.is_iam_message() {
                    if let Some(name) = received_message.get_iam_value() {
                        let mut book = address_book.lock().unwrap();
                        book.insert(peer_addr, name.to_string());
                        Message::write_message(
                            &mut stream,
                            Message::new(MessageKind::Confirmation(received_message.id), None),
                        );
                    }
                } else {
                    Message::write_message(
                        &mut stream,
                        Message::new(MessageKind::Confirmation(received_message.id), None),
                    )
                    // // echo received messaged to all clients
                    // for client_conn in all_connections.iter_mut() {
                    //     client_conn.write(msg.as_bytes()).unwrap();
                    //     client_conn.flush().unwrap();
                    // }
                }
            }
        });
    }
    Ok(())
}
