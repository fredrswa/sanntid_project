const static N_FLOORS | u8 = 4;
const static N_BUTTONS | u8 = 3;

pub enum DIRN {
    DIRN_UP = 1,
    DIRN_STOP = 0,
    DIRN_DOWN = -1,
}

pub enum BUTTON{
    B_HallUp,
    B_HallDown,
    B_Cab
}

