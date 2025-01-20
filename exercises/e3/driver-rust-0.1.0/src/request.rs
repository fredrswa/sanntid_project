

pub fn requests_above(e: &Elevator) -> bool {
    for f in (e.floor + 1)..e.elev_num_floors {
        for btn in 0..3 { // A HALL_UP, HALL_DOWN, og CAB, can also write: for &btn in &[HALL_UP, HALL_DOWN, CAB]
            if e.call_button(f, btn) {
                return true; //request found
            }
        }
    }
    false 
}

pub fn requests_below(e: &Elevator) -> bool {
    for f in 0..e.floor {
        for btn in 0..3 {
            if e.call_button(f, btn) {
                return true;
            }
        }
    }
    false
}

pub fn request_here(e: &Elevator) -> bool {
    for btn in 0..3 {
        if e.call_button(e.floor, btn) {
            return true;
        }
    }
    false
}

pub struct DirnBehaviourPair {
    pub dirn: u8,
    pub behaviour: u8,
}

pub fn requests_choose_direction(e: &Elevator) -> DirnBehaviourPair {
    if e.dirn == DIRN_UP {
        if requests_above(e) {
            DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
        } else if requests_here(e) {
            DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_DOOR_OPEN }
        } else if requests_below(e) {
            DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
        } else {
            DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
        }
    } else if e.dirn == DIRN_DOWN {
        if requests_below(e) {
            DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
        } else if requests_here(e) {
            DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_DOOR_OPEN }
        } else if requests_above(e) {
            DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
        } else {
            DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
        }
    } else {
        if requests_here(e) {
            DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_DOOR_OPEN }
        } else if requests_above(e) {
            DirnBehaviourPair { dirn: DIRN_UP, behaviour: EB_MOVING }
        } else if requests_below(e) {
            DirnBehaviourPair { dirn: DIRN_DOWN, behaviour: EB_MOVING }
        } else {
            DirnBehaviourPair { dirn: DIRN_STOP, behaviour: EB_IDLE }
        }
    }
}