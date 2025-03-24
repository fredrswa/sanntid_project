//! main coordinates the initialization of all threads
//! Creates the required channels
//! Spawns modules
//! Then "listens" for recovery using timeout_rx which sends a struct indicating which module has failed.

/// INCLUDES
use std::{env, io::Result, net::UdpSocket};
use std::time::Duration;
use config::*;
use crossbeam_channel::{select, unbounded, Sender, Receiver};
use std::thread::{spawn, sleep};
use once_cell::sync::Lazy;
use static_toml;
/// DRIVER
use driver_rust::elevio::poll as sensor_polling;


/// MODULES
mod config;
mod mod_fsm;
mod mod_io;
mod mod_network;
mod mod_backup;
mod mod_hardware;


use crate::config::CONFIG;


/// main function
fn main() -> Result<()> {
    //Read command line arguments
    let command_line_arguments: Vec<String>= env::args().collect();
    let is_primary: bool = command_line_arguments.get(1).expect("Specify primary -- id true/false").parse().unwrap();

    let mut world_view = EntireSystem::template();
    //Create state
    if !is_primary {
        // get from backup
        mod_backup::backup_state();
    } else {
        let _ = mod_hardware::init();
    }
    
    mod_backup::spawn_secondary();


    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();

    { spawn(move || run_modules(timeout_tx)); }


    //Recovery Scripts
    loop{
        select! {
            recv(timeout_rx) -> timout_struct => {
                
            }

            default => {
                
            }
        }
    }
}
///module init
fn run_modules(timeout_tx: Sender<Timeout_type>) {
    
    /* ######### Elevator System for current peer ################################################################# */
    
    let mut es = ElevatorSystem::new();
    /* ################# CHANNELS TO PASS ############# (Allows modules to communicate) ########################### */
    let (io_call_tx,io_call_rx) = unbounded::<sensor_polling::CallButton>();

    let (network_to_io_tx, network_to_io_rx) = unbounded::<EntireSystem>();
    let (io_to_network_tx, io_to_network_rx) = unbounded::<EntireSystem>();
    
    let (io_to_fsm_requests_tx, io_to_fsm_requests_rx) = unbounded::<Vec<Vec<bool>>>();
    
    let (fsm_to_io_es_tx, fsm_to_io_es_rx) = unbounded::<ElevatorSystem>();
    let (io_to_fsm_es_tx, io_to_fsm_es_rx) = unbounded::<ElevatorSystem>();
    /* ############################################################################################################ */
    


    println!("Spawning Modules");
    {
        /* ######### Run FSM module ################################################################## */
        let mut es1 = es.clone();
        spawn(move || {mod_fsm::run(
                                    &mut es1,
                                    &io_call_rx,
                                    &timeout_tx,
                                    &fsm_to_io_es_tx,
                                    &io_to_fsm_es_rx,
                                    &io_to_fsm_requests_rx
                                    );});
        
        
        /* ######### Run IO module ################################################################### */
        let mut es2 = es.clone();
        spawn(move || {mod_io::run(
            &mut es2, 
            &io_call_tx,
            &network_to_io_rx,
            &io_to_network_tx,
            &io_to_fsm_requests_tx,
            &fsm_to_io_es_rx,
            &io_to_fsm_es_tx,
        );});
        
        /* ######### Run NETWORK module ############################################################## */
        spawn(move || {mod_network::run(&network_to_io_tx, &io_to_network_rx);});
    }

    loop {
        
        select! {
            default => {}
        }
    }
}
