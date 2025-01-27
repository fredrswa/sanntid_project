use std::thread::*;
use std::time::*;

use crossbeam_channel as cbc;

use driver_rust::elevio;
use driver_rust::elevio::elev::Button;
use driver_rust::elevio::elev as e;
use driver_rust::elevop::fsm as fsm;

static NUM_FLOORS: usize = 4;
static NUM_BUTTONS: usize = 3;


//Main returns standard I/O Result
//T is of type () meaning no meaningful value
//Used mainly to enable the return of std::io::Error in case of failure
fn main() -> std::io::Result<()> {
    let elevator = e::Elevator::init("localhost:15657", NUM_FLOORS, NUM_BUTTONS)?; //Initialize new elevator using TCP
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

    fsm::fsm_on_init_between_floors(&mut elevator.clone());

    let mut button: e::Button;
    loop {
        cbc::select! {
            recv(call_button_rx) -> cb_message => {
                let call_button = cb_message.unwrap(); //Unwrap is used to get the value from the constant a
                println!("{:#?}", call_button);
                match call_button.call {
                    0 => button = e::Button::BHallup,
                    1 => button = e::Button::BHalldown,
                    _ => button = e::Button::BCab
                }
                fsm::fsm_on_request_button_press(&mut elevator.clone(), call_button.floor as usize, button);
            }
            recv(floor_sensor_rx) -> fs_message => {
                let floor = fs_message.unwrap();
                println!("Floor: {:#?}", floor);
                fsm::fsm_on_floor_arrival(&mut elevator.clone(), floor as usize);
            }
            recv(stop_button_rx) -> sb_message => {
                let stop = sb_message.unwrap();
                println!("Stop button: {:#?}", stop);

                
            }
            recv(obstruction_rx) -> ob_message => {
                let obstr = ob_message.unwrap();
                println!("Obstruction: {:#?}", obstr);
                elevator.motor_direction(e::DIRN_STOP);
            }


        }
        fsm::set_all_lights(&mut elevator.clone());
        fsm::fsm_on_door_timeout(&mut elevator.clone());
    }


    /* loop { //Loop that listens to the different channels
        cbc::select! { //Select is used to listen to multiple channels at the same time
            recv(call_button_rx) -> a => { //a is a constant that is used to store the value that is received from the channel
                let call_button = a.unwrap(); //Unwrap is used to get the value from the constant a
                println!("{:#?}", call_button);
                elevator.call_button_light(call_button.floor as usize, call_button.call, true); //Turns on the light of the call button that is pressed
            },
            recv(floor_sensor_rx) -> a => {
                let floor = a.unwrap();
                println!("Floor: {:#?}", floor);
                dirn =
                    if floor == 0 {
                        e::DIRN_UP
                    } else if floor == (elevator.num_floors as u8 - 1) {
                        e::DIRN_DOWN
                    } else {
                        dirn
                    };
                elevator.motor_direction(dirn);
            },
            recv(stop_button_rx) -> a => {
                let stop = a.unwrap();
                println!("Stop button: {:#?}", stop);
                for f in 0..elevator.num_floors {
                    for c in 0..3 {
                        elevator.call_button_light(f as usize, c, false);
                    }
                }
            },
            recv(obstruction_rx) -> a => {
                let obstr = a.unwrap();
                println!("Obstruction: {:#?}", obstr);
                elevator.motor_direction(if obstr { e::DIRN_STOP } else { dirn });
            },
        }
    } */
}
