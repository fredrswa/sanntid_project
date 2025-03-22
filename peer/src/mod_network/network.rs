//Handles peer-to-peer communication between elevators using UDP and JSON-based messages 
// Detetect dead elevators by sending and receiving heartbeats

///Includes
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::net::UdpSocket;
use std::io;

///
use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel as cbc;
use std::thread;

///Crates
use crate::config::*;

//Constants
const TIMEOUT_MS: u64 = 5000; // how long before we consider an elevator dead
const CHECK_INTERVAL_MS: u64 = 1000; // how often we check for dead elevators


pub fn udp_create_socket(addr: &String) -> UdpSocket {
    let socket = match UdpSocket::bind(addr) {
        Ok(socket) => {
            println!("Socket bound successfully.");
            socket
        },
        Err(e) => {
            panic!("Could'nt bind to socket: {}", e);
        }
    };
    return socket;
}

/* pub fn udp_receive(socket: UdpSocket, udp_listener_tx: Sender<String>) -> io::Result<()> {
    
    let mut heartbeats: HashMap<String, Instant> = HashMap::new();
    let id_self = &CONFIG.id;
    let id_1 = "id_1".to_string();
    let id_2 = "id_2".to_string();

    let mut buffer = [0; 1024]; 


    loop {
        match socket.recv_from(&mut buffer) {
            Ok((n_bytes, _src)) => {
                let message = String::from_utf8_lossy(&buffer[..n_bytes]).to_string();
    
                if message.contains("heartbeat") {
                    if message.contains(&id_1) {
                        heartbeats.insert(id_1.clone(), Instant::now());
                    } else if message.contains(&id_2) {
                        heartbeats.insert(id_2.clone(), Instant::now());
                    }
                } else {
                    udp_listener_tx.send(message).unwrap();
                }
            }
            Err(_) => {
                let now = Instant::now();
                let mut dead_elevators = Vec::new();
    
                // check if one of the elevators is dead
                if let Some(last_seen) = heartbeats.get(&id_1) {
                    if now.duration_since(*last_seen).as_millis() > TIMEOUT_MS as u128 {
                        dead_elevators.push(id_1.clone());
                    }
                } else {
                    dead_elevators.push(id_1.clone());
                }
    
                if let Some(last_seen) = heartbeats.get(&id_2) {
                    if now.duration_since(*last_seen).as_millis() > TIMEOUT_MS as u128 {
                        dead_elevators.push(id_2.clone());
                    }
                } else {
                    dead_elevators.push(id_2.clone());
                }
    
                // if only one elevator is dead, send a heartbeat_dead message
                for id in &dead_elevators {
                    udp_heartbeat_dead_tx.send(format!("heartbeat_dead {}", id)).unwrap();
                    heartbeats.remove(id);
                }
    
                // if both elevators are dead, send a self_dead message
                if dead_elevators.len() == 2 {
                    udp_heartbeat_dead_tx.send(format!("self_dead {}", id_self)).unwrap();
                }
    
                std::thread::sleep(Duration::from_millis(CHECK_INTERVAL_MS));
            }
        }
    }
}
 */

 //Receive UDP messages and send them to the channel
 // Message contain the entire system state
pub fn udp_receive(socket: &UdpSocket, udp_listener_tx: Sender<EntireSystem>) {
    let mut buffer = [0; 1024];

    loop {
        let (n_bytes, _src) = match socket.recv_from(&mut buffer){
            Ok((n_bytes, _src)) => (n_bytes, _src),
            Err(e) => {
                panic!("An error occurred when recieving from UdpSocket: {}", e);
            }
        };

        let received_msg = String::from_utf8_lossy(&buffer[..n_bytes]).to_string();

        let sys: EntireSystem = match serde_json::from_str(&received_msg) {
            Ok(sys) => sys,
            Err(e) => {
                panic!("Failed to parse incoming state!: {}", e)
            }
        };

        match udp_listener_tx.send(sys) {
            Ok(ok) => ok,
            Err(e) => {panic!("Message was not sent to peer: {}", e)}
        };
    }
}


                                             //Hva skal egentlig sendes?
pub fn udp_send(socket: &UdpSocket, peer_addresses: String, udp_sender_rx: Receiver<EntireSystem>) {  
    loop {
        cbc::select! {
            recv(udp_sender_rx) -> sys => {
                let sys = sys.unwrap();
                
                let json_msg = match serde_json::to_string(&sys){
                    Ok(json_msg) => json_msg,
                    Err(e) => {
                        panic!("Failed to serialize message to send over Udp!: {}", e)
                    }    
                };
        
                    match socket.send_to(json_msg.as_bytes(), peer_addresses.clone()) {
                        Ok(ok) => ok,//Ack send to io
                        Err(e) => {
                            panic!("Failed to send message {:#?} on adress {:#?}: \n {}", json_msg, peer_addresses, e)
                        }
                    };
                
            }
        }
    }  
} 

//Send heartbeats to all peers to indicate that the elevator is still alive
pub fn send_heartbeat(heartbeat_socket: &UdpSocket, peer_id: &String, peer_addresses: &Vec<String>) -> std::io::Result<()> {
    loop {
        for peer_address in peer_addresses.iter(){
            match heartbeat_socket.send_to( &peer_id.as_bytes(), &peer_address){
                Ok(_) => println!(""),//println!("Heartbeat sent to: {}", peer_address),
                Err(e) => {eprintln!("Failed to send heartbeat to {}: {}", &peer_address , e);}
            };
        }
        thread::sleep(Duration::from_millis(1000));
    }
}

//Receive heartbeats from all peers to detect dead elevators
pub fn receive_hearbeat(heartbeat_socket: &UdpSocket, heartbeat_tx: Sender<(String, bool)>) {

    //HashMap to keep track of heartbeats
    let mut heartbeats: HashMap<String, Instant> = HashMap::new();
    let mut buffer = [0; 1024]; 

    heartbeat_socket.set_nonblocking(true).expect("Failed to set non-blocking!");

    loop {
        match heartbeat_socket.recv(&mut buffer) {
            Ok(n_bytes) => {
                let id = String::from_utf8_lossy(&buffer[..n_bytes]).to_string();
                //println!("Heartbeat received from: {}", id);

                heartbeats.insert(id.clone(), Instant::now());

                
            },
            //If there is no heartbeat waiting, dont block s.t. heartbeat can not be sent. 
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                //println!("No heartbeat waiting...");
                thread::sleep(Duration::from_millis(1000));
            }
            Err(e) => {
                eprintln!("An error occured when receiving heartbeat: {}", e);
            }
        }
        for (id, time) in &heartbeats {
            if Instant::now() - *time < Duration::from_millis(5000) {
                heartbeat_tx.send((id.clone(), true)).expect(&format!("Failed to pass heartbeat {} over channel.", &id));
            } else {
                heartbeat_tx.send((id.clone(), false)).expect(&format!("Failed to pass heartbeat {} over channel.", &id));
            }
        }
    }
}

