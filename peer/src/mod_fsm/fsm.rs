
#![allow(dead_code)]
use std::thread::*;
use driver_rust::elevio::elev::{self, Elevator};


use super::config::*;
use super::requests::*;
use super::timer::Timer;


#[derive(Clone)]
pub struct FSM {
    pub elevator: Elevator,
    pub current_floor: u8,
    pub behavior: Behavior,
    pub motor_direction: u8,
    pub clear_requests: ClearRequestVariant,
    pub requests: [[bool; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
    pub door_blocked: bool,
}

impl FSM {
    // ! Not Implemented
    pub fn run(&self) {

    }
    // ! Not Implemented
    pub fn init(&self) {

    }
    // ! Not Implemented
    pub fn new(e: Elevator) -> Self {
        FSM {
            elevator: e,
            current_floor: u8::MAX,
            behavior: Behavior::Idle,
            motor_direction: elev::DIRN_STOP,
            clear_requests: ClearRequestVariant::ClearAll,
            requests: [[false; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
        }
    }
    // * Passed
    pub fn set_all_lights(&self) {
        for floor in 0..NUM_FLOORS {
            for button in 0..NUM_BUTTONS {
                self.elevator.call_button_light(floor, button, self.requests[floor as usize][button as usize]);
            }
        }
    }
    // * Passed
    pub fn init_between_floors(&mut self) {
        self.elevator.motor_direction(elev::DIRN_DOWN);
        self.motor_direction = elev::DIRN_DOWN;
        self.behavior = Behavior::Moving;
    }
    // ! Not Implemented
    pub fn fsm_on_request_button_press(&mut self, timer: &mut Timer, btn_floor: u8, btn_type: ButtonType) {
        match self.behavior {
            Behavior::DoorOpen => {
                if self.door_blocked {
                    timer.start();
                }
                if requests_should_clear_immediately()
            }
        }
    }
    pub fn on_floor_arrival(&mut self, new_floor: u8) {
        self.current_floor = new_floor;
        self.elevator.floor_indicator(new_floor);

        match self.behavior {
            Behavior::Moving => {
                if requests_should_stop(self) {
                    self.elevator.motor_direction(elev::DIRN_STOP);
                    self.motor_direction = elev::DIRN_STOP;
                    self.elevator.door_light(true);
                    requests_clear_at_current_floor(fsm, clear_variant);
                },
            _=> {

            }
        }

    }
    pub fn fsm_on_door_timout(&mut self) {
        match self.behavior {
            Behavior::DoorOpen => {
                requests_clear_at_current_floor(self, ClearRequestVariant::ClearAll);
                let pair: DirnBehaviorPair = requests_choose_direction(self);
                self.motor_direction = pair.direction;
                self.behavior = pair.behavior;
                match pair.behavior {
                    Behavior::DoorOpen => {
                        self.elevator.door_light(true);
                        sleep(Duration::from_secs(3));
                        requests_clear_at_current_floor(self, ClearRequestVariant::ClearAll);
                    },
                    Behavior::Moving => {
                        self.elevator.motor_direction(pair.direction);
                    },
                    Behavior::Idle => {
                        self.elevator.motor_direction(elev::DIRN_STOP);
                    }
                }
            }
            _ => {
            }
        }
    }
}
}


