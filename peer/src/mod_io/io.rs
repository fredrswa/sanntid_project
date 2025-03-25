use std::process::Command;
use std::collections::HashMap;
use std::cmp::max;

use crate::config::*;

//Calls the hall_request_assigner, responsible for assigning which elevator should take what order
pub fn call_assigner(sys: EntireSystem) -> AssignerOutput{

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
pub fn update_own_state (world_view: EntireSystem, current_elevator_system: ElevatorSystem) -> EntireSystem {
    let mut world_view = world_view;

    //Sets a hall request button to TRUE if either the world view or the elevator system thinks there is a call
    //Updates cab requests in world view according to elevator system 
    for (i, val) in current_elevator_system.requests.iter().enumerate() {
        world_view.hallRequests[i][0] |= val[0];
        world_view.hallRequests[i][1] |= val[1];
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
        hallRequests: merge_hall_requests(created_completed_timestamps),
        states: merge_states(CONFIG.peer.id.to_string(), world_view.states, incoming_world_view.states)
    };
    return new_world_view;
}

//Logical OR between all values in the hall_requests vector
pub fn merge_hall_requests(created_completed_timestamps: Vec<Vec<(i64, i64)>>,) -> Vec<[bool; 2]> {
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
    let mut merged_states: HashMap<String, States>  = wws.into_iter()
        .filter(|(key, _)| key != &own_id)
        .map(|(key, mut val1)| {
            if let Some(val2) = iwws.get(&key) {
                val1.behavior = val2.behavior;
                val1.direction = val2.direction;
                val1.floor = val2.floor;
                
                val1.cabRequests = val1.cabRequests.iter()
                    .zip(&val2.cabRequests)
                    .map(|(&a, &b)| a || b)
                    .collect();
            }
            (key, val1)
        })
        .collect();

    merged_states.insert(own_id.clone(), iwws[&own_id.clone()].clone());

    return merged_states;
}


//When the assigner is called, update the FSM's requests accordingly
pub fn update_es_from_assigner (elevator_system: ElevatorSystem, assigner_output: AssignerOutput) -> ElevatorSystem {
    let mut es = elevator_system;

    es.requests = assigner_output.elevators[CONFIG.peer.id].clone() ;
    return es;
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
