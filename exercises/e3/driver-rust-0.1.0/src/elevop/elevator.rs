use crate::elevio::elev::*;
use super::timer::*;


fn eb_to_string(eb: ElevatorBehaviour) -> String{
  match eb {
    EbIdle => return "EB_Idle".to_string(),
    EbDoorOpen => return "EB_DoorOpen".to_string(),
    EbMoving => return "EB_Moving".to_string(),
    _ => return "EB_UNDEFINED".to_string()
  }
}

fn elevio_dirn_to_string(dirn: u8) -> String{
  match dirn {
    DIRN_UP => return "DIRN_UP".to_string(),
    DIRN_DOWN => return "DIRN_DOWN".to_string(),
    DIRN_STOP => return "DIRN_STOP".to_string(),
    _ => return "DIRN_UNDEFINED".to_string()
  }
}

fn elevator_print(e: Elevator){
  println!("  +--------------------+\n");
  println!(
      "|floor = {}| \n 
      |dirn  = {}|\n 
      |behav = {}|\n",
      e.floor,
      elevio_dirn_to_string(e.dirn),
      eb_to_string(e.behaviour)
  );
  println!("  +--------------------+\n");
  println!("  |  | up  | dn  | cab |\n");
  for f in e.num_floors-1..0{
      println!("  | {}", f);
      for btn in 1..3 {
          if (f == e.num_floors-1 && btn == HALL_UP)  || 
             (f == 0 && btn == HALL_DOWN) 
          {
              println!("|     ");
          } else {
              if e.requests[f][btn] {
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

fn elevator_uninitialized() -> Elevator {
  return Elevator {
    socket: Arc::new(Mutex::new(TcpStream::connect(addr)?)),
    num_floors: 4,
    floor: 0,
    dirn: DIRN_STOP,
    behaviour: EbIdle,
    requests: vec![vec![false; 3]; 8],
  };
}