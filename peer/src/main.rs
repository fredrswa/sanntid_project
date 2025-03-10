#[allow(dead_code)]
use std::io::Result;
use std::fs;
use config::*;
use crossbeam_channel::{select, unbounded, Sender, Receiver};
use std::thread::{spawn, sleep};

use driver_rust::elevio::poll as sensor_polling;

pub mod config;
pub mod mod_fsm;
pub mod mod_io;
//pub mod mod_network;


fn main() -> Result<()> {
    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();
    loop{
        select! {
            recv(timeout_rx) -> timout_struct => {
                
            }
        }
    }
}

fn run_modules() {
    let (io_call_tx,io_call_rx) = unbounded::<sensor_polling::CallButton>();
    let addr = "localhost:15657";
    let es: ElevatorSystem = ElevatorSystem::new();
    {
        let mut es1 = es.clone();
        spawn(move || {mod_fsm::run(&mut es1, &io_call_rx);});
        let mut es2 = es.clone();
        spawn(move || {mod_io::run(&mut es2, &io_call_tx);});
    }
    loop {
        cbc::select! {
            default => {}
        }
    }
}
