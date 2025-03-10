#[allow(dead_code)]
use std::io::Result;
use std::fs;
use driver_rust::elevio::elev;
use peer::{config, mod_fsm::run_fsm};
use once_cell::sync::Lazy;
use crossbeam_channel as cbc;
use std::thread::spawn;


//use peer::mod_fsm;
//mod config;

fn main() -> Result<()> {
    Lazy::force(&config::CONFIG); //Forces read of config on start of runtime in order to ensure safety

    spawn(move || run_fsm());

    loop {}
}
