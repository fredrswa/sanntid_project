use crossbeam_channel as cbc;
use std::thread::{spawn, sleep};
use std::net::UdpSocket;

pub fn run_network(/* CHANNELS*/) {
    

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