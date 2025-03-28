//! IO Module
//! Responsible for handling all input operations.
//! Communicates with the assigner and network to align world views. 
//! Uses world views to generate orders for fsm.

/// Sub Modules
pub mod io;

/// Standard Library
use std::{
    thread::{spawn, sleep},
    time::Duration
    };

/// External Crates
use crossbeam_channel as cbc;
use driver_rust::elevio::poll as sensor_polling;


/// Internal Crates
use crate::config::*;
use crate::mod_backup::send_latest_primary;
use crate::mod_io::io::{merge_entire_systems, update_own_state, merge_timestamps};
use io::call_assigner;

/// CONFIG
static NUM_PEERS: usize = CONFIG.network.peers as usize;
static NUM_FLOORS: usize = CONFIG.elevator.num_floors as usize;

/// Run IO Module | Runs the logic in IO module when called in Main.
pub fn run (
    mut world_view: EntireSystem,
    elevator_system: &mut ElevatorSystem,

    call_button_from_io_tx: &cbc::Sender<sensor_polling::CallButton>,
    io_to_fsm_requests_tx: &cbc::Sender<Vec<Vec<bool>>>,
    fsm_to_io_es_rx: &cbc::Receiver<ElevatorSystem>,
    
    network_to_io_rx: &cbc::Receiver<TimestampsEntireSystem>,
    io_to_network_tx: &cbc::Sender<TimestampsEntireSystem>,
    connected_peers_rx: &cbc::Receiver<[bool; NUM_PEERS]>,

    timestamps_to_io_rx: &cbc::Receiver<Vec<Vec<(i64, i64)>>>,
    ){

    let mut created_completed_timestamps: Vec<Vec<(i64, i64)>> = vec![vec![(0, 1); 3];  NUM_FLOORS];

    let mut connected_peers = [false; NUM_PEERS as usize];
    let self_id: usize = SELF_ID.to_string().parse().expect("Was not able to parse SELF ID as int");
    connected_peers[self_id] = true;

    let poll_period = Duration::from_millis(25);

    let (call_button_tx, call_button_rx) = cbc::unbounded::<sensor_polling::CallButton>();
    let (io_to_backup_state_tx, io_to_backup_state_rx) = cbc::unbounded::<EntireSystem>(); 
    {
       let elevator = elevator_system.elevator.clone();
       spawn(move || sensor_polling::call_buttons(elevator, call_button_tx, poll_period));
       spawn(move || send_latest_primary(io_to_backup_state_rx)); 
    }

    loop {
        cbc::select! {

            // Receives all button presses from the hardware.
            recv(call_button_rx) -> cb_message => {
                if let Ok(call_button) = cb_message {
                    call_button_from_io_tx.send(call_button).unwrap();
                }
            }

            // Receives timestamps from FSM.
            recv(timestamps_to_io_rx) -> timestamps => {
                if let Ok(new_timestamps) = timestamps {
                    created_completed_timestamps = merge_timestamps(created_completed_timestamps, new_timestamps);
                }
            }

            // Receives which peers are connected from network. Reassigns hall requests if a peer connects or disconnects.
            recv(connected_peers_rx) -> cp_message => {
                let prev = connected_peers.clone();
                if let Ok(cp) = cp_message {
                    connected_peers = cp;
                }
                if prev != connected_peers {
                    let assigner_output = call_assigner(world_view.clone(), connected_peers.clone());
                    let requests = assigner_output.elevators[&*SELF_ID].clone();
                    
                    let _ = match io_to_fsm_requests_tx.send(requests) {
                        Ok(ok) => ok,
                        Err(e) => {panic!("Failed to send Elevator System from IO to FSM: {}", e)}
                    };
                }
            }

            // Gets updated elevator system from FSM. 
            recv(fsm_to_io_es_rx) -> current_es => {
                if let Ok(current_elevator_system) = current_es {
                    world_view = update_own_state(world_view, current_elevator_system.clone(), created_completed_timestamps.clone());

                    let _ = match io_to_network_tx.send(TimestampsEntireSystem{es: world_view.clone(), timestamps: created_completed_timestamps.clone()}) {
                        Ok(ok) => ok,
                        Err(e) => {panic!("Failed to send World View from IO to Network: {}", e)}
                    };

                    let assigner_output = call_assigner(world_view.clone(), connected_peers.clone());
                    let requests = assigner_output.elevators[&*SELF_ID].clone();

                    let _ = match io_to_fsm_requests_tx.send(requests) {
                        Ok(ok) => ok,
                        Err(e) => {panic!("Failed to send Elevator System from IO to FSM: {}", e)}
                    };

                    elevator_system.set_all_lights_world_view(&world_view);
                }
            }

            // Handles incoming world view from another peer.
            recv(network_to_io_rx) -> incoming_world_view => {
                if let Ok(iww) = incoming_world_view {
                    created_completed_timestamps = merge_timestamps(created_completed_timestamps, iww.timestamps);

                    world_view = merge_entire_systems(world_view.clone(), iww.es, created_completed_timestamps.clone());
                    
                    let _ = io_to_backup_state_tx.send(world_view.clone());

                    let assigner_output = call_assigner(world_view.clone(), connected_peers.clone());
                    
                    let requests = assigner_output.elevators[&*SELF_ID].clone();
                    let _ = match io_to_fsm_requests_tx.send(requests) {
                        Ok(ok) => ok,
                        Err(e) => {panic!("Failed to send Elevator System from IO to FSM: {}", e)}
                    };

                    elevator_system.set_all_lights_world_view(&world_view);
                }
            }
            default => {sleep(Duration::from_millis(25))}
        }
    }
}