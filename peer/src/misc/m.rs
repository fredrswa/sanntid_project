use crossbeam_channel as cbc;

mod modules;

pub struct FSM_Channels {
    pub timeout_tx: cbc::Sender<()>,
    pub io_new_order_rx: cbc::Sender<sensor_polling::CallButton>,

}
pub struct FSM{
    pub channels: FSM_Channels,
    pub es: ElevatorSystem,
}
pub struct Network_Channels {
    pub timeout_tx: cbc::Sender<()>,
    //pub io_network_new_order_rx: cbc::Receiver<sensor_polling::CallButton>,
    //pub network_io_distribute_order_tx: cbc::Sender<sensor_polling::CallButton>,
}