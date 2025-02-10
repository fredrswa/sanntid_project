use std::io::Result;
use std::thread::spawn;
use std::time::Duration;
use crossbeam_channel as cbc;

use fsm_elevator_rust::elevator_operation::fsm::*;
use fsm_elevator_rust::elevator_io::elevator::*;
use fsm_elevator_rust::elevator_operation::defs::*;
use fsm_elevator_rust::elevator_io::sensor_polling;
use fsm_elevator_rust::elevator_operation::timer::*;

fn main() -> Result<()> {
    let mut elevator = Elevator::init("localhost:15657", NUM_FLOORS, NUM_BUTTONS)?;

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

    //let (stop_button_tx, stop_button_rx) = cbc::unbounded::<bool>(); 
    //{
    //    let elevator = elevator.clone();
    //    spawn(move || sensor_polling::stop_button(elevator, stop_button_tx, poll_period)); 
    //}

    let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>(); 
    {
        let elevator = elevator.clone();
        spawn(move || sensor_polling::obstruction(elevator, obstruction_tx, poll_period)); 
    }

    let mut timer = Timer::new(Duration::from_secs(DOOR_OPEN_S));
    fsm_init(&mut elevator);
    println!("{}", &elevator);
    
    loop {
        cbc::select! {
            recv(call_button_rx) -> cb_message => {
                let call_button = cb_message.unwrap();
                println!("{}", &elevator);
                fsm_on_request_button_press(&mut elevator, &mut timer, call_button.floor as usize, ButtonType::from_u8(call_button.call).unwrap());
            }

            recv(floor_sensor_rx) -> fs_message => {
                let floor = fs_message.unwrap();
                fsm_on_floor_arrival(&mut elevator, &mut timer, floor as usize);
            }

            //recv(stop_button_rx) -> sb_message => {
            //    let stop = sb_message.unwrap();
            //}

            recv(obstruction_rx) -> ob_message => {
                let obs = ob_message.unwrap();
                if !obs{
                    timer.start();
                }
                elevator.blocked = obs;                    
            }

            default => {}
            
        }
        if timer.is_expired() && !elevator.blocked {
            fsm_on_door_timeout(&mut elevator, &mut timer);
        }  
    }
}
