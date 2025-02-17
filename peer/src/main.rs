use std::io::Result;
use peer::mod_fsm::fsm;
use peer::mod_network::network;
use peer::mod_assigner::json_test;

use serde::{Serialize, Deserialize};
use serde_json;
use peer::mod_fsm::config::Status;


fn test_script_json() {
    // Create an instance of the struct
    let stat = Status::new();

    // Serialize the struct into a JSON string
    let serialized = serde_json::to_string(&stat).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back into the struct
    let deserialized: Status = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}


fn main() -> Result<()> {
    //fsm::test_script_elevator_system();
    //network::test_script_network_module();
    json_test::test_script_json();
    Ok(())

}
