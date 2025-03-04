#[allow(dead_code)]
use std::io::Result;
use std::fs;



pub mod modules;
pub mod config;

pub fn create_format() {
    let config = config::Config {
        num_floors: 4,
        num_buttons: 3,
        num_elevators: 3,
        door_open_s: 3,
        id: "id:001".to_string(),
        elev_addr: "localhost:15765".to_string(),
        udp_send_port: "placeholder".to_string(),
        udp_recv_port: "placeholder".to_string(),
    };
    let config_string = serde_json::to_string_pretty(&config).unwrap();
    fs::write("config.json", config_string);
}


fn main() -> Result<()> {
    create_format();
    Ok(())
}
