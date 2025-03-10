pub mod network;

use crossbeam_channel as cbc;
use std::thread::{spawn, sleep};
<<<<<<< HEAD
use std::sync::Arc;

=======


use driver_rust::elevio::poll::{CallButton};
>>>>>>> 01a0b1eb3da5f541d4d6d5873b5a20e6b191420a
use crate::config::*;
use crate::mod_network::network::*;

pub fn run(/* Channels */) {
    // Simulate Channels Here //
    let (network_io_tx, network_io_rx) = cbc::unbounded::<String>();
    let (network_io_redistribute_tx, network_io_redistribute_rx) = cbc::unbounded::<String>(); //ID
<<<<<<< HEAD
    //let (network_io_neworder_tx, network_io_neworder_rx) = cbc::unbounded::<CallOrder>();
=======
    let (network_io_neworder_tx, network_io_neworder_rx) = cbc::unbounded::<CallButton>();
>>>>>>> 01a0b1eb3da5f541d4d6d5873b5a20e6b191420a
    let (network_io_peer_state_tx, netork_io_peer_state_tx) = cbc::unbounded::<PeerState>();
    //           -            //

    //let socket = udp_create_socket();

    let (udp_listener_tx, udp_listener_rx) = cbc::unbounded::<String>();
    let (udp_heartbeat_tx, udp_heartbeat_rx) = cbc::unbounded::<String>();

    /* {
        spawn(move || udp_receive(socket, udp_listener_tx));
    } */

    let ps = PeerState {
        id: PeerStateCONFIG.id.clone(),
        ip: PeerStateCONFIG.ip.clone(),
        peers: PeerStateCONFIG.peers.clone(),
        connected: PeerStateCONFIG.connected.clone(),
    };

    let heartbeat_socket = Arc::new(udp_create_socket(ps.ip));

    let send_heartbeat_socket = Arc::clone(&heartbeat_socket);
    let receive_heartbeat_socket = Arc::clone(&heartbeat_socket);

    {spawn(move || send_heartbeat(&send_heartbeat_socket, &ps.id, &ps.peers))};
    {spawn(move || receive_hearbeat(&receive_heartbeat_socket, udp_heartbeat_tx))};

    loop {
        cbc::select! {
<<<<<<< HEAD
            recv(udp_heartbeat_rx) -> heartbeat => {
                let hb = heartbeat.unwrap();
                println!("{}", hb);
                //update_peer_state(&ps);
            }
            



            /* recv(udp_listener_rx) -> udp_message => {
                if let Ok(message) = udp_message {
                    if let Ok(new_call_order) = serde_json::from_str::<CallOrder>(&message) {
                        network_io_neworder_tx.send(new_call_order).unwrap();
                    } else if let Ok(peer_state) = serde_json::from_str::<PeerState>(&message) {
                        network_io_peer_state_tx.send(peer_state).unwrap();
                    }
                }
=======
            recv(udp_listener_rx) -> udp_message => {
                // if let Ok(message) = udp_message {
                //     if let Ok(new_call_order) = serde_json::from_str::<CallButton>(&message) {
                //         network_io_neworder_tx.send(new_call_order).unwrap();
                //     } else if let Ok(peer_state) = serde_json::from_str::<PeerState>(&message) {
                //         network_io_peer_state_tx.send(peer_state).unwrap();
                //     }
                // }
>>>>>>> 01a0b1eb3da5f541d4d6d5873b5a20e6b191420a
            }
            recv(udp_heartbeat_dead_rx) -> id => {
                if CONFIG.id == id.unwrap() {
                    //
                } 
<<<<<<< HEAD
                network_io_redistribute_tx.send(id);
            } */
            
        
=======
                //network_io_redistribute_tx.send(id);
            }
>>>>>>> 01a0b1eb3da5f541d4d6d5873b5a20e6b191420a
        }

        if true {
        }
    }
}