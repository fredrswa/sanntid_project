



//Standard Library
use std::{error::Error, 
        process::{exit, Child, Command, Stdio},
        thread::sleep,
        time::Duration,
        net::UdpSocket};


use crate::config::*;

pub fn init() {
    let sim: bool = CONFIG.hardware.sim;
    let port = CONFIG.hardware.addr;

    let executable: &str = if sim {
        "./../tools/elevatorServers/SimElevatorServer"
    } else {
        "./../tools/elevatorServers/elevatorserver"
    };


    if !check_socket() {
        let child = Command::new("setsid")
            .args(["xterm","-fa", "Monospace","-fs", "16", "-e", executable, "--port", port.to_string().as_str()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

            match child {
                Ok(terminal) => {
                    println!("Successfully opened terminal. \nRunning process at localhost:{}", port);
                    let pid = terminal.id();
                    println!("With pid: {:#?}\n", pid);
                    let wait = Duration::from_millis(3000);
                    sleep(wait);
                }

                Err(e) => {
                    eprintln!("Terminal was not opened!: {}", e);
                }
            }
        }
    }  



fn check_socket() -> bool {
    if UdpSocket::bind(CONFIG.hardware.addr.to_string()).is_ok() {
        true
    } else {
        false
    }
}

#[test]
fn test_hardware() {
    let sim: bool = CONFIG.hardware.sim;
    let port = CONFIG.hardware.addr;

    if sim {
        println!("Opening simulated elevator on port {}", port);
    } else {
        println!("Opening physical elevator on port {}", port);
    }
    init();
}