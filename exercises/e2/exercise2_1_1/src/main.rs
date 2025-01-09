use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use std::io; 

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
    // Bind the UDP socket to the local address
    let socket = UdpSocket::bind("10.100.23.22:20001").await.unwrap();

    // Example of sending a message
    udp_send(&socket, "10.100.23.22:20000", b"Hello, world!").await.unwrap();

    // Example of receiving and responding to a message
    udp_receive(&socket).await.unwrap();
}
