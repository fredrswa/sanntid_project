use crossbeam_channel as cbc;

pub fn run_network(/* CHANNELS*/) {
    


    let (udp_recv_tx, udp_recv_rx) = cbc::unbounded::<u8>();
    let (udp_send_tx, udp_send_rx) = cbc::unbounded::<u8>();

    {
        let udp_recv = udp_recv_tx.clone();
        let udp_send = udp_send_rx.clone();
        spawn(move || udp::recv(udp_recv, udp_send));
        let udp_recv = udp_recv_tx.clone();
        let udp_send = udp_send_rx.clone();
        spawn(move || udp::send(udp_send_tx));
    }

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