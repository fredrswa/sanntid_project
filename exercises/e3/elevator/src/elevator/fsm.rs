
#![allow(dead_code)]
use std::thread::*;
use std::time::*;

use driver_rust::elevio::elev::{self, Elevator};


use super::config::*;
use super::requests::*;


#[derive(Clone)]
pub struct FSM {
    pub elevator: Elevator,
    pub current_floor: u8,
    pub behavior: Behavior,
    pub motor_direction: u8,
    pub clear_requests: ClearRequestVariant,
    pub requests: [[bool; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
}

impl FSM {
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
    pub fn set_all_lights(&self) {
        for floor in 0..NUM_FLOORS {
            for button in 0..NUM_BUTTONS {
                self.elevator.call_button_light(floor, button, self.requests[floor as usize][button as usize]);
            }
        }
    }

    pub fn init_between_floors(&mut self) {
        self.elevator.motor_direction(elev::DIRN_DOWN);
        self.motor_direction = elev::DIRN_DOWN;
        self.behavior = Behavior::Moving;
    }
    pub fn fsm_on_request_button_press(&mut self, btn_floor: u8, btn_type: ButtonType) {
        match self.behavior {
            Behavior::DoorOpen => {
                if requests_should_clear_immediatly(self) {
                    requests_clear_at_current_floor(self, ClearRequestVariant::ClearInDirection); //KA SKJER HER?
                    sleep(Duration::from_secs(3));

                } else {
                    self.requests[btn_floor as usize][btn_type as usize] = true;
                }
            }
            Behavior::Moving => {
                self.requests[btn_floor as usize][btn_type as usize] = true;
            }
            Behavior::Idle => {
                self.requests[btn_floor as usize][btn_type as usize] = true;
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
        }
        self.set_all_lights();
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


