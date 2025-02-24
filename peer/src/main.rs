#[allow(dead_code)]
use std::io::Result;
use peer::mod_fsm::fsm;
use peer::mod_network::network;
use peer::mod_assigner::json_testing;





fn main() -> Result<()> {
    //fsm::test_script_elevator_system();
    //network::test_script_network_module();
    json_testing::test_script_json();
    Ok(())

}
