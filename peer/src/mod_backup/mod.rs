

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
use crossbeam_channel::Receiver;
use crate::mod_hardware;


//config.rs has configuration og structs and debug
use crate::config::*;
static SELF_ID: &str = CONFIG.peer.id;
static SLEEP_MILLI: u64 = CONFIG.backup.sleep_dur_milli as u64;
static BACKUP_ADDR: &str = CONFIG.backup.sec_recv;
static PRIMARY_ADDR: &str = CONFIG.backup.pri_send;
static MAX_ATTEMPTS: i64 = CONFIG.backup.attempts;



//Implement bincode (use bincode;) to reduce sending size if time and needed.

pub fn backup_state() -> (EntireSystem, Option<ElevatorSystem>){
    println!("Starting this process as secondary");

    //Create the variable we will return
    let mut world_view: EntireSystem = EntireSystem::template();
    //Parameters for the loop and break condition
    let sleep_dur = Duration::from_millis(SLEEP_MILLI - 50);
    let mut buffer: [u8; 1024] = [0; 1024]; //Adjust if packages are bigger
    let mut attempts = 0;

    //Socket to listen on:
    let backup_socket: UdpSocket = UdpSocket::bind(BACKUP_ADDR.to_string()).expect("Could'nt setup receiver");


    //Enter loop that listens to the state of primary, if primary dies, we take over based on the last seen state.
    loop {
        //Do not just 
        sleep(sleep_dur);
        println!("Secondary Loop: attempt {}", attempts);
        //Makes sure we are not stuck in this loop and not incrementing attempts.
        backup_socket.set_nonblocking(true).expect("Backup: Failed to set non_blocking");
        match backup_socket.recv_from(&mut buffer) {
            Ok((amt, _)) => {

                let received = String::from_utf8_lossy(&buffer[..amt]);

                if let Ok(parsed) = serde_json::from_str::<EntireSystem>(&received.trim()) {
                    world_view = parsed;
                    println!("Received valid state from primary");
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
        if attempts >= MAX_ATTEMPTS {
            drop(backup_socket); //Allows the next secondary to be spawned securely.
            let _ = mod_hardware::init();
            let mut elevator_sys = ElevatorSystem::new();
            
            return (world_view, Some(elevator_sys));
        }    

    }
}


pub fn send_latest_primary(latest_updated_state: Receiver<EntireSystem>) {
    let dur = Duration::from_millis(SLEEP_MILLI);

    let pri_send = UdpSocket::bind(CONFIG.backup.pri_send.to_string()).expect("Could'nt setup receiver");

    let mut ww = EntireSystem::template();
    let mut serialized = serde_json::to_string(&ww).unwrap();
    loop {
        sleep(dur);
        ww = match latest_updated_state.try_recv() {
            Ok(sys) => sys,
            _ => ww,
        };

        serialized = serde_json::to_string(&ww).unwrap();
        pri_send.send_to(serialized.as_bytes(), BACKUP_ADDR);
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
        .arg(primary)
        .stdout(Stdio::null())  // Avoid blocking by suppressing stdout
        .stderr(Stdio::null())  // Suppress stderr
        .spawn()
        .expect("Failed to start secondary process in new xterm terminal. Start it yourself with cargo run -- false");
}
