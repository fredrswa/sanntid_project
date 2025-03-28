//! Network Submodule   |   
//! Handles peer-to-peer communication between elevators using UDP and JSON-based messages 
//! Detetect dead elevators by sending and receiving heartbeats

/// Standard Library
use std::{collections::HashMap,
    time::{Instant, Duration},
    net::UdpSocket, 
    io,
    thread::{self, sleep}};

/// External Crates
use crossbeam_channel::{select, Receiver, Sender};
use crossbeam_channel as cbc;
use driver_rust::elevio::elev;

/// Internal Crates
use crate::config::*;

/// Configurations
static UDP_RECV_PORT: &str = CONFIG.network.udp_recv;
static UDP_SEND_PORT: &str = CONFIG.network.udp_send;
static HB_SLEEP_TIME: u64  = CONFIG.network.hb_time as u64;
static ST_SLEEP_TIME: u64  = CONFIG.network.state_time as u64;


/// UDP Create Socket | Creates a UDP socket and binds it to known address
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

/// UDP Receive | Receives messages and redirects them to the correct channel
pub fn udp_receive (socket: &UdpSocket, udp_to_heartbeat_tx: Sender<String>, udp_to_world_view_tx: Sender<TimestampsEntireSystem>) {
    
    // Buffer for incoming messages
    let mut buffer = [0; 1024];
    socket.set_nonblocking(true).expect("Failed to set non-blocking!");
    
    // Loop
    loop {
        let (n_bytes, _src) = match socket.recv_from(&mut buffer){
            Ok((_n_bytes, _src)) => (_n_bytes, _src),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(e) => {
                panic!("Error received from udp: {}", e);
            }
        };

        let received_msg = String::from_utf8_lossy(&buffer[..n_bytes]).to_string();

        // Message is a heartbeat
        if n_bytes < 5 {
            udp_to_heartbeat_tx.send(received_msg).expect("Could'nt pass to heartbeat");
        } 
        // Message is a state
        else {
            let sys: TimestampsEntireSystem = match serde_json::from_str(&received_msg) {
                Ok(sys) => sys,
                Err(e) => {
                    panic!("Failed to parse incoming state!: {}", e)
                }
            };
        
            match udp_to_world_view_tx.send(sys.clone()) {
                Ok(ok) => ok,
                Err(e) => {panic!("Message was not sent to peer: {}", e)}
            };
        }
    }
}

/// UDP Send | Sends messages to peers
pub fn udp_send(socket: &UdpSocket, peer_address: String, udp_sender_rx: Receiver<TimestampsEntireSystem>) {  
    
    // Initial state
    let mut created_completed_timestamps: Vec<Vec<(i64, i64)>> = vec![vec![(0, 1); 3]; CONFIG.elevator.num_floors as usize];
    let mut world_view = EntireSystem::template();
    let mut curr_sys = TimestampsEntireSystem{ es: world_view, timestamps: created_completed_timestamps};

    // Loop
    loop {
        sleep(Duration::from_millis(25));
        cbc::select! {
            recv(udp_sender_rx) -> sys => {
                let sys = sys.unwrap();
                
                if curr_sys != sys {
                    curr_sys = sys;
                    let json_msg = match serde_json::to_string(&curr_sys){
                        Ok(json_msg) => json_msg,
                        Err(e) => {
                            panic!("Failed to serialize message to send over Udp!: {}", e)
                        }    
                    };
                    for _ in 0..3{
                    //Send more than once
                        match socket.send_to(json_msg.as_bytes(), UDP_SEND_PORT.to_string()) {
                            Ok(ok) => ok,//Ack send to io
                            Err(e) => {
                                 // panic!("Failed to send message {:#?} on adress {:#?}: \n {}", json_msg, peer_address, e)
                                 println!("Network disconnected, trying again in 30 seconds");
                                 break;
                            }
                        };
                    }
                }
            }
        }
    }  
} 

/// Send Heartbeat | Sends a heartbeat to all peers, can turn on and off based on detection of obs and hardware fault
pub fn send_heartbeat(heartbeat_socket: &UdpSocket, peer_id: &String, send_heartbeat_rx: Receiver<(bool)>) -> std::io::Result<()> {
    let mut between_floors_or_obstruced: bool = true;
    
    let hb_time = Duration::from_millis(HB_SLEEP_TIME);
    //println!("Sending Heartbeat");

    loop {
        select! {
            recv(send_heartbeat_rx) -> send_heartbeat => {
                between_floors_or_obstruced = send_heartbeat.unwrap();
            }
            default => {}
        }

        if !between_floors_or_obstruced {
            match heartbeat_socket.send_to( peer_id.as_bytes(), UDP_SEND_PORT.to_string()){
                Ok(_) => { },//println!("Heartbeat sent to: {}", peer_address),
                Err(e) => { }, //eprintln!("Failed to send heartbeat: {}", e);}
            };
        }

        thread::sleep(hb_time);
    }
}

/// Receive Heartbeat | Receives heartbeats from all peers and detects if they are dead or alive
pub fn receive_hearbeat(udp_to_heartbeat_rx: Receiver<String>, heartbeat_to_network_tx: Sender<(usize, bool)>) { 
    let mut heartbeats: HashMap<String, Instant> = HashMap::new();

    loop {
        select! {
            recv(udp_to_heartbeat_rx) -> heartbeat_id => {
                if let Ok(id) = heartbeat_id {
                    heartbeats.insert(id.clone(), Instant::now());
                }
            }
        }
                
        for (id, time) in &heartbeats {
            if Instant::now() - *time < Duration::from_millis(5000) {
                let incoming_id: usize = id.clone().parse().expect("Was not able to parse incoming id as int");
                let _ = heartbeat_to_network_tx.send((incoming_id.clone(), true));
            } else {
                let incoming_id: usize = id.clone().parse().expect("Was not able to parse incoming id as int");
                let _ = heartbeat_to_network_tx.send((incoming_id.clone(), false));
            }
        }

    }
}

/// Between Floors | Detects if the elevator is between floors
pub fn between_floors(elev: elev::Elevator, ch: cbc::Sender<bool>, period: Duration) {
    let mut prev: Option<u8> = Some(u8::MAX);
    loop {
        let f: Option<u8> = elev.floor_sensor();

        if f.is_none() && f != prev {
            ch.send(true).unwrap();
            prev = f;
        } else if f != prev {
            ch.send(false).unwrap();
            prev = f;
        }

        thread::sleep(period)
    }
}

