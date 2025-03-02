
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
    hall_requests: [[bool; 2]; 4],
    states: HashMap<String, ElevatorInfo>,
}
#[test]
fn client_test() {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket, 
        Err(e) => panic!("Failed to bind socket: {}", e)
    };

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
            hall_requests: [[false, false]; 4],
            states,
        };

        let json_msg = match serde_json::to_string(&test_state){
            Ok(json_msg) => json_msg,
            Err(e) => {
                panic!("Failed to parse JSON test state: {}", e)
            }
        };

        match socket.send_to(json_msg.as_bytes(), "localhost:20003") {
            Ok(_) => println!("Sent to localhost:20003: {}", json_msg),
            Err(e) => panic!("Failed to send: {}", e),
        }

        thread::sleep(Duration::from_secs(1));
    }
}
