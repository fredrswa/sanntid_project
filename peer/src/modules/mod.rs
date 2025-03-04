use std::io::Result;
use std::thread;
use crossbeam_channel as cbc;



mod mod_fsm;
mod mod_network;
mod mod_assigner;
mod mod_io;


use crate::modules::mod_fsm::fsm::ElevatorSystem;
use driver_rust::elevio::poll as sensor_polling;

pub fn run() {
    let (io_call_tx,io_call_rx) = cbc::unbounded::<sensor_polling::CallButton>();
    let addr = "localhost:15657";
    let es: ElevatorSystem = ElevatorSystem::new();
    
    let mut es1 = es.clone();
    thread::spawn(move || {mod_fsm::run(&mut es1, &io_call_rx);});
    let mut es2 = es.clone();
    thread::spawn(move || {mod_io::run(&mut es2, &io_call_tx);});

    loop {
        cbc::select! {
            default => {}
        }
    }
}