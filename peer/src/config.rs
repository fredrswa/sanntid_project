#![allow(dead_code)]
use std::u8;
use std::fmt;
use serde::{Serialize, Deserialize};
use std::{fs, os::unix::raw::ino_t};
use serde_json;
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
pub requests: [[bool; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
pub status: Status,
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
    pub udp_recv_port: String,
}
impl Config {
    pub fn import() -> Config {
        let config_string = fs::read_to_string("config.json").expect("Unable to read file");
        let config: Config = serde_json::from_str(&config_string).expect("JSON was not well-formatted");
        config
    }
}
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


pub struct PeerState {
    id: String,
    //......
}

pub enum Timeout_type {
    fsm_obstruction = 0,
    fsm_doortimeout = 1,
    fsm_powerloss   = 2,

    network_disconnect = 3,
    network_
}


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