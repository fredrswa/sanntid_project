

use std::{env, io::Result, net::{UdpSocket}};

use mod_backup::{create_socket, string_to_bool};




pub mod mod_fsm;
pub mod mod_backup;
pub mod config;


fn main() -> Result<()>  {
    let args: Vec<String> = env::args().collect();

    let id: usize                     = args.get(1).expect("Illegal id passed").to_string().parse().unwrap();
    let primary                 = string_to_bool(args.get(2).expect("Missing Primary Bolean"));
    let udp_secondary_recv    = args.get(3).expect("Pass secondary backup string").to_string();
    let elevator_addr         = args.get(4).expect("Pass Elevator address").to_string();
    let num_floors: usize             = args.get(5).expect("Pass Number of floors").parse().unwrap();
    let num_peer: usize               = args.get(6).expect("Pass Number of Peers").parse().unwrap();
    let lab_server: bool              = string_to_bool(args.get(2).expect("Specify lab server(1) or local server (0)"));


    

    if !primary {
        let listening_socket = create_socket(udp_secondary_recv);
        

    }
    loop {

    }
}
