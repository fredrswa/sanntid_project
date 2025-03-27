use std::process::Command;
use std::collections::HashMap;
use std::cmp::max;

use crate::config::*;

//Calls the hall_request_assigner, responsible for assigning which elevator should take what order
pub fn call_assigner(sys: EntireSystem, connected_peers:[bool; CONFIG.network.peers as usize]) -> AssignerOutput{

    let mut sys = sys;

    //println!("{:#?}", connected_peers);
    for (id, connected) in connected_peers.iter().enumerate() {
        if !*connected {
            sys.states.remove(&format!("{}", id));
        }
    }

    //Serializes the world view into a JSON string
    let elev_states = match serde_json::to_string(&sys) {
        Ok(json) => json,
        Err(e) => {
            panic!("Failed to serialize JSON: {}", e);
        }
    }; 

    //Calls the hall_request_assigner, passing it the JSON string
    let program = "../tools/assigner/hall_request_assigner";
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

    //String output from assigner
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    //Error returned from assigner
    let stderr = String::from_utf8_lossy(&output.stderr); 
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

    return new_states;
}

//Updates its own world view according to its own elevator system, given to the IO module by FSM module.
pub fn update_own_state (world_view: EntireSystem, current_elevator_system: ElevatorSystem, created_completed_timestamps: Vec<Vec<(i64, i64)>>) -> EntireSystem {
    let mut world_view = world_view;

    //Sets a hall request button to TRUE if timestamp of created order is newer than completed timestamp
    world_view.hallRequests = decide_hall_requests(created_completed_timestamps);

    //Updates cab requests in world view according to elevator system 
    for (i, val) in current_elevator_system.requests.iter().enumerate() {
        if let Some(state) = world_view.states.get_mut(CONFIG.peer.id) {
            state.cabRequests[i] = val[2];
        }
    }
    
    //Sets other parameters given by the elevator system
    world_view.states.get_mut(CONFIG.peer.id).unwrap().behavior = current_elevator_system.status.behavior;
    world_view.states.get_mut(CONFIG.peer.id).unwrap().floor = current_elevator_system.status.curr_floor as isize;
    world_view.states.get_mut(CONFIG.peer.id).unwrap().direction = current_elevator_system.status.curr_dirn;

    return world_view;
}

//Merges its own world view with an incoming world view from another peer
pub fn merge_entire_systems (
    world_view: EntireSystem, 
    incoming_world_view: EntireSystem, 
    created_completed_timestamps: Vec<Vec<(i64, i64)>>
) -> EntireSystem {
    let new_world_view = EntireSystem {
        hallRequests: decide_hall_requests(created_completed_timestamps),
        states: merge_states(CONFIG.peer.id.to_string(), world_view.states, incoming_world_view.states)
    };
    return new_world_view;
}

//Decides which hallRequests that are still active by using timestamps
pub fn decide_hall_requests(created_completed_timestamps: Vec<Vec<(i64, i64)>>,) -> Vec<[bool; 2]> {
    // Iterate over each inner vector in created_completed_timestamps
    let new_ww: Vec<[bool; 2]> = created_completed_timestamps.iter()
        .map(|timestamps| {
            [timestamps[0].0 > timestamps[0].1, timestamps[1].0 > timestamps[1].1]
        })
        .collect();
    return new_ww;
}

//Merges the states for all elevators in the system. Used in the update of world views (merge_entire_systems)
//Update own world view only if not self, DDN TF U DOING.
pub fn merge_states (own_id: String, wws: HashMap<String, States>, iwws: HashMap<String, States>) -> HashMap<String, States> { 
    let mut merged_states: HashMap<String, States> = HashMap::new();
    
    // First, copy all states from our world view
    for (key, val) in &wws {
        merged_states.insert(key.clone(), val.clone());
    }
    
    // Then update with states from incoming world view, except our own
    for (key, incoming_state) in iwws {
        if key != own_id {
            if let Some(existing_state) = merged_states.get_mut(&key) {
                // Update state attributes
                existing_state.behavior = incoming_state.behavior;
                existing_state.direction = incoming_state.direction;
                existing_state.floor = incoming_state.floor;
                
                // IMPORTANT: For other elevators, accept their cab requests
                // We don't modify or merge cab requests - each elevator owns its own
                existing_state.cabRequests = incoming_state.cabRequests;
            } else {
                // This is a new peer we haven't seen before
                merged_states.insert(key, incoming_state);
            }
        }
        // CRITICAL: We NEVER update our own cab requests from incoming state
        // Our cab requests are strictly local and managed by our FSM module
    }
    
    return merged_states;
}

pub fn merge_timestamps(
    own_timestamps: Vec<Vec<(i64, i64)>>,
    incoming_timestamps: Vec<Vec<(i64, i64)>>,
) -> Vec<Vec<(i64, i64)>> {

    let mut merged_timestamps: Vec<Vec<(i64, i64)>> = vec![Vec::new(); CONFIG.elevator.num_floors as usize ];

    for i in 0..CONFIG.elevator.num_floors {
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

    return merged_timestamps;
}
