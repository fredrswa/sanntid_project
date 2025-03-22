use crate::elevator_io::elevator::*;
use super::requests::*;
use super::defs::*;
use super::timer::Timer;

pub fn fsm_init(elevator: &mut Elevator){
  loop {
    elevator.floor = elevator.floor_sensor().unwrap_or(u8::MAX) as usize;
    match elevator.floor {
      255 => {
        fsm_on_init_between_floors(elevator);
      }
      _ => {
        elevator.motor_direction(Dirn::DirnStop as u8);
        elevator.dirn = Dirn::DirnStop;
        elevator.behaviour = ElevatorBehaviour::Idle;
        break;
      }
    }
  }  
}

pub fn fsm_on_init_between_floors(elevator: &mut Elevator){
  elevator.motor_direction(Dirn::DirnDown as u8);
  elevator.dirn = Dirn::DirnDown;
  elevator.behaviour = ElevatorBehaviour::Moving;
}

pub fn set_all_lights(elevator: &mut Elevator) {
  for floor in 0..NUM_FLOORS {
    for button in 0..NUM_BUTTONS {
      elevator.call_button_light(floor, button as u8, elevator.requests[floor][button]);
    }
  }
}

pub fn fsm_on_request_button_press(elevator: &mut Elevator, timer: &mut Timer, btn_floor: usize, btn_type: ButtonType) {
  match elevator.behaviour {
    ElevatorBehaviour::DoorOpen => {
      if elevator.blocked {
        timer.start();
      }

      if requests_should_clear_immediately(elevator, btn_floor, btn_type) {
        elevator.door_light(true);
        requests_clear_at_current_floor(elevator);
        timer.start();
      } else {
        elevator.requests[btn_floor][btn_type as usize] = true;
      }
    }
    ElevatorBehaviour::Moving => {
      elevator.requests[btn_floor][btn_type as usize] = true;
    }
    ElevatorBehaviour::Idle => {
      if requests_should_clear_immediately(elevator, btn_floor, btn_type) {
        elevator.door_light(true);
        requests_clear_at_current_floor(elevator);
        timer.start();
      } else {
        elevator.requests[btn_floor][btn_type as usize] = true;
        let db_pair: DirnBehaviourPair = requests_choose_direction(elevator);
        elevator.dirn = db_pair.dirn;
        elevator.behaviour = db_pair.behaviour;
        match elevator.behaviour {
          ElevatorBehaviour::DoorOpen => {
            elevator.door_light(true);
            timer.start();
            requests_clear_at_current_floor(elevator);
          }
          ElevatorBehaviour::Moving => {
            elevator.motor_direction(elevator.dirn.clone() as u8);
          }
          ElevatorBehaviour::Idle => {}  
        }
      }
    }
  }
  set_all_lights(elevator);
}

pub fn fsm_on_floor_arrival(elevator: &mut Elevator, timer: &mut Timer, new_floor: usize) {
  
  elevator.floor = new_floor;
  elevator.floor_indicator(new_floor as u8);

  match elevator.behaviour {
    ElevatorBehaviour::Moving => {
      if requests_should_stop(elevator) {
        requests_clear_at_current_floor(elevator);
        elevator.motor_direction(Dirn::DirnStop as u8);
        elevator.door_light(true);
        timer.start();
        set_all_lights(elevator);
        elevator.behaviour = ElevatorBehaviour::DoorOpen;
      }
    }
    ElevatorBehaviour::DoorOpen => {println!("ERROR");}
    ElevatorBehaviour::Idle => {println!("ERROR");}
  }
}

pub fn fsm_on_door_timeout(elevator: &mut Elevator, timer: &mut Timer) {
  match elevator.behaviour {
    ElevatorBehaviour::DoorOpen => {
      if elevator.blocked {
        timer.start();
      } else {
        let db_pair: DirnBehaviourPair = requests_choose_direction(elevator);
        elevator.dirn = db_pair.dirn;
        elevator.behaviour = db_pair.behaviour;

        requests_clear_at_current_floor(elevator);
        set_all_lights(elevator);
        
        match elevator.behaviour {
          ElevatorBehaviour::DoorOpen => {
            timer.start();
          }
          ElevatorBehaviour::Moving => {
            elevator.door_light(false);
            elevator.motor_direction(elevator.dirn.clone() as u8);
          }
          ElevatorBehaviour::Idle => {
            elevator.door_light(false);
          }
        }
      }
    }
    ElevatorBehaviour::Moving => {elevator.door_light(false);}
    ElevatorBehaviour::Idle => {elevator.door_light(false);}
  }
}
