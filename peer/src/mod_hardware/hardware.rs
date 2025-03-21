use std::error::Error;
use std::process::{exit, Child, Command, Stdio};
use std::env;
use std::thread::sleep;
use std::time::Duration;

use driver_rust::elevio::elev::Elevator;
use crate::config::CONFIG;

fn init_hardware (port_number: usize, sim: bool) -> Result<Child,Box<dyn Error>> {
    let os = env::consts::OS;

    let hardware: &str = if sim {
        "./../tools/elevatorServers/SimElevatorServer"
    } else {
        "./../tools/elevatorServers/elevatorserver"
    };

    match os {
        "windows" => {
            println!("No terminal spawner is implemented for Windows yet.");
            exit(0);
        }
        "linux" => {
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

                        return Ok(terminal);
                    }
         
                    Err(e) => {
                        eprintln!("Terminal was not opened!: {}", e);
                        return Err(Box::new(e))
                    }
                }
            }
        
        "macos" => {
            let child = Command::new("osascript")
                .args([
                    "-e", 
                    &format!("tell application \"Terminal\" to do script \"./SimElevatorServer --port {}\"", port_number)
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match child {
                Ok(terminal) => {
                    println!("Successfully opened terminal. \nRunning process at localhost:{}", port_number);
                    let pid = terminal.id();
                    println!("With pid: {:#?}\n", pid);
                    return Ok(terminal);
                }
                Err(e) => {
                    eprintln!("Terminal was not opened!: {}", e);
                    return Err(Box::new(e))
                }
            }
        }
        _ => {
            println!("Unrecognized OS.");
            println!("SHUTTING DOWN.");
            exit(0);
        }
    }    
}

pub fn init_elevator (addr: String, mut trial_count: usize, sim: bool) -> Result<Elevator, Box<dyn Error>>  {

    trial_count += 1;
    if trial_count >= 4 {
        panic!("C R A S H   A N D   B U R N: it didnt work.");
    }

    let split_addr: Vec<&str> = addr.split(':').collect();
    let port_number = match split_addr[1].parse::<usize>() {
        Ok(port_number) => port_number,
        Err(e) => {
            panic!("Wasnt able to split and parse IP adress, in order to retreive port number: {}", e);
        }
    };

    let mut child = match init_hardware(port_number, sim) {
        Ok(terminal) => terminal,
        Err(e) => {
            panic!("Failed to spawn hardware instance: {}", e)
        }
    };

    sleep(Duration::from_secs(3));
    
    //let addr = format!("localhost:{}", port_number.to_string());
    let e = match Elevator::init(&addr, CONFIG.num_floors as u8) {
        Ok(elevator) => elevator,
        Err(e) => {
            println!("{}", "#".repeat(80));
            eprintln!("Failed to connect to elevator at {}: {}", addr, e);
            println!("Trying again");
            println!("{}\n\n", "#".repeat(80));
            
            match child.kill() {
                Ok(its_ok) => its_ok,
                Err(e) => {
                    panic!("THE CHILD IS NOT DEAD: {}", e)
                }
            };
        
            return init_elevator(addr, trial_count, sim)
        }      
    };
    return Ok(e)    
} 