
use crossbeam_channels as cbc;

use std::thread::{spawn, sleep};



use crate::config::Config;


pub fn run(/* Channels */) {
    // Simulate Channels Here //
    let (network_io_tx, network_io_rx) = cbc::unbounded::<String>();
    let (network_io_redistribute_tx, network_io_redistribute_tx) = cbc::unbounded::<String>(); //ID

    //           -            //
    let config = Config::import();
    let socket = udp_create_socket();

    let (udp_listener_tx, udp_listener_rx) = cbc::unbounded::<String>();
    let (udp_heartbeat_dead_tx, udp_heartbeat_dead_rx) = cbc::unbounded::<String>();

    {
        spawn(move ||udp_receive(socket, udp_listener_tx, udp_heartbeat_dead_tx));
    }

    loop {
        cbc::select!{
            recv(udp_listener_rx) -> udp_message => {
                let Ok(message) = udp_message.unwrap() {
                    network_io_tx.send(message);
                }
            }
            recv(udp_heartbeat_dead_rx) -> id => {
                if config.id == id.unwrap() {
                    //
                } 
                network_io_redistribute_tx.send(id);
                

            }
        }
    }
}