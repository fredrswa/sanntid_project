use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::io::{Write, Read, Result};
use std::fmt;

use crate::elevator_operation::defs::*;

#[derive(Debug, Clone)]
pub struct Elevator {
  tcp_socket: Arc<Mutex<TcpStream>>,
  pub num_floors: usize,
  pub floor: usize,
  pub dirn: Dirn,
  pub requests: Vec<Vec<bool>>,
  pub behaviour: ElevatorBehaviour,
}

impl Elevator {
  pub fn init(address: &str, num_floors: usize, num_buttons: usize) -> Result<Elevator> {
    Ok(Self {
      tcp_socket: Arc::new(Mutex::new(TcpStream::connect(address)?)),
      num_floors: num_floors,
      floor: 0,
      dirn: Dirn::DirnStop,
      requests: vec![vec![false; num_buttons]; num_floors],
      behaviour: ElevatorBehaviour::Idle,
    })
  }

  pub fn motor_direction(&self, dirn: u8) {
    let buf = [1, dirn, 0, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
  }

  pub fn call_button_light(&self, floor: usize, call: u8, on: bool) {
    let buf = [2, call, floor as u8, on as u8];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
  }

  pub fn floor_indicator(&self, floor: u8) {
    let buf = [3, floor, 0, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
  }

  pub fn door_light(&self, on: bool) {
    let buf = [4, on as u8, 0, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
  }

  pub fn stop_button_light(&self, on: bool) {
    let buf = [5, on as u8, 0, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
  }

  pub fn call_button(&self, floor: u8, call: u8) -> bool {
    let mut buf = [6, call, floor, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&mut buf).unwrap();
    sock.read(&mut buf).unwrap();
    buf[1] != 0
  }

  pub fn floor_sensor(&self) -> Option<u8> {
    let mut buf = [7, 0, 0, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
    sock.read(&mut buf).unwrap();
    if buf[1] != 0 {
        Some(buf[2])
    } else {
        None
    }
  }

  pub fn stop_button(&self) -> bool {
    let mut buf = [8, 0, 0, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
    sock.read(&mut buf).unwrap();
    buf[1] != 0
  }

  pub fn obstruction(&self) -> bool {
    let mut buf = [9, 0, 0, 0];
    let mut sock = self.tcp_socket.lock().unwrap();
    sock.write(&buf).unwrap();
    sock.read(&mut buf).unwrap();
    buf[1] != 0
  }
}

impl fmt::Display for Elevator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      writeln!(f, "Elevator State:")?;
      writeln!(f, "  Current Floor: {}", self.floor)?;
      writeln!(f, "  Direction: {:?}", self.dirn)?;
      writeln!(f, "  Behaviour: {:?}\n", self.behaviour)?;

      // Print the requests matrix
      for (floor, row) in self.requests.iter().enumerate() {
          write!(f, "  Floor {}: [", floor)?;
          for (i, &request) in row.iter().enumerate() {
              if i > 0 {
                  write!(f, ", ")?;
              }
              write!(f, "{}", if request { "1" } else { "0" })?;
          }
          writeln!(f, "]")?;
      }

      Ok(())
  }
}