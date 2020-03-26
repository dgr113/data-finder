use std::fs;
use serde_yaml;



/// Load config ///
pub fn load_config(path: &str) -> serde_yaml::Value {
    let config_str = fs::read_to_string(path).expect("Not config file!");
    let config = serde_yaml::from_str(&config_str).unwrap();
    config
}
