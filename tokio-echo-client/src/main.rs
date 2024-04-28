use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const ECHO_SERVER: &str = "tcpbin.com:4242";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Connecting] {ECHO_SERVER}");
    let mut stream = TcpStream::connect(ECHO_SERVER).await?;

    let addr = stream.local_addr().unwrap();
    println!("[Connected] {}:{}", addr.ip(), addr.port());

    // writing message
    let message = "Hello World!";
    let terminal_char = "\n";
    let num_bytes_sent = stream
        .write(format!("{message}{terminal_char}").as_bytes())
        .await?;
    stream.flush().await?;
    println!("[Sent] ({num_bytes_sent} bytes): {}", message);

    // reading message
    let mut read_buffer: [u8; 1024] = [0; 1024];
    let num_bytes_read = stream.read(&mut read_buffer).await?;
    println!(
        "[Received] ({num_bytes_read} bytes): {}",
        String::from_utf8_lossy(&read_buffer[..num_bytes_read])
    );

    Ok(())
}
