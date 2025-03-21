

#[allow(dead_code)]
use std::{
        thread::sleep, 
        time::Duration, 
        process::{Stdio, Command},
        net::UdpSocket,
        io,
        collections::HashMap};


use serde_json;
use static_toml;



//config.rs has configuration og structs and debug
use crate::config::*;
//CONFIG is created at runtime from Config.toml: handles ports, id, etc.
static_toml::static_toml! {
    static CONFIG = include_toml!("Config.toml");
}




//Implement bincode (use bincode;) to reduce sending size if time and needed.

pub fn backup_state() -> EntireSystem{
    println!("Starting this process as secondary");

    //Create the variable we will return
    let mut system_state: EntireSystem = EntireSystem::template();

    //Parameters for the loop and break condition
    let sleep_dur = Duration::from_millis(CONFIG.backup.sleep_dur_milli as u64);
    let mut buffer: [u8; 1024] = [0; 1024]; //Adjust if packages are bigger
    let mut attempts = 0;
    let max_attempts = CONFIG.backup.attempts;


    //Socket to listen on:
    let listening_socket_addr: String = CONFIG.backup.sec_recv.to_string();
    let listening_socket = create_socket(listening_socket_addr);


    //Enter loop that listens to the state of primary, if primary dies, we take over based on the last seen state.
    loop {
        //Do not just 
        sleep(sleep_dur);
        println!("Secondary Loop: attempt {}", attempts);
        //Makes sure we are not stuck in this loop and not incrementing attempts.
        listening_socket.set_nonblocking(true).expect("Backup: Failed to set non_blocking");
        match listening_socket.recv_from(&mut buffer) {
            Ok((amt, _)) => {

                let received = String::from_utf8_lossy(&buffer[..amt]);

                if let Ok(parsed) = serde_json::from_str::<EntireSystem>(&received.trim()) {
                    system_state = parsed;
                    println!("SystemState = {:?}", system_state);
                    attempts = 0; // We have a good state, reset attempts.
                } else {
                    attempts += 1; //We count invalid messages towards attempt.
                }

            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                attempts +=1; //We iterated without getting anything from primary.
            },
            Err(_) => {
                attempts +=1; //Same case as above
            }
        }
        if attempts >= max_attempts {
            drop(listening_socket); //Allows the next secondary to be spawned securely.
            return system_state;
        }    

    }
}


pub fn spawn_secondary() {
    let id = CONFIG.peer.id.to_string();
    let primary = "false".to_string();


    let _secondary = Command::new("setsid")
        .arg("xterm")
        .arg("-e")
        .arg("cargo")
        .arg("run")
        .arg(id)
        .arg(primary)
        .stdout(Stdio::null())  // Avoid blocking by suppressing stdout
        .stderr(Stdio::null())  // Suppress stderr
        .spawn()
        .expect("Failed to start secondary process in new xterm terminal. Start it yourself with cargo run -- false");
}

pub fn create_socket(addr: String) -> UdpSocket {
    let socket = UdpSocket::bind(addr).expect("Could'nt setup receiver");
    socket
}