use std::fmt;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use toml::Value;

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

#[derive(Debug)]
pub struct Token{
    host: String,
    path: String,
    tokenize: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let host = request.headers().get("Host").next().unwrap();
        let path = request.uri().path();
        let token = Token::new(host.to_string(), path.to_string());

        Outcome::Success(token)
    }
}

impl Token {
    fn expand_path(token: Vec<&str>, config: toml::Value) -> String {
        let mut current = config;
        let mut path: String = String::new();

        for key in token {
            current = match current.get(key) {
                None => break,
                Some(x) => {
                    if let Some(value) = x.get("expand") {
                        // if part of the parth existis add a / for
                        // the next part of the url
                        if path.chars().count() >= 1 {
                          path.push_str("/");
                        }
                        // add to the path
                        path.push_str(value.as_str().unwrap());
                    };
                    x.clone()
                },
            };
        }

        path
    }
}

impl Token {
    fn new(host: String, path: String) -> Token {
        let tokenize = String::from(format!("{}{}", host, path));
        Token { host, path, tokenize }
    }

    pub fn get_path(&self, config: toml::Value) -> String {
        let mut path: String = String::new();
        let mut url_split: Vec<&str> = self.tokenize.split("/").collect();
        let path = Token::expand_path(url_split, config);

        path
    }
}
