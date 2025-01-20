use std::sync::Mutex; //Mutex is used to lock the variable so that only one thread can access it at a time
use std::time::{Duration, Instant}; //Duration is used to specify the time and Instant is used to get the current time
//må legge til dette i Cargo.toml for å bruke lazy_static
//[dependencies]
//lazy_static = "1.4"


// static variables for the timer
lazy_static::lazy_static! {
    static ref TIMER_END_TIME: Mutex<Option<Instant>> = Mutex::new(None);
    static ref TIMER_ACTIVE: Mutex<bool> = Mutex::new(false);
}

// Funksjon for å starte timeren
pub fn timer_start(duration: f64) {
    let duration_instant = Instant::now() + Duration::from_secs_f64(duration);
    let mut end_time = TIMER_END_TIME.lock().unwrap();
    *end_time = Some(duration_instant);

    let mut active = TIMER_ACTIVE.lock().unwrap();
    *active = true;
}

// Funksjon for å stoppe timeren
pub fn timer_stop() {
    let mut active = TIMER_ACTIVE.lock().unwrap();
    *active = false;
}

// Funksjon for å sjekke om timeren har gått ut
pub fn timer_timed_out() -> bool {
    let active = TIMER_ACTIVE.lock().unwrap();
    if !*active {
        return false;
    }

    let end_time = TIMER_END_TIME.lock().unwrap();
    if let Some(end) = *end_time {
        return Instant::now() > end;
    }

    false
}
