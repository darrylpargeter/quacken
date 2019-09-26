#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate quacken;
extern crate regex;

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
