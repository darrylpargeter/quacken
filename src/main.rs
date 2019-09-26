#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate quacken;
extern crate regex;

use quacken::{ config };
use quacken::token::Token;
use std::path::PathBuf;
use rocket::response::Redirect;
use regex::Regex;

// for file reading
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;

use hotwatch::{Hotwatch, Event};

const DELIMT_START: &str = "### quacken shortcuts :start ###\n";
const DELIMT_END: &str = "### quacken shortcuts :end ###\n";

// TODO move this into its own file 
// if you can 
#[get("/<params..>")]
fn expand_params(params: PathBuf, token: Token) -> Redirect {
    redirect_to_host(token)
}

#[get("/")]
fn expand_without_params(token: Token) -> Redirect {
    redirect_to_host(token)
}

fn redirect_to_host(token: Token) -> Redirect {
// get the config
    let config = config::get_config();
// build path
    let path = token.get_path(config);
// format into url
    let fmt = format!("https://{}", path);
// redirect to the expanded url
    Redirect::to(fmt)
}

fn main() {
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    //  TODO unhard code the config.toml path
    hotwatch.watch("./config.toml", |event: Event| {
        if let Event::Write(path) = event {
            let path = "/etc/hosts";
            let config = config::get_config();
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
            if !re.is_match(buffer.as_str()) {
                // TODO write to file
                println!("{:#?}", buffer);
                let mut data = buffer.to_owned();
                let updatedHost = replacemnt.to_owned();
                data.push_str(&updatedHost);
                // fs::write("/etc/hosts", format!("{}{}", buffer, replacemnt));
                println!("data: {}", data);
                fs::write(path, data).expect("Unable to write file");
            } else {
                // TODO update file
                println!("quacken exists");
            }
            /*
            for line in buf_read.lines() {
            }
            */
        }
    }).expect("failed to watch file!");
    // start the web server with the bash path
    rocket::ignite()
        .mount("/", routes![
           expand_params,
           expand_without_params
        ])
        .launch();
}
