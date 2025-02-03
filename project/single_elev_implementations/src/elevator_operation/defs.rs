use std::fmt;

pub const NUM_FLOORS: usize = 4;
pub const NUM_BUTTONS: usize = 3;
pub const DOOR_OPEN_S: u64 = 3;

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonType{
  HallUp,
  HallDown,
  Cab,
}

impl ButtonType {
  pub fn from_u8(value: u8) -> Option<Self> {
      match value {
          0 => Some(ButtonType::HallUp),
          1 => Some(ButtonType::HallDown),
          2 => Some(ButtonType::Cab),
          _ => None,
      }
  }
}

impl fmt::Debug for ButtonType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ButtonType::HallUp => write!(f, "Hall -> Up"),
      ButtonType::HallDown => write!(f, "Hall -> Down"),
      ButtonType::Cab => write!(f, "Cab"),
    }
  }
}

#[derive(PartialEq, Clone)]
pub enum ElevatorBehaviour{
  Idle,
  DoorOpen,
  Moving,
}

impl fmt::Debug for ElevatorBehaviour {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ElevatorBehaviour::Idle => write!(f, "Idle"),
      ElevatorBehaviour::DoorOpen => write!(f, "Door Open"),
      ElevatorBehaviour::Moving => write!(f, "Moving"),
    }
  }
}

#[derive(PartialEq, Clone)]
pub enum Dirn{
  DirnDown = 255,
  DirnStop = 0,
  DirnUp = 1,
}

impl fmt::Debug for Dirn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Dirn::DirnDown => write!(f, "Dirn -> Down"),
      Dirn::DirnStop => write!(f, "Dirn -> Stop"),
      Dirn::DirnUp => write!(f, "Dirn -> Up"),
    }
  }
}

pub struct DirnBehaviourPair {
  pub dirn: Dirn,
  pub behaviour: ElevatorBehaviour,
}