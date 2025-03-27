use std::vec;
use chrono::{naive::serde::ts_microseconds::deserialize, Utc};
use std::fs::File;
use std::io::prelude::*;

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

    Dirn::Stop => {
      if es.requests[es.status.curr_floor as usize][ButtonType::HallDown as usize] {
        es.requests[es.status.curr_floor as usize][ButtonType::HallDown as usize] = false;
        println!("CLEARED DOWN");
      } else if es.requests[es.status.curr_floor as usize][ButtonType::HallUp as usize] {
        es.requests[es.status.curr_floor as usize][ButtonType::HallUp as usize] = false;
        println!("CLEARED UP");
      }
    }
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

pub fn update_timestamps (completed_array: Vec<Vec<bool>>, created_completed_timestamps: Vec<Vec<(i64, i64)>>) -> Vec<Vec<(i64, i64)>> {

  let mut new_created_completed_timestamps: Vec<Vec<(i64, i64)>> = created_completed_timestamps;
  
  for val in completed_array.iter().enumerate() {
    if val.1[0] == true {new_created_completed_timestamps[val.0][0] = (Utc::now().timestamp_millis()-1000, Utc::now().timestamp_millis());}
    if val.1[1] == true {new_created_completed_timestamps[val.0][1] = (Utc::now().timestamp_millis()-1000, Utc::now().timestamp_millis());}
  }

  return new_created_completed_timestamps;
}

pub fn cab_backup (cab_requests: Vec<bool>) {
  	let cab_recovery = Recovery { cab_requests };
    
  	let toml_string = toml::to_string(&cab_recovery).expect("Failed to serialize into TOML string");
    
  	let mut file = File::create("cab_recover.toml").expect("Failed to create TOML file");
  	file.write_all(toml_string.as_bytes()).expect("Failed to write to TOML file");
}

pub fn read_cab_backup () -> Vec<bool> {
	// Example of reading it back
	let mut file = File::open("cab_recover.toml").expect("Failed to open TOML file");
	let mut toml_content = String::new();
	file.read_to_string(&mut toml_content).expect("Failed to read content of TOML file into string");

	let cab_recovery: Recovery = toml::from_str(&toml_content).expect("Failed to deserialize from TOML into struct");

	return cab_recovery.cab_requests;
}