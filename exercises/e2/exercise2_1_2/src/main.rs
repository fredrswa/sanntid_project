use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;


async fn read_data(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut buffer = vec![0; 1024]; // Buffer size of 1024 bytes

    // Read data from the stream asynchronously
    let n = match stream.read(&mut buffer).await{
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return Err(e.into());
        }
    };

    // n==0 in TCP indicates that the connection was closed by the server
    if n == 0 {
        println!("Server closed the connection.");
        return Ok(String::new());
    }

    let received_data = String::from_utf8_lossy(&buffer[..n]);
    Ok(received_data.to_string())
}

async fn init_stream(ip: &str) -> TcpStream {
    // Connect to the peer (make sure the server is running)
    let mut stream = TcpStream::connect(ip).await.unwrap();
    let data = read_data(&mut stream).await.unwrap();
    println!("{}", data);
    stream
}

fn read_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    //Trim removes the newline character added by read_line and then converts from &str to String
    // "\0" is added s.t. the message is null-terminated 
    input.trim().to_string() + "\0"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let mut stream = init_stream("10.100.23.23:33546").await;

    loop{
        // Read user input
        let input = read_user_input();

        stream.write_all(input.as_bytes()).await?;

        // Read data from the stream asynchronously
        // Data need to be read before writing
        let data = read_data(&mut stream).await?;
        println!("Received data => {} \n", data);
    }
    
    Ok(())
}
