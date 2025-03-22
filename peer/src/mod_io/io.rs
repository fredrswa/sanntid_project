use std::process::Command;
use std::fs;
use std::collections::{HashMap};

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
    
    println!("{}", stdout);
    let mut new_states = AssignerOutput::new(CONFIG.elevator.num_floors as usize, CONFIG.network.peers as usize);
    new_states = match serde_json::from_str(&stdout) {
        Ok(new_states) => new_states,
        Err(e) => {
            panic!("Failed to deserilize new state to JSON format: {}", e);
        }
    }; 

    println!("{:#?}", new_states);

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


//Merges world views
pub fn merge_entire_systems (own_id: String, world_view: EntireSystem, incoming_world_view: EntireSystem) -> EntireSystem {
    let new_world_view = EntireSystem {
        hallRequests: merge_hall_requests(world_view.hallRequests, incoming_world_view.hallRequests),
        states: merge_states(own_id, world_view.states, incoming_world_view.states)
    };

    new_world_view
}

//Logical OR between all values in the hall_requests vector
pub fn merge_hall_requests (wwhr: Vec<[bool; 2]>, iwwhr: Vec<[bool; 2]>) -> Vec<[bool; 2]> { 
    let new_ww: Vec<[bool; 2]> = wwhr.iter()
        .zip(iwwhr.iter())
        .map(|(&a, &b)| [a[0] || b[0], a[1] || b[1]])
        .collect();
    
    new_ww
}

//Merges the states for all elevators in the system. Return new, updates, state.
//Update only if not self, DDN TF U DOING.
pub fn merge_states (own_id: String, wws: HashMap<String, States>, iwws: HashMap<String, States>) -> HashMap<String, States> { 
    let mut new_wws: HashMap<String, States> = HashMap::new();
    
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

    return new_wws;
}