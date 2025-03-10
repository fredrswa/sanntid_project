pub mod network;

use crossbeam_channel as cbc;
use std::thread::{spawn, sleep};



use crate::config::*;
use crate::mod_network::network::*;

pub fn run(/* Channels */) {
    // Simulate Channels Here //
    let (network_io_tx, network_io_rx) = cbc::unbounded::<String>();
    let (network_io_redistribute_tx, network_io_redistribute_rx) = cbc::unbounded::<String>(); //ID
    let (network_io_neworder_tx, network_io_neworder_rx) = cbc::unbounded::<CallOrder>();
    let (network_io_peer_state_tx, netork_io_peer_state_tx) = cbc::unbounded::<PeerState>();
    //           -            //
    let config = Config::import();
    let socket = udp_create_socket();

    let (udp_listener_tx, udp_listener_rx) = cbc::unbounded::<String>();
    let (udp_heartbeat_dead_tx, udp_heartbeat_dead_rx) = cbc::unbounded::<String>();

    {
        spawn(move ||udp_receive(socket, udp_listener_tx, udp_heartbeat_dead_tx));
    }
    loop {
        cbc::select! {
            recv(udp_listener_rx) -> udp_message => {
                if let Ok(message) = udp_message {
                    if let Ok(new_call_order) = serde_json::from_str::<CallOrder>(&message) {
                        network_io_neworder_tx.send(new_call_order).unwrap();
                    } else if let Ok(peer_state) = serde_json::from_str::<PeerState>(&message) {
                        network_io_peer_state_tx.send(peer_state).unwrap();
                    }
                }
            }
    
           
            recv(udp_heartbeat_dead_rx) -> id => {
                if config.id == id.unwrap() {
                    //
                } 
                network_io_redistribute_tx.send(id);


            }
        }

        if true {
        }
    }
}