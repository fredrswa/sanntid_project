#![allow(dead_code)]

use crate::elevio::elev::*;
use super::elevator::*;
use super::request::*;
use super::timer::*;

const DOOR_OPEN_DURATION: f64 = 3.0;

//PROBABLY THE WRONG WAY TO INIT FSM!
pub fn fsm_init(addr: &str, num_floors: usize, num_buttons: usize) -> Result<Elevator, std::io::Error> {
  let elevator: Result<Elevator, std::io::Error> = Elevator::init(addr, num_floors, num_buttons);
  elevator
}

pub fn set_all_lights(elevator: &Elevator) {
  for floor in 0..elevator.num_floors {
    for btn in 0..3 {
      elevator.call_button_light(floor, btn, true);
    }
  }
}

pub fn fsm_on_init_between_floors(elevator: &mut Elevator){
  elevator.motor_direction(DIRN_DOWN);
  elevator.dirn = DIRN_DOWN;
  elevator.behaviour = EB_MOVING;
}

pub fn fsm_on_request_button_press(elevator: &mut Elevator, btn_floor: usize, btn_type: Button) {
  println!("\n{}\n{}\n{}\n", "fsm_on_request_button_press", btn_floor, elevio_button_to_string(btn_type.clone()));
  elevator_print(elevator);

  match elevator.behaviour {
    EB_DOOROPEN => {
      if requests_should_clear_immediately(elevator, btn_floor, btn_type.clone()){
        timer_start(DOOR_OPEN_DURATION);
      } else {
        elevator.requests[btn_floor][btn_type as usize] = true;
      }
    }

    EB_MOVING => {
      elevator.requests[btn_floor][btn_type as usize] = true;
    }

    EB_IDLE => {
    elevator.requests[btn_floor][btn_type as usize] = true;
    let pair: DirnBehaviourPair = requests_choose_direction(elevator);
    elevator.dirn = pair.dirn;
    elevator.behaviour = pair.behaviour;
    match elevator.behaviour {
        EB_DOOROPEN => {
          elevator.door_light(true);
          timer_start(DOOR_OPEN_DURATION);
          *elevator = requests_clear_at_current_floor(elevator);
        }
        EB_MOVING => {
          elevator.motor_direction(elevator.dirn);
        }
        EB_IDLE => {

        }
    }
    }
  }
}



pub fn fsm_on_floor_arrival(elevator: &mut Elevator, new_floor: usize) {
  println!("\n{}\n{}\n", "fsm_on_floor_arrival", new_floor);
  elevator_print(elevator);

  elevator.floor = new_floor;

  match elevator.behaviour {
    EB_MOVING => {
      if requests_should_stop(elevator) {
        elevator.motor_direction(DIRN_STOP);
        elevator.dirn = DIRN_STOP;
        elevator.door_light(true);
        timer_start(DOOR_OPEN_DURATION);
        set_all_lights(elevator);
        elevator.behaviour = EB_DOOROPEN;
      }
    }
    _ => {

    }
  }
}

pub fn fsm_on_door_timeout(elevator: &mut Elevator) {
  println!("\n{}\n", "fsm_on_door_timeout");
  elevator_print(elevator);

  match elevator.behaviour {
    EB_DOOROPEN => {
      let pair: DirnBehaviourPair = requests_choose_direction(elevator);
      elevator.dirn = pair.dirn;
      elevator.behaviour = pair.behaviour;
      
      match elevator.behaviour {
        EB_DOOROPEN => {
          timer_start(DOOR_OPEN_DURATION);
          *elevator = requests_clear_at_current_floor(elevator);
          elevator.door_light(true);
        }
        EB_MOVING => {
        }
        EB_IDLE => {
        elevator.door_light(false);
        elevator.motor_direction(elevator.dirn);
        }
      }
    }
    _ => {

    }
  }
}