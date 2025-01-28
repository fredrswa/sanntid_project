use std::net::UdpSocket;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant}; //bruker ikke instant enda, vet ikke om det er nødvendig

fn main() {
    let args: Vec<String> = std::env::args().collect(); // Collects the arguments from the command line and stores them in a vector
    // args are the values that are being sent to the program from the command line
    if args.len() > 1 && args[1] == "child" {
        child_process();
    } else {
        parent_process();
    }
}

// Primary process
fn parent_process() {
    println!("Starting as primary");

    // starting new instance of itself, with the argument "child" such that the new instance will start as a backup
    let _ = Command::new(std::env::current_exe().unwrap()) // current_exe is used to get the path of the current executable
        .arg("child") // arg is used to add an argument to the command
        .spawn() // spawn is used to start a new process
        .expect("Failed to spawn backup process"); // expect is used to handle the error

    // Creating a socket to send data to the backup process
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Could not bind socket");
    socket.connect("127.0.0.1:34255").expect("Could not connect to backup");

    let mut count = 1;
    loop {
        // Count and send data to the backup process
        println!("Parent counting: {}", count);
        socket.send(&count.to_string().as_bytes()).expect("Failed to send data");
        count += 1;

        thread::sleep(Duration::from_secs(1));
    }
}

// Backup process
fn child_process() {
    println!("Starting as backup");

    // Socket to receive data from the primary process
    let socket = UdpSocket::bind("127.0.0.1:34255").expect("Could not bind socket");

    // Set a timeout for the socket to ckeck if the parent is still alive ( 3 sec)
    socket.set_read_timeout(Some(Duration::from_secs(3))).expect("Failed to set read timeout");

    let mut buffer = [0; 1024];
    let mut last_count = 0;
    loop {
        // Child process is listening to the parent process and storing the last count
        match socket.recv_from(&mut buffer) {
            Ok((size, _)) => {
                let message = std::str::from_utf8(&buffer[..size]).expect("Failed to convert buffer to string"); // from_utf8 is used to convert the buffer to a string
                
                // Parse the message to an integer
                last_count = message.parse::<i32>().expect("Failed to parse message");
                println!("Child received: {}", last_count);
            }

            //if child process does not receive any data from the parent process within 3 sec, it will take over as primary
            Err(_) => {
                println!("Parent not responding. Taking over as primary");

                //litt usikker på denne
                // Close the socket before starting a new backup process
                drop(socket);

                // Become primary
                start_as_primary(last_count);
                break;
            }
        }
    }
}

fn start_as_primary(last_count: i32) {
    //settign up a new socket to send data to the backup process
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Could not bind socket");
    socket.connect("127.0.0.1:34255").expect("Could not connect to backup");

    let mut count = last_count + 1; // Continue counting from the last count, +1 to not repeat the last count
    loop {
        println!("Parent counting: {}", count);
        socket.send(&count.to_string().as_bytes()).expect("Failed to send data");
        count += 1;

        thread::sleep(Duration::from_secs(1));
    }
}