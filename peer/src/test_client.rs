
use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct ElevatorInfo {
    id: String,
    behavior: String,
    floor: u8,
    direction: String,
    cabRequests: [bool; 4]
}

#[derive(Serialize, Deserialize, Debug)]
struct ElevatorState {
    hallRequests: [[bool; 2]; 4],
    states: HashMap<String, ElevatorInfo>,
}
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    loop {
        let mut states = HashMap::new();
        states.insert("test_elevator".to_string(), ElevatorInfo {
            id: "test_elevator".to_string(),
            behavior: "Moving".to_string(),
            floor: 3,
            direction: "up".to_string(),
            cabRequests: [false, true, false, false],
        });

        let test_state = ElevatorState {
            hallRequests: [[false, false]; 4],
            states,
        };

        let json_msg = serde_json::to_string(&test_state).unwrap();

        println!("Sending to 127.0.0.1:20003: {}", json_msg);
        match socket.send_to(json_msg.as_bytes(), "127.0.0.1:20003") {
            Ok(_) => println!("Sent test elevator state..."),
            Err(e) => println!("Failed to send: {}", e),
        }

        thread::sleep(Duration::from_secs(1));
    }
}
