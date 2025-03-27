use std::{collections::HashSet,
        str,
        net::UdpSocket};
use rand::Rng;




fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let loopback = args.get(1).map_or(false, |arg| arg == "loopback");
    let loss_probability = args.get(2).map_or(0.0, |arg| arg.parse().expect("Failed to parse loss probability"));
    let socket = UdpSocket::bind("0.0.0.0:2000").expect("Could'nt bind to address");
    println!("Simulated lab server started at server: {}, loopback: {}, packetloss: {}", socket.local_addr().unwrap(), loopback, loss_probability);
    
    let mut peers: HashSet<String> = HashSet::new();
    let mut buf = [0u8; 1024];

    
    let mut rng = rand::thread_rng();

    let loop_back = false;
    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive data");
        let msg = String::from_utf8_lossy(&buf[..amt]);
        //println!("Received {} bytes from {}: {}", msg, src, msg);

        let src_str = src.to_string();
        if peers.insert(src_str.clone()) {
            println!("Registered new peer: {}", src_str);
        }
        for peer in peers.iter() {
            if (peer != &src_str) || loopback {
                if rng.r#gen::<f64>() < loss_probability {
                    // println!{"Simulated drop: Not forwarding packet to {}", peer};
                    continue;
                }
                socket.send_to(msg.as_bytes(), peer).expect("Failed to send packet");
                //println!("Forwared packet to {}", peer);
            }
        }
    
    }  
}