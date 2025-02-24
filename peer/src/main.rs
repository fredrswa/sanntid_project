#[allow(dead_code)]
use std::io::Result;
use peer::mod_fsm::fsm;
use peer::mod_network::network;
use peer::mod_assigner::json_testing;
mod mod_io;


use crossbeam_channel as cbc;
use driver_rust::elevio::poll as sensor_polling;
use std::thread::spawn;



fn main() -> Result<()> {


    let (call_button_io_tx, call_button_io_rx) = cbc::unbounded::<sensor_polling::CallButton>();
    
    spawn(|| {
        mod_io::run_io(call_button_io_tx);
    });
    spawn(|| {
        fsm::test_script_elevator_system(call_button_io_rx);
    });
    //pass all or a subset of channels into each mod::run_..
    //vola
    loop {
        
    }
}
