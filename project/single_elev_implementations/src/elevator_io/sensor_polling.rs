use crossbeam_channel as cbc;
use std::thread;
use std::time;

use super::elevator::*;

#[derive(Debug)]
pub struct CallButton {
    pub floor: u8,
    pub call: u8,
}

pub fn call_buttons(elevator: Elevator, channel: cbc::Sender<CallButton>, period: time::Duration) {
    let mut prev = vec![[false; 3]; elevator.num_floors.into()];
    loop {
        for f in 0..elevator.num_floors {
            for c in 0..3 {
                let v = elevator.call_button(f as u8, c);
                if v && prev[f as usize][c as usize] != v {
                    channel.send(CallButton { floor: f as u8, call: c}).unwrap();
                }
                prev[f as usize][c as usize] = v;
            }
        }
        thread::sleep(period)
    }
}

pub fn floor_sensor(elevator: Elevator, channel: cbc::Sender<u8>, period: time::Duration) {
    let mut prev = u8::MAX;
    loop {
        if let Some(f) = elevator.floor_sensor() {
            if f != prev {
                channel.send(f).unwrap();
                prev = f; 
            }
        }
        thread::sleep(period)
    }
}

pub fn stop_button(elevator: Elevator, channel: cbc::Sender<bool>, period: time::Duration) {
    let mut prev = false;
    loop {
        let v = elevator.stop_button();
        if prev != v {
            channel.send(v).unwrap();
            prev = v;
        }
        thread::sleep(period)
    }
}

pub fn obstruction(elevator: Elevator, channel: cbc::Sender<bool>, period: time::Duration) {
    let mut prev = false;
    loop {
        let v = elevator.obstruction();
        if prev != v {
            channel.send(v).unwrap();
            prev = v;
        }
        thread::sleep(period)
    }
}


