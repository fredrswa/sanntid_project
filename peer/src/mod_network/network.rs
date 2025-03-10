use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::net::UdpSocket;
use std::io;
use crossbeam_channel::Sender;
use crate::config::*;

const TIMEOUT_MS: u64 = 5000; // how long before we consider an elevator dead
const CHECK_INTERVAL_MS: u64 = 1000; // how often we check for dead elevators

pub fn udp_create_socket() -> UdpSocket {
    let socket = match UdpSocket::bind("0.0.0.0:20003") {
        Ok(socket) => {
            println!("Socket bound successfully.");
            socket
        },
        Err(e) => {
            panic!("Could'nt bind to socket");
        }
    };
    return socket;
}

pub fn udp_receive( socket: UdpSocket, udp_listener_tx: Sender<String>, udp_heartbeat_dead_tx: Sender<String>) -> io::Result<()> {
    let mut heartbeats: HashMap<String, Instant> = HashMap::new();
    let id_self = Config::import().id;
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
