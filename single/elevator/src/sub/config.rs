#![allow(dead_code)]

pub const NUM_FLOORS: u8 = 4;
pub const NUM_BUTTONS: u8 = 3;
pub const DOOR_OPEN_TIME: u64 = 3000;

use std::u8;

use driver_rust::elevio::elev::{self, Elevator};

#[derive(Copy, Clone)]
pub enum Behavior {
    Idle,
    Moving,
    DoorOpen,
}

pub enum ButtonType {
    Hallup = 0,
    Halldown = 1,
    Cab = 2,
}

#[derive(Copy, Clone)]
pub enum ClearRequestVariant {
    ClearAll,
    ClearInDirection,
}

pub fn call_to_button_type(call: u8) -> ButtonType {
    match call {
        0 => ButtonType::Hallup,
        1 => ButtonType::Halldown,
        2 => ButtonType::Cab,
        _ => panic!("Invalid button type"),
    }
}
