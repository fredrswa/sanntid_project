use std::io::Result;
use peer::mod_fsm::fsm;
use peer::mod_network::network;


use serde::{Serialize, Deserialize};
use serde_json;
use peer::mod_fsm::config::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u32,
}
fn test_script_json() {
     // Create an instance of the struct
     let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Serialize the struct into a JSON string
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back into the struct
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}


fn main() -> Result<()> {
    //fsm::test_script_elevator_system();
    //network::test_script_network_module();
    test_script_json();
    Ok(())

}
