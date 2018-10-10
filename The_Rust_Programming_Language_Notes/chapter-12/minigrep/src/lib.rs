use std::fs;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // TODO Change for a more efficient
        let filename = args[2].clone(); // TODO way

        Ok(Config{query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{ // NOTE dyn (dynamic) Error ( Error trait )
    let content = fs::read_to_string(config.filename)?;

    println!("With text: {}\n", content);

    Ok(())
}
