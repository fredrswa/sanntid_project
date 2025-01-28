use std::sync::{Arc, Mutex};

const N_FLOORS: u8 = 4;
const N_BUTTONS: u8 = 3;

#[derive(Clone, Debug, Copy, PartialEq)]
enum DIRN {
    DOWN = -1,
    STOP = 0,
    UP = 1,
}
#[derive(Clone, Debug, Copy, PartialEq)]
enum BUTTON {
    HALL_UP = 0,
    HALL_DOWN = 1,
    CAB = 2,
}
#[derive(Clone, Debug, Copy, PartialEq)]
enum ELEV_BEHEVIOUR {
    IDLE,
    DOOR_OPEN,
    MOVING,
}
#[derive(Clone, Debug, Copy, PartialEq)]
enum CLEAR_REQ_VARIANT {
    CRV_ALL,
    CRV_IN_DIRN,
}
#[derive(Debug)]
struct ELEVATOR {
    floor: i32m
    dirn: DIRN,
    behaviour: ELEV_BEHEVIOUR::EB_IDLE,
    requests [[bool; N_BUTTONS]; N_FLOORS],
    behaviour: ELEV_BEHEVIOUR,
    config: ELEV_CONFIG,
}
#[derive(Clone, Debug)]
struct ELEV_CONFIG {
    clear_request_variant: CLEAR_REQ_VARIANT,
    door_open_duration_sec: f64,
}

impl Elevator {
    fn new() -> self {
        Elevator {
            floor: -1,
            dirn: DIRN::STOP,
            behaviour: ELEV_BEHEVIOUR::IDLE,
            requests: [[false; N_BUTTONS]; N_FLOORS],
            config: ELEV_CONFIG {
                clear_request_variant: CLEAR_REQ_VARIANT::CRV_ALL,
                door_open_duration_sec: 3.0,
            },
        }
    }
    fn print_state(&self) {
        println!("+-----------------+");
        println!("|  floor     = {:<2} |", self.floor);
        println!("|  dirn      = {:<2} |", self.dirn);
        println!("|  behaviour = {:<2} |", self.behaviour);
        println!("+-----------------+");
        println!("|   | UP  | DOWN | CAB  |");
        for f in (0..N_FLOORS).rev() {
            print!("| {}", f);
            for btn in 0..N_BUTTONS {
                print!("|  {}  ", if self.requests[f][btn] { "#" } else { "-" });
            }
            println!("|");
        }
        println!("+--------------------+");
    }
}

struct ElevatorSystem;

impl ElevatorSystem {
    pub fn init(addr: &str, num_floors: u8) -> Result<Elevator> {
        Ok(Self {
            socket: Arc::new(Mutex::new(TcpStream::connect(addr)?)),
            num_floors,
        })
    }
    pub fn motor_direction(&self, dirn: u8) {
        let buf = [1, dirn, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }
    pub fn call_button_light(&self, floor: u8, call: u8, on: bool) {
        let buf = [2, call, floor, on as u8];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }
    pub fn floor_indicator(&self, floor: u8) {
        let buf = [3, floor, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }
    pub fn door_light(&self, on: bool) {
        let buf = [4, on as u8, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }
    pub fn stop_button_light(&self, on: bool) {
        let buf = [5, on as u8, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }
    pub fn call_button(&self, floor: u8, call: u8) -> bool {
        let mut buf = [6, call, floor, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&mut buf).unwrap();
        sock.read(&mut buf).unwrap();
        buf[1] != 0
    }
    pub fn floor_sensor(&self) -> Option<u8> {
        let mut buf = [7, 0, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&mut buf).unwrap();
        sock.read(&mut buf).unwrap();
        if buf[1] == 0 {
            None
        } else {
            Some(buf[1])
        }
    }
    pub fn stop_button(&self) -> bool {
        let mut buf = [8, 0, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&mut buf).unwrap();
        sock.read(&mut buf).unwrap();
        buf[1] != 0
    }
    pub fn obstruction(&self) -> bool {
        let mut buf = [9, 0, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&mut buf).unwrap();
        sock.read(&mut buf).unwrap();
        buf[1] != 0
    }
}

impl fmt::Display for Elevator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = self.socket.lock().unwrap().peer_addr().unwrap();
    }
}