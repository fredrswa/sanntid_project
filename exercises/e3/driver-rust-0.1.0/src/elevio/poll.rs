use crossbeam_channel as cbc;
use std::thread;
use std::time;

use super::elev;

#[derive(Debug)]
//struct for call button, with floor and call(up/down) as parameters
pub struct CallButton {
    pub floor: u8,
    pub call: u8,
}

//chanel to send the call button
//checks if the call button is pressed in any of the floors
pub fn call_buttons(elev: elev::Elevator, ch: cbc::Sender<CallButton>, period: time::Duration) {
    let mut prev = vec![[false; 3]; elev.num_floors.into()];
    loop {
        for f in 0..elev.num_floors {
            for c in 0..3 {
                let v = elev.call_button(f as u8, c);
                if v && prev[f as usize][c as usize] != v {
                    ch.send(CallButton { floor: f as u8, call: c }).unwrap();
                }
                prev[f as usize][c as usize] = v;
            }
        }
        thread::sleep(period)
    }
}

//prev is holdign the previous value of the floor sensor
//checking which floor the elevator is on, updating the value of the floor sensor if it has changed since prev

pub fn floor_sensor(elev: elev::Elevator, ch: cbc::Sender<u8>, period: time::Duration) {
    let mut prev = u8::MAX;
    loop {
        if let Some(f) = elev.floor_sensor() {
            if f != prev {
                ch.send(f).unwrap();
                prev = f;
            }
        }
        thread::sleep(period)
    }
}

//checks if the stop button is pressed
//change the status trough the channel if it has changed
pub fn stop_button(elev: elev::Elevator, ch: cbc::Sender<bool>, period: time::Duration) {
    let mut prev = false;
    loop {
        let v = elev.stop_button();
        if prev != v {
            ch.send(v).unwrap();
            prev = v;
        }
        thread::sleep(period)
    }
}

//checks if the obstruction button is pressed, sendign the status through the channel if it has changed
pub fn obstruction(elev: elev::Elevator, ch: cbc::Sender<bool>, period: time::Duration) {
    let mut prev = false;
    loop {
        let v = elev.obstruction();
        if prev != v {
            ch.send(v).unwrap();
            prev = v;
        }
        thread::sleep(period)
    }
}
