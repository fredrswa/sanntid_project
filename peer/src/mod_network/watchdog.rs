use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::thread;
use serde_json::Value;
use std::collections::HashMap;
use crossbeam_channel as cbc;

use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::{Duration, Instant};
use serde_json::json;

const TIMEOUT_MS: u64 = 5000; // how long before we consider an elevator dead
const MAX_RETRIES: u32 = 3;   // how many missed ACKs before we consider an elevator dead

fn watchdog(socket: &UdpSocket) {
    let mut heartbeats: HashMap<String, Instant> = HashMap::new();
    let mut missed_acks: HashMap<String, u32> = HashMap::new(); // missing ACKs for each elevator

    loop {
        let mut buf = [0; 1024];

        // try to receive a message
        if let Ok((size, src)) = socket.recv_from(&mut buf) {
            let received_msg = std::str::from_utf8(&buf[..size]).unwrap();

            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(received_msg) {
                if parsed["type"] == "heartbeat" {
                    let id = parsed["id"].as_str().unwrap().to_string();
                    heartbeats.insert(id.clone(), Instant::now());
                    missed_acks.insert(id.clone(), 0); // zero out missed ACKs
                    println!("Received heartbeat from {}", id);

                    // Send ACK back to sender of heartbeat
                    let ack = json!({
                        "type": "ack",
                        "id": id,
                    });

                    socket.send_to(ack.to_string().as_bytes(), src).unwrap();
                    println!("Sent ACK to {}", id);
                } 
                else if parsed["type"] == "ack" {
                    let id = parsed["id"].as_str().unwrap().to_string();
                    missed_acks.insert(id.clone(), 0); // zero out missed ACKs
                    println!("Received ACK from {}", id);
                }
            }
        }

        // remove dead elevators based on timeouts and missed ACKs
        let now = Instant::now();
        heartbeats.retain(|id, last_seen| {
            if now.duration_since(*last_seen).as_millis() > TIMEOUT_MS
                && *missed_acks.get(id).unwrap_or(&0) >= MAX_RETRIES
            {
                println!("Removing dead elevator: {}", id);
                false // remove from hashmap
            } else {
                true
            }
        });

        // increment missed ACKs for all elevators that didn't send an ACK
        for (id, count) in missed_acks.iter_mut() {
            *count += 1;
        }
    }
}
