use std::error::Error;
use std::process::{exit, Child, Command, Stdio};
use std::env;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

fn initialize_hardware (port_number: usize) -> Result<Child,Box<dyn Error>> {
    let os = env::consts::OS;

    match os {
        "windows" => {
            println!("No terminal spawner is implemented for Windows yet.");
            exit(0);
        }
        "linux" => {
            let child = Command::new("open")
                .args(["-fa", "Monospace","-fs", "16", "-e", "./SimElevatorServer", "--port", port_number.to_string().as_str()])
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
            println!("No terminal spawner is implemented for macOS yet.");

            let script = format!(
                "tell application \"Terminal\" to do script \"cd {} && ./SimElevatorServer --port {}\"",
                std::env::current_dir()?.display(),
                port_number
            );

            let child = Command::new("osascript")
                .args(["-e", &script])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match child {
                Ok(terminal) => {
                    println!("Successfully opened Terminal. \nRunning process at localhost:{}", port_number);
                    let pid = terminal.id();
                    println!("With pid: {:#?}\n", pid);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Terminal was not opened!: {}", e);
                    Err(Box::new(e))
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

struct TcpConnectionHandler {
    hardware_stream: TcpStream,
}

impl TcpConnectionHandler {
    fn init (port_number: usize, mut trial_count: usize) -> Result<TcpConnectionHandler, Box<dyn Error>>  {
        //Handler will try to open terminal -> start process in terminal -> connect to process over tcp
        //If the handler is unable to connect to the process, it will kill it and try to restart it
        //After x tries of restarting, the handler will panic (port number could be increased at the risk of introducing floating terminals!)
        //If the handler dies, something should ensure that the controller instance dies with it (if no heartbeat is detected, terminate)
        //If the process dies, the handler will simply kill the terminal and reinitialize
        trial_count += 1;
        if trial_count >= 4 {
            panic!("C R A S H   A N D   B U R N: it didnt work.");
        }

        println!("reached");
        let child = initialize_hardware(port_number);

        //Sleeps for 3 seconds s.t. the hardware can be properly initalized before the handler tries to connect
        //Not a dynamic solution, but feedback from terminal seems hard
        sleep(Duration::from_secs(3));
        
        //Upon init the handler connects to the hardware
        //One handler connects to one hardware instance
        
        match TcpStream::connect(format!("localhost:{}", port_number.to_string())) {
            Ok(hardware_stream) => {
                //The stream that connects to the hardware.
                //Wrapped in a ARC and Mutex s.t. only one peer can access the stream at a time. (Blocking)
                //let hardware_stream = Arc::new(Mutex::new(stream));

                    Ok(Self {
                        hardware_stream: hardware_stream,
                    })
                }
            Err(e) => {
                println!("{}", "#".repeat(80));
                eprintln!("Connection to the server failed: {} \nTrying to reinitiate...", e);
                println!("{}\n\n", "#".repeat(80));
                    
                child?.kill().expect("Unable to kill process.");
            
                return Self::init(port_number, trial_count)
            }
        }    
    } 
}

fn main() {
    // Also possible to use 0, s.t. the OS assigns a port that is available
    let child = initialize_hardware(15666);

    
    //let tcp_handler = TcpConnectionHandler::init(15666, 0);
    loop {}
}

    

