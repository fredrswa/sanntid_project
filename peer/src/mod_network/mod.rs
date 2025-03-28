//! Network Module |  
//! This module handles the operation of the network module.
//! The network module is responsible for sending and receiving messages between peers.

/// Sub Modules
pub mod network;

/// Standard Library
use std::{thread::spawn,
          sync::Arc};

/// External Crates
use crossbeam_channel as cbc;
use core::time::Duration;

/// Internal Crates
use crate::config::*;
use crate::mod_network::network::{udp_create_socket, udp_receive, udp_send, send_heartbeat, receive_hearbeat, between_floors};

/// Configurations
static UDP_RECV_PORT: &str = CONFIG.network.udp_recv;
static UDP_SEND_PORT: &str = CONFIG.network.udp_send;

pub fn run(
    es: &ElevatorSystem,
    //Communication with IO module
    network_to_io_tx: &cbc::Sender<TimestampsEntireSystem>,
    io_to_network_rx: &cbc::Receiver<TimestampsEntireSystem>,
    connected_peers_tx: &cbc::Sender<[bool; CONFIG.network.peers as usize]>,
    obstruction_to_io_rx: &cbc::Receiver<bool>,) {
    
    println!("Running network module");
    let socket = Arc::new(udp_create_socket(&UDP_RECV_PORT.to_string()));
    socket.set_broadcast(true).unwrap();

    // UDP Communication
    let (udp_sender_tx, udp_sender_rx) = cbc::unbounded::<TimestampsEntireSystem>();
    let (udp_to_world_view_tx, udp_to_world_view_rx) = cbc::unbounded::<TimestampsEntireSystem>();
    let (udp_to_heartbeat_tx, udp_to_heartbeat_rx) = cbc::unbounded::<String>();
    
    let udp_socket = Arc::clone(&socket);
    
    let udp_send_socket = Arc::clone(&udp_socket);
    let udp_receive_socket = Arc::clone(&udp_socket);

    {spawn(move || udp_send(&udp_send_socket, UDP_SEND_PORT.to_string(),udp_sender_rx));}
    {spawn(move || udp_receive(&udp_receive_socket, udp_to_heartbeat_tx, udp_to_world_view_tx));}

    // Heartbeat Communication
    let mut connected_peers = [false; CONFIG.network.peers as usize];
    let self_id: usize = SELF_ID.to_string().parse().expect("Was not able to parse SELF ID as int");
    connected_peers[self_id] = true;

    let (udp_send_heartbeat_tx, udp_send_heartbeat_rx) = cbc::unbounded::<(bool)>();
    let (heartbeat_to_network_tx, heartbeat_to_network_rx) = cbc::unbounded::<(usize, bool)>();

    let heartbeat_socket: Arc<std::net::UdpSocket> = Arc::clone(&socket);
    
    let send_heartbeat_socket: Arc<std::net::UdpSocket> = Arc::clone(&heartbeat_socket);

    {spawn(move || send_heartbeat(&send_heartbeat_socket, &SELF_ID.to_string(), udp_send_heartbeat_rx))};
    {spawn(move || receive_hearbeat(udp_to_heartbeat_rx, heartbeat_to_network_tx))};

    // Between Floors Detection
    let poll_period = Duration::from_millis(25);

    let (between_floors_tx, between_floors_rx) = cbc::unbounded::<bool>(); 
    {
        let elevator = es.elevator.clone();
        spawn(move || between_floors(elevator, between_floors_tx, poll_period));  
    }

    // Loop
    loop {
        cbc::select! {
            // Heartbeat on/off
            recv(heartbeat_to_network_rx) -> heartbeat => {
                let (id, val) = heartbeat.unwrap();
                if id.to_string() != *SELF_ID {
                    connected_peers[id] = val;
                    connected_peers_tx.send(connected_peers);
                }                
            }

            // Incomming world_view
            recv(udp_to_world_view_rx) -> incoming_sys => {
                if let Ok(sys) = incoming_sys {
                    network_to_io_tx.send(sys);
                }
            }

            // Outgoing world_view
            recv(io_to_network_rx) -> outgoing_sys => {
                if let Ok(sys) = outgoing_sys {
                    udp_sender_tx.send(sys);
                }
            }

            // Between Floors
            recv(between_floors_rx) -> bf_message => {
                if let Ok(between) = bf_message{
                    udp_send_heartbeat_tx.send(between);
                }
            }

            // Obstruction Detection
            recv(obstruction_to_io_rx) -> ob_message => {
                if let Ok(obstructed) = ob_message{
                    udp_send_heartbeat_tx.send(obstructed);
                }
            }
        }
    }
}