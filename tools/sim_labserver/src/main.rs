use std::{collections::HashSet,
        str,
        net::UdpSocket};
use rand::Rng;




fn main() {
    //find_actual_server_port();
    let socket = UdpSocket::bind("127.0.0.1:40012").expect("Could'nt bind to address");
    println!("Simulated lab server started at server: {}", socket.local_addr().unwrap());
    
    let mut peers: HashSet<String> = HashSet::new();
    let mut buf = [0u8; 1024];

    let loss_probability = 0.0;
    let mut rng = rand::thread_rng();

    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive data");
        let msg = String::from_utf8_lossy(&buf[..amt]);
        println!("Received {} bytes from {}: {}", msg, src, msg);

        let src_str = src.to_string();
        if peers.insert(src_str.clone()) {
            println!("Registered new peer: {}", src_str);
        }
        for peer in peers.iter() {
            if peer != &src_str {
                if rng.r#gen::<f64>() < loss_probability {
                    println!{"Simulated drop: Not forwarding packet to {}", peer};
                    continue;
                }
                socket.send_to(msg.as_bytes(), peer).expect("Failed to send packet");
                println!("Forwared packet to {}", peer);
            }
        }
    
    }  
}

pub fn find_actual_server_port() {
    let mut buf = [0u8; 1024];

    let socket = UdpSocket::bind("255.255.255.255:3000").expect("Could'nt bind");


    let (amt, src) = socket.recv_from(&mut buf).expect("LabServer not live");
    println!("Labserver on : {}", src);
}
