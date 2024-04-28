pub mod message {
    use ::bincode;
    use serde::{Deserialize, Serialize};
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum MessageKind {
        Iam,
        Standard,
        Confirmation(uuid::Uuid),
        Kill,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Message {
        pub id: uuid::Uuid,
        pub kind: MessageKind,
        pub content: Option<String>,
        pub created_at: OffsetDateTime,
    }

    impl Message {
        pub fn new(kind: MessageKind, content: Option<String>) -> Message {
            Message {
                id: Uuid::new_v4(),
                kind,
                content,
                created_at: OffsetDateTime::now_utc(),
            }
        }

        pub fn is_iam_message(&self) -> bool {
            self.kind == MessageKind::Iam
        }

        pub fn get_iam_value(&self) -> Option<String> {
            match self.content.clone() {
                Some(content) => {
                    let parts: Vec<&str> = content.split("iam:").collect();
                    match parts.get(1) {
                        Some(name) => Some(name.to_string()),
                        None => None,
                    }
                }
                None => None,
            }
        }

        pub fn write_message(stream: &mut TcpStream, message: Message) -> () {
            let encoded: Vec<u8> = bincode::serialize(&message).unwrap();
            stream.write(&encoded).unwrap();
            stream.flush().unwrap();
        }

        pub fn read_message(stream: &mut TcpStream) -> Result<Message, Box<bincode::ErrorKind>> {
            let mut read_buffer: [u8; 1024] = [0; 1024];
            let num_bytes_read = stream.read(&mut read_buffer).unwrap();
            bincode::deserialize(&read_buffer[..num_bytes_read])
        }
    }
}
