#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate quacken;
extern crate regex;
extern crate dirs;

use quacken::{ config };
use quacken::token::Token;
use std::path::PathBuf;
use rocket::response::Redirect;

use hotwatch::{Hotwatch, Event};

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
    println!("redirecting to: {}", path);
// format into url
    let fmt = format!("https://{}", path);
// redirect to the expanded url
    Redirect::to(fmt)
}

fn main() {
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    // get the config path
    let path = config::get_config_path();

    // at start up make sure that /etc/hosts holds the current keys
    config::update_config();

    // watch the config file
    hotwatch.watch(path, |event: Event| {
        if let Event::Write(_path) = event {
            // reload and update the config and /etc/hosts file
            config::update_config();
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
