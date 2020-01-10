use toml;
use std::fs;
use toml::Value;
use dirs;

// for file reading
use std::fs::File;
use std::io::prelude::*;
use regex::{Regex, NoExpand};
use std::path::PathBuf;

const DELIMT_START: &str = "### quacken shortcuts :start ###\n";
const DELIMT_END: &str = "### quacken shortcuts :end ###\n";
const CONFIG_LOCATION: &str = "quacken/config.toml";
const HOST_LOCATION: &str = "/etc/hosts";

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push(CONFIG_LOCATION);

    path
}

pub fn get_config() -> toml::Value {
    // get the config dir
    let file_name = get_config_path();

    // get the contents from config file
    let file_contents = fs::read_to_string(&file_name)
    .expect("Something went wrong reading the file");

    // parse the config file
    let config = file_contents.parse::<Value>().unwrap();

    config
}

pub fn update_config() {
    let config = get_config();
    let table = config.as_table().unwrap();
    let mut replacemnt = String::new();

    // generate payload
    replacemnt.push_str(DELIMT_START);
    for key in table.keys() {
        replacemnt.push_str(format!("127.0.0.1 {}\n", key).as_str());
    }
    replacemnt.push_str(DELIMT_END);

    let mut buffer = String::new();
    let mut hosts = File::open(HOST_LOCATION)
        .expect("File not found or cannot be opened");

    // read the host file into a buffer
    hosts.read_to_string(&mut buffer)
        .unwrap();

    // build the regex
    let re = Regex::new(DELIMT_START).unwrap();
    let mut updated_content: String = String::new();

    // check to see host file has quacken config
    // if not add it
    if !re.is_match(buffer.as_str()) {
        println!("adding quacken to hosts file");
        let mut data = buffer.to_owned();
        let updated_host = replacemnt.to_owned();
        data.push_str(&updated_host);
        updated_content = data;
    } else {
        // get the correct area of the hosts file and update it with
        // the latest keys
        println!("updating hosts file");
        let re = Regex::new(r"(###(.*)##)\n(.|\n)*(###(.*)##\n)").unwrap();
        updated_content = re.replace_all(buffer.as_str(), NoExpand(replacemnt.as_str())).into_owned();
    }

    // write the content to the file
    fs::write(HOST_LOCATION, updated_content).expect("Unable to write file");
}
