use std::vec;
use std::time::{Duration, Instant};
use chrono::{Utc, DateTime};

use crate::config::*;


static NUM_FLOORS: i32 = CONFIG.elevator.num_floors as i32;
static NUM_BUTTONS: i32 = 3;

pub fn requests_above(es: &ElevatorSystem) -> bool {
  for floor in (es.status.curr_floor as usize + 1)..NUM_FLOORS as usize {
    for button in 0..NUM_BUTTONS as usize {
      if es.requests[floor][button] {
        return true;
      }
    }
  }
  return false;
}

pub fn requests_below(es: &ElevatorSystem) -> bool {
  for floor in 0..es.status.curr_floor as usize {
    for button in 0..NUM_BUTTONS as usize {
      if es.requests[floor][button] {
        return true;
      }
    }
  }
  return false;
}

pub fn requests_here(es: &ElevatorSystem) -> bool {
  for button in 0..NUM_BUTTONS as usize {
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

//Checks if to elevator systems are different. If different, an order has been cleared.
pub fn is_completed (elevator_before: ElevatorSystem, elevator_after: ElevatorSystem) -> Vec<Vec<bool>> {
  
  let mut completed_array = vec![vec![false; 2]; CONFIG.elevator.num_floors as usize];

  //Iterates through all fllor, checks if HallUp or HallDown has changed, if changed set true else false
  for floor in (elevator_before.requests.iter().zip(elevator_after.requests.iter())).enumerate() {
    completed_array[floor.0][0] = floor.1.0[0] != floor.1.1[0];
    completed_array[floor.0][1] = floor.1.0[1] != floor.1.1[1];  
  }

  return completed_array;
}

pub fn update_timestamps (completed_array: Vec<Vec<bool>>) -> Vec<Vec<(DateTime<Utc>, DateTime<Utc>)>> {

  let mut new_created_completed_timestamps: Vec<Vec<(DateTime<Utc>, DateTime<Utc>)>> = vec![vec![(Utc::now(), Utc::now()); 3]; CONFIG.elevator.num_floors as usize];
  
  for val in completed_array.iter().enumerate() {
    if val.1[0] == true {new_created_completed_timestamps[val.0][0] = (Utc::now()-Duration::from_secs(1), Utc::now());}
    if val.1[1] == true {new_created_completed_timestamps[val.0][1] = (Utc::now()-Duration::from_secs(1), Utc::now());}
  }

  return new_created_completed_timestamps;
}