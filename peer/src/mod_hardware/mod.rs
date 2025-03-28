//! Hardware Module

/// Standard Library
use std::{error::Error, 
        process::{exit, Child, Command, Stdio},
        thread::sleep,
        time::Duration,
        net::UdpSocket};
/// External Crates
use driver_rust::elevio::elev::Elevator;

/// Internal Crates
use crate::config::*;

/// Configurations
static SIM: bool = CONFIG.hardware.sim;
static PORT: i64 = CONFIG.hardware.addr;
static LOAD_TIME: i64 = CONFIG.hardware.load_time;
static NUM_FLOORS: i64 = CONFIG.elevator.num_floors;

/// Init | Initialize the hardware
pub fn init () {

    // Executables
    let sim_executable: &str =  "./../tools/SimElevatorServer";
    let phy_executable: &str =  "./../tools/elevatorserver";
    let port = PORT.to_string();
    let num_floors = NUM_FLOORS.to_string();


    // Command Line Arguments
    let args  = match SIM {
        true => vec!["xterm","-fs", "10", "-e", sim_executable, 
        "--port", port.as_str(), 
        "--numfloors", num_floors.as_str()],
        false => vec!["xterm","-fs", "10", "-e", phy_executable, 
        "--port", port.as_str()]
    };

    // Only spawns if hardware is not already running
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


/// Check Socket | Check if the socket is already open
fn check_socket() -> bool {
    if Elevator::init(CONFIG.elevator.addr, 4).is_ok() {
        true
    } else {
        false
    }
}


/// Test function
#[test]
fn test_hardware() {
    if SIM {
        println!("Openings SIMulated elevator on PORT {}", PORT);
    } else {
        println!("Opening physical elevator on PORT {}", PORT);
    }
    init();
}