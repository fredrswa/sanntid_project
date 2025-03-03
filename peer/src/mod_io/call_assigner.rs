use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;
use std::process::Command;

use crate::mod_fsm::config::{Behavior, Dirn};
use crate::mod_fsm::config::NUM_FLOORS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct States {
    pub behavior: Behavior,
    pub floor: isize,
    pub direction: Dirn,
    pub cab_requests: [bool; NUM_FLOORS as usize],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntireSystem {
    pub hallRequests: [[bool; 2]; NUM_FLOORS],
    pub states: HashMap<String, States>,
}

//Hardcoded size of assigner output... 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssignerOutput{
    pub one: [[bool; 3]; NUM_FLOORS],
    pub two: [[bool; 3]; NUM_FLOORS],
    pub three: [[bool; 3]; NUM_FLOORS],
}

pub fn call_assigner(sys: EntireSystem) -> Result<AssignerOutput, Box<dyn Error>>{
    
    let elev_states = match serde_json::to_string(&sys) {
        Ok(json) => json,
        Err(e) => {
            panic!("Failed to serialize JSON: {}", e);
        }
    }; 

    let program = "./src/mod_assigner/hall_request_assigner";

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

    let new_states: AssignerOutput = match serde_json::from_str(&stdout) {
        Ok(new_states) => new_states,
        Err(e) => {
            panic!("Failed to deserilize new state to JSON format: {}", e);
        }
    }; 

    println!("{:#?}", new_states);

    Ok(new_states)
}
