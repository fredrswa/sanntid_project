use std::sync::{Arc, Mutex};

const N_FLOORS: u8 = 4;
const N_BUTTONS: u8 = 3;

#[derive(Clone, Debug, Copy, PartialEq)]
enum DIRN {
    DOWN = -1,
    STOP = 0,
    UP = 1,
}
#[derive(Clone, Debug, Copy, PartialEq)]
enum BUTTON {
    HALL_UP = 0,
    HALL_DOWN = 1,
    CAB = 2,
}
#[derive(Clone, Debug, Copy, PartialEq)]
enum ELEV_BEHEVIOUR {
    IDLE,
    DOOR_OPEN,
    MOVING,
}
#[derive(Clone, Debug, Copy, PartialEq)]
enum CLEAR_REQ_VARIANT {
    CRV_ALL,
    CRV_IN_DIRN,
}
#[derive(Debug)]
struct ELEVATOR {
    floor: i32m
    dirn: DIRN,
    behaviour: ELEV_BEHEVIOUR::EB_IDLE,
    requests [[bool; N_BUTTONS]; N_FLOORS],
    behaviour: ELEV_BEHEVIOUR,
    config: ELEV_CONFIG,
}
#[derive(Clone, Debug)]
struct ELEV_CONFIG {
    clear_request_variant: CLEAR_REQ_VARIANT,
    door_open_duration_sec: f64,
}

impl Elevator {
    fn new() -> self {
        Elevator {
            floor: -1,
            dirn: DIRN::STOP,
            behaviour: ELEV_BEHEVIOUR::IDLE,
            requests: [[false; N_BUTTONS]; N_FLOORS],
            config: ELEV_CONFIG {
                clear_request_variant: CLEAR_REQ_VARIANT::CRV_ALL,
                door_open_duration_sec: 3.0,
            },
        }
    }
}

