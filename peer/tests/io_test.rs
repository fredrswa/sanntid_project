use peer::mod_io::hardware::init_elevator;

#[test]
fn test_init_elevator () {
    let e = match init_elevator(15666, 0, true) {
        Ok(e) => e,
        Err(e) => {
            panic!("Failed to init elevator: {}", e)
        }
    };
}



