use toml;
use std::fs;
use toml::Value;

// for file reading
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::{Regex, NoExpand};

const DELIMT_START: &str = "### quacken shortcuts :start ###\n";
const DELIMT_END: &str = "### quacken shortcuts :end ###\n";

pub fn get_config() -> toml::Value {
    let file_name: &str = "./config.toml";

    // get the contents from config file
    let file_contents = fs::read_to_string(file_name)
    .expect("Something went wrong reading the file");

    // parse the config file
    let config = file_contents.parse::<Value>().unwrap();

    config
}

pub fn update_config() {
    let path = "/etc/hosts";
    let config = get_config();
    let table = config.as_table().unwrap();
    let mut replacemnt = String::new();

    // generate payload
    replacemnt.push_str(DELIMT_START);
    for key in table.keys() {
        replacemnt.push_str(format!("127.0.0.1 {}\n", key).as_str());
    }
    replacemnt.push_str(DELIMT_END);

    // TODO get the config file and update its contents
    let mut buffer = String::new();
    //let mut hosts = File::open("/etc/hosts")
    let mut hosts = File::open(path)
        .expect("File not found or cannot be opened");

    hosts.read_to_string(&mut buffer)
        .unwrap();

    let re = Regex::new(DELIMT_START).unwrap();
    let mut updated_content: String = String::new();

    if !re.is_match(buffer.as_str()) {
        let mut data = buffer.to_owned();
        let updatedHost = replacemnt.to_owned();
        data.push_str(&updatedHost);
        updated_content = data;
    } else {
        let re = Regex::new(r"(###(.*)##)\n(.|\n)*(###(.*)##\n)").unwrap();
        updated_content = re.replace_all(buffer.as_str(), NoExpand(replacemnt.as_str())).into_owned();
    }

    // write the content to the file
    fs::write(path, updated_content).expect("Unable to write file");
}
