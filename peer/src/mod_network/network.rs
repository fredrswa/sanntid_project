use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use crossbeam_channel as cbc;


//Should include heartbeat?
pub fn udp_receive(socket: UdpSocket, udp_listener_tx: cbc::Sender<String>) {
    let mut buffer = [0; 1024];

    loop {
        let (n_bytes, _src) = match socket.recv_from(&mut buffer){
            Ok((n_bytes, _src)) => (n_bytes, _src),
            Err(e) => {
                panic!("An error occurred when recieving from UdpSocket: {}", e);
            }
        };

        let received_msg = String::from_utf8_lossy(&buffer[..n_bytes]);
        udp_listener_tx.send(received_msg.to_string());
    }
}

/* 
                                            //Hva skal egentlig sendes?
fn udp_send(socket: &UdpSocket, addr: &str, message: &ElevatorState) {
    let json_msg = match serde_json::to_string(&ElevatorMessage {elevator_state: message.clone()}){
        Ok(json_msg) => json_msg,
        Err(e) => {
            panic!("Failed to serialize message to send over Udp!: {}", e)
        }    
    };
    
    match socket.send_to(json_msg.as_bytes(), addr) {
        Ok(ok) => ok,
        Err(e) => {
            panic!("Failed to send message {:#?} on adress {:#?}: \n {}", json_msg, addr, e)
        }
    };
}
    */