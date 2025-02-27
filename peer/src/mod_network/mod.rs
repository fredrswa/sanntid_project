use crossbeam_channel as cbc;

pub fn run_network(/* CHANNELS*/) {
    return Ok(());

    // * Starter receive (udp_recv_rx, udp_recv_tx) <String>

    // * Starte heartbeat () evt flytt til watchdog

    loop {
        cbc::select! {
            recv(udp_recv_rx) {
                //check om heartbeat eller state

                if( heartbeat) {
                    //update heartbeat timer
                }
                if (state) {

                }
            }
            // ! Vente på IO


            // ! Vente på receive
        }


        // ! Sende
    }
}