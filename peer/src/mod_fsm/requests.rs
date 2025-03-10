
use crate::mod_fsm::fsm::ElevatorSystem;
use crate::config::*;

pub fn requests_above(es: &ElevatorSystem) -> bool {
  for floor in (es.status.curr_floor as usize + 1)..es.num_floors {
    for button in 0..es.num_buttons {
      if es.requests[floor][button] {
        return true;
      }
    }
  }
  return false;
}

pub fn requests_below(es: &ElevatorSystem) -> bool {
  for floor in 0..es.status.curr_floor as usize {
    for button in 0..es.num_buttons {
      if es.requests[floor][button] {
        return true;
      }
    }
  }
  return false;
}

pub fn requests_here(es: &ElevatorSystem) -> bool {
  for button in 0..es.num_buttons {
    if es.requests[es.status.curr_floor as usize][button] {
      return true;
    }
  }
  return false;
}

pub fn requests_choose_direction(es: &ElevatorSystem) -> DirnBehaviorPair {
  match es.status.curr_dirn {
    Dirn::Down => {
      if requests_below(es) {
        return DirnBehaviorPair { direction: Dirn::Down, behavior: Behavior::Moving }
      } else if requests_here(es) {
        return DirnBehaviorPair { direction: Dirn::Up, behavior: Behavior::DoorOpen }
      } else if requests_above(es) {
        return DirnBehaviorPair { direction: Dirn::Up, behavior: Behavior::Moving }
      } else {
        return DirnBehaviorPair { direction: Dirn::Stop, behavior: Behavior::Idle }
      }
    }

    Dirn::Stop  => {
      if requests_here(es) {
        return DirnBehaviorPair { direction: Dirn::Stop, behavior: Behavior::DoorOpen }
      } else if requests_above(es){
        return DirnBehaviorPair { direction: Dirn::Up, behavior: Behavior::Moving }
      } else if requests_below(es){
        return DirnBehaviorPair { direction: Dirn::Down, behavior: Behavior::Moving }
      } else {
        return DirnBehaviorPair { direction: Dirn::Stop, behavior: Behavior::Idle }
      }
    }

    Dirn::Up  => {
      if requests_above(es) {
        return DirnBehaviorPair { direction: Dirn::Up, behavior: Behavior::Moving }
      } else if requests_here(es) {
        return DirnBehaviorPair { direction: Dirn::Down, behavior: Behavior::DoorOpen }
      } else if requests_below(es) {
        return DirnBehaviorPair { direction: Dirn::Down, behavior: Behavior::Moving }
      } else {
        return DirnBehaviorPair { direction: Dirn::Stop, behavior: Behavior::Idle }
      }
    }
  }
}

pub fn requests_should_stop(es: &ElevatorSystem) -> bool {
    match es.status.curr_dirn {
      Dirn::Down => {
        es.requests[es.status.curr_floor][ButtonType::HallDown as usize] || 
        es.requests[es.status.curr_floor][ButtonType::Cab as usize] || 
        !requests_below(es)
      }
      Dirn::Stop => {true}
      Dirn::Up => {
        es.requests[es.status.curr_floor][ButtonType::HallUp as usize] || 
        es.requests[es.status.curr_floor][ButtonType::Cab as usize] || 
        !requests_above(es)
      }
  }
}
pub fn requests_should_clear_immediately(es: &ElevatorSystem, btn_floor: usize, btn_type: ButtonType) -> bool {
  es.status.curr_floor as usize == btn_floor && (
      (es.status.curr_dirn as usize == Dirn::Up   as usize   && btn_type as usize == ButtonType::HallUp   as usize)  ||
      (es.status.curr_dirn as usize == Dirn::Down as usize   && btn_type as usize == ButtonType::HallDown as usize)  ||
      es.status.curr_dirn  as usize == Dirn::Stop as usize ||
      btn_type as usize == ButtonType::Cab as usize
  )
}

pub fn requests_clear_at_current_floor(es: &mut ElevatorSystem) {
  es.requests[es.status.curr_floor as usize][ButtonType::Cab as usize] = false;
  match es.status.curr_dirn {
    Dirn::Down => {
      if !requests_below(es) && !es.requests[es.status.curr_floor as usize][ButtonType::HallDown as usize] {
        es.requests[es.status.curr_floor as usize][ButtonType::HallUp as usize] = false;
      }
      es.requests[es.status.curr_floor as usize][ButtonType::HallDown as usize] = false;
    }
    
    Dirn::Up => {
      if !requests_above(es) && !es.requests[es.status.curr_floor as usize][ButtonType::HallUp as usize] {
        es.requests[es.status.curr_floor as usize][ButtonType::HallDown as usize] = false;
      }
      es.requests[es.status.curr_floor as usize][ButtonType::HallUp as usize] = false;
    }

    Dirn::Stop => {}
  }
}