//! Backup module for single peer
// Runs as a secondary process, listening to the primary process.
// If the primary process dies, the secondary process takes over.


/// Standard Library
use std::{
        thread::sleep, 
        time::Duration, 
        process::{Stdio, Command},
        net::UdpSocket,
        io};


/// External Crates
use serde_json;
use crossbeam_channel::Receiver;


/// Internal Modules
use crate::mod_hardware;
use crate::mod_fsm;
use crate::config::*;

/// CO
static SLEEP_MILLI: u64 = CONFIG.backup.sleep_dur_milli as u64;
static BACKUP_ADDR: &str = CONFIG.backup.sec_recv;
static PRIMARY_ADDR: &str = CONFIG.backup.pri_send;
static MAX_ATTEMPTS: i64 = CONFIG.backup.attempts;


pub fn backup_state () -> (EntireSystem, Option<ElevatorSystem>) {
    
    // Parameters
    let sleep_dur = Duration::from_millis(SLEEP_MILLI - 30);
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut attempts = 0;
    let mut world_view: EntireSystem = EntireSystem::template();

    // Socket
    let backup_socket: UdpSocket = UdpSocket::bind(BACKUP_ADDR.to_string()).expect("Could'nt setup receiver");

    loop {
        sleep(sleep_dur);
        backup_socket.set_nonblocking(true).expect("Backup: Failed to set non_blocking");
        match backup_socket.recv_from(&mut buffer) {
            Ok((amt, _)) => {

                let received = String::from_utf8_lossy(&buffer[..amt]);
                if let Ok(parsed) = serde_json::from_str::<EntireSystem>(&received.trim()) {
                    world_view = parsed;
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
            elevator_sys.update_cab_requests_from_world_view(&world_view);
            return (world_view, Some(elevator_sys));
        }    

    }
}



static UDP_RECV: &str = CONFIG.network.udp_recv;

pub fn humble_state() -> (EntireSystem, Option<ElevatorSystem>) {
    println!("Starting in humble state: Listening on network for latest state");
    let socket = UdpSocket::bind(UDP_RECV).expect("Could'nt setup receiver");
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut world_view: EntireSystem = EntireSystem::template();
    let _ = mod_hardware::init();
    let mut es = ElevatorSystem::new();
    let start_time = std::time::Instant::now();

    while start_time.elapsed().as_secs() < 3 {
        socket.set_nonblocking(true).expect("Failed to set non_blocking");
        match socket.recv_from(&mut buffer) {
            Ok((amt, _)) => {
                let received = String::from_utf8_lossy(&buffer[..amt]);
                if let Ok(parsed) = serde_json::from_str::<EntireSystem>(&received.trim()) {
                    world_view = parsed;
                    drop(socket);
                    es.update_cab_requests_from_backup();
                    return (world_view, Some(es));
                }
            },
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(_) => {
                continue;
            }
        }
        
    }
    drop(socket);
    es.update_cab_requests_from_backup();
    return (world_view, Some(es));
}


pub fn send_latest_primary(latest_updated_state: Receiver<EntireSystem>) {
    let dur = Duration::from_millis(SLEEP_MILLI);

    let pri_send = UdpSocket::bind(PRIMARY_ADDR.to_string()).expect("Could'nt setup receiver");

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


pub fn spawn_secondary_exe() {
    let path = "./peer";
    let _secondary = Command::new("setsid")
        .arg("xterm")
        .arg("-e")
        .arg(path)
        .arg(SELF_ID.to_string())
        .stdout(Stdio::null())  // Avoid blocking by suppressing stdout
        .stderr(Stdio::null())  // Suppress stderr
        .spawn()
        .expect("Failed to start secondary process in new xterm terminal. Start it yourself with cargo run false");
}