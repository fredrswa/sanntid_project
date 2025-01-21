use crate::elevio::elev::*;
use super::elevator::*;
use super::request::*;
use super::timer::*;

//PROBABLY THE WRONG WAY TO INIT FSM!
fn fsm_init(addr: &str, num_floors: usize, floor: usize, dirn: u8) -> Result<Elevator, std::io::Error> {
  let elevator: Result<Elevator, std::io::Error> = Elevator::init(addr, num_floors, floor, DIRN_UP);
  elevator
}

fn set_all_lights(elevator: &Elevator) {
  for floor in 0..elevator.num_floors {
    for btn in 0..3 {
      elevator.call_button_light(floor, btn, true);
    }
  }
}

fn fsm_on_init_between_floors(elevator: &mut Elevator){
  elevator.motor_direction(DIRN_DOWN);
  elevator.dirn = DIRN_DOWN;
  elevator.behaviour = EB_MOVING;
}

fn fsm_on_request_button_press(btn_floor: usize, btn_type: Button, elevator: &mut Elevator) {
  println!("\n{}\n{}\n{}\n", "fsm_on_request_button_press", btn_floor, elevio_button_to_string(btn_type));
  elevator_print(elevator);

  match elevator.behaviour {
    EB_DOOROPEN => {
      if (requests_should_clear_immediately(elevator, btn_floor, btn_type)){
        timer_start(duration);
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
    match pair.behaviour {
        EB_DOOROPEN => {
          elevator.door_light(true);
          timer_start(duration);
          elevator = requests_clear_at_current_floor(elevator);
        }
        EB_MOVING => {
          elevator.motor_direction(dirn);
        }
        EB_IDLE => {

        }
    }
    }
  }
}



fn fsm_on_floor_arrival(new_floor: usize, elevator: &Elevator) {
  println!("\n{}\n{}\n", "fsm_on_floor_arrival", new_floor);
  elevator_print(elevator);

  elevator.floor = new_floor;

  match elevator.behaviour {
    EB_MOVING => {
      if requests_should_stop(elevator) {
        elevator.motor_direction(DIRN_STOP);
        elevator.dirn = DIRN_STOP;
        elevator.door_light(true);
        timer_start(duration);
        set_all_lights(elevator);
        elevator.behaviour = EB_DOOROPEN;
      }
    }
    _ => {

    }
  }
}

fn fsm_on_door_timeout() {
  println!("\n{}\n", "fsm_on_door_timeout");
  elevator_print(elevator);

  match elevator.behaviour {
    EB_DOOROPEN => {
      let pair: DirnBehaviourPair = requests_choose_direction(elevator);
      elevator.dirn = pair.dirn;
      elevator.behaviour = pair.behaviour;
      
      match pair.behaviour {
        EB_DOOROPEN => {
          timer_start(duration);
          elevator = requests_clear_at_current_floor(elevator);
          elevator.door_light(true);
        }
        EB_MOVING => {
        }
        EB_IDLE => {
        elevator.door_light(false);
        elevator.motor_direction(dirn);
        }
      }
    }
    _ => {

    }
  }
}