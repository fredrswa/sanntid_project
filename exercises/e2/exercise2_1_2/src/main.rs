use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the peer (make sure the server is running)
    let mut stream = TcpStream::connect("10.100.23.22:33546").await?;


    //stream.flush().await?;
    // Write the data.
    stream.write_all(b"hello world!").await?;

    // Ensure the data is flushed to the stream
    //stream.flush().await?;

    println!("Message sent to the server");

    Ok(())
}
