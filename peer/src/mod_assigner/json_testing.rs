#[allow(dead_code)]
use std::fs;
use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;
use std::process::Command;
use crate::mod_fsm::config::Behavior;

use crate::mod_fsm::config::{NUM_BUTTONS, NUM_FLOORS, NUM_ELEVATORS};

#[derive(Serialize, Deserialize, Clone)]
pub enum Dirn{
    up = 1,
    stop = 0,
    down = -1,
}
impl fmt::Debug for Dirn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dirn::up => write!(f, "up"),
            Dirn::stop => write!(f, "stop"),
            Dirn::down => write!(f, "down"),
        }
    }
}

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
        direction: Dirn::up,
        cabRequests: [false, true, false, true],
    };
    let two =  States {
        behavior: Behavior::Moving,
        floor: 2,
        direction: Dirn::up,
        cabRequests: [false; NUM_FLOORS as usize],
    };
    let three =  States {
        behavior: Behavior::Moving,
        floor: 2,
        direction: Dirn::up,
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
pub fn json_init() {
    let sys = test_struct();
    let filename = "assigner.json";

    let argument = serde_json::to_string(&sys).unwrap();
    let program = "./mod_assigner/hall_request_assigner";
    //let argument: String = fs::read_to_string("mod_assigner/assigner.json").unwrap();
    let test  = argument.clone();


    let output = Command::new(program)
        .arg("-i")
        .arg(&argument)
        .output()
        .expect("Failed");


    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{:?}",stdout);
    println!("{:?}",stderr);

    fs::write("mod_assigner/assigner.json",test);


}

pub fn test_script_json() {
    json_init();
}