
mod fsm;
mod timer;
mod requests;
mod setup;


use crossbeam_channel::Receiver;
use crossbeam_channel as cbc;
use driver_rust::elevio::poll as sensor_polling;
use peer::mod_fsm::fsm::*;
use peer::mod_fsm::timer::*;
use peer::mod_fsm::config::*;
use std::thread::{spawn, sleep};
use core::time::Duration;



pub fn run(es: &mut ElevatorSystem,call_from_io_rx: &cbc::Receiver<sensor_polling::CallButton>) {
    let poll_period = Duration::from_millis(25);
    let mut timer = Timer::new(Duration::from_secs(DOOR_OPEN_S));
    let (floor_sensor_tx, floor_sensor_rx) = cbc::unbounded::<u8>(); 
    let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>(); 
    {
        let elevator = es.elevator.clone();
        spawn(move || sensor_polling::floor_sensor(elevator, floor_sensor_tx, poll_period)); 
        let elevator = es.elevator.clone();
        spawn(move || sensor_polling::obstruction(elevator, obstruction_tx, poll_period)); 
    }

    es.init();

    loop {
        cbc::select! {
            recv(call_from_io_rx) -> cb_message => {
                if let Ok(call_button) = cb_message {
                    println!{"{}", &es};
                    es.on_request_button_press(&mut timer, call_button.floor as usize, call_to_button_type(call_button.call));
                }
            }
            recv(floor_sensor_rx) -> fs_message => {
                if let Ok(floor) = fs_message {
                    es.on_floor_arrival(&mut timer, floor as usize);
                }
            }
            recv(obstruction_rx) -> ob_message => {
                if let Ok(obs) = ob_message {
                    if !obs {
                        timer.start();
                    }
                    es.status.door_blocked = obs;
                }
            }
            default => {sleep(poll_period);}
        }
        if timer.is_expired() && !es.status.door_blocked {
            es.on_door_timeout(&mut timer);
        }
    }
}