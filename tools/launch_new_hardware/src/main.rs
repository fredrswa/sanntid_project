use std::error::Error;
use std::process::{exit, Child, Command, Stdio};
use std::env;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn initialize_hardware (port_number: usize, sim: bool) -> Result<Child,Box<dyn Error>> {
    let os = env::consts::OS;

    let hardware: &str = if sim {
        "./SimElevatorServer"
    } else {
        "./elevatorserver"
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

pub fn init_elevator_tcp_stream (port_number: usize, mut trial_count: usize, sim: bool) -> Result<TcpStream, Box<dyn Error>>  {
    
    trial_count += 1;
    if trial_count >= 4 {
        panic!("C R A S H   A N D   B U R N: it didnt work.");
    }

    let child = initialize_hardware(port_number, sim);

    sleep(Duration::from_secs(3));
    
    match TcpStream::connect(format!("localhost:{}", port_number.to_string())) {
        Ok(hardware_stream) => {
                println!("TCP Connection established between elevator and hardware.");
                return Ok(hardware_stream);
            }
        Err(e) => {
            println!("{}", "#".repeat(80));
            eprintln!("Connection to the hardware failed: {} \nTrying to reinitiate...", e);
            println!("{}\n\n", "#".repeat(80));
                
            child?.kill().expect("Unable to kill hardware process.");
        
            return init_elevator_tcp_stream(port_number, trial_count, sim)
        }
    }    
} 


fn main() {
    let _elev_tcp_stream = init_elevator_tcp_stream(15667, 0, true);

    loop {}
}

    

