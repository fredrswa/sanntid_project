use std::u8;
use std::fmt;
use serde::{Serialize, Deserialize};
use std::sync::LazyLock;
use std::collections::HashMap;

use driver_rust::elevio::elev::Elevator;

pub static SELF_ID: LazyLock<String> = LazyLock::new(|| {
    let args: Vec<String> = std::env::args().collect();
    args.get(1).expect("No argument provided").clone()
});

pub static PRIMARY: LazyLock<bool> = LazyLock::new(|| {
    let args: Vec<String> = std::env::args().collect();
    args.get(2).map_or(false, |x| x == "primary" || x == "humble")
});

pub static HUMBLE: LazyLock<bool> = LazyLock::new(|| {
    let args: Vec<String> = std::env::args().collect();
    args.get(2).map_or(false, |x| x == "humble")
});

static_toml::static_toml! {
    pub static CONFIG = include_toml!("./../tools/config_files/config_peer_local_3.toml"); 
}

#[derive(Clone, Debug)]
pub struct ElevatorSystem {
    pub elevator: Elevator,
    pub requests: Vec<Vec<bool>>,
    pub status: Status,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct EntireSystem {
    pub hallRequests: Vec<[bool; 2]>,
    pub states: HashMap<String, States>,
} 

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimestampsEntireSystem {
    pub es: EntireSystem,
    pub timestamps: Vec<Vec<(i64, i64)>>, 
}

impl EntireSystem {
    pub fn template() -> EntireSystem {
        let es = EntireSystem {
            hallRequests: vec![[false; 2]; CONFIG.elevator.num_floors as usize],
            states: HashMap::new(),
        };
    es
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct States {
    pub behavior: Behavior,
    pub floor: isize,
    pub direction: Dirn,
    pub cabRequests: Vec<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssignerOutput {
    pub elevators: HashMap<String, Vec<Vec<bool>>>,
}

pub enum Timeout_type {
    fsm_obstruction = 0,
    fsm_doortimeout = 1,
    fsm_powerloss   = 2,

    network_disconnect = 3,
}

///////////////FSM////////////////////

#[derive(Copy, Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Behavior {
    Idle,
    Moving,
    DoorOpen,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ButtonType {
    HallUp = 0,
    HallDown = 1,
    Cab = 2,
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut requests_str = String::new();
        for (floor, buttons) in self.requests.iter().enumerate() {
            requests_str.push_str(&format!(
                "  Floor {}: [{}]\n",
                floor,
                buttons
                    .iter()
                    .map(|&b| if b { "X" } else { " " }) // "X" for active requests, " " for none
                    .collect::<Vec<_>>()
                    .join(" | ")
            ));
        }

        write!(
            f,
            "Requests:\n{}\nStatus:\n  Floor: {}\n  Direction: {:?}\n  Behavior: {:?}\n  Door Blocked: {}\n  Clear Requests: {:?}",
            requests_str,
            self.status.curr_floor,
            self.status.curr_dirn,
            self.status.behavior,
            self.status.door_blocked,
            self.status.clear_requests
        )
    }
}

impl fmt::Display for TimestampsEntireSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        // Format the EntireSystem part
        write!(f, "EntireSystem:\n")?;
        write!(f, "Hall Requests:\n")?;
        for hall_request in &self.es.hallRequests {
            write!(f, "{:?}\n", hall_request)?;
        }

        write!(f, "States:\n")?;
        for (key, state) in &self.es.states {
            write!(f, "{}: {:?}\n", key, state.cabRequests)?;
        }

        write!(f, "\nTimestamps:\n")?;
        for row in &self.timestamps {
            write!(f, "[")?;
            for tuple in row {
                write!(f, "({},{}) ", tuple.0, tuple.1)?;
            }
            write!(f, "]\n")?;
        }

        Ok(())
    }
}