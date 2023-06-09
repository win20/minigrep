use colored::*;
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let mut _x = 10;
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query.yellow());
    println!("In file {}", config.filename.red());
    println!("---------------");

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
