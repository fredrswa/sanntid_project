#[allow(dead_code)]
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::mod_fsm::config::Dirn;
use crate::mod_fsm::config::Behavior;

use crate::mod_fsm::config::{NUM_BUTTONS, NUM_FLOORS, NUM_ELEVATORS};



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

pub fn json_init() {
    let filename = "assigner.json";

    // if the file already exists, dont overwrite it
    if std::path::Path::new(filename).exists() {
        return;
    }
    
    let mut states = HashMap::new();

    states.insert("one".to_string(), States {
        behavior: Behavior::Moving,
        floor: 2,
        direction: Dirn::Up,
        cabRequests: [false; NUM_FLOORS as usize],
    });

    states.insert("two".to_string(), States {
        behavior: Behavior::Idle,
        floor: 0,
        direction: Dirn::Stop,
        cabRequests: [false; NUM_FLOORS as usize],
    });

    states.insert("three".to_string(), States {
        behavior: Behavior::Idle,
        floor: 0,
        direction: Dirn::Stop,
        cabRequests: [false; NUM_FLOORS as usize],
    });

    let ES = EntireSystem {
        hallRequests: [[false; 2]; NUM_FLOORS],
        states,
    };

    let serialized = serde_json::to_string_pretty(&ES).unwrap(); //pretty for human readability

    if let Err(e) = fs::write("assigner.json", serialized) {
        eprintln!("Failed to write to file: {}", e);
    }
}

pub fn read_json() -> Option<EntireSystem> {
    let filename = "assigner.json";
    let data = fs::read_to_string(filename).ok()?; // read file to string
    let system: EntireSystem = serde_json::from_str(&data).ok()?; // Parse JSON to Rust-struct
    Some(system)
}

pub fn test_script_json() {
    json_init();
    test_read_json(); 
}

pub fn test_read_json() {
    match read_json() {
        Some(system) => {
            println!("Successfully read JSON file:");
            println!("{:?}", system);
        }
        None => {
            eprintln!("Could not read or parse JSON-filen.");
        }
    }
}