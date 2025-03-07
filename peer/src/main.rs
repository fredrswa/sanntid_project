#[allow(dead_code)]
use std::io::Result;
use std::fs;


pub mod config;


fn main() -> Result<()> {

    loop{
        cbc::select! {
            recv(timeout_rx) -> timout_struct => {
                
            }
        }
    }
}
