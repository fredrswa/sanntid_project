
// ^ Driver
use driver_rust::elevio::elev::Elevator;

// ^ Crates
use crate::config::*;

// ^ mod_fsm
use super::requests::*;
use super::timer::Timer;

static NUM_FLOORS: i32 = CONFIG.elevator.num_floors as i32;
static NUM_BUTTONS: i32 = 3;

impl ElevatorSystem {
    pub fn new() -> ElevatorSystem {
        let elevator = match Elevator::init(CONFIG.elevator.addr, CONFIG.elevator.num_floors as u8) {
          Ok(elev) => elev,
          Err(_) => {
            panic!("mod_fsm-fsm: Could'nt connect to elevator");
          },
        };

        ElevatorSystem {          
          elevator,
          //Requests size is dictated at runtime, therefore it is a vector.
          requests: vec![vec![false; 4]; NUM_FLOORS as usize],
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
                self.elevator.call_button_light(floor as u8, 2, self.requests[floor as usize][2]);
            }
        }
    }

    pub fn set_all_lights_world_view(&mut self, world_view: &EntireSystem) {
      // Set hall request lights for all floors
      for (floor, hall_req) in world_view.hallRequests.iter().enumerate() {
          // Set HallUp button light (index 0)
          self.elevator.call_button_light(floor as u8, ButtonType::HallUp as u8, hall_req[0]);
          
          // Set HallDown button light (index 1)
          self.elevator.call_button_light(floor as u8, ButtonType::HallDown as u8, hall_req[1]);
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
                self.status.behavior = Behavior::DoorOpen; // Changed this from Idle to DoorOpen
              
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
                  Behavior::Idle => {
                    self.elevator.door_light(false);
                  
                  }  
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
            _=> {}
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
          Behavior::Idle => {
          
           // let db_pair: DirnBehaviorPair = requests_choose_direction(self);
            //self.status.curr_dirn = db_pair.direction;
            //self.status.behavior = db_pair.behavior;
      
            //requests_clear_at_current_floor(self);
            //self.set_all_lights();
            //self.elevator.door_light(false);

            println!("Stuck here Idle");
          }
          Behavior::Moving => {
          
          }
        }
    }      

  pub fn execute_new_requests(&mut self, timer: &mut Timer) {
    for floor in 0..self.elevator.num_floors {
      for button in 0..3 {
        if self.requests[floor as usize][button.clone()] {
          self.on_request_button_press(&mut *timer, floor as usize, call_to_button_type(button as u8));
        }
      }
    }
  }        
  pub fn update_requets(&mut self, new_hall_requets: Vec<Vec<bool>>) {
    for floor in 0..self.elevator.num_floors {
      self.requests[floor as usize][0] = new_hall_requets[floor as usize][0];
      self.requests[floor as usize][1] = new_hall_requets[floor as usize][1];
    }
  }
}