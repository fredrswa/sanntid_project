use std::default;
#[allow(dead_code)]
use std::fs;
use std::collections::HashMap;
use std::io::repeat;
use serde::{Serialize, Deserialize};
use serde_json;
use std::process::Command;
use std::path::Path;

use peer::mod_fsm::config::{Behavior, Dirn};
use peer::mod_fsm::config::NUM_FLOORS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct States {
    pub behavior: Behavior,
    pub floor: isize,
    pub direction: Dirn,
    pub cabRequests: [bool; NUM_FLOORS as usize],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntireSystem {
    pub hallRequests: [[bool; 2]; NUM_FLOORS],
    pub states: HashMap<String, States>, // changed from array to hashmap
}

pub fn test_struct() -> EntireSystem {

    let one =  States {
        behavior: Behavior::Moving,
        floor: 2,
        direction: Dirn::Up,
        cabRequests: [false, true, false, true],
    };

    let two =  States {
        behavior: Behavior::Moving,
        floor: 2,
        direction: Dirn::Up,
        cabRequests: [false; NUM_FLOORS as usize],
    };

    let three =  States {
        behavior: Behavior::Moving,
        floor: 2,
        direction: Dirn::Up,
        cabRequests: [true, true, true, true],
    };

    let mut all: HashMap <String, States> = HashMap::new();
    all.insert("one".to_string(), one);
    all.insert("two".to_string(), two);
    all.insert("three".to_string(), three);
    let es = EntireSystem {
        hallRequests: [[true; 2]; NUM_FLOORS],
        states: all,
    };
    return es;
}

#[test]
pub fn assigner_test() {
    let sys = test_struct();

    let argument = match serde_json::to_string(&sys) {
        Ok(json) => json,
        Err(e) => {
            panic!("Failed to serialize JSON: {}", e);
        }
    }; 

    println!("{}", argument);
    println!("{}", "#".repeat(100));
    let program = "./src/mod_assigner/hall_request_assigner";
    //let argument: String = fs::read_to_string("mod_assigner/assigner.json").unwrap();
    let test  = argument.clone();

    let output = match Command::new(program)
        .arg("-i")
        .arg(&argument)
        .output()
        {
            Ok(output) => { 
                println!("Assigner succesfully called: {:#?}", output);
                output
            }
            Err(e) => {
                panic!("Failed to run hall request assigner: {}", e);
            }
        };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{}", "#".repeat(100));
    println!("{:#?}",stdout);
    println!("{}", "#".repeat(100));
    println!("{:#?}",stderr);

    let save_path = "./tests/assigner.json";

    let _res = match fs::write(save_path,test) {
        Ok(res) => { 
            println!("JSON succesfully written: {:#?}", res);
            res
        }
        Err(e) => {
            panic!("Failed to write to file: {}", e);
        }
    };

    assert!(Path::new(save_path).exists(), "The file does not exist at the expected path");
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssignerOutput{
    pub one: [[bool; 3]; NUM_FLOORS],
    pub two: [[bool; 3]; NUM_FLOORS],
    pub three: [[bool; 3]; NUM_FLOORS],
}


#[test]
fn assigner_test_2 () {
    let sys = test_struct();    
    
    let elev_states = match serde_json::to_string(&sys) {
        Ok(json) => json,
        Err(e) => {
            panic!("Failed to serialize JSON: {}", e);
        }
    }; 

    let program = "./src/mod_assigner/hall_request_assigner";
    //let elev_states: String = fs::read_to_string("mod_assigner/assigner.json").unwrap();

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
}
