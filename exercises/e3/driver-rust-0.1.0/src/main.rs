use std::thread::*;
use std::time::*;

use crossbeam_channel as cbc;

use driver_rust::elevio;
use driver_rust::elevio::elev as e;

//Main returns standard I/O Result
//T is of type () meaning no meaningful value
//Used mainly to enable the return of std::io::Error in case of failure
fn main() -> std::io::Result<()> {
    let elev_num_floors = 4; // Number of floors in elevator
    let elevator = e::Elevator::init("localhost:15657", elev_num_floors)?; //Initialize new elevator using TCP
    println!("Elevator started:\n{:#?}", elevator);

    let poll_period = Duration::from_millis(25); // polling period, time between each poll

    //Spawns a thread for each polling function
    //cbc::unbounded is a channel that can be used to send and receive messages (unbounded means no limit)
    //Functions mostly the same as channels in go
    let (call_button_tx, call_button_rx) = cbc::unbounded::<elevio::poll::CallButton>(); 
    {
        let elevator = elevator.clone(); //Creates new instance of the elevator in order to pass ownership to the thread
        //Thread loop over pulling function, then sleeps for the poll period
        spawn(move || elevio::poll::call_buttons(elevator, call_button_tx, poll_period)); //Spawn is being used to create a new thread
    } //This is the call button thread

    let (floor_sensor_tx, floor_sensor_rx) = cbc::unbounded::<u8>();
    {
        let elevator = elevator.clone(); 
        spawn(move || elevio::poll::floor_sensor(elevator, floor_sensor_tx, poll_period));
    }

    let (stop_button_tx, stop_button_rx) = cbc::unbounded::<bool>();
    {
        let elevator = elevator.clone();
        spawn(move || elevio::poll::stop_button(elevator, stop_button_tx, poll_period));
    }

    let (obstruction_tx, obstruction_rx) = cbc::unbounded::<bool>();
    {
        let elevator = elevator.clone();
        spawn(move || elevio::poll::obstruction(elevator, obstruction_tx, poll_period));
    }
    
    let mut dirn = e::DIRN_DOWN; 
    if elevator.floor_sensor().is_none() { // if the floor sensor is not working, the elevator will go down
        elevator.motor_direction(dirn); // the elevator will go down
    }

    loop { //Loop that listens to the different channels
        cbc::select! { //Select is used to listen to multiple channels at the same time
            recv(call_button_rx) -> a => { //a is a constant that is used to store the value that is received from the channel
                let call_button = a.unwrap(); //Unwrap is used to get the value from the constant a
                println!("{:#?}", call_button);
                elevator.call_button_light(call_button.floor, call_button.call, true); //Turns on the light of the call button that is pressed
            },
            recv(floor_sensor_rx) -> a => {
                let floor = a.unwrap();
                println!("Floor: {:#?}", floor);
                dirn =
                    if floor == 0 {
                        e::DIRN_UP
                    } else if floor == (elev_num_floors as u8 - 1) {
                        e::DIRN_DOWN
                    } else {
                        dirn
                    };
                elevator.motor_direction(dirn);
            },
            recv(stop_button_rx) -> a => {
                let stop = a.unwrap();
                println!("Stop button: {:#?}", stop);
                for f in 0..elev_num_floors {
                    for c in 0..3 {
                        elevator.call_button_light(f as u8, c, false);
                    }
                }
            },
            recv(obstruction_rx) -> a => {
                let obstr = a.unwrap();
                println!("Obstruction: {:#?}", obstr);
                elevator.motor_direction(if obstr { e::DIRN_STOP } else { dirn });
            },
        }
    }
}
