use crate::elevio::elev::*;

//Checks if there are any requests above the current floor
pub fn requests_above(e: &Elevator) -> bool {
    for f in (e.floor + 1)..e.num_floors {
        for btn in 0..3 { // A HALL_UP, HALL_DOWN, og CAB, can also write: for &btn in &[HALL_UP, HALL_DOWN, CAB]
            if e.call_button(f as u8, btn) {
                return true; //request found
            }
        }
    }
    false 
}

//Checks if there are any requests below the current floor
pub fn requests_below(e: &Elevator) -> bool {
    for f in 0..e.num_floors {
        for btn in 0..3 {
            if e.call_button(f as u8, btn) {
                return true;
            }
        }
    }
    false
}

//Checks if there are any requests at the current floor
pub fn requests_here(e: &Elevator) -> bool {
    for btn in 0..3 {
        if e.call_button(e.floor as u8, btn) {
            return true;
        }
    }
    false
}

pub struct DirnBehaviourPair {
    pub dirn: u8,
    pub behaviour: u8,
}


//Chooses the direction and sets the associated behaviour
pub fn requests_choose_direction(e: &Elevator) -> DirnBehaviourPair {
    match e.dirn {
        DIRN_UP => {
            if requests_above(e) {
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
            } else if requests_here(e) {
                return DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_DOOROPEN }
            } else if requests_below(e) {
                return DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
            } else {
                return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
            }
        }
        DIRN_DOWN => {
            if requests_below(e) {
                return DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
            } else if requests_here(e) {
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_DOOROPEN }
            } else if requests_above(e) {
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
            } else {
                return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
            }
        }

        DIRN_STOP => {
            if requests_here(e) {
                return DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_DOOROPEN }
            } else if requests_above(e){
                return DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
            } else if requests_below(e){
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
pub fn requests_should_stop(e: &Elevator) -> bool {
    match e.dirn {
        DIRN_DOWN => {
            e.requests[e.floor][HALL_DOWN as usize] || 
            e.requests[e.floor][CAB as usize] || 
            !requests_below(e)
        },
        DIRN_UP => {
            e.requests[e.floor][HALL_UP as usize] || 
            e.requests[e.floor][CAB as usize] || 
            !requests_above(e)
        },
        DIRN_STOP => return true, // Always stop if the direction is Stop
        _ => return false
    }
}

pub fn requests_should_clear_immediately(e: &Elevator, btn_floor: usize, btn_type: Button) -> bool {
    e.floor == btn_floor && (
        (e.dirn == DIRN_UP   && btn_type == Button::BHallup)    ||
        (e.dirn == DIRN_DOWN && btn_type == Button::BHalldown)  ||
        e.dirn == DIRN_STOP ||
        btn_type == Button::BCab
    )
}


//Clears all requests at current floor
//Returns elevator?? (Might be useful in rust pga. ownership)
fn requests_clear_at_current_floor(e: &mut Elevator) -> Elevator {
    match e.dirn {
        DIRN_UP => {
            if !requests_above(e) && !e.requests[e.floor][HALL_UP as usize] {
                e.requests[e.floor][HALL_DOWN as usize] = false;
            }
            e.requests[e.floor][HALL_UP as usize] = false;
        }

        DIRN_DOWN => {
            if !requests_below(e) && !e.requests[e.floor][HALL_DOWN as usize] {
                e.requests[e.floor][HALL_UP as usize] = false;
            }
            e.requests[e.floor][HALL_DOWN as usize] = false;
        }

        DIRN_STOP => {}

        _ => {
            e.requests[e.floor][HALL_UP as usize] = false;
            e.requests[e.floor][HALL_DOWN as usize] = false;
        }
    }

    e.clone()
}
