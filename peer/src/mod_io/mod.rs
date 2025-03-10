pub mod io_funcs;


use crossbeam_channel as cbc;

use crate::{config::EntireSystem, mod_io::io_funcs::*};

pub fn run(/* Channels */) {
    // Simulate Channels Here //
    let (network_io_tx, network_io_rx) = cbc::unbounded::<EntireSystem>();
    let (fsm_io_tx, fsm_io_rx) = cbc::unbounded::<String>();
    
    loop {
        cbc::select!{
            recv(network_io_rx) -> network_message => {
                let message = network_message.unwrap(); 
                    let new_states = call_assigner(message);
                    //save_system_state_to_json(new_states);
                    //Send states to where they belong (FSM and network?)   
            }
            recv(fsm_io_rx) -> fsm_message => {
                let message = fsm_message.unwrap();
                    //What will come from fsm?
                
            }
        }
    }
}