#![allow(dead_code)]

use std::sync::Mutex; //Mutex is used to lock the variable so that only one thread can access it at a time
use std::time::{Duration, Instant}; // Duration is used to specify the time, Instant is used to get the current time
use lazy_static::lazy_static; // handle static variables safely
// lazy_static gjør så variabelen ikke blir initialisert før den blir brukt for første gang
//nødvendig ved bruk av tråder

//må legge til dette i Cargo.toml for å bruke lazy_static
//[dependencies]
//lazy_static = "1.4"//må legge til dette i Cargo.toml for å bruke lazy_static


// Static variables for the timer
lazy_static! {
    static ref TIMER_END_TIME: Mutex<Option<Instant>> = Mutex::new(None); //instant represents a  spesific point in time
    static ref TIMER_ACTIVE: Mutex<bool> = Mutex::new(false);
}

// starts the timer with the given duration
// if the timer is already running, it will be reset
pub fn timer_start(duration: f64) { //duration is the time the timer will run
    let end_time = Instant::now() + Duration::from_secs_f64(duration);

    {
        let mut end_time_lock = TIMER_END_TIME.lock().expect("could not lock the TIMER_END_TIME"); //locks the TIMER_END_TIME, exeption if it fails
        *end_time_lock = Some(end_time); //end_time_lock lockal variable
    }

    {
        let mut active_lock = TIMER_ACTIVE.lock().expect("Could not lock TIMER_ACTIVE"); //locks the TIMER_ACTIVE, exeption if it fails
        *active_lock = true;
    }
}

// stops the timer by setting the active flag to false
pub fn timer_stop() {
    let mut active_lock = TIMER_ACTIVE.lock().expect("Could not lock TIMER_ACTIVE");
    *active_lock = false;
}

// check if the timer has timed out
pub fn timer_timed_out() -> bool {
    let active = {
        let active_lock = TIMER_ACTIVE.lock().expect("Could not lock TIMER_ACTIVE");
        *active_lock
    };

    if !active {
        return false;
    }

    let end_time = {
        let end_time_lock = TIMER_END_TIME.lock().expect("Could not lock TIMER_END_TIME");
        *end_time_lock
    };

    if let Some(end) = end_time { //some is used to check the value inside the option-value. some in either current time or none
        return Instant::now() > end;
    }

    false
}

// resets the timer by setting the end time to None and the active flag to false'
pub fn timer_reset() {
    {
        let mut end_time_lock = TIMER_END_TIME.lock().expect("Could not lock TIMER_END_TIME");
        *end_time_lock = None;
    }

    {
        let mut active_lock = TIMER_ACTIVE.lock().expect("Could not lock TIMER_ACTIVE");
        *active_lock = false;
    }
}

//Hvordan bruke og kalle på timeren

// fn main() {
//     // Start timeren med en varighet på 5 sekunder
//     timer_start(5.0);
//     println!("Timer started for 5 seconds.");

//     // Loop som sjekker om timeren har utløpt
//     loop {
//         if timer_timed_out() {
//             println!("Timer has timed out!");
//             break;
//         } else {
//             println!("Timer is still running...");
//         }
//         // Vent litt før neste sjekk (for å unngå å bruke CPU unødvendig)
//         thread::sleep(Duration::from_secs(1));
//     }

//     // Stopp timeren eksplisitt (valgfritt her siden den allerede har utløpt)
//     timer_stop();
//     println!("Timer stopped.");
// }