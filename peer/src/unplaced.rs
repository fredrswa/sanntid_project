use std::error::Error;
use std::process::{exit, Child, Command, Stdio};
use std::env;
use std::thread::sleep;
use std::time::Duration;

use driver_rust::elevio::elev::Elevator;



pub  fn init_hardware (port_number: usize, sim: bool) -> Result<Child,Box<dyn Error>> {
    let hardware: &str = if sim {
        "./../tools/elevatorServers/SimElevatorServer"
    } else {
        "./../tools/elevatorServers/elevatorserver"
    };

    
    let child = Command::new("xterm")
        .args(["-fa", "Monospace","-fs", "16", "-e", hardware, "--port", port_number.to_string().as_str()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    
        match child {
            Ok(terminal) => {
                println!("Successfully opened terminal. \nRunning process at localhost:{}", port_number);
                let pid = terminal.id();
                println!("With pid: {:#?}\n", pid);
                sleep(Duration::from_secs(1));
                return Ok(terminal);
            }
    
            Err(e) => {
                eprintln!("Terminal was not opened!: {}", e);
                return Err(Box::new(e))
            }
        }
    }

