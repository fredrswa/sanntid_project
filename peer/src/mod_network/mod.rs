/// Sub Modules
pub mod network;

//Includes
use crossbeam_channel as cbc;
use std::thread::{spawn, sleep};
use std::sync::Arc;

///Crates
use crate::config::*;
use crate::mod_network::network::{udp_create_socket, udp_receive, udp_send, send_heartbeat, receive_hearbeat};
use crate::mod_io::io::{call_assigner, save_system_state_to_json};

static SELF_ID: &str = CONFIG.peer.id;
static HOST: &str = CONFIG.network.host;
static UDP_RECV_PORT: i64 = CONFIG.network.udp_recv;
static UDP_SEND_PORT: i64 = CONFIG.network.udp_send;

pub fn run(
    
    //Communication with IO module
    network_to_io_tx: &cbc::Sender<EntireSystem>,
    io_to_network_rx: &cbc::Receiver<EntireSystem>,) {
    println!("Running network module");
    let udp_recv_addr = format!("{}:{}", HOST, UDP_RECV_PORT);
    let socket = Arc::new(udp_create_socket(&udp_recv_addr));
    // Simulate Channels Here //
    let (network_io_redistribute_tx, network_io_redistribute_rx) = cbc::unbounded::<String>(); //ID
    //let (network_io_neworder_tx, network_io_neworder_rx) = cbc::unbounded::<CallOrder>();
    let (network_io_peer_state_tx, netork_io_peer_state_tx) = cbc::unbounded::<PeerState>();
    //           -            //

    /* ########################### Udp #################################################################################### */
    let (udp_sender_tx, udp_sender_rx) = cbc::unbounded::<EntireSystem>();
    let (udp_listener_tx, udp_listener_rx) = cbc::unbounded::<EntireSystem>();
    let udp_recv_addr = format!("{}:{}",HOST, UDP_RECV_PORT);
    let udp_send_addr = format!("{}:{}",HOST, UDP_SEND_PORT);
    let udp_socket = Arc::clone(&socket);
    
    let udp_send_socket = Arc::clone(&udp_socket);
    let udp_receive_socket = Arc::clone(&udp_socket);



    {spawn(move || udp_send(&udp_send_socket, udp_send_addr,udp_sender_rx));}
    {spawn(move || udp_receive(&udp_receive_socket, udp_listener_tx));}
    /* #################################################################################################################### */

    /* ########################### Hearbeat ############################################################################### */
    let (udp_heartbeat_tx, udp_heartbeat_rx) = cbc::unbounded::<(String, bool)>();

    let heartbeat_socket: Arc<std::net::UdpSocket> = Arc::clone(&socket);
    
    let send_heartbeat_socket: Arc<std::net::UdpSocket> = Arc::clone(&heartbeat_socket);
    let receive_heartbeat_socket: Arc<std::net::UdpSocket> = Arc::clone(&heartbeat_socket);


    // SPAWN HEATBEAT FUNCTIONS
    {spawn(move || send_heartbeat(&send_heartbeat_socket, &SELF_ID.to_string()))};
    {spawn(move || receive_hearbeat(&receive_heartbeat_socket, udp_heartbeat_tx))};
    /* #################################################################################################################### */


    loop {
        cbc::select! {
            recv(udp_heartbeat_rx) -> heartbeat => {
                let (id, val) = heartbeat.unwrap();
                
                //ps.connected.insert(id.clone(), val);
                
            // println!("###########");
            // for (_id, _val) in &ps.connected {
            //     println!("{}->{}", _id, _val);
            // }
            // println!("###########\n");
            }
            recv(udp_listener_rx) -> sys => {
                let sys = sys.unwrap();

            }



            /* recv(udp_listener_rx) -> udp_message => {
                if let Ok(message) = udp_message {
                    if let Ok(new_call_order) = serde_json::from_str::<CallOrder>(&message) {
                        network_io_neworder_tx.send(new_call_order).unwrap();
                    } else if let Ok(peer_state) = serde_json::from_str::<PeerState>(&message) {
                        network_io_peer_state_tx.send(peer_state).unwrap();
                    }
                }
            }
            recv(udp_heartbeat_dead_rx) -> id => {
                if CONFIG.id == id.unwrap() {
                    //
                } 
                network_io_redistribute_tx.send(id);
            } */
            
        
        }

        if true {
        }
    }
}