use std::env;
use std::fs;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Config{
        let query = args[1].clone(); // TODO Change for a more efficient
        let filename = args[2].clone(); // TODO way

        Config{query, filename}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("Filename : {}", config.filename);

    let content = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text: {}\n", content);
}
