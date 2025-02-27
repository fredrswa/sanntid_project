/**IO.RS
 * Responsible for handling all input operations, communicates with the assigner.
*/

use driver_rust::elevio::elev::Elevator;
use driver_rust::elevio::poll as sensor_polling;
use peer::mod_fsm::config::{NUM_BUTTONS, NUM_FLOORS};
use crossbeam_channel as cbc;
use std::thread::spawn;
use std::time::Duration;


pub fn run_io(call_button_from_io_tx: cbc::Sender<sensor_polling::CallButton>) {
   let addr = "localhost:15657";
   let e = match Elevator::init(addr, NUM_FLOORS as u8) {
    Ok(elevator) => elevator,
    Err(e) => {
        println!("Failed to connect to elevator at {}: {}", addr, e);
        return;
    }   
    };

   let poll_period = Duration::from_millis(25);

   let (call_button_tx, call_button_rx) = cbc::unbounded::<sensor_polling::CallButton>(); 
   let (floor_sensor_tx, floor_sensor_rx) = cbc::unbounded::<u8>(); 
   let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>(); 
   {
       let elevator = e.clone();
       spawn(move || sensor_polling::call_buttons(elevator, call_button_tx, poll_period)); 
       let elevator = e.clone();
       spawn(move || sensor_polling::floor_sensor(elevator, floor_sensor_tx, poll_period)); 
       let elevator = e.clone();
       spawn(move || sensor_polling::obstruction(elevator, obstruction_tx, poll_period)); 
   }

   loop {
    cbc::select! {
        recv(call_button_rx) -> cb_message => {
            if let Ok(call_button) = cb_message {
                call_button_from_io_tx.send(call_button).unwrap();
            }
            // * if cab: Trigger order and network
            // * if hall: Trigger assigner and network, wait for confirmation to take order.
        }
    }
}
}