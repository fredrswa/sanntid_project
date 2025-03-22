//! IO.RS
//! Responsible for handling all input operations, communicates with the assigner and network to generate orders for fsm.

/// Sub Modules
pub mod io;

///
use crate::config::*;
use driver_rust::elevio::poll as sensor_polling;

///Includes
use crossbeam_channel as cbc;
use std::thread::spawn;
use std::time::Duration;
//use hardware::init_elevator;

pub fn run(
    es: &mut ElevatorSystem, 
    call_button_from_io_tx: &cbc::Sender<sensor_polling::CallButton>,
    network_to_io_rx: &cbc::Receiver<EntireSystem>,
    io_to_network_tx: &cbc::Sender<EntireSystem>,
    fsm_to_io_rx: &cbc::Receiver<ElevatorSystem>,
    ){

    let mut world_view = EntireSystem {
        hallRequests: LAST_SEEN_STATES.hallRequests.clone(),
        states: LAST_SEEN_STATES.states.clone(), 
    };

    println!("{:#?}", world_view);

    /* ########################### Call Button ################################################################################## */
    let poll_period = Duration::from_millis(25);

    let (call_button_tx, call_button_rx) = cbc::unbounded::<sensor_polling::CallButton>(); 
    {
       let elevator = es.elevator.clone();
       spawn(move || sensor_polling::call_buttons(elevator, call_button_tx, poll_period)); 
    }
    /* ########################################################################################################################## */

    loop {
        cbc::select! {
            recv(call_button_rx) -> cb_message => {
                if let Ok(call_button) = cb_message {
                    call_button_from_io_tx.send(call_button).unwrap();
                }
            // * if cab: Trigger order and network
            // * if hall: Trigger assigner and network, wait for confirmation to take order.
            }
            recv(fsm_to_io_rx) -> current_es => {
                let current_es = current_es.unwrap();


                //Update EntireSystem with current es
            }
            recv(network_to_io_rx) -> ww => {
                //let ww = ww.unwrap();

                //Update EntireSystem with ww
            }
        }
    }
}