#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate quacken;

use quacken::config;
use quacken::expand_path;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use std::path::PathBuf;

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

#[derive(Debug)]
struct Token(String);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let host = request.headers().get("Host").next().unwrap();
        let path = request.uri().path();
        let mut token = String::new();

        token.push_str(host);
        token.push_str(path);

        Outcome::Success(Token(token))
    }
}



#[get("/<params..>")]
fn index(params: PathBuf, token: Token) -> String {
    let Token(token) = token;
    println!("token: {}", token);

    let config = config::get_config();

    let path = expand_path::get_path(&token, config);
    println!("path: {}", path);
    // TODO Redirect
    String::from("YAY")
}

fn main() {
    let config = config::get_config();
    let url: &str = "g/d/p";

    let path = expand_path::get_path(url, config);
    println!("path: {}", path);

    rocket::ignite().mount("/", routes![index]).launch();
}
