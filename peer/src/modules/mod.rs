use std::io::Result;
use std::thread;
use crossbeam_channel as cbc;



mod mod_fsm;
mod mod_network;
mod mod_assigner;
mod mod_io;


use crate::modules::mod_fsm::fsm::ElevatorSystem;
use driver_rust::elevio::poll::{self as sensor_polling, CallButton};

pub fn run() {

    //IO Sending Channels
    let (io_fsm_call_tx,io_fsm_call_rx) = cbc::unbounded::<sensor_polling::CallButton>();

    //FSM Sending Channels
    let (fsm_timed_out_tx, fsm_timed_out_rx) = cbc::unbounded::<bool>();



    let es: ElevatorSystem = ElevatorSystem::new();
    {
        let mut es1 = es.clone();
        thread::spawn(move || {mod_fsm::run(&mut es1, &io_fsm_call_rx);});
        let mut es2 = es.clone();
        thread::spawn(move || {mod_io::run(&mut es2, &io_fsm_call_tx);});
    }

    loop {
        cbc::select! {
            default => {}
        }
    }
}

pub struct IoChannels {
    // To FSM
    io_fsm_call_tx: cbc::Sender<CallButton>,
    // To Network
    io_network_new_order_tx: cbc::Sender<CallButton>,
}
pub struct FSMChannels {
    

    //From IO
    io_fsm_call_rx: cbc::Receiver<CallButton>,


    //Error Channels
}