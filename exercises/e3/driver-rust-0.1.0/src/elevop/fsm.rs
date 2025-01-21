use crate::elevio::elev::*;
use super::elevator::*;
use super::request::*;
use super::timer::*;



fn set_all_lights(e: &Elevator) {
  for f in 0..e.num_floors {
    for btn in 0..3 {
      //outputDevice.requestButtonLight(floor, btn, es.requests[floor][btn]);
    }
  }
}


fn on_init_between_floors(){
  //outputDevice.motorDirection(D_Down);
  //elevator.dirn = D_Down;
  //elevator.behaviour = EB_Moving;
}

fn fsm_on_request_button_press(btn_floor: usize, btn_type: Button, e: &mut Elevator) {
  
}