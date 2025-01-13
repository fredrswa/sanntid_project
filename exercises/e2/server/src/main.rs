use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind the listener to the address
    let ip = "10.100.23.23:33545";
    let listener = TcpListener::bind(ip).await?;

    println!("Server listening on {}", ip);

    loop {
        // Accept an incoming connection
        let (mut socket, _) = listener.accept().await?;

        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            // Read data from the socket
            match socket.read(&mut buffer).await {
                Ok(n) if n == 0 => return, // Connection was closed
                Ok(n) => {
                    // Print the received message
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

                    // Echo the message back to the client
                    if let Err(e) = socket.write_all(&buffer[..n]).await {
                        eprintln!("Failed to write to socket; err = {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket; err = {:?}", e);
                }
            }
        });
    }
}
