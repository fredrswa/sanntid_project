//! IO.RS
//! Responsible for handling all input operations, communicates with the assigner and network to generate orders for fsm.

/// Sub Modules
pub mod io;

use crate::config::config::backup::sleep_dur_milli;
///
use crate::config::*;
use crate::mod_backup::send_latest_primary;
use crate::mod_io::io::{merge_entire_systems, update_own_state, merge_timestamps};
use driver_rust::elevio::poll as sensor_polling;

///Includes
use crossbeam_channel as cbc;
use io::call_assigner;
use std::thread::{spawn, sleep};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
//use hardware::init_elevator;


pub fn run(
    mut world_view: EntireSystem,
    es: &mut ElevatorSystem, 
    call_button_from_io_tx: &cbc::Sender<sensor_polling::CallButton>,
    network_to_io_rx: &cbc::Receiver<TimestampsEntireSystem>,
    io_to_network_tx: &cbc::Sender<TimestampsEntireSystem>,
    connected_peers_rx: &cbc::Receiver<[bool; CONFIG.network.peers as usize]>,

    io_to_fsm_requests_tx: &cbc::Sender<Vec<Vec<bool>>>,
    
    fsm_to_io_es_rx: &cbc::Receiver<ElevatorSystem>,
    io_to_fsm_es_tx: &cbc::Sender<ElevatorSystem>,

    timestamps_to_io_rx: &cbc::Receiver<Vec<Vec<(i64, i64)>>>,
    ){

    let mut created_completed_timestamps: Vec<Vec<(i64, i64)>> = vec![vec![(0, 1); 3]; CONFIG.elevator.num_floors as usize];

    let mut connected_peers = [false; CONFIG.network.peers as usize];
    let self_id: usize = SELF_ID.to_string().parse().expect("Was not able to parse SELF ID as int");
    connected_peers[self_id] = true;


    /* ########################### Call Button ################################################################################## */
    let poll_period = Duration::from_millis(25);

    let (call_button_tx, call_button_rx) = cbc::unbounded::<sensor_polling::CallButton>();
    let (io_to_backup_state_tx, io_to_backup_state_rx) = cbc::unbounded::<EntireSystem>(); 
    {
       let elevator = es.elevator.clone();
       spawn(move || sensor_polling::call_buttons(elevator, call_button_tx, poll_period));
       spawn(move || send_latest_primary(io_to_backup_state_rx)); 
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
            recv(timestamps_to_io_rx) -> timestamps => {
                if let Ok(new_timestamps) = timestamps {
                    created_completed_timestamps = merge_timestamps(created_completed_timestamps, new_timestamps);
                }
            }

            recv(connected_peers_rx) -> cp_message => {
                if let Ok(cp) = cp_message {
                    connected_peers = cp;
                }
            }
            /* Gets elevator system from FSM
               Uses the elevator system to update knowlegde about own world view
               Uses world view to call the assigner, updating who takes which order
               Send the updated hall orders back to the FSM */
            recv(fsm_to_io_es_rx) -> current_es => {
                if let Ok(current_elevator_system) = current_es {
                    
                    world_view = update_own_state(world_view, current_elevator_system.clone(), created_completed_timestamps.clone());

                    //println!("{}", TimestampsEntireSystem{es: world_view.clone(), timestamps: created_completed_timestamps.clone()});

                    let _ = match io_to_network_tx.send(TimestampsEntireSystem{es: world_view.clone(), timestamps: created_completed_timestamps.clone()}) {
                        Ok(ok) => ok,
                        Err(e) => {panic!("Failed to send World View from IO to Network: {}", e)}
                    };

                    //Only pass Elevators that are still active from heartbeats.
                    let assigner_output = call_assigner(world_view.clone(), connected_peers.clone());
                    
                 
                    //////////////////////////
                    es.set_all_lights_world_view(&world_view);

                    let requests = assigner_output.elevators[&*SELF_ID].clone();
                    //println!("{:#?}", requests);
                    
                    let _ = match io_to_fsm_requests_tx.send(requests) {
                        Ok(ok) => ok,
                        Err(e) => {panic!("Failed to send Elevator System from IO to FSM: {}", e)}
                    };
                }
            }

            /* Handles incoming world view from another peer
               Merges incoming world view with its own
               Calls the assigner with the new world view
               Gives FSM the updated requests from the assigner */
            recv(network_to_io_rx) -> incoming_world_view => {
                if let Ok(iww) = incoming_world_view {

                    created_completed_timestamps = merge_timestamps(created_completed_timestamps, iww.timestamps);


                    world_view = merge_entire_systems(world_view.clone(), iww.es, created_completed_timestamps.clone());
                    
                    //println!("{}", TimestampsEntireSystem{es: world_view.clone(), timestamps: created_completed_timestamps.clone()});
                    
                    // Try here first
                    io_to_backup_state_tx.send(world_view.clone());
                    let assigner_output = call_assigner(world_view.clone(), connected_peers.clone());
                    

                    //////////////////////////
                    es.set_all_lights_world_view(&world_view);

                    let requests = assigner_output.elevators[&*SELF_ID].clone();
                    let _ = match io_to_fsm_requests_tx.send(requests) {
                        Ok(ok) => ok,
                        Err(e) => {panic!("Failed to send Elevator System from IO to FSM: {}", e)}
                    };
                }
            }

            default => {
                sleep(Duration::from_millis(25))}
            
        }
    }
}