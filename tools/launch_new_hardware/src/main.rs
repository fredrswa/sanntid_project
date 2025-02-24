use std::error::Error;
use std::process::{exit, Child, Command, Stdio};
use std::env;
use tokio::net::{TcpListener, TcpStream};
use std::net::TcpStream as tcp_stream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, split};
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;

fn initialize_hardware (port_number: usize) -> Result<Child,Box<dyn Error>> {
    let os = env::consts::OS;

    match os {
        "windows" => {
            println!("No terminal spawner is implemented for Windows yet.");
            exit(0);
        }
        "linux" => {
            let child = Command::new("xterm")
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
            exit(0);
        }
        _ => {
            println!("Unrecognized OS.");
            println!("SHUTTING DOWN.");
            exit(0);
        }
    }    
}

struct TcpConnectionHandler {
    hardware_stream: Arc<Mutex<TcpStream>>,
    peer_listener: TcpListener,
    num_elevators: usize,
}

impl TcpConnectionHandler {
    async fn init (port_number: usize, num_elevators: usize, mut trial_count: usize) -> Result<TcpConnectionHandler, Box<dyn Error>>  {
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
        sleep(Duration::from_secs(3)).await;
        
        //Upon init the handler connects to the hardware
        //One handler connects to one hardware instance
        
        match TcpStream::connect(format!("localhost:{}", port_number.to_string())).await {
            Ok(stream) => {
                //The stream that connects to the hardware.
                //Wrapped in a ARC and Mutex s.t. only one peer can access the stream at a time. (Blocking)
                let hardware_stream = Arc::new(Mutex::new(stream));

                 //Creates a peer listener
                let peer_listener = TcpListener::bind(format!("localhost:{}", (port_number+1).to_string())).await;
                

                match peer_listener {
                    Ok(pl) => {
                        println!("Successfully opened peer listener at localhost:{}", port_number+1);
                        Ok(Self {
                            hardware_stream: hardware_stream,
                            peer_listener: pl,
                            num_elevators: num_elevators,
                        })
                    }
                    Err(e) => {
                        
                        println!("{}", "#".repeat(80));
                        eprintln!("TCP Peer listener failed to initate: {} \nTrying to reinitiate...", e);
                        println!("{}\n\n", "#".repeat(80));
                
                        child?.kill().expect("Unable to kill process.");
                
                        return Box::pin(Self::init(port_number, num_elevators, trial_count)).await
                    }
                }
            } 
            Err(e) => {
                println!("{}", "#".repeat(80));
                eprintln!("Connection to the server failed: {} \nTrying to reinitiate...", e);
                println!("{}\n\n", "#".repeat(80));
                
                child?.kill().expect("Unable to kill process.");
                
                return Box::pin(Self::init(port_number, num_elevators, trial_count)).await
            }
        }
    }    
}

fn clone_hardware_stream (hardware_stream: TcpStream) -> TcpStream {
    let std_stream = hardware_stream.into_std().expect("Something");
    let ss_clone = std_stream.try_clone().expect("Something"); 

    TcpStream::from_std(ss_clone).expect("Somehting")
}

async fn handle_peer(mut peer_stream: TcpStream, hardware_stream: TcpStream) {

    let hardware_stream_clone = clone_hardware_stream(hardware_stream);

    let (mut peer_reader, mut peer_writer) = split(peer_stream);
    let (mut hardware_reader, mut hardware_writer) = split(hardware_stream_clone);

    let peer_to_hardware = tokio::spawn(async move {
        let mut buffer = [0; 1024];

       

        loop {
            let bytes_read = match peer_reader.read(&mut buffer).await {
                Ok(0) => {
                    println!("Peer disconnected.");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading from peer: {}", e);
                    break;
                }
            };

            println!("Received from peer: {:?}", &buffer[..bytes_read]);

            if let Err(e) = hardware_writer.write_all(&buffer[..bytes_read]).await {
                eprintln!("Failed to send data to hardware: {}", e);
                break;
            }
        }
    });



    let hardware_to_peer = tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        loop {
            let bytes_read = match hardware_reader.read(&mut buffer).await {
                Ok(0) => {
                    println!("Hardware disconnected.");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading from hardware: {}", e);
                    break;
                }
            };

            if let Err(e) = peer_writer.write_all(&buffer[..bytes_read]).await {
                eprintln!("Failed to send data to peer: {}", e);
                break;
            }
        }
    });
}

async fn tcp_stream_handler (tcp_ch: &TcpConnectionHandler) {
    //Creates a thread for each peer which connects to the peer listener
    //This process should itself be handles by a thread. 
    loop {
        match tcp_ch.peer_listener.accept().await {
            Ok((peer_stream, addr)) => {
                println!("New peer connected: {}", addr);
                let hardware_stream_clone = Arc::clone(&tcp_ch.hardware_stream);
                tokio::spawn(handle_peer(peer_stream, hardware_stream_clone));
            }
            Err(e) => eprintln!("Failed to accept peer connection: {}", e),
        }
    }
}


#[tokio::main]
async fn main() {
    // Also possible to use 0, s.t. the OS assigns a port that is available
    let tcp_handler = TcpConnectionHandler::init(15666, 3, 0).await;
    match tcp_handler {
        Ok(tcp_ch) => {
            tcp_stream_handler(&tcp_ch).await;
        }

        Err(e) => {
            eprintln!("Error with the server launcher: {}", e);
        }
    }
}
    

