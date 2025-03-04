use crossbeam_channel as cbc;
use std::thread::{spawn, sleep};
use std::net::UdpSocket;


mod network;
use super::mod_network::network::*;

pub fn run_network(/* CHANNELS*/) {
    
    let (server_receiver_tx, server_receiver_rx) = cbc::unbounded::<String>();
    let (server_sender_tx, server_sender_rx) = cbc::unbounded::<String>();

    
    //spawn listener with tx channel


    //spawn sender with rx channel

    loop {
        cbc::select! {
            // recv(udp_recv_rx) -> udp_receive_message => {
            //     //check om heartbeat eller state

            //     if( heartbeat) {
            //         //update heartbeat timer
            //     }
            //     if (state) {

            //     }
            // }
            // // ! Vente på IO


            // // ! Vente på receive
            default => {}
        }


        // ! Sende
    }
}

#[test]
pub fn network_test () {

}