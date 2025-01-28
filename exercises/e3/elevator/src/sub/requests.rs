#![allow(dead_code)]



use super::config::*;


use driver_rust::elevio::elev;
use crate::sub::fsm::FSM;

pub struct DirnBehaviorPair {
    pub direction: u8,
    pub behavior: Behavior,
}


pub fn requests_above(fsm: &FSM) -> bool {
    for floor in (fsm.current_floor + 1)..NUM_FLOORS {
        for button in 0..NUM_BUTTONS {
            if fsm.requests[floor as usize][button as usize] {
                return true;
            }
        }
    }
    false
}

pub fn requests_below(fsm: &FSM) -> bool {
    for floor in 0..fsm.current_floor {
        for button in 0..NUM_BUTTONS {
            if fsm.requests[floor as usize][button as usize] {
                return true;
            }
        }
    }
    false
}

pub fn requests_choose_direction(fsm: &FSM) -> DirnBehaviorPair {
    match fsm.motor_direction {
        elev::DIRN_UP => {
            if requests_above(fsm) {
                return DirnBehaviorPair {
                    direction: elev::DIRN_UP,
                    behavior: Behavior::Moving,
                };
            } else if requests_below(fsm) {
                return DirnBehaviorPair {
                    direction: elev::DIRN_DOWN,
                    behavior: Behavior::Moving,
                };
            } else {
                return DirnBehaviorPair {
                    direction: elev::DIRN_STOP,
                    behavior: Behavior::Idle,
                };
            }
        }
        elev::DIRN_DOWN => {
            if requests_below(fsm) {
                return DirnBehaviorPair {
                    direction: elev::DIRN_DOWN,
                    behavior: Behavior::Moving,
                };
            } else if requests_above(fsm) {
                return DirnBehaviorPair {
                    direction: elev::DIRN_UP,
                    behavior: Behavior::Moving,
                };
            } else {
                return DirnBehaviorPair {
                    direction: elev::DIRN_STOP,
                    behavior: Behavior::Idle,
                };
            }
        }
        elev::DIRN_STOP => {
            if requests_above(fsm) {
                return DirnBehaviorPair {
                    direction: elev::DIRN_UP,
                    behavior: Behavior::Moving,
                };
            } else if requests_below(fsm) {
                return DirnBehaviorPair {
                    direction: elev::DIRN_DOWN,
                    behavior: Behavior::Moving,
                };
            } else {
                return DirnBehaviorPair {
                    direction: elev::DIRN_STOP,
                    behavior: Behavior::Idle,
                };
            }
        }
        _ => {
            return DirnBehaviorPair {
            direction: elev::DIRN_STOP,
            behavior: Behavior::Idle,
            };
        }
    }
}

pub fn requests_should_stop(fsm: &FSM) -> bool {
    match fsm.motor_direction {
        elev::DIRN_UP => {
            if fsm.requests[fsm.current_floor as usize][ButtonType::Hallup as usize] || fsm.requests[fsm.current_floor as usize][ButtonType::Cab as usize] {
                return true;
            } else if !requests_above(fsm) {
                return true;
            } else {
                return false;
            }
        }
        elev::DIRN_DOWN => {
            if fsm.requests[fsm.current_floor as usize][ButtonType::Halldown as usize] || fsm.requests[fsm.current_floor as usize][ButtonType::Cab as usize] {
                return true;
            } else if !requests_below(fsm) {
                return true;
            } else {
                return false;
            }
        }
        elev::DIRN_STOP => {
            return true;
        }
        _ => {
            return false;
        }
    }
}
pub fn requests_should_clear_immediatly(fsm: &FSM) -> bool {
    match fsm.clear_requests {
        ClearRequestVariant::ClearAll => {
            return true;
        }
        ClearRequestVariant::ClearInDirection => {
            match fsm.motor_direction {
                elev::DIRN_UP => {
                    if !requests_above(fsm) {
                        return true;
                    } else {
                        return false;
                    }
                }
                elev::DIRN_DOWN => {
                    if !requests_below(fsm) {
                        return true;
                    } else {
                        return false;
                    }
                }
                elev::DIRN_STOP => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
    }
}

pub fn requests_clear_at_current_floor(fsm: &mut FSM, clear_variant: ClearRequestVariant) {
    match clear_variant {
        ClearRequestVariant::ClearAll => {
            for button in 0..NUM_BUTTONS {
                fsm.requests[fsm.current_floor as usize][button as usize] = false;
            }
        },
        ClearRequestVariant::ClearInDirection => {
            match fsm.motor_direction {
                elev::DIRN_UP => {
                    fsm.requests[fsm.current_floor as usize][ButtonType::Hallup as usize] = false;
                    fsm.requests[fsm.current_floor as usize][ButtonType::Cab as usize] = false;
                },
                elev::DIRN_DOWN => {
                    fsm.requests[fsm.current_floor as usize][ButtonType::Halldown as usize] = false;
                    fsm.requests[fsm.current_floor as usize][ButtonType::Cab as usize] = false;
                },
                elev::DIRN_STOP => {
                    for button in 0..NUM_BUTTONS {
                        fsm.requests[fsm.current_floor as usize][button as usize] = false;
                    }
                },
                _ => {},
            }
        }
    }
}