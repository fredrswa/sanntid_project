use std::process::Command;
use std::fs;
use std::collections::{HashMap};

use tokio::runtime::EnterGuard;

use crate::config::*;

pub fn call_assigner(sys: EntireSystem) -> AssignerOutput{

    let elev_states = match serde_json::to_string(&sys) {
        Ok(json) => json,
        Err(e) => {
            panic!("Failed to serialize JSON: {}", e);
        }
    }; 

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

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !stderr.is_empty() {
        panic!("Assigner call returened an error!: {}", stderr);
    }
    
    let assigner_string = format!("{{\"elevators\": {}}}", stdout);
    
    let new_states: AssignerOutput = match serde_json::from_str(&assigner_string) {
        Ok(new_states) => new_states,
        Err(e) => {
            panic!("Failed to deserilize new state to JSON format: {}", e);
        }
    }; 

    new_states
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

pub fn update_own_state (world_view: EntireSystem, current_elevator_system: ElevatorSystem) -> EntireSystem {
    let mut world_view = world_view;

    let mut i = 0;
    for val in current_elevator_system.requests.iter() {
        world_view.hallRequests[i][0] = val[0];
        world_view.hallRequests[i][1] = val[1];
        world_view.states.get_mut(CONFIG.peer.id).unwrap().cabRequests[i] = val[2];
        i+=1;

    }

    world_view.states.get_mut(CONFIG.peer.id).unwrap().behavior = current_elevator_system.status.behavior;
    world_view.states.get_mut(CONFIG.peer.id).unwrap().floor = current_elevator_system.status.curr_floor as isize;
    world_view.states.get_mut(CONFIG.peer.id).unwrap().direction = current_elevator_system.status.curr_dirn;

    return world_view;
}

//Merges world views
pub fn merge_entire_systems (own_id: String, world_view: EntireSystem, incoming_world_view: EntireSystem) -> EntireSystem {
    let new_world_view = EntireSystem {
        hallRequests: merge_hall_requests(world_view.hallRequests, incoming_world_view.hallRequests),
        states: merge_states(own_id, world_view.states, incoming_world_view.states)
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

//Merges the states for all elevators in the system. Return new, updates, state.
//Update only if not self, DDN TF U DOING.
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

    merged_states

    
    //Asked claude to refactor the code below, dont know if above works but look right (and is a lot cleaner)
    /* let mut new_wws: HashMap<String, States> = HashMap::new();
    
    for ((key1, val1),(key2,val2)) in wws.iter().zip(iwws.iter()) {
        if *key1 != own_id {
            new_wws.insert(key1.clone(), val1.clone());
            new_wws.get_mut(key1).unwrap().behavior = val2.behavior;
            new_wws.get_mut(key1).unwrap().direction = val2.direction;
            new_wws.get_mut(key1).unwrap().floor = val2.floor;
            new_wws.get_mut(key1).unwrap().cab_requests = val1.cab_requests.iter()
            .zip(val2.cab_requests.iter())
            .map(|(&a, &b)| a || b)
            .collect();
        }
    };

    return new_wws; */
}



pub fn update_es_from_assigner (elevator_system: ElevatorSystem, assigner_output: AssignerOutput) -> ElevatorSystem {
    let mut es = elevator_system;

    es.requests = assigner_output.elevators[CONFIG.peer.id].clone() ;
    return es;
}