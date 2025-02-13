use std::io::Result;
use peer::mod_fsm::fsm;




fn main() -> Result<()> {
    fsm::test_script_elevator_system();
    Ok(())
}
