
use std::net::UdpSocket;
use std::{thread::{sleep}, time::Duration, process::{Stdio, Command}};



fn secondary_state(listening_socket: &UdpSocket) -> isize {
    println!("Starting as secondary");
    let duration = Duration::from_millis(1000);
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
            return count;
        }

    }



    count
}







pub fn string_to_bool(s: &str) -> bool {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => true,
        "false" | "0" | "no" => false,
        _ => panic!("Invalid boolean string: {}", s),
    }
}

pub fn create_socket(addr: String) -> UdpSocket {
    let socket = UdpSocket::bind(addr).expect("Could'nt setup receiver");
    socket
}

pub fn get_free_socket() -> String  {
    let mut port = 40000;
    loop {
        let addr = format!("localhost:{}", port);
        if UdpSocket::bind(&addr).is_ok() {
            let udp_send = format!("localhost:{}", port).to_string();
            return udp_send;
        }
        port += 1;
    }
}

pub fn spawn_secondary(udp_send: &str, elevator_addr: &str, floors: usize, peers: usize, lab: bool) {
    let secondary = Command::new("setsid")
        .arg("xterm")
        .arg("-e")
        .arg("cargo")
        .arg("run")
        .arg("1")
        .arg("false")
        .arg(udp_send)
        .arg(elevator_addr)
        .arg(format!("{}",floors))
        .arg(format!("{}",peers))
        .arg(format!("{}",lab))
        .stdout(Stdio::null())  // Avoid blocking by suppressing stdout
        .stderr(Stdio::null())  // Suppress stderr
        .spawn()
        .expect("Failed to start secondary process in new xterm terminal");
    println!("Secondary spawned in a new xterm window with recv address: {}", udp_send);
}

