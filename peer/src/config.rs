





struct Config {
    NUM_FLOORS: u8,
    NUM_BUTTONS: u8,
    NUM_ELEVATORS: u8,
    DOOR_OPEN_S: u8,
}
impl Config {
    pub fn new() -> Config {
        Config {
            NUM_FLOORS: Config::read_from_toml("Config.toml")?,
            NUM_BUTTONS: Config::read_from_toml("Config.toml")?,
            NUM_ELEVATORS: Config::read_from_toml("Config.toml")?,
            DOOR_OPEN_S: Config::read_from_toml("Config.toml")?,
        }
    }
}
