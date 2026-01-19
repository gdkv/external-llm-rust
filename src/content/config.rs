extern crate serde;

use std::fs;
use std::io::Error;
use self::serde::{Deserialize};
#[derive(Deserialize, Debug)]
pub struct Config {
    pub openai_api_key: String,
    pub model_name: String
}


impl Config {
    pub fn load() -> Result<Config, Error> {
        let text = fs::read_to_string("config.json");

        match text {
            Ok(t) => Ok(serde_json::from_str(&t)?),
            Err(_) => Err(Error::other("config.json could not be opened")),
        }
    }
}