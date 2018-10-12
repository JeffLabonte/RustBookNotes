extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("There was an error: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("Filename : {}", config.filename);
    if let Err(e) = minigrep::run(config){
        eprintln!("There was an error: {}", e);
        process::exit(1);
    }
}

