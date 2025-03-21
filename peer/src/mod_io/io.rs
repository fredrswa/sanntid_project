use std::process::Command;
use std::fs;
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