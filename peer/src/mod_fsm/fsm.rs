#![allow(dead_code)]
// ^ Packages
use std::thread::{spawn,sleep};
use std::time::Duration;
use std::fmt;
use core::panic;
use crossbeam_channel as cbc;

// ^ Driver
use driver_rust::elevio::elev::Elevator;
use driver_rust::elevio::poll as sensor_polling;

// ^ mod_fsm
use super::config::*;
use super::requests::*;
use super::timer::Timer;



#[derive(Clone)]
pub struct ElevatorSystem {
    pub elevator: Elevator,
    pub requests: [[bool; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
    pub status: Status,
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

impl ElevatorSystem {
    pub fn new(addr: &str) -> ElevatorSystem {
        ElevatorSystem {
            elevator: Elevator::init(addr, NUM_FLOORS as u8).unwrap(),
            requests: [[false; NUM_BUTTONS as usize]; NUM_FLOORS as usize],
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
        for floor in 0..NUM_FLOORS {
            for btn in 0..NUM_BUTTONS {
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







pub fn test_script_elevator_system() {
    let addr = "localhost:15657";
    let mut es = ElevatorSystem::new(addr);
    let mut timer = Timer::new(Duration::from_secs(DOOR_OPEN_S));

    let poll_period = Duration::from_millis(25);

    let (call_button_tx, call_button_rx) = cbc::unbounded::<sensor_polling::CallButton>(); 
    let (floor_sensor_tx, floor_sensor_rx) = cbc::unbounded::<u8>(); 
    let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>(); 
    {
        let elevator = es.elevator.clone();
        spawn(move || sensor_polling::call_buttons(elevator, call_button_tx, poll_period)); 
        let elevator = es.elevator.clone();
        spawn(move || sensor_polling::floor_sensor(elevator, floor_sensor_tx, poll_period)); 
        let elevator = es.elevator.clone();
        spawn(move || sensor_polling::obstruction(elevator, obstruction_tx, poll_period)); 
    }
    es.init();
    loop {
        cbc::select! {
            recv(call_button_rx) -> cb_message => {
                if let Ok(call_button) = cb_message {
                    println!("{}", &es);
                    es.on_request_button_press(&mut timer, call_button.floor as usize, call_to_button_type(call_button.call));
                }
            }

            recv(floor_sensor_rx) -> fs_message => {
                if let Ok(floor) = fs_message {
                    es.on_floor_arrival(&mut timer, floor as usize);
                }
            }

            recv(obstruction_rx) -> ob_message => {
                if let Ok(obs) = ob_message {
                    if !obs {
                        timer.start();
                    }
                    es.status.door_blocked = obs;
                }
            }
            default => {sleep(poll_period);}
        }

        if timer.is_expired() && !es.status.door_blocked {
            es.on_door_timeout(&mut timer);
        }
    }
}