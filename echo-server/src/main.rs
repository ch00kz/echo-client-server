use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use common::message::{Message, MessageKind};

const ECHO_SERVER_ADDR: &str = "127.0.0.1:8000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(ECHO_SERVER_ADDR)?;
    println!("Echo Server Listening: {:?}", listener);
    let address_book = Arc::new(Mutex::new(HashMap::<SocketAddr, String>::new()));
    let connections = Arc::new(Mutex::new(Vec::<TcpStream>::new()));

    for conn in listener.incoming() {
        // clones the arc pointers, making another reference to the same allocation
        let address_book = Arc::clone(&address_book);
        let connections = Arc::clone(&connections);

        println!("New connection");

        // spawn thread, so we can accept new connections, otherwise the loop blocks
        thread::spawn(move || {
            println!("New thread");
            let mut stream = conn.unwrap();
                        
            let peer_addr = stream.peer_addr().unwrap();
            println!("New connection from {peer_addr}");

            loop {
                // read incoming message
                let received_message = Message::read_message(&mut stream).unwrap();
                
                match received_message.kind {
                    MessageKind::Iam => {
                        println!("{:#?}", received_message);
                        if let Some(name) = received_message.content {
                            let mut book = address_book.lock().unwrap();
                            let mut active_connections = connections.lock().unwrap();
                            book.insert(peer_addr, name.to_string());
                            active_connections.push(stream.try_clone().unwrap());
                            // send confirmation of IAM
                            Message::write_message(
                                &mut stream,
                                &Message::new(MessageKind::Confirmation(received_message.id), None),
                            );
                        }
                    }
                    MessageKind::Standard => {
                        // print incoming message
                        let book = address_book.lock().unwrap();
                        let sender = match book.get(&peer_addr) {
                            Some(name) => name,
                            None => "Anonymous",
                        };
                        println!("From: {sender}\n{:#?}", received_message);
                        // send confirmation
                        Message::write_message(
                            &mut stream,
                            &Message::new(MessageKind::Confirmation(received_message.id), None)
                        );

                        // attempt to send message to all clients
                        let message_to_clients = Message::new(
                            MessageKind::Standard, 
                            Some(format!("{sender}: {}", received_message.content.unwrap()))
                        );
                        
                        let mut active_connections = connections.lock().unwrap();
                        println!("{:#?}", active_connections);
                        for conn in active_connections.iter_mut() {
                            if conn.peer_addr().unwrap().to_string() != peer_addr.to_string() {
                                Message::write_message(conn, &message_to_clients);
                            }
                        }
                    }
                    MessageKind::Kill => stream.shutdown(std::net::Shutdown::Both).unwrap(),
                    MessageKind::Confirmation(_) => panic!("Who is sending confirmation messages to the server?"),
                }
            }
        });
    }
    Ok(())
}
