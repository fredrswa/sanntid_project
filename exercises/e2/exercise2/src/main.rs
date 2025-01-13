use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use std::io; 

const localIP: &str= "10.24.84.235";
const serverIP: &str= "{localIP}:20000";
const receivIP: &str= "{localIP}:20001";

// Asynchronous function to handle receiving and responding
async fn udp_receive(socket: &UdpSocket) -> io::Result<()> {
    let mut buffer = [0; 1024]; // Buffer to hold the incoming data

    // Receive data from the socket
    let (n_bytes, src) = socket.recv_from(&mut buffer).await?;

    // Adjust the buffer length to only include valid received data
    let buffer = &mut buffer[..n_bytes];

    // Reverse the buffer contents
    buffer.reverse();

    // Send the reversed data back to the source
    socket.send_to(buffer, &src).await?;
    Ok(())
}

// Asynchronous function to send a message
async fn udp_send(socket: &UdpSocket, addr: &str, message: &[u8]) -> io::Result<()> {
    socket.send_to(message, addr).await?;
    Ok(())
}

// Main entry point
#[tokio::main]
async fn main() {
    // Create a new UDP socket
    let socket = UdpSocket::bind(receivIP).await.unwrap();

    // Create a new runtime
    let mut rt = Runtime::new().unwrap();

    // Spawn a new task to handle receiving and responding
    rt.spawn(udp_receive(&socket));

    // Create a new message to send
    let message = b"Hello, world!";

    // Send the message to the server
    udp_send(&socket, serverIP, message).await.unwrap();
}
