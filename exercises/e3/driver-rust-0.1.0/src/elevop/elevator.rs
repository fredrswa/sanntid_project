#![allow(dead_code)]

use crate::elevio::elev::*;

pub fn eb_to_string(eb: ElevatorBehaviour) -> String{
  match eb {
    EB_IDLE => return "EB_IDLE".to_string(),
    EB_DOOROPEN => return "EB_DOOROPEN".to_string(),
    EB_MOVING => return "EB_MOVING".to_string(),
  }
}

pub fn elevio_dirn_to_string(dirn: u8) -> String{
  match dirn {
    DIRN_UP => return "DIRN_UP".to_string(),
    DIRN_DOWN => return "DIRN_DOWN".to_string(),
    DIRN_STOP => return "DIRN_STOP".to_string(),
    _ => return "DIRN_UNDEFINED".to_string()
  }
}

pub fn elevio_button_to_string(btn: Button) -> String{
  match btn {
    Button::BHallup => return "BHALLUP".to_string(),
    Button::BHalldown => return "BHALLDOWN".to_string(),
    Button::BCab => return "BCAB".to_string(),
  }
}

pub fn elevator_print(elevator: &mut Elevator){
  println!("  +--------------------+\n");
  println!(
      "|floor = {}| \n 
      |dirn  = {}|\n 
      |behav = {}|\n",
      elevator.floor,
      elevio_dirn_to_string(elevator.dirn.clone()),
      eb_to_string(elevator.behaviour.clone())
  );
  println!("  +--------------------+\n");
  println!("  |  | up  | dn  | cab |\n");
  for floor in elevator.num_floors-1..0{
      println!("  | {}", floor);
      for btn in 1..3 {
          if (floor == elevator.num_floors-1 && btn == HALL_UP as usize)  || 
             (floor == 0 && btn == HALL_DOWN as usize) 
          {
              println!("|     ");
          } else {
              if elevator.requests[floor][btn] {
                println!("|  #  ")
              } else {
                println!("|  -  ")
              };
          }
      }
      println!("|\n");
  }
  println!("  +--------------------+\n");
}


//CREATING UNINITIALIZED STRUCT IS STRONGLY DISCOURAGED IN RUST!
/* pub fn elevator_uninitialized() -> Elevator {
  return Elevator {
    socket: Arc::new(Mutex::new(TcpStream::connect(addr)?)),
    num_floors: 4,
    floor: 0,
    dirn: DIRN_STOP,
    requests: vec![vec![false; 3]; 8],
    behaviour: ElevatorBehaviour::EbIdle,
  };
} */