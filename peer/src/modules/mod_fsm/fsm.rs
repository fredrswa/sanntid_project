
use std::fmt;
// ^ Driver
use driver_rust::elevio::elev::Elevator;

// ^ Crates
use crate::config::Config;

// ^ mod_fsm
use super::setup::*;
use super::requests::*;
use super::timer::Timer;



#[derive(Clone)]
pub struct ElevatorSystem {
    pub elevator: Elevator,
    pub requests: Vec<Vec<bool>>,
    pub status: Status,

    
    pub num_floors: usize,
    pub num_buttons: usize,
    pub door_open_s: usize,
    pub addr: String,

}


impl ElevatorSystem {
    pub fn new() -> ElevatorSystem {
        let config = Config::import();
        ElevatorSystem {
          //Constants Read from Config file
          num_floors: config.num_floors,
          num_buttons: config.num_buttons,
          door_open_s: config.door_open_s,
          addr: config.elev_addr.clone(),

          elevator: match Elevator::init(&config.elev_addr, config.num_floors as u8) {
            Ok(e) => e,
            Err(e) => {panic!("Cannot start without elevator connection");},
          },
          //Requests size is dictated at runtime, therefore it is a vector.
          requests: vec![vec![false; config.num_buttons as usize]; config.num_floors as usize],
          status: Status::new(),
        }
    }
    
    pub fn init(&mut self) {
        self.status.curr_floor = self.elevator.floor_sensor().unwrap_or(u8::MAX) as usize;
        println!("Floor {}", self.status.curr_floor);
        match self.status.curr_floor {
            255 => {
                println!("Started Between Floors");
                self.init_between_floors();
            }
            _ => {
                println!("Started At Floor");
                self.elevator.motor_direction(Dirn::Stop as u8);
                self.status.curr_dirn = Dirn::Stop;
                self.status.behavior = Behavior::Moving;
            }
        } 
    }

    pub fn init_between_floors(&mut self) {
        self.elevator.motor_direction(Dirn::Down as u8);
        self.status.curr_dirn = Dirn::Down;
        self.status.behavior = Behavior::Moving;
    }

    pub fn set_all_lights(&mut self){
        for floor in 0..self.num_floors {
            for btn in 0..self.num_buttons{
                self.elevator.call_button_light(floor as u8, btn as u8, self.requests[floor as usize][btn as usize]);
            }
        }
    }
    pub fn on_request_button_press(&mut self, timer: &mut Timer, btn_floor: usize, btn_type: ButtonType) {
        match self.status.behavior {
            Behavior::DoorOpen => {
              if self.status.door_blocked {
                timer.start();
              }
        
              if requests_should_clear_immediately(self, btn_floor, btn_type) {
                self.elevator.door_light(true);
                requests_clear_at_current_floor(self);
                timer.start();
              } else {
                self.requests[btn_floor][btn_type as usize] = true;
              }
            }
            Behavior::Moving => {
              self.requests[btn_floor][btn_type as usize] = true;
            }
            Behavior::Idle => {
              if requests_should_clear_immediately(self, btn_floor, btn_type) {
                self.elevator.door_light(true);
                requests_clear_at_current_floor(self);
                timer.start();
              } else {
                self.requests[btn_floor][btn_type as usize] = true;
                let db_pair: DirnBehaviorPair = requests_choose_direction(self);
                self.status.curr_dirn = db_pair.direction;
                self.status.behavior = db_pair.behavior;
                match self.status.behavior {
                  Behavior::DoorOpen => {
                    self.elevator.door_light(true);
                    timer.start();
                    requests_clear_at_current_floor(self);
                  }
                  Behavior::Moving => {
                    self.elevator.motor_direction(self.status.curr_dirn.clone() as u8);
                  }
                  Behavior::Idle => {}  
                }
              }
            }
          }
          self.set_all_lights();
    }


    pub fn on_floor_arrival(&mut self, timer: &mut Timer, new_floor: usize) {
        self.status.curr_floor = new_floor;
        self.elevator.floor_indicator(self.status.curr_floor as u8);

        match self.status.behavior {
            Behavior::Moving => {
                if requests_should_stop(self) {
                    self.elevator.motor_direction(Dirn::Stop as u8);
                    self.elevator.door_light(true);
                    requests_clear_at_current_floor(self);
                    timer.start();
                    self.set_all_lights();
                    self.status.behavior = Behavior::DoorOpen;
                }
            }
            _=> { }
        }
    }
    pub fn on_door_timeout(&mut self, timer: &mut Timer) {
        match self.status.behavior {
          Behavior::DoorOpen => {
            if self.status.door_blocked {
              timer.start();
            } else {
              let db_pair: DirnBehaviorPair = requests_choose_direction(self);
              self.status.curr_dirn = db_pair.direction;
              self.status.behavior = db_pair.behavior;
      
              requests_clear_at_current_floor(self);
              self.set_all_lights();
              
              match self.status.behavior {
                Behavior::DoorOpen => {
                  timer.start();
                }
                Behavior::Moving => {
                  self.elevator.door_light(false);
                  self.elevator.motor_direction(self.status.curr_dirn.clone() as u8);
                }
                Behavior::Idle => {
                  self.elevator.door_light(false);
                }
              }
            }
          }
          _=> {}
        }
      }
}





impl fmt::Display for ElevatorSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       
        write!(f, "     ")?;
        println!("\tHU, \tHD, \tCab");
        
        write!(f, "\n")?;
        for (floor, row) in self.requests.iter().enumerate().rev() {
            write!(f, "F{}\t: ", floor + 1)?;
            for &request in row.iter() {
            write!(f, "{}\t ", if request { "X" } else { " " })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


