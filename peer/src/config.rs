#![allow(dead_code)]
use std::net::IpAddr;
use std::u8;
use std::fmt;
use serde::{Serialize, Deserialize};
use std::{fs, os::unix::raw::ino_t};
use serde_json;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;

use driver_rust::elevio::elev::Elevator;

////////STRUCTURE//////////
/// /////////////////// ///
/// ------Structs------ ///
/// /////////////////// ///
/// /////////////////// ///
/// /////////////////// ///
/// ---
/// 
#[derive(Clone)]
pub struct ElevatorSystem {
    pub elevator: Elevator,
    pub requests: Vec<Vec<bool>>,
    pub status: Status,

    pub num_floors: usize,
    pub num_buttons: usize,
    pub door_open_s: usize,
    pub addr: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct States {
    pub behavior: Behavior,
    pub floor: isize,
    pub direction: Dirn,
    pub cab_requests: Vec<bool>,
}

 #[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntireSystem {
    pub hallRequests: Vec<[bool; 2]>,
    pub states: HashMap<String, States>,
} 
pub static LAST_SEEN_STATES: Lazy<EntireSystem> = Lazy::new(|| {
    let config_str = fs::read_to_string("./entire_system.json").expect("Unable to read config file");
    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
});

//Dynamically sized struct, makes it possible with an arbitrary number of elevators
#[derive(serde::Deserialize, Debug)]
pub struct AssignerOutput {
    pub elevators: Vec<Option<Vec<Vec<bool>>>>,
}
impl AssignerOutput {
    pub fn new(num_floors: usize, num_elevators: usize) -> Self {
        let states = vec![vec![false; 3]; num_floors];
        let mut elevators = Vec::with_capacity(num_elevators);

        for _ in 0..num_elevators {
            elevators.push(Some(states.clone()));
        }

        AssignerOutput { elevators }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub num_floors: usize,
    pub num_buttons: usize,
    pub num_elevators: usize,
    pub door_open_s: usize,
    pub id: String,
    pub elev_addr: String,
    pub udp_socket_addr: String,
    pub udp_others_addr: Vec<String>,
    pub udp_recv_port: String,
}

/* impl Config {
    pub fn import() -> Config {
        let config_string = fs::read_to_string("config.json").expect("Unable to read file");
        let config: Config = serde_json::from_str(&config_string).expect("JSON was not well-formatted");
        config
    }
} */

//Bedre ?? Gjør at config bare må leses en gang. 
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the elevator id number as a command-line argument!");
    }
    let elev_num: usize = args[1].parse().expect("Invalid elevator number!");

    let config_str = fs::read_to_string(format!("../tools/generate_json/config_id:{}.json", elev_num)).expect("Unable to read config file");
    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
});

#[derive(Clone)]
pub struct Status {
    pub curr_floor: usize,
    pub curr_dirn: Dirn,
    pub behavior: Behavior,
    pub door_blocked: bool,    
    pub clear_requests: ClearRequestVariant,
}
impl Status {
    pub fn new() -> Status {
        Status {
            curr_floor: 0,
            curr_dirn: Dirn::Stop,
            behavior: Behavior::Idle,
            door_blocked: false,
            clear_requests: ClearRequestVariant::ClearAll,
        }
    }
}

//Kan bygges ut dersom det trengs flere states
#[derive(Serialize, Deserialize, Clone)]
pub struct PeerState {
    pub id: String,
    pub ip: String, 
    pub peers: Vec<String>, //Peer heartbeat ip adresses 
    pub connected: HashMap<String, bool>, //[id -> connected true or false] If udp dont receive heartbeat -> not connected
}

pub static PeerStateCONFIG: Lazy<PeerState> = Lazy::new(|| {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the elevator id number as a command-line argument!");
    }
    let elev_num: usize = args[1].parse().expect("Invalid elevator number!");

    let config_str = fs::read_to_string(format!("../tools/generate_json/peer_state_id:{}.json", elev_num)).expect("Unable to read config file");
    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
});

pub enum Timeout_type {
    fsm_obstruction = 0,
    fsm_doortimeout = 1,
    fsm_powerloss   = 2,

    network_disconnect = 3,
}

///////////////FSM////////////////////

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Behavior {
    Idle,
    Moving,
    DoorOpen,
}

#[derive(Copy, Clone)]
pub enum ButtonType {
    HallUp = 0,
    HallDown = 1,
    Cab = 2,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dirn{
    Up = 1,
    Stop = 0,
    Down = -1,
}

#[derive(Copy, Clone)]
pub enum ClearRequestVariant {
    ClearAll,
    ClearInDirection,
}

#[derive(Copy, Clone)]
pub struct DirnBehaviorPair {
    pub direction: Dirn,
    pub behavior: Behavior,
}

pub fn call_to_button_type(call: u8) -> ButtonType {
    match call {
        0 => ButtonType::HallUp,
        1 => ButtonType::HallDown,
        2 => ButtonType::Cab,
        _ => panic!("Invalid button type"),
    }
}


///////////////DEBUGS////////////////////


impl fmt::Debug for Behavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Behavior::Idle => write!(f, "Idle"),
            Behavior::DoorOpen => write!(f, "Door Open"),
            Behavior::Moving => write!(f, "Moving"),
        }
    }
}

impl fmt::Debug for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ButtonType::HallUp => write!(f, "Hall -> Up"),
            ButtonType::HallDown => write!(f, "Hall -> Down"),
            ButtonType::Cab => write!(f, "Cab"),
        }
    }
}
impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status {
                curr_floor,
                curr_dirn,
                behavior,
                door_blocked,
                clear_requests,
            } => f.debug_struct("Status")
                .field("curr_floor", curr_floor)
                .field("curr_dirn", curr_dirn)
                .field("behavior", behavior)
                .field("door_blocked", door_blocked)
                .field("clear_requests", clear_requests)
                .finish(),
        }
    }
}
impl fmt::Debug for Dirn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dirn::Up => write!(f, "up"),
            Dirn::Stop => write!(f, "stop"),
            Dirn::Down => write!(f, "down"),
        }
    }
}

impl fmt::Debug for ClearRequestVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClearRequestVariant::ClearAll => write!(f, "Clear All"),
            ClearRequestVariant::ClearInDirection => write!(f, "Clear In Direction"),
        }
    }
}

impl fmt::Display for ElevatorSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       
        write!(f, "     ")?;
        println!("\tHU, \tHD, \tCab");
        
        write!(f, "\n")?;
        for (floor, row) in self.requests.iter().enumerate().rev() {
            write!(f, "F{}\t: ", floor + 1)?;
            for &request in row.iter() {
            write!(f, "{}\t ", if request { "X" } else { " " })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}