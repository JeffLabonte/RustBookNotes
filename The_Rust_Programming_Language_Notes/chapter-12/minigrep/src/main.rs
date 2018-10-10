use std::env;
use std::fs;
use std::io::prelude::*;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // TODO Change for a more efficient
        let filename = args[2].clone(); // TODO way

        Ok(Config{query, filename})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("There was an error: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("Filename : {}", config.filename);

    let content = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text: {}\n", content);
}
