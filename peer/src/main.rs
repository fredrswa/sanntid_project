#[allow(dead_code)]
use std::io::Result;
use std::thread;
use crossbeam_channel as cbc;
use tokio::fs::read;

mod modules;
mod misc;





fn main() -> Result<()> {
    let config = Config::new();
    





    modules::run();
    loop {
        cbc::select! {
            recv(timout_rx) -> _ => {
                //trigger recovery, warn network.
            }
        }
    }
}
