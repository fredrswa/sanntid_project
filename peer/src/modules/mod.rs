


pub fn run() {
    let (io_call_tx,io_call_rx) = cbc::unbounded::<sensor_polling::CallButton>();
    let addr = "localhost:15657";
    let es: ElevatorSystem = ElevatorSystem::new(&addr);
    
    let mut es1 = es.clone();
    thread::spawn(move || {mod_fsm::run(&mut es1, &io_call_rx);});
    let mut es2 = es.clone();
    thread::spawn(move || {mod_io::run(&mut es2, &call_from_io_tx);});

    loop {
        cbc::select! {
            recv(timout_rx) -> _ => {
                //trigger recovery, warn network.
            }
        }

    }
    Ok(())
}