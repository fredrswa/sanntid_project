//! This module handles the operation of the finite state machine (elevator)
//! Or


/// Sub Modules (created from handout)
pub mod fsm;            //Handles implemantation logic
pub mod requests;       //Handles logic regarding requests
pub mod timer;          //Timer for generating timout and handling door_open time and obstruction.
      //Handles hardware interaction


///Crates
use crate::config::*;                   //Config has every struct
use crate::mod_fsm::timer::Timer;       
use crate::mod_fsm::requests::{is_completed, update_timestamps};

///
use crossbeam_channel as cbc;
use driver_rust::elevio::poll as sensor_polling;
use std::{thread::{sleep, spawn}, vec};
use core::time::Duration;
use chrono::Utc;

/// Runs the FSM_module
/// - Interacts with IO to handle and generate order
pub fn run(
    es: &mut ElevatorSystem,
    call_from_io_rx: &cbc::Receiver<sensor_polling::CallButton>,
    timout_tx: &cbc::Sender<Timeout_type>,
    fsm_to_io_tx: &cbc::Sender<ElevatorSystem>,
    io_to_fsm_es_rx: &cbc::Receiver<ElevatorSystem>,
    io_to_fsm_requests_rx: &cbc::Receiver<Vec<Vec<bool>>>,
    timestamps_to_io_tx: &cbc::Sender< Vec<Vec<(i64, i64)>>>
    ) {


    /* ########################### FSM Sensors ######################################################################## */
    let poll_period = Duration::from_millis(25);
    let mut timer = Timer::new(Duration::from_secs(CONFIG.elevator.door_open_s as u64));
    let (floor_sensor_tx, floor_sensor_rx) = cbc::unbounded::<u8>(); 
    let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>(); 
    {
        let elevator = es.elevator.clone();
        spawn(move || sensor_polling::floor_sensor(elevator, floor_sensor_tx, poll_period)); 
        let elevator = es.elevator.clone();
        spawn(move || sensor_polling::obstruction(elevator, obstruction_tx, poll_period)); 
    }
    /* ############################################################################################################### */

    /* ########################### Hall Requests Timestamps ########################################################## */
    
    let mut created_completed_timestamps: Vec<Vec<(i64, i64)>> = vec![vec![(0, 0); 3]; CONFIG.elevator.num_floors as usize];
   
    /* ############################################################################################################### */


    es.init();

    loop {
        cbc::select! {
            recv(call_from_io_rx) -> cb_message => {
                if let Ok(call_button) = cb_message {
                    // println!{"{}", &es};
                    
                    let button_type = call_to_button_type(call_button.call);
                    let floor = call_button.floor as usize;

                    let now = Utc::now().timestamp_millis();
                    created_completed_timestamps[floor][button_type as usize] = (now, now - 1000);
                        
                    //es.on_request_button_press(&mut timer, call_button.floor as usize, button_type);

                    timestamps_to_io_tx.send(created_completed_timestamps.clone()).expect("Could not send timestamps from FSM to IO");
                    fsm_to_io_tx.send(es.clone()).expect("Could not send state from FSM to IO");
                }
            }
            recv(floor_sensor_rx) -> fs_message => {
                if let Ok(floor) = fs_message {

                    let es_before = es.clone();

                    es.on_floor_arrival(&mut timer, floor as usize);

                    let completed_array = is_completed(es_before, es.clone());

                    println!("Completed Array: {:#?}", completed_array);

                    created_completed_timestamps = update_timestamps(completed_array, created_completed_timestamps.clone());
                    
                    timestamps_to_io_tx.send(created_completed_timestamps.clone()).expect("Could not send timestamps from FSM to IO");
                    fsm_to_io_tx.send(es.clone()).expect("Could not send state from FSM to IO");
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
            recv(io_to_fsm_requests_rx) -> updated_request_vector => {
                if let Ok(req) = updated_request_vector {
                    es.requests = req;
                    es.new_requests(&mut timer);
                }
            }
            /* recv(io_to_fsm_es_rx) -> updated_es => {
                if let Ok(upt) = updated_es {
                    *es = upt.clone();
                }
            } */
            default => {sleep(poll_period);}
        }
        if timer.is_expired() && !es.status.door_blocked {
            es.on_door_timeout(&mut timer);
        }

        
        // send own state
        // send confirmation on taken order
        



    }
}