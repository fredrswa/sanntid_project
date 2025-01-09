//Local IP 10.100.23.22
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use std::io;

async fn udp_receive(socket: &UdpSocket) -> io::Result<()> {
    let mut buffer = [0; 1024];
    // bytes = antall bytes som ble lagret of motatt i bufferen
    let (n_bytes, src) = socket.recv_from(&mut buffer).await?;

    let buffer = &mut buffer[..n_bytes];
    buffer.reverse();
    socket.send_to(buffer, &src).await?;
    Ok(())
}

async fn udp_send(socket: &UdpSocket, addr: &str, message: &[u8]) -> io::Result<()> {
    socket.send_to(message, addr).await?;
    Ok(())
}

fn main() {
    test();
}


async fn test () {
    let rt = Runtime::new().unwrap();
    let socket = UdpSocket::bind("10.100.23.22:30000").await.unwrap();

    rt.block_on(async {
        // Example of sending a message
        udp_send(&socket, "10.100.23.22:30000", b"Hello, world!").await.unwrap();

        // Example of receiving and responding to a message
        udp_receive(&socket).await.unwrap();
    });
}
