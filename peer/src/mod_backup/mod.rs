
use std::net::UdpSocket;
use std::{thread::{sleep}, time::Duration, process::{Stdio, Command}};
use serde::{self, Serialize, Deserialize};
use bincode;
use crate::config::*;

pub fn secondary_state() -> SystemState {
    println!("Starting as secondary");
    let duration = Duration::from_millis(1000);
    let mut buf = [0;10];
    let mut system_state: SystemState = SystemState::new(0, 0, false);
    let mut attempts = 0;

    loop {
        // Let the primaru send first
        sleep(duration);

        // Try receiving, it cannot block, or we will never become primary.
        listening_socket.set_nonblocking(true).expect("Failed to set non-blocking");
        match listening_socket.recv_from(&mut buf) {
            // If we receive data, we try to parse it.
            Ok((amt, _)) => {
                let received = String::from_utf8_lossy(&buf[..amt]);
                // Check if it is valid
                if let Ok(parsed) = serde_json::from_str::<SystemState>(&received.trim()) {
                    system_state = parsed;
                    println!("Secondary has received a valid state from primary");
                    attempts = 0;
                } else {
                    attempts += 1;
                }
            }
            // If we get a would block error, we skip and increment attempts
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No data to read, skip this iteration
            attempts +=1;
            }
            Err(_) => {
            attempts += 1;
            }
        }
        if attempts >= 3 {
            return system_state;
        }
    }
}

pub fn create_socket(addr: String) -> UdpSocket {
    let socket = UdpSocket::bind(addr).expect("Could'nt setup receiver");
    socket
}

pub fn spawn_secondary() {
    let secondary = Command::new("setsid")
        .arg("xterm")
        .arg("-e")
        .arg("cargo")
        .arg("run")
        .arg("1")
        .arg("false")
        .stdout(Stdio::null())  // Avoid blocking by suppressing stdout
        .stderr(Stdio::null())  // Suppress stderr
        .spawn()
        .expect("Failed to start secondary process in new xterm terminal");
    println!("Secondary spawned in a new xterm window with recv address: {}", udp_send);
}

