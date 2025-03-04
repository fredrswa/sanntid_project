use serde::{Serialize, Deserialize};
use std::fs;
use serde_json;


#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub num_floors: usize,
    pub num_buttons: usize,
    pub num_elevators: usize,
    pub door_open_s: usize,
    pub id: String,
    pub elev_addr: String,
    pub udp_send_port: String,
    pub udp_recv_port: String,
}
impl Config {
    pub fn import() -> Config {
        let config_string = fs::read_to_string("config.json").expect("Unable to read file");
        let config: Config = serde_json::from_str(&config_string).expect("JSON was not well-formatted");
        config
    }

    
}


