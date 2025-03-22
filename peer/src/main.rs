//! main coordinates the initialization of all threads
//! Creates the required channels
//! Spawns modules
//! Then "listens" for recovery using timeout_rx which sends a struct indicating which module has failed.

/// INCLUDES
use std::{env, io::Result};
use std::time::Duration;
use config::*;
use crossbeam_channel::{select, unbounded, Sender, Receiver};
use mod_backup::create_socket;
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
    let is_primary: bool = command_line_arguments.get(2).expect("Specify primary -- id true/false").parse().unwrap();


    
    if !is_primary {
        mod_backup::backup_state();
    } else {
        let _ = mod_hardware::init();
    }


    

    mod_backup::spawn_secondary();


    
    

    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();

    { spawn(move || run_modules(timeout_tx)); }

 

    let ss = EntireSystem::template();
    let pri_send = create_socket(CONFIG.backup.pri_send.to_string());
    let ss_serialized = serde_json::to_string(&ss).unwrap();
    let sec_recv = CONFIG.backup.sec_recv;


    //Recovery Scripts
    loop{
        sleep(Duration::from_millis(CONFIG.backup.sleep_dur_milli as u64));

        pri_send.send_to(ss_serialized.as_bytes(),  sec_recv);
        //println!("Sent: {}", ss_serialized);
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

    let ss = EntireSystem::template();
    let pri_send = create_socket(CONFIG.backup.pri_send.to_string());
    let ss_serialized = serde_json::to_string(&ss).unwrap();
    let sec_recv = CONFIG.backup.sec_recv;
    loop {
        sleep(Duration::from_millis(1000));

        pri_send.send_to(ss_serialized.as_bytes(),  sec_recv);
        select! {
            default => {}
        }
    }
}
