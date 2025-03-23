use std::process::Command;
use std::fs;
use std::collections::{HashMap};

use tokio::runtime::EnterGuard;

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

pub fn save_system_state_to_json(sys: EntireSystem) {
    let save_path = "./mod_io/system_state.json";
    
    let argument = match serde_json::to_string(&sys) {
        Ok(json) => json,
        Err(e) => {
            panic!("Failed to serialize JSON: {}", e);
        }
    }; 

    let result = match fs::write(save_path, argument) {
        Ok(result) => { 
            println!("JSON succesfully written: {:#?}", result);
            result
        }
        Err(e) => {
            panic!("Failed to write to file: {}", e);
        }
    };
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
pub fn merge_entire_systems (world_view: EntireSystem, incoming_world_view: EntireSystem) -> EntireSystem {
    let new_world_view = EntireSystem {
        hallRequests: merge_hall_requests(world_view.hallRequests, incoming_world_view.hallRequests),
        states: merge_states(CONFIG.peer.id.to_string(), world_view.states, incoming_world_view.states)
    };

    return new_world_view;
}

//Logical OR between all values in the hall_requests vector
pub fn merge_hall_requests (wwhr: Vec<[bool; 2]>, iwwhr: Vec<[bool; 2]>) -> Vec<[bool; 2]> { 
    let new_ww: Vec<[bool; 2]> = wwhr.iter()
        .zip(iwwhr.iter())
        .map(|(&a, &b)| [a[0] || b[0], a[1] || b[1]])
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