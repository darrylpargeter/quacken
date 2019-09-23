use toml;
use std::fs;
use toml::Value;

pub fn get_config() -> toml::Value {
    let file_name: &str = "./config.toml";

    // get the contents from config file
    let file_contents = fs::read_to_string(file_name)
    .expect("Something went wrong reading the file");

    // parse the config file
    let config = file_contents.parse::<Value>().unwrap();

    config
}
