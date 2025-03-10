use std::net::UdpSocket;
use std::io;
use crossbeam_channel::Sender;

fn udp_receive( socket: UdpSocket, udp_listener_tx: Sender<String>, udp_heartbeat_dead_tx: Sender<String>) -> io::Result<()> {
    let mut buffer = [0; 1024]; 

    loop {
        // receive udp message
        let (n_bytes, src) = socket.recv_from(&mut buffer)?;

        // convert to string
        let message = String::from_utf8_lossy(&buffer[..n_bytes]).to_string();

        // Send meldingen til riktig kanal
        if message.contains("heartbeat_dead") {
            udp_heartbeat_dead_tx.send(message).unwrap();
        } else {
            udp_listener_tx.send(message).unwrap();
        }
    }
}
