
use serde::{Serialize, Deserialize};
use serde_json;

use crate::mod_fsm::config::Dirn;
use crate::mod_fsm::config::Behavior;

use crate::mod_fsm::config::{NUM_BUTTONS, NUM_FLOORS};



#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub floor: isize,
    pub dirn: Dirn,
    pub requests: [[bool; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
    pub behavior: Behavior,
}
pub fn test_script_json() {
     // Create an instance of the struct
    let state = State{
        floor: 0,
        dirn: Dirn::Stop,
        requests: [[false; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
        behavior: Behavior::Idle,
    };
    // Serialize the struct into a JSON string
    let serialized = serde_json::to_string(&state).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back into the struct
    let deserialized: State = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}