




pub mod mod_fsm;
pub mod mod_backup;
pub mod mod_io;
//pub mod mod_network;
pub mod config;
pub mod unplaced;

use core::{num, panic};
use std::{env, io::Result, net::{UdpSocket}};

use mod_backup::{create_socket, get_free_socket, string_to_bool};

use config::*;

use driver_rust::elevio::elev::Elevator;
use driver_rust::elevio::poll as sensor_polling;
use crossbeam_channel::{select, unbounded, Sender, Receiver};
use std::thread::{spawn, sleep};




fn main() -> Result<()>  {


    // Arguments from command-line, current order.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // "cargo run id primary udp_secondary_socket elevator_ip floors peers lab_server sim"
    let id: usize                     = args.get(1).expect("Illegal id passed").to_string().parse().unwrap();
    let primary                 = string_to_bool(args.get(2).expect("Missing Primary Bolean"));
    let udp_secondary_recv    = args.get(3).expect("Pass secondary backup string").to_string();
    let elevator_addr: usize          = args.get(4).expect("Pass Elevator address").parse().unwrap();
    let num_floors: usize             = args.get(5).expect("Pass Number of floors").parse().unwrap();
    let num_peer: usize               = args.get(6).expect("Pass Number of Peers").parse().unwrap();
    let lab_server: bool              = string_to_bool(args.get(7).expect("Specify lab server(1) or local server (0)"));
    let sim: bool                     = string_to_bool(args.get(8).expect("Specify simulation (1) or real hardware (0)"));


    let mut systemState = SystemState::new(id, num_peer, lab_server);    
    // Secondary backup state
    if !primary {
        let listening_socket = create_socket(udp_secondary_recv);
        systemState = mod_backup::secondary_state(&listening_socket);
        drop(listening_socket);
    } else {
        
        // Spawn hardware
    }


    unplaced::init_hardware(elevator_addr.clone(), sim);
    let elevator_system = ElevatorSystem::new(elevator_addr, num_floors, 3);
    
    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();

    { spawn(move || run_modules(timeout_tx, elevator_system,systemState, )); }

    let backup_socket = get_free_socket();
    mod_backup::spawn_secondary(&backup_socket, elevator_addr, num_floors, num_peer, lab_server);

    loop {
        sleep(std::time::Duration::from_millis(1000));
        // select! {
        //     recv(timeout_rx) -> timout_struct => {
        //         panic!("Timeout")
        //     }
        //     default => {
                
        //     }
        // }
    }
}

fn run_modules(timeout_tx: Sender<Timeout_type>, es: ElevatorSystem, system_state: SystemState) {
       
    /* ################# CHANNELS TO PASS ############# (Allows modules to communicate) ########################### */
    let (io_call_tx,io_call_rx) = unbounded::<sensor_polling::CallButton>();

    let (network_to_io_tx, network_to_io_rx) = unbounded::<EntireSystem>();
    let (io_to_network_tx, io_to_network_rx) = unbounded::<EntireSystem>();
    
    let (fsm_to_io_tx, fsm_to_io_rx) = unbounded::<ElevatorSystem>();
    /* ############################################################################################################ */

    {
        /* ######### Run FSM module ################################################################## */
        let mut es1 = es.clone();
        spawn(move || {mod_fsm::run(
            &mut es1,
            &io_call_rx,
            &timeout_tx,
            &fsm_to_io_tx,
        );});
        
        
        /* ######### Run IO module ################################################################### */
        let mut es2 = es.clone();
        spawn(move || {mod_io::run(
            &mut es2, 
            &io_call_tx,
            &network_to_io_rx,
            &io_to_network_tx,
            &fsm_to_io_rx,
        );});
        
        /* ######### Run NETWORK module ############################################################## */
        //spawn(move || {mod_network::run(&network_to_io_tx, &io_to_network_rx);});
    }


    loop {
        select! {
            default => {}
        }
    }
}
