#[allow(dead_code)]
use std::io::Result;
use std::fs;
use config::Timeout_type;
use crossbeam_channel::{select, unbounded, Sender, Receiver};

pub mod config;


fn main() -> Result<()> {
    let (timeout_tx, timeout_rx) = unbounded::<Timeout_type>();
    loop{
        select! {
            recv(timeout_rx) -> timout_struct => {
                
            }
        }
    }
}
