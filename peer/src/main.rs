//! main coordinates the initialization of all threads
//! Creates the required channels
//! Spawns modules
//! Then "listens" for recovery using timeout_rx which sends a struct indicating which module has failed.

/// INCLUDES
use std::io::Result;
use config::*;
use crossbeam_channel::{select, unbounded, Sender, Receiver};
use std::thread::{spawn, sleep};
use once_cell::sync::Lazy;

/// DRIVER
use driver_rust::elevio::poll as sensor_polling;


/// MODULES
pub mod config;
pub mod mod_fsm;
pub mod mod_io;
pub mod mod_network;


/// main function
fn main() -> Result<()> {
    Lazy::force(&config::CONFIG); //Forces read of config on start of runtime in order to ensure safety
    Lazy::force(&config::PeerStateCONFIG);

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
    let es: ElevatorSystem = ElevatorSystem::new();
    
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
        spawn(move || {mod_network::run(&network_to_io_tx, &io_to_network_rx);});
    }


    loop {
        select! {
            default => {}
        }
    }
}
