//Standard Library
use std::{error::Error, 
        process::{exit, Child, Command, Stdio},
        thread::sleep,
        time::Duration,
        net::UdpSocket};
use driver_rust::elevio::elev::Elevator;

use crate::config::*;
static SIM: bool = CONFIG.hardware.sim;
static PORT: i64 = CONFIG.hardware.addr;
static LOAD_TIME: i64 = CONFIG.hardware.load_time;
static NUM_FLOORS: i64 = CONFIG.elevator.num_floors;

pub fn init() {
    let sim_executable: &str =  "./../tools/elevatorServers/SimElevatorServer";
    let phy_executable: &str =  "./../tools/elevatorServers/elevatorserver";
    let port = PORT.to_string();
    let num_floors = NUM_FLOORS.to_string();

    let args  = match SIM {
        true => vec!["xterm","-fa", "Monospace","-fs", "16", "-e", sim_executable, 
        "--port", port.as_str(), 
        "--numfloors", num_floors.as_str()],
        false => vec!["xterm","-fa", "Monospace","-fs", "16", "-e", phy_executable, 
        "--port", port.as_str()]
    };



    if !check_socket() {
        let child = Command::new("setsid")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

            match child {
                Ok(terminal) => {
                    println!("Successfully opened terminal. \nRunning process at localhost:{}", PORT);
                    let pid = terminal.id();
                    println!("With pid: {:#?}\n", pid);
                    let wait = Duration::from_millis(LOAD_TIME as u64);
                    sleep(wait);
                }

                Err(e) => {
                    eprintln!("Terminal was not opened!: {}", e);
                }
            }
        }
    }  



fn check_socket() -> bool {
    if Elevator::init(CONFIG.elevator.addr, 4).is_ok() {
        true
    } else {
        false
    }
}

#[test]
fn test_hardware() {
    if SIM {
        println!("Openings SIMulated elevator on PORT {}", PORT);
    } else {
        println!("Opening physical elevator on PORT {}", PORT);
    }
    init();
}