#[allow(dead_code)]
use std::io::Result;
use std::fs;
use once_cell::sync::Lazy;
use crossbeam_channel as cbc;

mod config;


fn main() -> Result<()> {
    Lazy::force(&config::CONFIG); //Forces read of config on start of runtime in order to ensure safety
    

    loop {

    }
}
