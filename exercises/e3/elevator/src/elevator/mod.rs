#![allow(dead_code)]

use std::thread::*;
use std::time::*;
use crossbeam_channel as cbc;
use driver_rust::elevio;
use driver_rust::elevio::elev::{self, Elevator};

mod fsm;
mod requests;
mod config;

use config::*;
use fsm::FSM;

pub fn run() -> std::io::Result<()> {
    let mut fsm1 = FSM::new(Elevator::init("localhost:15657", NUM_FLOORS).unwrap());
    println!("Elevator started:\n{:#?}", fsm1.elevator);

    let poll_period = Duration::from_millis(25);

    let (call_button_tx, call_button_rx) = cbc::unbounded::<elevio::poll::CallButton>();
    {
        let elevator = fsm1.elevator.clone();
        spawn(move || elevio::poll::call_buttons(elevator, call_button_tx, poll_period));
    }

    let (floor_sensor_tx, floor_sensor_rx) = cbc::unbounded::<u8>();
    {
        let elevator = fsm1.elevator.clone();
        spawn(move || elevio::poll::floor_sensor(elevator, floor_sensor_tx, poll_period));
    }

    let (stop_button_tx, stop_button_rx) = cbc::unbounded::<bool>();
    {
        let elevator = fsm1.elevator.clone();
        spawn(move || elevio::poll::stop_button(elevator, stop_button_tx, poll_period));
    }

    let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>();
    {
        let elevator = fsm1.elevator.clone();
        spawn(move || elevio::poll::obstruction(elevator, obstruction_tx, poll_period));
    }
    
    fsm1.init_between_floors();

    loop {
        cbc::select!{
            recv(call_button_rx) -> msg => {
                if let Ok(call_button) = msg {
                    fsm1.requests[call_button.floor as usize][call_button.call as usize] = true;
                    fsm1.fsm_on_request_button_press(call_button.floor, call_to_button_type(call_button.call));
                }
            },
            recv(floor_sensor_rx) -> msg => {
                let prev = u8::MAX;
                if let Ok(floor) = msg {
                    if floor != prev {
                        fsm1.on_floor_arrival(floor);
                    }
                }
            },
        }
    }
}