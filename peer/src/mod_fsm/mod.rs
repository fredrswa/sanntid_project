pub mod fsm;
pub mod requests;
pub mod timer;

use std::time::Duration;
use crossbeam_channel as cbc;
use std::thread::spawn;

use driver_rust::elevio::poll as sensor_polling;
use crate::config::{self, CONFIG};


pub fn run_fsm() {
    let es = config::ElevatorSystem::new(15665, true);
    let elevator = es.elevator.clone();
    let poll_period = Duration::from_millis(25);

    let (call_button_tx, call_button_rx) = cbc::unbounded::<sensor_polling::CallButton>(); 
    {
        let elevator = elevator.clone();
        spawn(move || sensor_polling::call_buttons(elevator, call_button_tx, poll_period)); 
    }

    let (floor_sensor_tx, floor_sensor_rx) = cbc::unbounded::<u8>(); 
    {
        let elevator = elevator.clone();
        spawn(move || sensor_polling::floor_sensor(elevator, floor_sensor_tx, poll_period)); 
    }

    let (stop_button_tx, stop_button_rx) = cbc::unbounded::<bool>(); 
    {
        let elevator = elevator.clone();
        spawn(move || sensor_polling::stop_button(elevator, stop_button_tx, poll_period)); 
    }

    let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>(); 
    {
        let elevator = elevator.clone();
        spawn(move || sensor_polling::obstruction(elevator, obstruction_tx, poll_period)); 
    }

    let timer = Timer::new(Duration::from_secs(CONFIG.door_open_s as u64));
    fsm_init(&mut elevator);
    println!("{}", &elevator);
    
    loop {
        cbc::select! {
            recv(call_button_rx) -> cb_message => {
                let call_button = cb_message.unwrap();
                println!("{}", &elevator);
                fsm_on_request_button_press(&mut elevator, &timer.clone(), call_button.floor as usize, ButtonType::from_u8(call_button.call).unwrap());
            }

            recv(floor_sensor_rx) -> fs_message => {
                let floor = fs_message.unwrap();
                fsm_on_floor_arrival(&mut elevator, &timer.clone(), floor as usize);
            }

            recv(stop_button_rx) -> sb_message => {
                let _stop = sb_message.unwrap();
            }

            recv(obstruction_rx) -> ob_message => {
                let _obstr = ob_message.unwrap();
            }
            default => {sleep(Duration::from_millis(25))}
            
        }
        if timer.is_expired() {
            fsm_on_door_timeout(&mut elevator, &timer.clone());
        }  
    }
    loop {
        
    }
}