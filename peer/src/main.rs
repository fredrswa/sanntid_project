
#![crate_name = "peer"]
//! System Overview
 
//! main.rs
// |->config.rs
// |   |-> CONFIG              "A compile time global static from Config.toml"
// |   |-> Structs             "All structs used throughout the project"
// |->modules
//     |->mod_hardware
//     |       |-> mod.rs      "Runs hardware, correct ports"
//     |->mod_backup
//     |       |-> mod.rs      "Handles primary state or backup state"
//     |->mod_io
//     |       |-> mod.rs      "Logic and communication with modules"
//     |       |-> io.rs       "functions regarding world_states and assigning"
//     |->mod_network
//     |       |-> mod.rs      "network channels and logic"
//     |       |-> network.rs  "functions: heartbeat, send, recv"
//     |->mod_fsm
//     |       |-> mod.rs      "order creation, state and communication with mod_io"
//     |       |-> fsm.rs      "finite state machine"
//     |       |-> timer.rs    "timer struct"
//     |       |-> request.rs  "request handling and updating" 

pub mod config;
mod mod_fsm;
mod mod_io;
mod mod_network;
mod mod_backup;
mod mod_hardware;

/// INCLUDES
use std::{env, 
          io::Result, 
          time::{Duration, Instant},
          thread::{spawn, sleep}};
use chrono::{DateTime, Utc};
use crossbeam_channel::{select, unbounded};

/// DRIVER
use driver_rust::elevio::poll as sensor_polling;
/// STRUCTS AND CONFIG
use config::*;


/// Main functions
fn main() -> Result<()> {
    //Command line argument
    let args: Vec<String> = env::args().collect();
    let primary: bool = args.get(1).expect("No arguments passed").parse().unwrap(); //A nice line of code

    //Handle primary/backup logic
    let mut world_view: EntireSystem = EntireSystem::template();
    let mut elev_sys: Option<ElevatorSystem> = None;

    let _ = mod_hardware::init();

    if !primary 
    {   // Start as backup state
        (world_view, elev_sys) = mod_backup::backup_state();
    } else {
        
        elev_sys = Some(ElevatorSystem::new());
    }

    mod_backup::spawn_secondary();
    let elev_sys: ElevatorSystem = elev_sys.unwrap();


    // Start initializing modules

    // CHANNELS
    let (io_call_tx,io_call_rx) = unbounded::<sensor_polling::CallButton>();

    let (network_to_io_tx, network_to_io_rx) = unbounded::<TimestampsEntireSystem>();
    let (io_to_network_tx, io_to_network_rx) = unbounded::<TimestampsEntireSystem>();
    
    let (io_to_fsm_requests_tx, io_to_fsm_requests_rx) = unbounded::<Vec<Vec<bool>>>();
    
    let (fsm_to_io_es_tx, fsm_to_io_es_rx) = unbounded::<ElevatorSystem>();
    let (io_to_fsm_es_tx, io_to_fsm_es_rx) = unbounded::<ElevatorSystem>();

    let (timestamps_to_io_tx, timestamps_to_io_rx) = unbounded::<Vec<Vec<(DateTime<Utc>, DateTime<Utc>)>>>();

    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();

    std::panic::set_hook(Box::new(|panic_info| {
        std::process::exit(1);
    }));

    // SPAWN MODULES
    {
        // FSM MODULE
        let mut es1 = elev_sys.clone();
        spawn(move || {mod_fsm::run(
            // FSM CHANNELS
            &mut es1,
            &io_call_rx,
            &timeout_tx,
            &fsm_to_io_es_tx,
            &io_to_fsm_es_rx,
            &io_to_fsm_requests_rx,
            &timestamps_to_io_tx
        );});
        
        // IO MODULE
        let mut es2 = elev_sys.clone();
        spawn(move || {mod_io::run(
            // IO CHANNELS
            &mut es2, 
            &io_call_tx,
            &network_to_io_rx,
            &io_to_network_tx,
            &io_to_fsm_requests_tx,
            &fsm_to_io_es_rx,
            &io_to_fsm_es_tx,
            &timestamps_to_io_rx,
        );});
        
        // NETWORK MODULE
        spawn(move || {mod_network::run(
            &network_to_io_tx, 
            &io_to_network_rx
        );});
    }



    let dur = Duration::from_millis(100);
    loop{
        select! {
            recv(timeout_rx) -> timout_struct => {
                panic!("Something happened, we need to CRASH");
            }

            default => {
                sleep(dur)
            }
        }
    }
}