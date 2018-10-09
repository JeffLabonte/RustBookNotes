use std::env;
use std::fs;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let configs = parse_config(&args);
    println!("Searching for {}", configs.query);
    println!("Filename : {}", configs.filename);

    let content = fs::read_to_string(configs.filename)
        .expect("Something went wrong reading the file");

    println!("With text: {}\n", content);
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // TODO Change for a more efficient
    let filename = args[2].clone(); // TODO way

    Config{query, filename}
}
