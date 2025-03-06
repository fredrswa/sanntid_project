use crossbeam_channel as cbc;

use crossbeam_channel::{unbounded, Receiver};
use std::net::UdpSocket;
use std::thread;

mod network;
mod watchdog;

pub fn run_network() -> std::io::Result<()> {
    //**** use different socket for sending and receiving to avoid blocking ****//
    
    // UDP socket for sending
    let send_socket = UdpSocket::bind("0.0.0.0:0")?;
    
    // UDP socket for receiving
    let recv_socket = UdpSocket::bind("0.0.0.0:5000")?;
    recv_socket.set_nonblocking(true)?;

    // communication channels
    let (udp_recv_tx, udp_recv_rx) = unbounded::<String>(); // Sender/mottaker for UDP-data

    // start watchdog thread
    let recv_socket_clone = recv_socket.try_clone()?;
    thread::spawn(move || {
        watchdog::watchdog(&recv_socket_clone);
    });

    // start sending heartbeats
    let send_socket_clone = send_socket.try_clone()?;
    thread::spawn(move || {
        network::start_heartbeat(&send_socket_clone, "elevator_1", "192.168.1.255:5000");
    });
    }

pub fn run_network(/* CHANNELS*/) {
    return Ok(());

    // * Starter receive (udp_recv_rx, udp_recv_tx) <String>

    // * Starte heartbeat () evt flytt til watchdog

    loop {
        cbc::select! {
            recv(udp_recv_rx) {
                //check om heartbeat eller state

                if( heartbeat) {
                    //update heartbeat timer
                }
                if (state) {

                }
            }
            // ! Vente på IO


            // ! Vente på receive
        }


        // ! Sende
    }
}