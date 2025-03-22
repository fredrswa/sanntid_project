use crate::elevator_io::elevator::*;
use super::defs::*;

pub fn requests_above(elevator: &Elevator) -> bool {
  for floor in (elevator.floor + 1)..NUM_FLOORS {
    for button in 0..NUM_BUTTONS {
      if elevator.requests[floor][button] {
        return true;
      }
    }
  }
  return false;
}

pub fn requests_below(elevator: &Elevator) -> bool {
  for floor in 0..elevator.floor {
    for button in 0..NUM_BUTTONS {
      if elevator.requests[floor][button] {
        return true;
      }
    }
  }
  return false;
}

pub fn requests_here(elevator: &Elevator) -> bool {
  for button in 0..NUM_BUTTONS {
    if elevator.requests[elevator.floor][button] {
      return true;
    }
  }
  return false;
}

pub fn requests_choose_direction(elevator: &Elevator) -> DirnBehaviourPair {
  match elevator.dirn {
    Dirn::DirnDown => {
      if requests_below(elevator) {
        return DirnBehaviourPair { dirn: Dirn::DirnDown, behaviour: ElevatorBehaviour::Moving }
      } else if requests_here(elevator) {
        return DirnBehaviourPair { dirn: Dirn::DirnUp, behaviour: ElevatorBehaviour::DoorOpen }
      } else if requests_above(elevator) {
        return DirnBehaviourPair { dirn: Dirn::DirnUp, behaviour: ElevatorBehaviour::Moving }
      } else {
        return DirnBehaviourPair { dirn: Dirn::DirnStop, behaviour: ElevatorBehaviour::Idle }
      }
    }

    Dirn::DirnStop  => {
      if requests_here(elevator) {
        return DirnBehaviourPair { dirn: Dirn::DirnStop, behaviour: ElevatorBehaviour::DoorOpen }
      } else if requests_above(elevator){
        return DirnBehaviourPair { dirn: Dirn::DirnUp, behaviour: ElevatorBehaviour::Moving }
      } else if requests_below(elevator){
        return DirnBehaviourPair { dirn: Dirn::DirnDown, behaviour: ElevatorBehaviour::Moving }
      } else {
        return DirnBehaviourPair { dirn: Dirn::DirnStop, behaviour: ElevatorBehaviour::Idle }
      }
    }

    Dirn::DirnUp  => {
      if requests_above(elevator) {
        return DirnBehaviourPair { dirn: Dirn::DirnUp, behaviour: ElevatorBehaviour::Moving }
      } else if requests_here(elevator) {
        return DirnBehaviourPair { dirn: Dirn::DirnDown, behaviour: ElevatorBehaviour::DoorOpen }
      } else if requests_below(elevator) {
        return DirnBehaviourPair { dirn: Dirn::DirnDown, behaviour: ElevatorBehaviour::Moving }
      } else {
        return DirnBehaviourPair { dirn: Dirn::DirnStop, behaviour: ElevatorBehaviour::Idle }
      }
    }
  }
}

pub fn requests_should_stop(elevator: &Elevator) -> bool {
  match elevator.dirn {
    Dirn::DirnDown => {
      elevator.requests[elevator.floor][ButtonType::HallDown as usize] || 
      elevator.requests[elevator.floor][ButtonType::Cab as usize] || 
      !requests_below(elevator)
    }
    Dirn::DirnStop => {return true;}
    Dirn::DirnUp => {
      elevator.requests[elevator.floor][ButtonType::HallUp as usize] || 
      elevator.requests[elevator.floor][ButtonType::Cab as usize] || 
      !requests_above(elevator)
    }
  }
}

pub fn requests_should_clear_immediately(elevator: &Elevator, btn_floor: usize, btn_type: ButtonType) -> bool {
  elevator.floor == btn_floor && (
      (elevator.dirn == Dirn::DirnUp   && btn_type == ButtonType::HallUp)    ||
      (elevator.dirn == Dirn::DirnDown && btn_type == ButtonType::HallDown)  ||
      elevator.dirn == Dirn::DirnStop ||
      btn_type == ButtonType::Cab
  )
}

pub fn requests_clear_at_current_floor(elevator: &mut Elevator) {
  elevator.requests[elevator.floor][ButtonType::Cab as usize] = false;
  match elevator.dirn {
    Dirn::DirnDown => {
      if !requests_below(elevator) && !elevator.requests[elevator.floor][ButtonType::HallDown as usize] {
        elevator.requests[elevator.floor][ButtonType::HallUp as usize] = false;
      }
      elevator.requests[elevator.floor][ButtonType::HallDown as usize] = false;
    }
    
    Dirn::DirnUp => {
      if !requests_above(elevator) && !elevator.requests[elevator.floor][ButtonType::HallUp as usize] {
        elevator.requests[elevator.floor][ButtonType::HallDown as usize] = false;
      }
      elevator.requests[elevator.floor][ButtonType::HallUp as usize] = false;
    }

    Dirn::DirnStop => {}
  }
}
