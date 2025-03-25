//Handles peer-to-peer communication between elevators using UDP and JSON-based messages 
// Detetect dead elevators by sending and receiving heartbeats

///Includes
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::net::UdpSocket;
use std::io;
use std::thread::sleep;
///
use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel as cbc;
use std::thread;

///Crates
use crate::config::*;

//Constants
const TIMEOUT_MS: u64 = 5000; // how long before we consider an elevator dead
const CHECK_INTERVAL_MS: u64 = 1000; // how often we check for dead elevators

static UDP_RECV_PORT: &str = CONFIG.network.udp_recv;
static UDP_SEND_PORT: &str = CONFIG.network.udp_send;
static HB_SLEEP_TIME: u64  = CONFIG.network.hb_time as u64;
static ST_SLEEP_TIME: u64  = CONFIG.network.state_time as u64;

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

 //Receive UDP messages and send them to the channel
 // Message contain the entire system state
pub fn udp_receive(socket: &UdpSocket, udp_listener_tx: Sender<TimestampsEntireSystem>) {
    let mut buffer = [0; 1024];
    socket.set_nonblocking(true).expect("Failed to set non-blocking!");
    loop {
        let (n_bytes, _src) = match socket.recv_from(&mut buffer){
            Ok((_n_bytes, _src)) => (_n_bytes, _src),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
                continue;
            },
            Err(_) => {
                panic!()
            }
        };

        let received_msg = String::from_utf8_lossy(&buffer[..n_bytes]).to_string();
        if received_msg.contains("heartbeat") {
            continue;
        }
        let sys: TimestampsEntireSystem = match serde_json::from_str(&received_msg) {
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


                            
pub fn udp_send(socket: &UdpSocket, peer_addresses: String, udp_sender_rx: Receiver<TimestampsEntireSystem>) {  
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
                
                //Send more than once
                match socket.send_to(json_msg.as_bytes(), UDP_SEND_PORT.to_string()) {
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
pub fn send_heartbeat(heartbeat_socket: &UdpSocket, peer_id: &String) -> std::io::Result<()> {
    let hb_time = Duration::from_millis(HB_SLEEP_TIME);
    println!("Sending Heartbeat");
    let hb_str = format!("heartbeat: {}", peer_id);
    let hb_bytes = hb_str.as_bytes();
    loop {
            
            match heartbeat_socket.send_to( hb_bytes, UDP_SEND_PORT.to_string()){
                Ok(_) => println!(""),//println!("Heartbeat sent to: {}", peer_address),
                Err(e) => {eprintln!("Failed to send heartbeat");}
            };
        
        thread::sleep(hb_time);
    }
}

//Receive heartbeats from all peers to detect dead elevators
pub fn receive_hearbeat(heartbeat_socket: &UdpSocket, heartbeat_tx: Sender<(String, bool)>) {

    //HashMap to keep track of heartbeats
    let mut heartbeats: HashMap<String, Instant> = HashMap::new();
    let mut buffer = [0; 1024]; 

    heartbeat_socket.set_nonblocking(true).expect("Failed to set non-blocking!");

    loop {
        sleep(Duration::from_millis(200));
        match heartbeat_socket.recv(&mut buffer) {
            Ok(n_bytes) => {
                let id = String::from_utf8_lossy(&buffer[..n_bytes]).to_string();
                println!("Heartbeat received from: {}", id);

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

