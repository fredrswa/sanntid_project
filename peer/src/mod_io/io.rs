//! IO Functions    |   
//! Defines functions used in IO Module

/// Standard Library
use std::{
    process::Command,
    collections::HashMap,
    cmp::max
    };

///Internal Crates
use crate::config::*;

/// CONFIG
static NUM_PEERS: usize = CONFIG.network.peers as usize;
static NUM_FLOORS: usize = CONFIG.elevator.num_floors as usize;

/// Call Assigner | Calls the hall_request_assigner executable, responsible for assigning which elevator should take what order.
pub fn call_assigner (sys: EntireSystem, connected_peers: [bool; NUM_PEERS]) -> AssignerOutput{

    let mut sys = sys;

    for (id, connected) in connected_peers.iter().enumerate() {
        if !*connected {
            sys.states.remove(&format!("{}", id));
        }
    }

    let elev_states = match serde_json::to_string(&sys) {
        Ok(json) => json,
        Err(e) => {
            panic!("Failed to serialize JSON: {}", e);
        }
    }; 

    let program = "../tools/hall_request_assigner";
    let output = match Command::new(program)
        .arg("-i")
        .arg(&elev_states)
        .output()
        {
            Ok(output) => output,
            Err(e) => {
                panic!("Failed to run hall request assigner: {}", e);
            }
        };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string(); //String output from assigner.
    
    let stderr = String::from_utf8_lossy(&output.stderr); //Error returned from assigner.
    if !stderr.is_empty() {
        panic!("Assigner call returened an error!: {}", stderr);
    }
    
    //Deserializes the assigner output into a AssignerOutput struct 
    let assigner_string = format!("{{\"elevators\": {}}}", stdout);
    let new_states: AssignerOutput = match serde_json::from_str(&assigner_string) {
        Ok(new_states) => new_states,
        Err(e) => {
            panic!("Failed to deserilize new state to JSON format: {}", e);
        }
    }; 

    new_states
}

/// Update Own State | Updates its own world view according to its own elevator system, given to the IO module by FSM module.
pub fn update_own_state (world_view: EntireSystem, current_elevator_system: ElevatorSystem, created_completed_timestamps: Vec<Vec<(i64, i64)>>) -> EntireSystem {
    
    let mut world_view = world_view;
    
    world_view.hallRequests = decide_hall_requests(created_completed_timestamps);
    let self_id = SELF_ID.to_string();

    //Updates cab requests in world view according to elevator system.
    for (i, val) in current_elevator_system.requests.iter().enumerate() {
        if let Some(state) = world_view.states.get_mut(&self_id) {
            state.cabRequests[i] = val[2];
        }
    }
    
    //Sets other parameters given by the elevator system.
    world_view.states.get_mut(&self_id).unwrap().behavior = current_elevator_system.status.behavior;
    world_view.states.get_mut(&self_id).unwrap().floor = current_elevator_system.status.curr_floor as isize;
    world_view.states.get_mut(&self_id).unwrap().direction = current_elevator_system.status.curr_dirn;

    world_view
}

/// Merge Entire System | Merges its own world view with an incoming world view from another peer.
pub fn merge_entire_systems (world_view: EntireSystem, incoming_world_view: EntireSystem, created_completed_timestamps: Vec<Vec<(i64, i64)>>) -> EntireSystem {
    
    let new_world_view = EntireSystem {
        hallRequests: decide_hall_requests(created_completed_timestamps),
        states: merge_states(SELF_ID.to_string(), world_view.states, incoming_world_view.states)
    };
    
    new_world_view
}

/// Decide Hall Requests | Decides which hallRequests that are still active by using unix timestamps. 
/// 
/// If created if greater than completed -> Active.
/// If completed is greater than created -> Inactive.
pub fn decide_hall_requests(created_completed_timestamps: Vec<Vec<(i64, i64)>>) -> Vec<[bool; 2]> {
    
    let active_hall_requests: Vec<[bool; 2]> = created_completed_timestamps.iter()
        .map(|timestamps| {
            [timestamps[0].0 > timestamps[0].1, timestamps[1].0 > timestamps[1].1]
        })
        .collect();
    
    active_hall_requests
}

/// Merge States | Merges the elevator states in two world views. Updates only other peers.
pub fn merge_states (own_id: String, wws: HashMap<String, States>, iwws: HashMap<String, States>) -> HashMap<String, States> { 
    
    let mut merged_states: HashMap<String, States> = HashMap::new();
    
    for (key, val) in &wws {
        merged_states.insert(key.clone(), val.clone());
    }

    for (incoming_id, incoming_state) in iwws {
        if incoming_id != own_id {
            if let Some(existing_state) = merged_states.get_mut(&incoming_id) {
                existing_state.behavior = incoming_state.behavior;
                existing_state.direction = incoming_state.direction;
                existing_state.floor = incoming_state.floor;
                existing_state.cabRequests = incoming_state.cabRequests;
            } else {
                merged_states.insert(incoming_id, incoming_state); // This is a new peer we haven't seen before.
            }
        }
    }
    
    merged_states
}

/// Merge Timestamps | Merges two timestamp matrices, keeping the newest timestamps.
pub fn merge_timestamps (own_timestamps: Vec<Vec<(i64, i64)>>, incoming_timestamps: Vec<Vec<(i64, i64)>>) -> Vec<Vec<(i64, i64)>> {

    let mut merged_timestamps: Vec<Vec<(i64, i64)>> = vec![Vec::new(); CONFIG.elevator.num_floors as usize ];

    for i in 0..NUM_FLOORS {
        let own_floor = own_timestamps.get(i as usize).unwrap();
        let incoming_floor = incoming_timestamps.get(i as usize).unwrap();

        let mut merged_floor = Vec::new();
        
        for (own, incoming) in own_floor.iter().zip(incoming_floor.iter()) {
            let start_time = max(own.0, incoming.0); // Keep the most recent start time
            let end_time = max(own.1, incoming.1);   // Keep the most recent end time
            merged_floor.push((start_time, end_time));
        }

        merged_timestamps[i as usize] = merged_floor;
    }

    merged_timestamps
}
