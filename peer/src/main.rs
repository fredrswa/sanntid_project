//! main coordinates the initialization of all threads
//! Creates the required channels
//! Spawns modules
//! Then "listens" for recovery using timeout_rx which sends a struct indicating which module has failed.

/// INCLUDES
use std::io::Result;
use config::*;
use crossbeam_channel::{select, unbounded, Sender, Receiver};
use std::thread::{spawn, sleep};

/// DRIVER
use driver_rust::elevio::poll as sensor_polling;


/// MODULES
pub mod config;
pub mod mod_fsm;
pub mod mod_io;
pub mod mod_network;


/// main function
fn main() -> Result<()> {
    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();

    { spawn(move || run_modules(timeout_tx)); }

    //Recovery Scripts
    loop{
        select! {
            recv(timeout_rx) -> timout_struct => {
                
            }
        }
    }
}
///module init
fn run_modules(timeout_tx: Sender<Timeout_type>) {
    let (io_call_tx,io_call_rx) = unbounded::<sensor_polling::CallButton>();
    let es: ElevatorSystem = ElevatorSystem::new();
    {
        let mut es1 = es.clone();
        spawn(move || {mod_fsm::run(&mut es1, &io_call_rx, &timeout_tx);});
        let mut es2 = es.clone();
        spawn(move || {mod_io::run(&mut es2, &io_call_tx);});
    }
    loop {
        select! {
            default => {}
        }
    }
}
