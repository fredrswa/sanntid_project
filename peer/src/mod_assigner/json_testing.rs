#[allow(dead_code)]
use std::fs;
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
    hallRequests: [[bool; 2]; NUM_FLOORS],
    states: [States; NUM_ELEVATORS],
}

pub fn json_init() {
    let one =  States {
        behavior: Behavior::Moving,
        floor: 2,
        direction: Dirn::Up,
        cabRequests: [false; NUM_FLOORS as usize],
    };
    let two   = one.clone();
    let three = one.clone();

    let ES = EntireSystem {
        hallRequests: [[false; 2]; NUM_FLOORS],
        states: [one, two, three],
    };


    let serialized = serde_json::to_string(&ES).unwrap();

    if let Err(e) = fs::write("assigner.json", serialized) {
        eprintln!("Failed to write to file: {}", e);
    }



}
pub fn test_script_json() {
    json_init();

    
}