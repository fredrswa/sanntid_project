use std::io;
use std::net::UdpSocket;
use std::thread;
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::time::{Instant, Duration};


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

//struct

#[derive(Serialize, Deserialize, Debug)]
struct HallRequests {
    hallRequests: [[bool; 2]; 4], 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ElevatorInfo {
    id: String,            
    behavior: String,      
    floor: u8,             
    direction: String,     
    cabRequests: [bool; 4], 
    #[serde(skip)] // skip this field when serializing
    last_updated: Option<Instant>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
struct ElevatorState {
    hallRequests: [[bool; 2]; 4],
    states: HashMap<String, ElevatorInfo>, 
}

type SharedElevatorState = Arc<Mutex<ElevatorState>>;

// merge elevator state from incoming elevator state into existing elevator state
fn merge_elevator_state(existing: &mut ElevatorInfo, incoming: &ElevatorInfo) {
    if incoming.floor != existing.floor {
        existing.floor = incoming.floor;
    }
    if incoming.direction != existing.direction {
        existing.direction = incoming.direction.clone();
    }
    if incoming.behavior != existing.behavior {
        existing.behavior = incoming.behavior.clone();
    }
    for i in 0..4 {
        existing.cabRequests[i] |= incoming.cabRequests[i];
    }
     // update last_updated
    existing.last_updated = Some(Instant::now());

}

// merge hall requests from incoming elevator state into existing elevator state
fn merge_hall_requests(existing: &mut [[bool; 2]; 4], incoming: &[[bool; 2]; 4]) {
    for floor in 0..4 {
        for btn in 0..2 {
            existing[floor][btn] |= incoming[floor][btn];
        }
    }
}

// remove stale elevators if they haven't been updated in a while
fn remove_stale_elevators(state: &mut ElevatorState) {
    let timeout = Duration::from_secs(5);
    let now = Instant::now();

    state.states.retain(|_, elevator| {
        if let Some(last_updated) = elevator.last_updated {
            now.duration_since(last_updated) < timeout
        } else {
            false
        }
    });
}




fn udp_receive(socket: UdpSocket, state: SharedElevatorState, rx: Receiver<String>) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        // check if we should stop
        if let Ok(message) = rx.try_recv() {
            println!("Received message to stop: {}", message);
            break;
        }
        //receive message and convert from buffer to string
        let (n_bytes, _src) = socket.recv_from(&mut buffer)?;
        let received_msg = String::from_utf8_lossy(&buffer[..n_bytes]);

        // try to parse the received message to elevator state
        if let Ok(parsed_msg) = serde_json::from_str::<ElevatorState>(&received_msg) {
            println!("Received: {:?}", parsed_msg);

            let mut state_lock = state.lock().unwrap();

            // update hall requests
            merge_hall_requests(&mut state_lock.hallRequests, &parsed_msg.hallRequests);

            // updaye elevator states
            for (id, incoming_info) in parsed_msg.states.iter() {
                if let Some(existing_info) = state_lock.states.get_mut(id) {
                    merge_elevator_state(existing_info, incoming_info);
                } else {
                    state_lock.states.insert(id.clone(), incoming_info.clone());
                }
            }
            // remove inactive elevators
            remove_stale_elevators(&mut state_lock);

            println!("Updated state: {:?}", state_lock);
        } else {
            println!("Invalid message format: {}", received_msg);
        }
    }
    Ok(())
}


#[derive(Serialize, Deserialize, Debug)]
struct ElevatorMessage {
    elevator_state: ElevatorState,
}



fn udp_send(socket: &UdpSocket, addr: &str, message: &ElevatorState, server_sender_tx: Sender::<String>, server_sender_rx: Receiver::<String>) -> io::Result<()> {
    let json_msg = serde_json::to_string(&ElevatorMessage { elevator_state: message.clone()}).unwrap();
    println!("Sending message to {}: {}", addr, json_msg);
    socket.send_to(json_msg.as_bytes(), addr)?;
    Ok(())
}


pub fn test_script_network_module() {
    println!("Binding socket...");
    let socket = match UdpSocket::bind("0.0.0.0:20003") {
        Ok(socket) => {
            println!("Socket bound successfully.");
            socket
        },
        Err(e) => {
            println!("Failed to bind socket: \"{}\"", e);
            return;
        }
    };
    
   
    
    println!("Starting UDP receiver...");
    let (tx, rx) = unbounded::<String>();
    let state: SharedElevatorState = Arc::new(Mutex::new(ElevatorState {
        hallRequests: [[false; 2]; 4], 
        states: HashMap::new(),
    }));

    {
        let socket = socket.try_clone().unwrap();
        let state_clone = Arc::clone(&state);
        thread::spawn(move || {
            udp_receive(socket, state_clone, rx).unwrap();
        });
    }
    println!("Entering main loop...");
    loop {
        thread::sleep(std::time::Duration::from_millis(200));
        
        let state_lock = state.lock().unwrap();
        //udp_send(&socket, "127.0.0.1:20001", &state_lock).unwrap();
        //udp_send(&socket, "127.0.0.1:20002", &state_lock).unwrap();

    }
}
