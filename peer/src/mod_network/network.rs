use std::io;
use std::net::UdpSocket;
use std::thread;
use crossbeam_channel::{unbounded, Sender, Receiver};

fn udp_receive(socket: UdpSocket, rx: Receiver<String>) -> io::Result<()> {
    let mut buffer = [0; 1024];
    loop {
        // Check if there is a message from the sender to stop receiving
        if let Ok(message) = rx.try_recv() {
            println!("Received message to stop: {}", message);
            break;
        }

        let (n_bytes, _src) = socket.recv_from(&mut buffer)?;
        println!("Received message: {}", String::from_utf8_lossy(&buffer[..n_bytes]));
    }
    Ok(())
}

fn udp_send(socket: &UdpSocket, addr: &str, message: &[u8]) -> io::Result<()> {
    socket.send_to(message, addr)?;
    Ok(())
}

pub fn test_script_network_module() {
    let socket = match UdpSocket::bind("10.100.23.23:20002") {
        Ok(socket) => socket,
        Err(e) => {
            println!("Failed to bind socket: \"{}\"", e);
            return;
        }
    };

    let (tx, rx) = unbounded::<String>();
    {
        let socket = socket.try_clone();
        thread::spawn(move || {
            udp_receive(socket.unwrap(), rx);                
        });
    }

    udp_send(&socket, "10.100.23.23:20000", b"Listening: ID:2");

    loop {
        thread::sleep(std::time::Duration::from_millis(200));
        udp_send(&socket, "10.100.23.23:20000",b"Heartbeat 2");
    }
    tx.send("Stop receiving".to_string()).unwrap();
}