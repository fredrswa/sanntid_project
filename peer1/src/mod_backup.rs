use core::panic;
use std::net::{UdpSocket, SocketAddr};
use std::{env, thread, time, process::Command};
use std::time::Duration;
use std::thread::sleep;
use std::process::Stdio;



fn secondary_state(listening_socket: &UdpSocket) -> SystemState {
    println!("Starting as secondary");
    let duration = Duration::from_millis(state.update_s);
    let mut buf = [0;10];

    let mut count: isize = 0;
    let mut attempts = 0;


    loop {
        sleep(duration);
        listening_socket.set_nonblocking(true).expect("Failed to set non-blocking");
        match listening_socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
            let received = String::from_utf8_lossy(&buf[..amt]);
            if let Ok(parsed) = received.trim().parse::<isize>() {
                count = parsed;
                println!("Secondary has received {}", count);
                attempts = 0;
            } else {
                attempts += 1;
            }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No data to read, skip this iteration
            attempts +=1;
            }
            Err(_) => {
            attempts += 1;
            }
        }
        if attempts >= 3 {
            //drop(listening_socket);
            return count;
        }

    }



    count
}