//! Initializes system state, sets up module communication channels, and spawns all modules

#![crate_name = "peer"]
/// Modules
pub mod config;
mod mod_fsm;
mod mod_io;
mod mod_network;
mod mod_backup;
mod mod_hardware;

/// Standard Library
use std::{io::Result, 
    time::Duration,
    thread::{spawn, sleep}};

/// External Crates
use crossbeam_channel::{select, unbounded};

/// Internal Crates
use config::*;

/// Driver
use driver_rust::elevio::poll as sensor_polling;

/// Main functions
fn main() -> Result<()> {
    println!("ID:{} Primary: {} Humble:{}", *SELF_ID, *PRIMARY, *HUMBLE);
    //Handle primary/backup logic
    let mut world_view: EntireSystem = EntireSystem::template();
    let mut elev_sys: Option<ElevatorSystem> = None;

    let _ = mod_hardware::init();

    match *PRIMARY {
        true => {
            match *HUMBLE {
                // HUMBLE PRIMARY
                true => {
                    (world_view, elev_sys) = mod_backup::humble_state();
                }
                // REGULAR PRIMARY
                false => {
                    println!("Starting as primary");
                    elev_sys = Some(ElevatorSystem::new());
                }
            }
        }
        // BACKUP
        false => {
            (world_view, elev_sys) = mod_backup::backup_state();
        }
    }

    mod_backup::spawn_secondary_exe();
    let elev_sys: ElevatorSystem = elev_sys.unwrap();


    // Start initializing modules

    // CHANNELS
    let (io_call_tx,io_call_rx) = unbounded::<sensor_polling::CallButton>();

    let (network_to_io_tx, network_to_io_rx) = unbounded::<TimestampsEntireSystem>();
    let (io_to_network_tx, io_to_network_rx) = unbounded::<TimestampsEntireSystem>();

    let (connected_peers_tx, connected_peers_rx) = unbounded::<[bool; CONFIG.network.peers as usize]>();
    
    let (io_to_fsm_requests_tx, io_to_fsm_requests_rx) = unbounded::<Vec<Vec<bool>>>();
    
    let (fsm_to_io_es_tx, fsm_to_io_es_rx) = unbounded::<ElevatorSystem>();
    let (io_to_fsm_es_tx, io_to_fsm_es_rx) = unbounded::<ElevatorSystem>();

    let (obstruction_to_io_tx, obstruction_to_io_rx) = unbounded::<bool>(); 

    let (timestamps_to_io_tx, timestamps_to_io_rx) = unbounded::<Vec<Vec<(i64, i64)>>>();

    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();

    std::panic::set_hook(Box::new(|panic_info| {
        std::process::exit(1);
    }));

    // SPAWN MODULES
    {
        // FSM MODULE
        let mut es1 = elev_sys.clone();
        spawn(move || 
            {mod_fsm::run(
            // FSM CHANNELS
            &mut es1,
            &io_call_rx,
            &timeout_tx,
            &fsm_to_io_es_tx,
            &io_to_fsm_es_rx,
            &io_to_fsm_requests_rx,
            &timestamps_to_io_tx,
            &obstruction_to_io_tx
        );});
        
        // IO MODULE
        let mut es2 = elev_sys.clone();
        spawn(move || 
            {mod_io::run(
            // IO CHANNELS
            world_view,
            &mut es2, 
            &io_call_tx,
            &io_to_fsm_requests_tx,
            &fsm_to_io_es_rx,
            &network_to_io_rx,
            &io_to_network_tx,
            &connected_peers_rx,
            &timestamps_to_io_rx,
        );});
        
        // NETWORK MODULE
        let es3 = elev_sys.clone();
        spawn(move || {
            loop {
            mod_network::run(
            &es3,
            &network_to_io_tx, 
            &io_to_network_rx,
            &connected_peers_tx,
            &obstruction_to_io_rx);
            
            sleep(Duration::from_secs(30));}
            println!("Whoops, broke the whole loop");});
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