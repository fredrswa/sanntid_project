#![allow(dead_code)]

use crate::elevio::elev::*;

//Checks if there are any requests above the current floor
pub fn requests_above(elevator: &Elevator) -> bool {
    for floor in (elevator.floor + 1)..elevator.num_floors {
        for btn in 0..3 { // A HALL_UP, HALL_DOWN, og CAB, can also write: for &btn in &[HALL_UP, HALL_DOWN, CAB]
            if elevator.call_button(floor as u8, btn) {
                return true; //request found
            }
        }
    }
    false 
}

//Checks if there are any requests below the current floor
pub fn requests_below(elevator: &Elevator) -> bool {
    for f in elevator.floor..0 {
        for btn in 0..3 {
            if elevator.call_button(f as u8, btn) {
                return true;
            }
        }
    }
    false
}

//Checks if there are any requests at the current floor
pub fn requests_here(elevator: &Elevator) -> bool {
    for btn in 0..3 {
        if elevator.call_button(elevator.floor as u8, btn) {
            return true;
        }
    }
    false
}

pub struct DirnBehaviourPair {
    pub dirn: u8,
    pub behaviour: ElevatorBehaviour,
}


//Chooses the direction and sets the associated behaviour
pub fn requests_choose_direction(elevator: &Elevator) -> DirnBehaviourPair {
    match elevator.dirn {
        DIRN_UP => {
            if requests_above(elevator) {
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
            } else if requests_here(elevator) {
                return DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_DOOROPEN }
            } else if requests_below(elevator) {
                return DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
            } else {
                return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
            }
        }
        DIRN_DOWN => {
            if requests_below(elevator) {
                return DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
            } else if requests_here(elevator) {
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_DOOROPEN }
            } else if requests_above(elevator) {
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
            } else {
                return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
            }
        }

        DIRN_STOP => {
            if requests_here(elevator) {
                return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_DOOROPEN }
            } else if requests_above(elevator){
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
            } else if requests_below(elevator){
                return DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
            } else {
                return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
            }
        }

        _ => {
            return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
        }
    }
}


//When is this function used?
pub fn requests_should_stop(elevator: &Elevator) -> bool {
    match elevator.dirn {
        DIRN_DOWN => {
            elevator.requests[elevator.floor][HALL_DOWN as usize] || 
            elevator.requests[elevator.floor][CAB as usize] || 
            !requests_below(elevator)
        },
        DIRN_UP => {
            elevator.requests[elevator.floor][HALL_UP as usize] || 
            elevator.requests[elevator.floor][CAB as usize] || 
            !requests_above(elevator)
        },
        DIRN_STOP => return true, // Always stop if the direction is Stop
        _ => return false
    }
}

pub fn requests_should_clear_immediately(elevator: &Elevator, btn_floor: usize, btn_type: Button) -> bool {
    elevator.floor == btn_floor && (
        (elevator.dirn == DIRN_UP   && btn_type == Button::BHallup)    ||
        (elevator.dirn == DIRN_DOWN && btn_type == Button::BHalldown)  ||
        elevator.dirn == DIRN_STOP ||
        btn_type == Button::BCab
    )
}


//Clears all requests at current floor
//Returns elevator?? (Might be useful in rust pga. ownership)
pub fn requests_clear_at_current_floor(elevator: &mut Elevator) -> Elevator {
    match elevator.dirn {
        DIRN_UP => {
            if !requests_above(elevator) && !elevator.requests[elevator.floor][HALL_UP as usize] {
                elevator.requests[elevator.floor][HALL_DOWN as usize] = false;
            }
            elevator.requests[elevator.floor][HALL_UP as usize] = false;
        }

        DIRN_DOWN => {
            if !requests_below(elevator) && !elevator.requests[elevator.floor][HALL_DOWN as usize] {
                elevator.requests[elevator.floor][HALL_UP as usize] = false;
            }
            elevator.requests[elevator.floor][HALL_DOWN as usize] = false;
        }

        DIRN_STOP => {}

        _ => {
            elevator.requests[elevator.floor][HALL_UP as usize] = false;
            elevator.requests[elevator.floor][HALL_DOWN as usize] = false;
        }
    }

    elevator.clone()
}